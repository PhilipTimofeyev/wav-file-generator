use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

const SAMPLE_RATE: u32 = 44110;

fn main() -> std::io::Result<()> {
    let mut file = File::create("sine.wav")?;

    let sin_buf = make_sin(3, 1000.0);
    let wav_buf = make_wav(sin_buf.len());

    file.write_all(&wav_buf)?;
    file.write_all(&sin_buf)?;

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

fn make_wav(samples: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend(b"RIFF");
    buf.extend(u32::to_be_bytes(20 + samples as u32)); //WAV Chunk size
    buf.extend(b"WAVE");
    buf.extend(b"fmt ");
    buf.extend(u32::to_be_bytes(16)); // fmt chunk size
    buf.extend(u16::to_be_bytes(1)); // format code (PCM)
    buf.extend(u16::to_be_bytes(1));
    
    buf

}