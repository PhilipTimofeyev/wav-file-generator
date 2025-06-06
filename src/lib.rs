use bincode::{Encode, config};
use std::f64::consts::PI;
// use std::fs::File;
// use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
// extern crate bincode;

const SAMPLE_RATE: u32 = 44100;

#[wasm_bindgen]
pub fn create_wav(seconds: u64, freq: f64, ) -> Vec<u8> {
    // let mut file = File::create("sine.wav").unwrap();
    let mut file: Vec<u8> = Vec::new();
    // let mut file = [0u8; 100];

    let sin_buf = make_sin(seconds, freq);
    
    let riff_chunk = RiffChunk::new(sin_buf.len());
    let fmt_chunk = FmtChunk::new();
    let data_chunk = DataChunk::new(
        sin_buf,
        fmt_chunk.num_of_channels,
        fmt_chunk.bits_per_sample,
    );
    
    // let encoded: Vec<u8> = bincode::serialize(&my_struct).unwrap();
    // let a = encoder(riff_chunk, file);
    let config: config::Configuration<config::LittleEndian, config::Fixint> = config::standard().with_fixed_int_encoding();
    let mut riff_bytes = bincode::encode_to_vec(riff_chunk, config).unwrap();
    let mut fmt_bytes = bincode::encode_to_vec(fmt_chunk, config).unwrap();
    let mut data_bytes = bincode::encode_to_vec(data_chunk, config).unwrap();
    file.append(&mut riff_bytes);
    file.append(&mut fmt_bytes);
    file.append(&mut data_bytes);
    file
}

#[wasm_bindgen]
pub fn make_sin(seconds: u64, frequency: f64) -> Vec<u8> {
    let samples = seconds as usize * SAMPLE_RATE as usize;
    let mut buf = Vec::with_capacity(samples);

    for t in 0..samples {
        let s = f64::sin((2.0 * PI * frequency * t as f64) / (SAMPLE_RATE as f64));
        let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
        buf.push(s)
    }

    buf
}

#[derive(Encode)]
struct RiffChunk {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
}

impl RiffChunk {
    fn new(samples: usize) -> RiffChunk {
        let chunk_id = *b"RIFF";
        let chunk_size = 20 + samples as u32;
        let format = *b"WAVE";

        RiffChunk {
            chunk_id,
            chunk_size,
            format,
        }
    }
}

#[derive(Encode)]
struct FmtChunk {
    subchunk_1_id: [u8; 4],
    subchunk_1_size: u32,
    audio_format: u16,
    num_of_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl FmtChunk {
    fn new() -> FmtChunk {
        let subchunk_1_id = *b"fmt ";
        let subchunk_1_size: u32 = 16;
        let audio_format = 1;
        let num_of_channels = 1;
        let bits_per_sample: u16 = 8;
        let byte_rate = SAMPLE_RATE * num_of_channels as u32 * (bits_per_sample / 8) as u32;
        let block_align = num_of_channels * (bits_per_sample / 8);

        FmtChunk {
            subchunk_1_id,
            subchunk_1_size,
            audio_format,
            num_of_channels,
            sample_rate: SAMPLE_RATE,
            byte_rate,
            block_align,
            bits_per_sample,
        }
    }
}

#[derive(Encode)]
struct DataChunk {
    subchunk_2_id: [u8; 4],
    subchunk_2_size: u32,
    data: Vec<u8>,
}

impl DataChunk {
    fn new(data: Vec<u8>, num_of_channels: u16, bits_per_sample: u16) -> DataChunk {
        let subchunk_2_size =
            data.len() as u32 * num_of_channels as u32 * (bits_per_sample / 8) as u32;

        DataChunk {
            subchunk_2_id: *b"data",
            subchunk_2_size,
            data,
        }
    }
}
