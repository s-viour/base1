use std::io::{self, Read, Write};
use std::env;
use base1;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 || args[1] != "encode" && args[1] != "decode" {
		println!("usage: {} [encode|decode]", args[0]);
	} else {
		let mut stdin = io::stdin();
		let mut stdout = io::stdout();
		let mut s = String::new();
		stdin.read_to_string(&mut s)?;

		if args[1] == "encode" {
			print!("{}", base1::encode(&s.as_bytes().to_vec()));
		} else {
			stdout.write(&base1::decode(&s))?;
		}
	}
	Ok(())
}