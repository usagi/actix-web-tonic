mod actix_impl;
mod parse_args;
mod tonic_impl;

use simple_logger::SimpleLogger;

fn main()
{
 SimpleLogger::new()
  .with_colors(true)
  .with_utc_timestamps()
  .with_level(log::LevelFilter::Info)
  .init()
  .expect("oops, failed to initialize SimpleLogger.");

 let (addr, port_actix, port_tonic) = parse_args::parse();
 log::info!("addr={addr}, port_actix={port_actix}, port_tonic={port_tonic}");

 let actix_worker_threads = (num_cpus::get_physical() * 1 / 4).max(1);
 let tonic_worker_threads = (num_cpus::get_physical() - actix_worker_threads).max(1);
 log::info!("actix_worker_threads={actix_worker_threads}, tonic_worker_threads={tonic_worker_threads}");

 // actix-web
 let actix_future = {
  use actix_web::{
   web,
   App,
   HttpServer
  };
  HttpServer::new(|| {
   App::new()
    .service(actix_impl::index)
    .route("/teapot", web::get().to(actix_impl::teapot))
  })
  .bind(format!("{addr}:{port_actix}"))
  .unwrap()
  .workers(actix_worker_threads)
  .run()
 };

 // tonic
 let tonic_future = {
  use tonic::transport::Server;
  use tonic_impl::{
   hello_world::greeter_server::GreeterServer,
   MyGreeter
  };
  let addr = format!("{addr}:{port_tonic}")
   .parse()
   .unwrap();
  let greeter = MyGreeter::default();
  Server::builder()
   .add_service(GreeterServer::new(greeter))
   .serve(addr)
 };

 // run in concurrent both of actix-web and tonic on the single bin process.🔀
 let r = actix_web_tonic::invoke(actix_future, tonic_future, tonic_worker_threads);
 log::info!("The app will be exit; The result of actix_web_tonic::invoke is ... {r:?}");
}
