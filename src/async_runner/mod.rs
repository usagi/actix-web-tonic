use crate::{AError, ActixWebTonicError, TError};
use futures::future::Future;
use retrieve::*;

#[mod_pub_use(runner_actix_web, runner_tonic)]
pub async fn async_main(
    actix_future: impl Future<Output = Result<(), AError>>,
    tonic_future: impl Future<Output = Result<(), TError>>,
) -> Result<(), ActixWebTonicError> {
    let r_actix = actix_main(actix_future);
    let r_tokio = tokio_main(tonic_future);

    let r = futures::future::join(r_actix, r_tokio).await;
    match r {
        (Ok(..), Ok(..)) => Ok(()),
        _ => Err(ActixWebTonicError::Either {
            actix: r.0,
            tonic: r.1,
        }),
    }
}
