use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest, OpReq, OpRsp,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use anyhow::Result;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn exe_plus(&self, req: Request<OpReq>) -> std::result::Result<Response<OpRsp>, Status> {
        let reply = OpRsp {
            res: req.get_ref().op_one + req.get_ref().op_two,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;
    let greeter = MyGreeter::default();

    println!("service bootup with {}", addr.to_string());
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
