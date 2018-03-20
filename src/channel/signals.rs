
use super::{SignalTrait, SignalCode};

#[derive(Debug)]
pub struct SigKill;

impl SignalTrait for SigKill{
    fn code(&self) -> SignalCode{
        1
    }
}