
use process::ProcessInfo;

#[derive(Fail, Debug)]
#[fail(display = "BrokenChannel: {}", _0)]
pub struct BrokenChannel(pub ProcessInfo);

#[derive(Fail, Debug)]
#[fail(display = "Poisoned:")]
pub struct Poisoned();