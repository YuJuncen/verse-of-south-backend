use std::fmt::Debug;
use std::str::FromStr;
use vos::database::util;
use clap::{Arg, App, SubCommand,};
use std::fs::File;
use std::io::Read;

fn load_contnet(path: &str) -> std::io::Result<String> {
    Ok(load_content_of_file(File::open(path)?))
}

fn load_content_of_file(mut file: File) -> String {
    let mut ret = String::new();
    let _ = file.read_to_string(&mut ret);
    ret
}

fn run_server(port: u16, debug_level: Option<&str>, listen_to_outer: bool) {
    use vos::wrapper::actors::rootactor::RootActor;
    use actix::prelude::*;
    if let Some(debug_level) = debug_level {
        ::std::env::set_var("RUST_LOG", debug_level);
    }
    env_logger::init();
    let sys = actix::System::new("verse-of-south");
    let _ = if !listen_to_outer {
        RootActor::listen_to_localhost(port) 
    } else {
        RootActor::listen_to_all_network(port)
    }.start();
 
    sys.run();
}

fn number_valiator<T>(p: String) -> Result<(), String> where
    T: FromStr,
    <T as FromStr>::Err : Debug {
    p.parse::<T>()
        .map(|_| ())
        .map_err(|e| format!{"failed to parse: {:#?}", e})
}

fn main() {
    dotenv::dotenv().ok();
    let conn = vos::database::establish_connection();
    let matches = App::new("南方之诗 - 控制台接口")
        .version("0.1.3")
        .about("这是在拥有管理节点之前的替代方案。")
        .subcommand(SubCommand::with_name("tag")
            .about("标签相关的管理。")
            .subcommand(SubCommand::with_name("ls")
                .about("列出标签。")
            )
            .subcommand(SubCommand::with_name("add")
                .arg(Arg::with_name("NAME")
                    .short("n")
                    .long("name")
                    .takes_value(true)
                    .required(true))
            )
        )
        .subcommand(SubCommand::with_name("post")
            .about("文章相关的管理。")
            .subcommand(SubCommand::with_name("add")
                .arg(Arg::with_name("BODY_FILE")
                    .short("b")
                    .long("body_file")
                    .allow_hyphen_values(true)
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("TITLE")
                    .long("title")
                    .required(true)
                    .takes_value(true)
                ).arg(Arg::with_name("INTRO")
                    .short("i")
                    .long("intro")
                    .takes_value(true)
                ).arg(Arg::with_name("INTRO_FILE")
                    .long("intro-file")
                    .takes_value(true)
                ).arg(Arg::with_name("TAG")
                    .short("t")
                    .long("tag")
                    .validator(number_valiator::<i32>)
                    .takes_value(true)
                    .multiple(true)
                )
            )
        )
        .subcommand(SubCommand::with_name("run")
                .about("启动服务器。")
                .arg(Arg::with_name("PORT")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .validator(number_valiator::<u16>)
                    .default_value("8000"))
                .arg(Arg::with_name("LOG_LEVEL")
                    .default_value("info")
                    .long("log-level")
                    .takes_value(true))
                .arg(Arg::with_name("LISTEN_TO_ALL_NETWORK")
                    .long("listen-to-all-network")
                ))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("tag") {
        if let Some(matches) = matches.subcommand_matches("add") {
            println!("Tag added: {:?} success!", util::add_tag(&conn, matches.value_of("NAME").expect("Must provide tag name.")))
        }
        if let Some(_) = matches.subcommand_matches("ls") {
            println!("{:04}\t{:40}", "ID", "name");
            for t in util::load_tags(&conn) {
                println!("{:04}\t{:40}", t.id, t.tag_name);
            }
        }
    }
    
    if let Some(matches) = matches.subcommand_matches("post") {
        if let Some(matches) = matches.subcommand_matches("add") {
            let mut s : String;
            let title = matches.value_of("TITLE").expect("must provide title!");
            let intro = if let Some(intro) = matches.value_of("INTRO") {
                Some(intro)
            } else if let Some(intro) = matches.value_of("INTRO_FILE") {
                s = load_contnet(&intro).expect("Invaild File...");
                Some(s.as_str())
            } else {
                None
            };
            let content = load_contnet(&matches.value_of("BODY_FILE").expect("must provide body-file...")).expect("Invaild file...");
            let post = util::types::NewPost {
                title, intro, body: content.as_str()
            };
            println!("published Post: {:#?}", util::publish_post(&conn, 
                post, &matches.values_of("TAG").expect("must provide tags.").map(|i| i.parse().expect("Cannot parse tag.")).collect::<Vec<i32>>()))
        }
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        run_server(matches.value_of("PORT").map(|e| e.parse().unwrap()).expect("?????"), matches.value_of("LOG_LEVEL"), matches.is_present("LISTEN_TO_ALL_NETWORK"))
    }
}