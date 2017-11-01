
/// The VBE info tag, as specified by the Multiboot spec v1.6 and later.
#[repr(C, packed)]
pub struct VbeInfoTag {
    _type: u32,
    size: u32,
    pub vbe_mode: VbeMode,

    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,

    control_info: [u8; 512],
    mode_info: [u8; 256],
}

/// Vbe Mode
#[repr(C, packed)]
pub struct VbeMode(u16);

impl VbeMode {
    #[allow(dead_code)]
    pub fn is_vesa(&self) -> bool {
        self.0 & (1 << 7) > 0
    }

    #[allow(dead_code)]
    pub fn is_flat_buffer(&self) -> bool {
        self.0 & (1 << 13) > 0
    }

    #[allow(dead_code)]
    pub fn code(&self) -> u8 {
        (self.0 & 0xF) as u8
    }
}


/// Vbe Control Info
#[allow(dead_code)]
#[repr(C, packed)]
pub struct VbeControlInfo {
    signature: [u8; 4],
    version: u16,
    oem: u32,
    capabilities: u32,
    video_modes: u32,
    video_memory: u16,
    software_rev: u16,
    vendor: u32,
    product_name: u32,
    product_rev: u32,
    reserved: [u8; 222],
    oem_data: [u8; 256],
}
