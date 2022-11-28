// error: internal compiler error: no errors encountered even though `delay_span_bug` issued
// error: internal compiler error: broken MIR

mod rpc;

use jsonrpsee::server::ServerBuilder;
use rpc::RpcServer;

#[tokio::main]
async fn main() {
    // set up server
    let serv = rpc::ServerImpl::new();
    let server = ServerBuilder::default().build("127.0.0.1:9000").await.unwrap();

    let handle = server.start(serv.into_rpc()).unwrap();
    handle.stopped().await;
}
