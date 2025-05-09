pub mod fixed;
pub mod plasma_amm;
pub mod plasma_error;
pub mod plasma_utils;
pub type SlotWindow = u64;

pub use fixed::I80F48;
pub use plasma_error::*;
pub use plasma_utils::*;

/// Private trait for safely downcasting between types
pub(crate) trait Downcast<To> {
    fn downcast(&self) -> Result<To, PlasmaStateError>;
}

impl Downcast<u64> for u128 {
    fn downcast(&self) -> Result<u64, PlasmaStateError> {
        if *self > u64::MAX as u128 {
            Err(PlasmaStateError::Overflow)
        } else {
            Ok(*self as u64)
        }
    }
}

/// Private trait for upcasting a larger integer type
pub(crate) trait Upcast<To> {
    fn upcast(&self) -> To;
}

impl Upcast<u128> for u64 {
    fn upcast(&self) -> u128 {
        *self as u128
    }
}

impl Upcast<u128> for u32 {
    fn upcast(&self) -> u128 {
        *self as u128
    }
}
