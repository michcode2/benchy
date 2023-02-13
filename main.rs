extern crate mandelbrot;
use std::time::{Instant, SystemTime};
use std::fs::OpenOptions;
use rug::Float;
use std::io::Write;
use dirs::home_dir;
use std::fs::File;

fn main() {
	let parameter = &mandelbrot::Parameters{
		zoom: Float::with_val(53, 19632494358.519295),
        low_x: Float::with_val(53, -4.4557090418709151e-2),
        low_y: Float::with_val(53, -9.8323872835975501e-1),
        radius_x: Float::with_val(53, 8.6844066927987583e-9),
        radius_y: Float::with_val(53, 8.6844066927987583e-9),
        quality: 600,
        bound: 750.0_f64.powf(2.0),
    	};

	let mut path = home_dir().unwrap();
	path.push("benchy");
	
	let mut out_file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(path.as_path())
		.unwrap();
	
	writeln!(out_file, "\n").unwrap();
	
	loop {
		run_and_write(parameter, &mut out_file);
	}
}

fn run_and_write(params: &mandelbrot::Parameters, file: &mut File) {
	let now = Instant::now();

	mandelbrot::int_calculate(params, 53);

	let duration = now.elapsed();
	
	writeln!(file, "run at {:?}, result of {:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(), duration).unwrap();
	println!("{:?}", duration);
}
