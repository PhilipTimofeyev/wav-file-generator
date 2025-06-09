use bincode::Encode;

#[derive(Encode)]
pub struct RiffChunk {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
}

impl RiffChunk {
    pub fn new(samples: usize) -> RiffChunk {
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
pub struct FmtChunk {
    subchunk_1_id: [u8; 4],
    subchunk_1_size: u32,
    audio_format: u16,
    pub num_of_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    pub bits_per_sample: u16,
}

impl FmtChunk {
    pub fn new(sample_rate: u32) -> FmtChunk {
        let subchunk_1_id = *b"fmt ";
        let subchunk_1_size: u32 = 16;
        let audio_format = 1;
        let num_of_channels = 1;
        let bits_per_sample: u16 = 8;
        let byte_rate = sample_rate * num_of_channels as u32 * (bits_per_sample / 8) as u32;
        let block_align = num_of_channels * (bits_per_sample / 8);

        FmtChunk {
            subchunk_1_id,
            subchunk_1_size,
            audio_format,
            num_of_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
        }
    }
}

#[derive(Encode)]
pub struct DataChunk {
    subchunk_2_id: [u8; 4],
    subchunk_2_size: u32,
    data: Vec<u8>,
}

impl DataChunk {
    pub fn new(data: Vec<u8>, num_of_channels: u16, bits_per_sample: u16) -> DataChunk {
        let subchunk_2_size =
            data.len() as u32 * num_of_channels as u32 * (bits_per_sample / 8) as u32;

        DataChunk {
            subchunk_2_id: *b"data",
            subchunk_2_size,
            data,
        }
    }
}