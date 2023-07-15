use crate::Result;
use serde::Serialize;

mod sku;
mod tag;

pub use sku::*;
pub use tag::*;

#[derive(Serialize, Debug)]
struct IpcError {
    message: String,
}

#[derive(Serialize, Debug)]
pub struct IpcResult<D>
where
    D: Serialize,
{
    pub data: D,
}

#[derive(Serialize, Debug)]
pub struct IpcResponse<D>
where
    D: Serialize,
{
    error: Option<IpcError>,
    result: Option<IpcResult<D>>,
}

impl<D> From<Result<D>> for IpcResponse<D>
where
    D: Serialize,
{
    fn from(res: Result<D>) -> Self {
        match res {
            Ok(data) => IpcResponse {
                error: None,
                result: Some(IpcResult { data }),
            },
            Err(err) => IpcResponse {
                error: Some(IpcError {
                    message: format!("{err}"),
                }),
                result: None,
            },
        }
    }
}
