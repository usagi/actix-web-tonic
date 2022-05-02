use futures::future::Future;
use std::io::Error;

pub async fn actix_main(actix_future: impl Future<Output = Result<(), Error>>) -> std::io::Result<()>
{
 let fake_future = futures::future::ok::<(), ()>(());
 let r = futures::future::join(actix_future, fake_future);
 r.await
  .0
}
