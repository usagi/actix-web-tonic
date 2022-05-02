use std::io::Error as AError;
use thiserror::Error;
use tonic::transport::Error as TError;

#[derive(Error, Debug)]
pub enum ActixWebTonicError
{
 #[error("actix-web either tonic has an error; actix=`{actix:?}` tonic=`{tonic:?}`")]
 Either
 {
  actix: Result<(), AError>,
  tonic: Result<(), TError>
 }
}
