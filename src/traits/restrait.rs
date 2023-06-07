use std::error::Error;

use super::ErrorTrait;

pub trait ResTrait
where
    Self: Sized,
{
    type Error: ErrorTrait + 'static;

    fn error(e: <Self as ResTrait>::Error) -> Self;

    fn status_code(&self) -> u16;

    fn from_res(res: Result<Self, Box<dyn Error>>) -> Self
    where
        Self::Error: Sized + 'static,
    {
        match res {
            Ok(res) => res,
            Err(e) => {
                let e = match e.downcast::<Self::Error>() {
                    Ok(downcasted) => *downcasted,
                    Err(e) => Self::Error::external(e),
                };

                Self::error(e)
            }
        }
    }
}
