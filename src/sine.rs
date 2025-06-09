use std::f64::consts::PI;

pub fn create_sin(seconds: u64, frequency: f64, sample_rate: u32) -> Vec<u8> {
    let samples = seconds as usize * sample_rate as usize;
    let mut buf = Vec::with_capacity(samples);

    for t in 0..samples {
        let s = f64::sin((2.0 * PI * frequency * t as f64) / (sample_rate as f64));
        let s = f64::floor(127.5 * (s + 1.0)) as u8;
        buf.push(s)
    }

    buf
}