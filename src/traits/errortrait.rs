use std::error::Error;

pub trait ErrorTrait
where
    Self: Error,
{
    fn external(e: Box<dyn Error>) -> Self;
    fn status_code(&self) -> u16;
}
