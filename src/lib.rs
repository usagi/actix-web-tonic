use futures::future::Future;
use retrieve::*;
use std::io::Error as AError;
use tonic::transport::Error as TError;

#[r#mod(async_runner)]
#[mod_pub_use(error)]
pub fn invoke(
 actix_future: impl Future<Output = Result<(), AError>>,
 tonic_future: impl Future<Output = Result<(), TError>>,
 tokio_worker_threads: usize
) -> Result<(), ActixWebTonicError>
{
 log::info!("actix-web and tonic futures will be start.");
 log::info!("(Note) Use **SIGINT(CTRL+C)** if you should stop the app.");

 actix_web::rt::System::with_tokio_rt(|| {
  tokio::runtime::Builder::new_multi_thread()
   .enable_all()
   .worker_threads(tokio_worker_threads)
   .thread_name("unaf::main::tokio")
   .build()
   .unwrap()
 })
 .block_on(async_runner::async_main(actix_future, tonic_future))?;

 log::info!("actix-web and tonic futures has been terminated.");
 Ok(())
}
