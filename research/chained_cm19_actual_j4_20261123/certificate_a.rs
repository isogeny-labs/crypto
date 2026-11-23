use crate::arithmetic::{K, Q};
use crate::schemes::Point;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Certificate {
    file: File,
    pub bytes: usize,
}
pub fn u16_bytes(out: &mut Vec<u8>, value: usize) {
    out.extend_from_slice(
        &u16::try_from(value)
            .expect("u16 certificate value")
            .to_le_bytes(),
    );
}
pub fn u32_bytes(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}
pub fn point_bytes(out: &mut Vec<u8>, p: Point) {
    out.push(u8::from(p.is_none()));
    let (x, y) = p.unwrap_or((K(0), K(0)));
    u32_bytes(out, x.0);
    u32_bytes(out, y.0);
}
impl Certificate {
    pub fn create(path: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("create original certificate once");
        let mut result = Self { file, bytes: 0 };
        let mut header = b"CTJ4A001".to_vec();
        u32_bytes(&mut header, 1);
        u32_bytes(&mut header, Q);
        for n in [16, 11, 4096, 64, 1331, 15] {
            u32_bytes(&mut header, n);
        }
        assert_eq!(header.len(), 40);
        result.append(&header);
        result
    }
    fn append(&mut self, data: &[u8]) {
        self.file
            .write_all(data)
            .expect("complete native certificate bytes");
        self.file.flush().expect("flush original section");
        self.bytes += data.len();
    }
    pub fn section(&mut self, tag: &[u8; 4], count: usize, width: usize, payload: &[u8]) {
        if width != 0 {
            assert_eq!(count * width, payload.len());
        }
        let mut header = tag.to_vec();
        u32_bytes(&mut header, count.try_into().expect("record count"));
        u32_bytes(&mut header, width.try_into().expect("record width"));
        u32_bytes(
            &mut header,
            payload.len().try_into().expect("payload length"),
        );
        self.append(&header);
        self.append(payload);
    }
    pub fn finish(&mut self) {
        self.append(b"COMPLETE");
        assert_eq!(self.bytes, 59325);
        println!(
            "COMPLETE_CERTIFICATE\tbytes={}\tall13_complete_sections\tPASS",
            self.bytes
        );
    }
}
