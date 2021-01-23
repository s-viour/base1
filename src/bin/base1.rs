use std::io::{self, Read, Write, BufWriter};
use std::env;
use base1;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("usage: {} [encode|decode|encodelong|size]", args[0]);
	} else {
		let mut stdin = io::stdin();
		let mut stdout = io::stdout();
		let arg = args[1].as_str();
		let mut s = String::new();
		stdin.read_to_string(&mut s)?;

		if arg == "encode" {
			print!("{}", base1::encode(&s.trim().as_bytes().to_vec()));
		} else if arg == "decode" {
			stdout.write(&base1::decode(&s.trim().to_string()))?;
		} else if arg == "encodelong" {
			let l = base1::encode_l(&s.trim().as_bytes().to_vec());
			let mut buffer = BufWriter::new(stdout);
			for c in base1::Base1Producer::new(l) {
				buffer.write(&[c as u8]).unwrap();
			}
		} else if arg == "size" {
			let num = base1::encode_l(&s.trim().as_bytes().to_vec());
			println!("{}", num)

		} else {
			println!("unknown command {}", arg);
		}
	}
	Ok(())
}