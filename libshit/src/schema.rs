use serde::{Deserialize, Serialize};
use std::io::Read as _;
use xz2::read::{XzDecoder, XzEncoder};

#[derive(Serialize, Deserialize)]
pub enum CompressionSchema {
    Uncompressed,
    flate3_DynamicBlockLarge,
    flate3_DynamicBlockSmall,
    flate3_StaticBlockLarge,
    flate3_StaticBlockSmall,
    lzma_xz_3,
    lzma_xz_6,
}

impl CompressionSchema {
    pub fn schema_string(&self) -> &str {
        use CompressionSchema::*;
        match self {
            Uncompressed => "Uncompressed",
            flate3_DynamicBlockLarge => "flate3_DynamicBlockLarge",
            flate3_DynamicBlockSmall => "flate3_DynamicBlockSmall",
            flate3_StaticBlockLarge => "flate3_StaticBlockLarge",
            flate3_StaticBlockSmall => "flate3_StaticBlockSmall",
            lzma_xz_3 => "lzma/xz_lv3",
            lzma_xz_6 => "lzma/xz_lv6",
        }
    }

    pub fn compress(&self, data: &[u8]) -> Vec<u8> {
        use CompressionSchema::*;
        match self {
            Uncompressed => data.to_vec(),
            flate3_DynamicBlockLarge => Self::compress_flate3(true, 0x8000, data),
            flate3_DynamicBlockSmall => Self::compress_flate3(true, 0x2000, data),
            flate3_StaticBlockLarge => Self::compress_flate3(false, 0x8000, data),
            flate3_StaticBlockSmall => Self::compress_flate3(false, 0x2000, data),
            lzma_xz_3 => {
                let mut out = vec![];
                XzEncoder::new(data, 3).read_to_end(&mut out);
                out
            }
            lzma_xz_6 => {
                let mut out = vec![];
                XzEncoder::new(data, 6).read_to_end(&mut out);
                out
            }
        }
    }

    fn compress_flate3(dynamic: bool, block_size: usize, data: &[u8]) -> Vec<u8> {
        flate3::Compressor {
            options: flate3::Options {
                dynamic_block_size: dynamic,
                block_size,
                matching: true,
                probe_max: 100,
                lazy_match: true,
                match_channel_size: 100,
            },
        }
        .deflate(data)
    }

    pub fn decompress(&self, data: &[u8]) -> Vec<u8> {
        use CompressionSchema::*;
        match self {
            Uncompressed => data.to_vec(),
            flate3_DynamicBlockLarge
            | flate3_DynamicBlockSmall
            | flate3_StaticBlockLarge
            | flate3_StaticBlockSmall => flate3::inflate(data),
            lzma_xz_6 | lzma_xz_3 => {
                let mut out = vec![];
                XzDecoder::new(data).read_to_end(&mut out);
                out
            }
        }
    }
}
