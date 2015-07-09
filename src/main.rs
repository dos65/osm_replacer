extern crate protobuf;
extern crate byteorder;

use std::fs::File;
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};

mod fileformat;

fn main() {
    let file = File::open("/home/dos65/Downloads/RU-TA.osm.pbf").unwrap();
    let mut reader = PbfReader::new(file);
    reader.parse();
}

struct PbfReader<R> {
    read: R,
}

impl <R: Read> PbfReader <R> {

    pub fn new(read: R) -> PbfReader<R> {
        PbfReader {read : read}
    }

    pub fn parse(&mut self) -> () {
       self.parse_next();
       self.parse_next();
    }

    pub fn parse_next(&mut self) -> Option<fileformat::Blob> {

        let size = self.read.by_ref().read_u32::<BigEndian>().unwrap();

        let header_buf = self.read_size(size as u64);
        let header: fileformat::BlobHeader = protobuf::parse_from_bytes(&header_buf).unwrap();

        let blob_buf = self.read_size(header.get_datasize() as u64);
        let blob: fileformat::Blob = protobuf::parse_from_bytes(&blob_buf).unwrap();

        return match header.get_field_type() {
            "OSMData" => Some(blob),
            _ => None
        }
    }

    fn read_size(&mut self, size: u64) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
        self.read.by_ref().take(size as u64).read_to_end(&mut buffer).unwrap();
        buffer
    }

}
