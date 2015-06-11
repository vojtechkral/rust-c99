use std::process::{Command,Output};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;


fn cmd_output_checked(c: &mut Command, msg: &str) -> Output
{
	let output = c.output().unwrap_or_else(|e| { panic!("{}: {}", msg, e); });
	if !output.status.success() { panic!("{}: {}", msg, String::from_utf8(output.stderr).unwrap()); }
	output
}

fn main()
{
	let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let base_path = Path::new(&base_dir);
	let o_dir = env::var("OUT_DIR").unwrap();
	let o_path = Path::new(&o_dir);

	let src_path = PathBuf::from(base_path.join("src"));
	let c99gen = PathBuf::from(o_path.join("c99-gen"));

	cmd_output_checked(Command::new("cmake").arg(src_path).current_dir(&o_path), "Failed to run CMake");
	cmd_output_checked(Command::new("make").current_dir(&o_path), "Failed to run make");

	let gen_out = cmd_output_checked(Command::new(c99gen).current_dir(&base_path), "Failed to run generator");
	let gen_path = PathBuf::from(o_path.join("gen.rs"));
	let mut gen_f = File::create(&gen_path).unwrap();

	gen_f.write_all(&*gen_out.stdout).unwrap();
}
