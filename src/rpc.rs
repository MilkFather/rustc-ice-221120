use jsonrpsee::{core::{RpcResult, async_trait}, proc_macros::rpc};

#[rpc(client, server)]
pub trait Rpc {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> RpcResult<String>;
}

pub struct ServerImpl;

impl ServerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl RpcServer for ServerImpl {
    async fn hello_world(&self) -> RpcResult<String> {
        Ok("hello_world".to_string())
    }
}
