# cbf-rs

Rust implementation of the CBF format that is utilized by [various LeapFrog devices](https://elinux.org/LeapFrog_Pollux_Platform:_File_Format_CBF).

## Installing
- `git clone https://github.com/leapbunny/cbf-rs`
- `cd cbf-rs`
- `cargo install --path .`

## Usage

You can use `cbf-rs --help` to view all arguments. Supports Version 1 and 2 of CBFs.

Example: `cbf-rs --version 2 --output kernel.cbf ./zImage`

