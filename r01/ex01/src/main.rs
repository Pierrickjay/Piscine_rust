fn main() {
	let b;
	{ // need to comment this and the other one to compil
		let a: i32 = 0;
		b = &a;
	} // cannot acces the adress of a thing that is going to be destroy
	println!("{b}");
}
