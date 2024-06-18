use serde::{Deserialize, Serialize};
use std::io::Read as _;
use xz2::read::{XzDecoder, XzEncoder};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum CompressionSchema {
    Uncompressed = 0,
    flate3_DynamicBlockLarge = 1,
    flate3_DynamicBlockSmall = 2,
    flate3_StaticBlockLarge = 3,
    flate3_StaticBlockSmall = 4,
    // starting lzma from later in case I figure out how to add more flate3 schemas later
    lzma_xz_3 = 33,
    lzma_xz_6 = 34,
}

impl CompressionSchema {
    pub fn from_id(id: u8) -> Self {
        use CompressionSchema::*;
        match id {
            0 => Uncompressed,
            1 => flate3_DynamicBlockLarge,
            2 => flate3_DynamicBlockSmall,
            3 => flate3_StaticBlockLarge,
            4 => flate3_StaticBlockSmall,
            33 => lzma_xz_3,
            34 => lzma_xz_6,
            _ => panic!("ID should be one of [0, 1, 2, 3, 4, 33, 34]"),
        }
    }

    pub fn get_all_schemas() -> [Self; 7] {
        use CompressionSchema::*;
        [
            Uncompressed,
            flate3_DynamicBlockLarge,
            flate3_DynamicBlockSmall,
            flate3_StaticBlockLarge,
            flate3_StaticBlockSmall,
            lzma_xz_3,
            lzma_xz_6,
        ]
    }

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
