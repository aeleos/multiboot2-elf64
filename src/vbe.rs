/// The VBE info tag, as spcified by the Multiboot spec v1.6 and later.
#[repr(C, packed)]
pub struct VbeInfoTag {
	typ: u32,
	// = 784
	size: u32,
	pub vbe_mode: u16,
	pub vbe_interface_seg: u16,
	pub vbe_interface_off: u16,
	pub vbe_interface_len: u16,
	pub vbe_control_info: [u8; 512],
	pub vbe_mode_info: [u8; 256],
}
