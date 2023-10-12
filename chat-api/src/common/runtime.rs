use std::net::SocketAddr;

use sqlx::{
    Pool, Postgres
};

use crate::common::router;
use crate::common::database;

pub struct Runtime {
    addr: Option<SocketAddr>,
    database_connection: Option<Pool<Postgres>>,
}

type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub struct RuntimeError;

impl Runtime {
    pub fn new () -> Runtime {
        Runtime { 
            addr: None,
            database_connection: None,
        }
    }

    // todo use env vars
    pub async fn serve(&self) -> RuntimeResult<Runtime> {
        Ok(self.server().await.unwrap())
    }

    pub async fn server(&self) -> RuntimeResult<Runtime> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let database_connection = database::connect().await; 
        
        Ok(Runtime {
            addr: Some(addr), 
            database_connection: Some(database_connection),
        })
    }

    pub async fn execute(self) {
        let dbp = self.database_connection.unwrap();
        let app = router::new(dbp).await;
        let svc = app.into_make_service();
        let lst = self.addr.unwrap();

        let _ = axum::Server::bind(&lst)
            .serve(svc)
            .await;
    }    
}
