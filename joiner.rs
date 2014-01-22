// Problem 0: join two files split with example from the tutorial part 2

use std::os;
use std::io::File;

fn main() {
	let args = os::args();
	if args.len() != 3 { fail!("Need two files to join !"); }
	
	let share1 = File::open(&Path::new(args[1].clone())).read_to_end();
	let share2 = File::open(&Path::new(args[2].clone())).read_to_end();
	for i in range(0, share1.len()) {
		print!("{}", (share1[i] ^ share2[i]) as char)
	}
	println!("");
}