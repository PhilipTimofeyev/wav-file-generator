use wasm_bindgen::prelude::*;
use bincode::config;

mod sine;
mod wav;
use wav::{RiffChunk, FmtChunk, DataChunk};

const SAMPLE_RATE: u32 = 44100;

#[wasm_bindgen]
pub fn build_wav(seconds: u64, freq: f64, ) -> Vec<u8> {
    let mut file_buffer: Vec<u8> = Vec::new();

    let sin_data = sine::create_sin(seconds, freq, SAMPLE_RATE);
    
    let riff_chunk = RiffChunk::new(sin_data.len());
    let fmt_chunk = FmtChunk::new(SAMPLE_RATE);
    let data_chunk = DataChunk::new(
        sin_data,
        fmt_chunk.num_of_channels,
        fmt_chunk.bits_per_sample,
    );
    
    let config = config::standard().with_fixed_int_encoding();

    bincode::encode_into_std_write(riff_chunk, &mut file_buffer, config).unwrap();
    bincode::encode_into_std_write(fmt_chunk,  &mut file_buffer, config).unwrap();
    bincode::encode_into_std_write(data_chunk,  &mut file_buffer, config).unwrap();

    file_buffer
}

