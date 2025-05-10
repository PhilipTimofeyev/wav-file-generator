use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

const SAMPLE_RATE: u16 = 44110;

fn main() -> std::io::Result<()> {
    let mut file = File::create("sine.pcm")?;

    let buf = make_sin(3, 1000.0);

    file.write_all(&buf)?;

    Ok(())
}

fn make_sin(seconds: u64, frequency: f64) -> Vec<u8> {
    let samples = seconds as usize * SAMPLE_RATE as usize;
    let mut buf = Vec::with_capacity(samples as usize);

    for t in 0..samples {
        let s = f64::sin(2.0 * PI * frequency * t as f64 / (SAMPLE_RATE as f64));
        let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
        buf.push(s)
    }

    buf

}