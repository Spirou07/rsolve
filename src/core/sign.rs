use std::ops::Neg;
#[derive(Clone, Copy, Debug)]
/// This enum trivially encapsulates the polarity (aka the sign) of a boolean variable
pub enum Sign { Positive, Negative }

impl Neg for Sign {
    type Output = Sign;
    fn neg(self) -> Sign {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}