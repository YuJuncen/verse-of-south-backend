pub mod handlers;
pub mod models;

use serde::ser::{ Serialize, Serializer, SerializeSeq };
use crate::wrapper::actors::pgdatabase::PGDatabase;
use std::ops::Deref;
use std::hash::Hash;

pub mod middlewares {
    use actix_web::*;
    use actix_web::middleware::{ Middleware, Started };
    use futures::future::*;
    use super::errors::RecaptchaError;
    pub struct CommentFilter(pub actix::Addr<client::ClientConnector>, /* The secret of recaptcha. */pub String);


    #[derive(Deserialize, Debug)]
    struct Recaptcha {
        recaptcha: String
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="kebab-case")]
    struct RecaptchaResult {
        success: bool,
        hostname: Option<String>,
        error_codes: Option<Vec<String>>,
    }

    impl <S: 'static> Middleware<S> for CommentFilter {
        fn start(&self, req: &HttpRequest<S>) -> Result<Started> { 
            /*
            在这个中间件中，假如说要获得来自 HTTP payload 的 JSON 中的某一个域，然后悄悄地还回去，该怎么做呢？
            json 方法要求 SerializeOwned，因为 actix 似乎是直接将 HTTP 的整个 payload 给 take 走了……
            大概是这么一回事：
                json 调用 JsonBody::new；
                JsonBody::new 调用 <req as HttpMessage>.payload()；
                HttpRequest 的 payload() 则会获得 self.request.inner.payload。
            然后 payload 可能是 Stream...
            tokio 的 Stream 正好似乎又不能多播……
            看起来，在 actix 1.0 的 alpha 版本中解决了这个问题（使用一个叫 Serivice 的新东西）。
            但是，1.0 做了一些大改（比如似乎中间件直接给砍掉了，而且我还找不到文档……）现在这个版本迁移过去的工作量相当之大。
            现在只想到了两个方法：
                1. 将 recaptcha 的代码放到 HTTP 头[或者 Query 中去]（我们究竟是不是在使用 HTTP 提供的服务？）。
                2. 内联这个中间件。
            将它放到 query 中去！
            BTW: 过一段时间，我可能会尝试用更加……简单易上手的 Actor 系统（akka 或者 vert.x 一类的）来重写这个后台，虽然看上去就像吃饱了撑的。
            不过大概得等我有啥新追求了。（比如……CI（为啥一个人的项目会需要这个？）或者 DDD（为啥这种规模的项目要这个？）？）
            （还不如跟着 z10 的脚步，去上 CS140，用 rust 写一个操作系统不是酷多了吗……那就是掌控内存的快感。）
             */
            let recaptcha_url = "https://www.recaptcha.net/recaptcha/api/siteverify";
            let q = req.query();
            let r = q.get("recaptcha");
            debug!("QUERY: {:?}", q);
            if r.is_none() {
                return Ok(Started::Response(RecaptchaError::RequestNoRecaptchaField.error_response()));
            }

            Ok(Started::Future(Box::new(
                client::post(recaptcha_url)
                .with_connector(self.0.clone())
                .form(
                    json!({
                        "secret": self.1,
                        "response": r.unwrap(),
                        "remoteip": req.peer_addr().map(|ip| format!("{}", ip)).unwrap_or(String::from("null"))
                    })
                ).expect("Failed to build the request!")
                .send()
                .map_err(|_| RecaptchaError::FailedToConnectToGoogle)
                .and_then(|res|{
                    res.json::<RecaptchaResult>()
                    .map_err(|e| {
                        warn!("GOOGLE RESPONSE WITH ERROR: {:?}", e);
                        RecaptchaError::RecaptchaServerBadResponse
                    })
                    .and_then(|jres| {
                        debug!("GET RESULT FROM GOOGLE: {:?}", jres);
                        if jres.success { ok(()) } else {err(RecaptchaError::FailedToVeryify)}
                })})
            .map(|_| None)
            .from_err())
            ))
        }
    }
}

pub mod errors {
    use actix_web::*;
    use crate::wrapper::actors::pgdatabase::DatabaseError;
    use std::fmt::{ Display, Formatter };

    #[derive(Fail, Debug)]
    #[fail(display = "fail to authorize.")]
    pub struct Unauthorized;

    #[derive(Fail, Debug, Serialize, Clone, Copy)]
    #[fail(display = "failed to pass the receptcha.")]
    #[serde(tag="reason", content="info")]
    pub enum RecaptchaError {
        FailedToVeryify,
        FailedToConnectToGoogle,
        RecaptchaServerBadResponse,
        RequestNoRecaptchaField
    }

    impl Display for RecaptchaError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "RECAPTCHA ERROR: {:?}.", *self)
        }
    }
    
    impl error::ResponseError for Unauthorized {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::new(http::StatusCode::FORBIDDEN)
        }
    }

    impl error::ResponseError for RecaptchaError {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(http::StatusCode::FORBIDDEN)
                .json(self.clone())
        }
    }

    impl error::ResponseError for DatabaseError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                DatabaseError::DieselGoesWrong(diesel::result::Error::NotFound) => HttpResponse::build(http::StatusCode::NOT_FOUND).json(self),
                _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).json(self)
            }
        }
    }
}

pub struct AppState {
    pub database: actix::Addr<PGDatabase>
}

pub fn deserizlize_pointer<T: Serialize, S: Serializer, P: Deref<Target=T>>(p: &P,  s: S) -> Result<S::Ok, S::Error> {
    p.serialize(s)
}

pub fn serialize_real_to_integer<S: Serializer>(p: &f64, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_i64(*p as i64)
}

pub fn serialize_vec_of_pointer<
    T: Serialize, 
    S: Serializer, 
    P: Deref<Target=T>, 
    >(ps: &Vec<P>, s: S) -> Result<S::Ok, S::Error> {
    let i = ps.iter();
    let mut seq = s.serialize_seq(i.size_hint().1)?;
    for ele in i {
        seq.serialize_element(&**ele)?;
    }
    seq.end()
}


pub fn serialize_set_of_pointer<
    T: Serialize, 
    S: Serializer, 
    P: Deref<Target=T> + Eq + Hash, 
    >(ps: &std::collections::HashSet<P>, s: S) -> Result<S::Ok, S::Error> {
    let i = ps.iter();
    let mut seq = s.serialize_seq(i.size_hint().1)?;
    for ele in i {
        seq.serialize_element(&**ele)?;
    }
    seq.end()
}