//
// asset binaries models and utils
//

use std::{path::PathBuf, io::{Read, BufReader, BufRead, Seek, Write}};
use anyhow::{Result, bail};

pub const ASSET_FILE_MAGIC: u64 = 0x3A68783A72673A01;

pub struct AssetFileDescriptor {
    pub file_path: PathBuf,
    pub contents: Vec<ChunkDescriptor>,
}

pub struct AssetFileHeader {
    pub toc_length: u32,
}

pub struct AssetFileTOC {
    pub contents: Vec<ChunkDescriptor>,
}

pub struct ChunkDescriptor {
    pub chunk_type: ChunkType,
    pub name: String,
    pub data_start: u32,
    pub data_len: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ChunkType {
    Audio = 32,
    ModelBinding = 128,
    Model = 1,
    Shader = 16,
    Texture = 2,
    Empty = 17,
}

impl ChunkType {
    fn from_u8(chunk_type: u8) -> Result<Self> {
        println!("{chunk_type}");
        match chunk_type {
            32 => Ok(ChunkType::Audio),
            17 => Ok(ChunkType::Empty),
            2 => Ok(ChunkType::Texture),
            16 => Ok(ChunkType::Shader),
            1 => Ok(ChunkType::Model),
            128 => Ok(ChunkType::ModelBinding),
            _ => Err(anyhow::anyhow!("Invalid chunk type"))
        }
    }
}

impl AssetFileDescriptor {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        use bytestream::*;

        let mut reader = BufReader::new(std::fs::File::open(path.clone())?);

        // Header
        let magic = u64::read_from(&mut reader, ByteOrder::BigEndian)?;
        if magic != ASSET_FILE_MAGIC { bail!("Incorrect file magic"); }
        let toc_length = u32::read_from(&mut reader, ByteOrder::LittleEndian)?;

        // Table of Contents
        let mut chunk_descriptors = Vec::new();
        let mut read_toc_bytes = 0x0;
        loop {
            let chunk_type = u8::read_from(&mut reader, ByteOrder::LittleEndian)?;
            let _ = u8::read_from(&mut reader, ByteOrder::LittleEndian)?;
            let mut name = Vec::new();
            let name_len = reader.read_until(0x00, &mut name)?;
            name.pop();
            let name = String::from_utf8(name)?;
            let data_start = u32::read_from(&mut reader, ByteOrder::LittleEndian)?;
            let data_len = u32::read_from(&mut reader, ByteOrder::LittleEndian)?;
            let _ = u32::read_from(&mut reader, ByteOrder::LittleEndian)?;
            read_toc_bytes += name_len as u32 + 4 + 4 + 4 + 1 + 1;
            chunk_descriptors.push(ChunkDescriptor {
                chunk_type: ChunkType::from_u8(chunk_type)?,
                name,
                data_start,
                data_len
            });
            if read_toc_bytes >= toc_length - 14 { break; }
        }
        
        Ok(AssetFileDescriptor {
            file_path: path,
            contents: chunk_descriptors
        })
    }

    pub fn read_chunk(&self, descriptor: &ChunkDescriptor) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(std::fs::File::open(self.file_path.clone())?);
        reader.seek(std::io::SeekFrom::Start(descriptor.data_start as u64))?;
        let mut data = Vec::new();
        data.resize(descriptor.data_len as usize, 0);
        reader.read_exact(&mut data)?;
        Ok(data)
    }

    pub fn save_chunk_to_file(&self, descriptor: &ChunkDescriptor, mut save_path: PathBuf) -> Result<()> {
        let data = self.read_chunk(descriptor)?;
        if save_path.is_dir() {
            let mut extension = match &descriptor.chunk_type {
                ChunkType::Audio => ".wav",
                ChunkType::ModelBinding => ".txt",
                ChunkType::Shader => ".glsl",
                ChunkType::Texture => ".png",
                ChunkType::Model => ".obj",
                _ => "",
            };

            if descriptor.chunk_type == ChunkType::Audio && descriptor.name == "loudness" {
                extension = ".ini";
            }

            let mut file_name = descriptor.name.clone();
            file_name.push_str(extension);
            save_path.push(file_name);
        }
        
        std::fs::write(save_path, data)?;

        Ok(())
    }

    pub fn unpack_to_folder(&self, save_dir: PathBuf) -> Result<()> {
        if !save_dir.exists() { std::fs::create_dir(save_dir.clone())?; }
        if !save_dir.is_dir() { bail!("Path is not a directory"); }

        let mut reader = BufReader::new(std::fs::File::open(self.file_path.clone())?);

        for descriptor in &self.contents {
            reader.seek(std::io::SeekFrom::Start(descriptor.data_start as u64))?;
            let mut data = Vec::new();
            data.resize(descriptor.data_len as usize, 0);
            reader.read_exact(&mut data)?;
            let mut extension = match &descriptor.chunk_type {
                ChunkType::Audio => ".wav",
                ChunkType::ModelBinding => ".txt",
                ChunkType::Shader => ".glsl",
                ChunkType::Texture => ".png",
                ChunkType::Model => ".obj",
                _ => "",
            };

            if descriptor.chunk_type == ChunkType::Audio && descriptor.name == "loudness" {
                extension = ".ini";
            }

            let mut file_name = descriptor.name.clone();
            file_name.push_str(extension);
            let mut file_save_path = save_dir.clone();
            file_save_path.push(file_name);
            std::fs::write(file_save_path, data)?;
        }

        Ok(())
    }
}

pub trait AssetChunk {
    fn encode<T: Read>(input: Vec<T>) -> Result<T>;
    fn decode<T: Read>(input: T) -> Result<Vec<T>>;
}

pub struct AudioChunk;
pub struct ModelBindingChunk;
pub struct ModelChunk;
pub struct ShaderChunk;
pub struct TextureChunk;
pub struct EmptyChunk;

impl AssetChunk for AudioChunk {
    fn encode<T: Read>(mut input: Vec<T>) -> Result<T> {
        // Just copy the bytes from input to output
        Ok(input.pop().expect("empty audio asset vector"))
    }

    fn decode<T: Read>(input: T) -> Result<Vec<T>> {
        // Just copy the bytes from input to output
        Ok(vec![input])
    }
}

impl AssetChunk for ModelBindingChunk {
    fn encode<T: Read>(mut input: Vec<T>) -> Result<T> {
        // Just copy the bytes from input to output
        Ok(input.pop().expect("empty model binding asset vector"))
    }

    fn decode<T: Read>(input: T) -> Result<Vec<T>> {
        // Just copy the bytes from input to output
        Ok(vec![input])
    }
}

fn get_buffer_sizes(width: &u32, height: &u32, mipmap_count: &u8) -> (u32, Vec<u32>) {
    let mut total_byte_len = width * height * 4;
    let mut mipmap_buf_sizes = Vec::with_capacity(*mipmap_count as usize);
    mipmap_buf_sizes[0] = total_byte_len;
    if width != height {
        let mut lmod = total_byte_len;
        for i in 1..(*mipmap_count as usize) {
            lmod /= 4;
            mipmap_buf_sizes[i] = lmod;
            total_byte_len += lmod;
        }
    } else {
        let mut lmod = *width;
        for i in 1..(*mipmap_count as usize) {
            lmod /= 2;
            let mipmap_size = lmod * lmod * 4; 
            mipmap_buf_sizes[i] = mipmap_size;
            total_byte_len += mipmap_size;
        } 
    }
    (total_byte_len, mipmap_buf_sizes)
}

impl AssetChunk for TextureChunk {
    fn encode<T: Read>(mut input: Vec<T>) -> Result<T> {
        Ok(input.pop().expect("empty model binding asset vector"))
    }

    fn decode<T: Read>(mut input: T) -> Result<Vec<T>> {
        use bytestream::*;

        // Read Header
        let _ = u16::read_from(&mut input, ByteOrder::LittleEndian)?;
        let w = u32::read_from(&mut input, ByteOrder::LittleEndian)?;
        let h = u32::read_from(&mut input, ByteOrder::LittleEndian)?;
        let mipmap_count = u8::read_from(&mut input, ByteOrder::LittleEndian)?;

        let (total_size, mipmap_sizes) = get_buffer_sizes(&w, &h, &mipmap_count);

        // Create Image from pixel data
        
        Ok(vec![input])
    }
}













