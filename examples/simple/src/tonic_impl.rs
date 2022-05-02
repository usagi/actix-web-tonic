// Note: This example is referenced from the official helloworld-tutorial
// https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

pub mod hello_world
{
 tonic::include_proto!("helloworld");
}

use tonic::{
 Request,
 Response,
 Status
};

use hello_world::{
 greeter_server::Greeter,
 HelloReply,
 HelloRequest
};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter
{
 async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status>
 {
  log::info!("Got a request: {:?}", request);

  let reply = hello_world::HelloReply {
   message: format!(
    "Hello {}!",
    request
     .into_inner()
     .name
   )
   .into()
  };

  Ok(Response::new(reply))
 }
}
