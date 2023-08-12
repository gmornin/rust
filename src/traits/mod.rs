#[cfg(feature = "restrait")]
mod errortrait;
#[cfg(feature = "restrait")]
mod restrait;

#[cfg(feature = "restrait")]
pub use errortrait::ErrorTrait;
#[cfg(feature = "restrait")]
#[cfg(feature = "restrait")]
pub use restrait::ResTrait;
mod taskres;
pub use taskres::*;
