//! The variables and literals are implemented as plain integers
#[allow(non_camel_case_types)]
pub type uint     = usize;
#[allow(non_camel_case_types)]
pub type iint     = isize;

pub type Variable = uint;
pub type Literal  = iint;

pub mod arrays;
pub mod branching;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
