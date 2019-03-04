use subcrate::macros::foo;

pub fn call_foo() {
	foo!();
}
#[cfg(test)]
mod tests {
use super::*;    

#[test]
    fn it_works() {
	call_foo();
    }
}
