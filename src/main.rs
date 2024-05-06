mod cbf;
use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;

use crate::cbf::CBF;

const MAGIC_NUMBER: i32 = -0x65432110;
const CBF_VERSION: u32 = 1;
const BLOCK_SIZE: u32 = 0x2_00_00;

#[derive(Parser)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 1)]
    version: usize,
    #[arg(short, long, default_value_t = false)]
    surgeon: bool,
    #[arg(short, long, default_value = "kernel.cbf")]
    output: String,
    #[arg()]
    file_name: String,
}

pub fn main() {
    let args = Arguments::parse();

    let kernel_load = match args.version {
        1 => 0x80_00,
        2 => 0x10_00_00,
        _ => panic!("Invalid Version Number!"),
    };

    let kernel_jump = kernel_load;
    let mut kernel_file = File::options().read(true).open(args.file_name).unwrap();
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(args.output)
        .unwrap();

    output_file.set_len(0).unwrap();

    let mut kernel_buf: Vec<u8> = Vec::new();
    kernel_file.read_to_end(&mut kernel_buf).unwrap();
    let kernel_buf_len = kernel_buf.len();
    println!("Buffer Len: {:#08x}", kernel_buf_len);

    let cbf: CBF = CBF::new(
        MAGIC_NUMBER,
        CBF_VERSION,
        kernel_jump,
        kernel_load,
        kernel_buf,
    );

    println!("Summary CRC: {:#08x}", cbf.get_summary_crc());
    println!("Kernel CRC: {:#08x}", cbf.get_kernel_crc());

    let cbf_bytes: Vec<u8> = cbf.into();
    let mut written_len: usize = 0;
    while written_len != cbf_bytes.len() {
        let written = output_file.write(&cbf_bytes[written_len..]).unwrap();
        written_len += written;
    }

    while written_len % BLOCK_SIZE as usize != 0 {
        written_len += output_file.write(&[0xFF]).unwrap();
    }

    output_file.sync_all().unwrap()
}
