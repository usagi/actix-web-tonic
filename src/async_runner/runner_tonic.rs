use futures::future::Future;
use to_unit::ToUnit;
use tonic::transport::Error;

/// Pseudo-#[tokio::main] specialized for tonic
/// with SIGINT(CTRL+C) aborter injection.
pub async fn tokio_main(
    tonic_future: impl Future<Output = Result<(), Error>>,
) -> Result<(), Error> {
    let (f_tonic, aborter) = futures::future::abortable(tonic_future);

    // inject the SIGINT(=kill=CTRL+C) waiting to the tonic future
    let f_sigint = async move {
        tokio::signal::ctrl_c().await.to_unit();
        aborter.abort();
    };

    let r = futures::future::join(f_tonic, f_sigint).await;
    match r.0 {
        Ok(Err(e_tonic)) => Err(e_tonic),
        _ => Ok(()),
    }
}
