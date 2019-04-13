use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::query_dsl::methods::LoadQuery;
use actix::prelude::*;

pub struct Database<C: Connection> {
    connection: C
}

impl<C: Connection> Database<C> {
    pub fn new(connection: C) -> Database<C> {
        Database { connection }
    }
}

pub struct LoadByDsl<U, C: Connection, Dsl: RunQueryDsl<C> + LoadQuery<C, U>> {
    dsl: Dsl,
    ghost_conn: std::marker::PhantomData<C>,
    ghost_rtype: std::marker::PhantomData<U>,
}

pub struct FirstByDsl<U, C: Connection, Dsl: RunQueryDsl<C> + LoadQuery<C, U>> {
    dsl: Dsl,
    ghost_conn: std::marker::PhantomData<C>,
    ghost_rtype: std::marker::PhantomData<U>,
}

impl<U, C: Connection, Dsl: RunQueryDsl<C> + LoadQuery<C, U>> LoadByDsl<U, C, Dsl> {
    pub fn new(dsl: Dsl) -> LoadByDsl<U, C, Dsl> {
        LoadByDsl {
            dsl,
            ghost_conn: std::marker::PhantomData,
            ghost_rtype: std::marker::PhantomData,
        }
    }
}

impl<U, C: Connection, Dsl: RunQueryDsl<C> + LoadQuery<C, U>> FirstByDsl<U, C, Dsl> {
    pub fn new(dsl: Dsl) -> FirstByDsl<U, C, Dsl> {
        FirstByDsl {
            dsl,
            ghost_conn: std::marker::PhantomData,
            ghost_rtype: std::marker::PhantomData,
        }
    }
}

impl<U, C, Dsl> Message for LoadByDsl<U, C, Dsl> where
    C: Connection + 'static, 
    Dsl: RunQueryDsl<C> + LoadQuery<C, U> {
    type Result = QueryResult<Vec<U>>;
}

impl<U, C, Dsl> Message for FirstByDsl<U, C, Dsl> where
    U: 'static,
    C: Connection + 'static, 
    Dsl: RunQueryDsl<C> + LoadQuery<C, U> {
    type Result = QueryResult<U>;
}


impl<C: Connection + 'static> Actor for Database<C> {
    type Context = SyncContext<Self>;
}

impl<U, C, Dsl> Handler<LoadByDsl<U, C, Dsl>> for Database<C> where
    U: 'static,
    C: Connection + 'static, 
    Dsl: RunQueryDsl<C> + LoadQuery<C, U> {
    type Result = QueryResult<Vec<U>>;
    fn handle(&mut self, msg: LoadByDsl<U, C, Dsl>, _ctx: &mut Self::Context) -> Self::Result {
        msg.dsl.load::<U>(&self.connection)
    }
}


impl<U, C, Dsl> Handler<FirstByDsl<U, C, Dsl>> for Database<C> where
    U: 'static,
    C: Connection + 'static, 
    Dsl: RunQueryDsl<C> + LoadQuery<C, U> {
    type Result = QueryResult<U>;
    fn handle(&mut self, msg: FirstByDsl<U, C, Dsl>, _ctx: &mut Self::Context) -> Self::Result {
        msg.dsl.get_result::<U>(&self.connection)
    }
}