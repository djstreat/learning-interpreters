pub mod byte_conversion {
    pub fn bytes_to_string(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }

    pub fn bytes_to_number(bytes: &[u8]) -> f64 {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(bytes);
        f64::from_bits(u64::from_be_bytes(buffer))
    }
}
