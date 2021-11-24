#![feature(ptr_metadata)]

mod eui48;

use eui48::Eui48 as Address;
use core::ops::Range;

pub struct EthernetFrame {
    header: [u8; 14],
    body: [u8],
}

impl EthernetFrame {
    pub fn new(buf: &[u8]) -> &Self {
        if buf.len() < 14 {
            panic!("buffer too short")
        }
        let ptr = core::ptr::from_raw_parts(
            buf.as_ptr() as *const (), 
            buf.len() - 14
        );
        unsafe { &*ptr }
    }
}
impl EthernetFrame {
    const DEST_ADDR: Range<usize> = 0..6;
    const SRC_ADDR: Range<usize> = 6..12;
    // const ETHERTYPE: Range<usize> = 12..14;
    pub fn dst_addr(&self) -> Address {
        Address::from_bytes(&self.header[Self::DEST_ADDR])
    }
    pub fn src_addr(&self) -> Address {
        Address::from_bytes(&self.header[Self::SRC_ADDR])
    }
    // // does not support IEEE 802.3 frames and 802.1Q fields
    // pub fn ethertype(&self) -> Type {
    //     let ty = NetworkEndian::read_u16(&self.inner.as_ref()[Self::ETHERTYPE]);
    //     Type::from(ty)
    // }
    pub fn payload(&self) -> &[u8] {
        &self.body
    }
}

fn main() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let frame = EthernetFrame::new(&a);
    println!("dst addr: {}", frame.dst_addr());
    println!("src addr: {}", frame.src_addr());
    println!("payload: {:?}", frame.payload());
}
