use std::fs::File;
use std::env;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let seconds: u64 = args[1].parse().unwrap();
    let frequency: f64 = args[2].parse().unwrap();
    
    let filename = format!("{}sec {}Hz sine.wav", seconds, frequency);
    let mut file = File::create(filename)?;

    let bytes = wav_file_generator::build_wav(seconds, frequency);

    file.write_all(&bytes)?;
    Ok(())
}