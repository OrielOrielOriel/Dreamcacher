use std::fs;
use std::io;

fn main() -> io::Result<()> {

	let contents: String = fs::read_to_string("/root/.bash_history")?;
	println!("{}", contents);
	
	Ok(())
}
