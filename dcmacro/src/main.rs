#[macro_use]
extern crate dcmacro;

fn main() {
	let a = add!(1,2,3);
	println!("{}", a);
}
