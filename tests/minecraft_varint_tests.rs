#[cfg(test)]
mod tests {
    use webscan::minecraft_varint::{encode_varint, decode_varint, encode_string, decode_string};
    use bytes::BytesMut;

    #[test]
    fn test_varint_zero() {
        assert_eq!(encode_varint(0), vec![0]);
    }

    #[test]
    fn test_varint_one() {
        assert_eq!(encode_varint(1), vec![1]);
    }

    #[test]
    fn test_varint_127() {
        assert_eq!(encode_varint(127), vec![127]);
    }

    #[test]
    fn test_varint_128() {
        assert_eq!(encode_varint(128), vec![0x80, 0x01]);
    }

    #[test]
    fn test_varint_large() {
        let encoded = encode_varint(2097151);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_varint_decode_zero() {
        let mut buf = BytesMut::from(&[0][..]);
        let result = decode_varint(&mut buf).unwrap();
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_varint_decode_one() {
        let mut buf = BytesMut::from(&[1][..]);
        let result = decode_varint(&mut buf).unwrap();
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_varint_decode_127() {
        let mut buf = BytesMut::from(&[127][..]);
        let result = decode_varint(&mut buf).unwrap();
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_varint_decode_128() {
        let mut buf = BytesMut::from(&[0x80, 0x01][..]);
        let result = decode_varint(&mut buf).unwrap();
        assert_eq!(result, Some(128));
    }

    #[test]
    fn test_varint_roundtrip() {
        for value in [0, 1, 127, 128, 255, 256, 16383, 16384, 2097151] {
            let encoded = encode_varint(value);
            let mut buf = BytesMut::from(&encoded[..]);
            let decoded = decode_varint(&mut buf).unwrap();
            assert_eq!(decoded, Some(value));
        }
    }

    #[test]
    fn test_string_empty() {
        let encoded = encode_string("");
        let mut buf = BytesMut::from(&encoded[..]);
        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, Some("".to_string()));
    }

    #[test]
    fn test_string_simple() {
        let encoded = encode_string("hello");
        let mut buf = BytesMut::from(&encoded[..]);
        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, Some("hello".to_string()));
    }

    #[test]
    fn test_string_utf8() {
        let encoded = encode_string("héllo wørld");
        let mut buf = BytesMut::from(&encoded[..]);
        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, Some("héllo wørld".to_string()));
    }

    #[test]
    fn test_incomplete_varint() {
        let mut buf = BytesMut::new();
        let result = decode_varint(&mut buf).unwrap();
        assert_eq!(result, None);
    }
}
