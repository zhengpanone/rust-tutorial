use proto::hello::greeter_service_server::GreeterService;
use proto::hello::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

// gRPC adapter
#[derive(Default)]
pub struct GrpcHelloService;


#[tonic::async_trait]
impl GreeterService for GrpcHelloService {
    // 简单 RPC
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        info!("Received SayHello request: {:?}", request);

        let req = request.into_inner();
        let reply = HelloReply {
            message: format!("Hello, {}!", req.name),
            timestamp: None,
            status: proto::common::Status::Success as i32,
        };

        Ok(Response::new(reply))
    }

    async fn lot_of_replies(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(HelloReply::default()))
    }

    async fn lot_of_greetings(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(HelloReply::default()))
    }

    async fn bid_hello(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(HelloReply::default()))
    }
}
