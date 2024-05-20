#[repr(C)]
#[derive(Debug, Clone)]
struct CBFSummary {
    magic_number: i32,
    cbf_version: u32,
    kernel_load: u32,
    kernel_jump: u32,
    kernel_size: u32,
}

impl From<CBFSummary> for Vec<u8> {
    fn from(value: CBFSummary) -> Self {
        let mut return_vec = Vec::new();
        return_vec.extend_from_slice(&value.magic_number.to_le_bytes());
        return_vec.extend_from_slice(&value.cbf_version.to_le_bytes());
        return_vec.extend_from_slice(&value.kernel_load.to_le_bytes());
        return_vec.extend_from_slice(&value.kernel_jump.to_le_bytes());
        return_vec.extend_from_slice(&value.kernel_size.to_le_bytes());
        return_vec
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
/// In memory representation of a CBF object.
pub struct CBF {
    summary: CBFSummary,
    summary_crc: i32,
    kernel: Vec<u8>,
    kernel_crc: i32,
}

impl CBF {
    pub fn crc(buffer: &Vec<u8>) -> i32 {
        let mut crc = 0;
        // Take chunks of 4 for the crc.
        for b in buffer.chunks(4) {
            let chunk: i32 = i32::from_le_bytes(b.try_into().unwrap());
            crc = 1 + (crc ^ chunk); 
        }

        crc
    }

    pub fn get_summary_crc(&self) -> i32 {
        self.summary_crc
    }

    pub fn get_kernel_crc(&self) -> i32 {
        self.kernel_crc
    }

    /// Create a new CBF object.
    pub fn new(
        magic_number: i32,
        cbf_version: u32,
        kernel_load: u32,
        kernel_jump: u32,
        kernel: Vec<u8>,
    ) -> Self {
        let summary = CBFSummary {
            magic_number,
            cbf_version,
            kernel_load,
            kernel_jump,
            kernel_size: kernel.len() as u32,
        };
        let summary_crc = CBF::crc(&summary.clone().into());
        let kernel_crc = CBF::crc(&kernel.clone());

        Self {
            summary,
            summary_crc,
            kernel,
            kernel_crc,
        }
    }
}

impl From<CBF> for Vec<u8> {
    fn from(value: CBF) -> Self {
        let mut return_vec = Vec::new();
        let summary_bytes: Vec<u8> = value.summary.into();
        return_vec.extend(summary_bytes);
        return_vec.extend_from_slice(&value.summary_crc.to_le_bytes());
        return_vec.extend(value.kernel);
        return_vec.extend_from_slice(&value.kernel_crc.to_le_bytes());
        return_vec
    }
}
