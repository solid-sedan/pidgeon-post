use crate::common::runtime::Runtime;

mod common;
mod controller;
mod handler;
mod model;

#[derive(Debug)]
struct AppError(anyhow::Error);

#[tokio::main]
async fn main() -> Result<(), AppError> {
    match Runtime::new().serve().await {
        Ok(x) => {
            x.execute().await;
            Ok(())
        }
        Err(_e) => Ok(()),
    }
}
