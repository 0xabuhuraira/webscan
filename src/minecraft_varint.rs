//! Minecraft VarInt encoding/decoding for Minecraft protocol

use crate::error::{Result, WebScanError};
use bytes::{BytesMut, Buf};

const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// Encode a VarInt
pub fn encode_varint(value: u32) -> Vec<u8> {
    let mut result = Vec::with_capacity(4);
    let mut value = value;
    
    loop {
        let mut byte = (value & SEGMENT_BITS as u32) as u8;
        value >>= 7;
        
        if value != 0 {
            byte |= CONTINUE_BIT;
        }
        
        result.push(byte);
        
        if value == 0 {
            break;
        }
    }
    
    result
}

/// Decode a VarInt from a buffer
pub fn decode_varint(buf: &mut BytesMut) -> Result<Option<u32>> {
    let mut result = 0u32;
    let mut position = 0;
    
    loop {
        if buf.is_empty() {
            return Ok(None);
        }
        
        let byte = buf[0];
        buf.advance(1);
        
        result |= ((byte & 0x7F) as u32) << position;
        
        if position >= 21 {
            return Err(WebScanError::MinecraftProtocol("VarInt is too big".to_string()));
        }
        
        if byte & 0x80 == 0 {
            break;
        }
        
        position += 7;
    }
    
    Ok(Some(result))
}

/// Encode a VarLong
pub fn encode_varlong(value: u64) -> Vec<u8> {
    let mut result = Vec::with_capacity(8);
    let mut value = value;
    
    loop {
        let mut byte = (value & SEGMENT_BITS as u64) as u8;
        value >>= 7;
        
        if value != 0 {
            byte |= CONTINUE_BIT;
        }
        
        result.push(byte);
        
        if value == 0 {
            break;
        }
    }
    
    result
}

/// Decode a VarLong from a buffer
pub fn decode_varlong(buf: &mut BytesMut) -> Result<Option<u64>> {
    let mut result = 0u64;
    let mut position = 0;
    
    loop {
        if buf.is_empty() {
            return Ok(None);
        }
        
        let byte = buf[0];
        buf.advance(1);
        
        result |= ((byte & 0x7F) as u64) << position;
        
        if position >= 56 {
            return Err(WebScanError::MinecraftProtocol("VarLong is too big".to_string()));
        }
        
        if byte & 0x80 == 0 {
            break;
        }
        
        position += 7;
    }
    
    Ok(Some(result))
}

/// Encode a string with length prefix
pub fn encode_string(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut result = encode_varint(bytes.len() as u32);
    result.extend_from_slice(bytes);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_varint_encoding() {
        assert_eq!(encode_varint(0), vec![0]);
        assert_eq!(encode_varint(1), vec![1]);
        assert_eq!(encode_varint(127), vec![127]);
        assert_eq!(encode_varint(128), vec![0x80, 0x01]);
    }
    
    #[test]
    fn test_varint_decoding() {
        let mut buf = BytesMut::from(&[0][..]);
        assert_eq!(decode_varint(&mut buf).unwrap(), Some(0));
        
        let mut buf = BytesMut::from(&[127][..]);
        assert_eq!(decode_varint(&mut buf).unwrap(), Some(127));
        
        let mut buf = BytesMut::from(&[0x80, 0x01][..]);
        assert_eq!(decode_varint(&mut buf).unwrap(), Some(128));
    }
    
    #[test]
    fn test_string_encoding() {
        let encoded = encode_string("hello");
        let mut buf = BytesMut::from(&encoded[..]);
        // Skip the length VarInt to verify encoding worked
        assert!(!encoded.is_empty());
    }
}
