use core::fmt;

enum FramebufferType {
    Indexed,
    Rgb,
    EgaText,
}

impl FramebufferType {
    fn from_u8(n: u8) -> Option<FramebufferType> {
        match n {
            0 => Some(FramebufferType::Indexed),
            1 => Some(FramebufferType::Rgb),
            2 => Some(FramebufferType::EgaText),
            _ => None,
        }
    }
}

#[repr(C, packed)]
struct FramebufferPalette {
    addr: u32,
    num_colors: u16,
}

#[repr(C, packed)]
struct FramebufferFields {
    red_position: u8,
    red_mask_size: u8,
    green_position: u8,
    green_mask_size: u8,
    blue_position: u8,
    blue_mask_size: u8,
}


#[repr(C, packed)]
pub struct FramebufferInfoTag {
    _type: u32,
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u16,
    color_info: [u8; 6],
}

impl FramebufferInfoTag {
    fn color_info_fields(&self) -> &'static FramebufferFields {
        unsafe { &*(&self.color_info as *const _ as *const FramebufferFields) }
    }

    fn color_info_palette(&self) -> &'static FramebufferPalette {
        unsafe { &*(&self.color_info as *const _ as *const FramebufferPalette) }
    }
}

#[allow(unused_must_use)]
impl fmt::Debug for FramebufferInfoTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "address: {:#X}, mode:{}x{}x{}, pitch: {}, type: {}",
            self.framebuffer_addr,
            self.framebuffer_width,
            self.framebuffer_height,
            self.framebuffer_bpp,
            self.framebuffer_pitch,
            self.framebuffer_type,
        );
        match FramebufferType::from_u8(self.framebuffer_type) {
            Some(FramebufferType::Indexed) => {
                let fb_palette = self.color_info_palette();
                write!(
                    f,
                    "color_info: palette, {} colors available",
                    fb_palette.num_colors
                )
            }
            Some(FramebufferType::Rgb) => {
                let fb_fields = self.color_info_fields();
                write!(f, "color_info: (position, mask_size), ");
                write!(
                    f,
                    "red: ({}, {}), green: ({}, {}), blue: ({}, {})",
                    fb_fields.red_position,
                    fb_fields.red_mask_size,
                    fb_fields.green_position,
                    fb_fields.green_mask_size,
                    fb_fields.blue_position,
                    fb_fields.blue_mask_size
                )
            }
            Some(FramebufferType::EgaText) => Ok(()),
            None => panic!("Something has gone terribly wrong!"),
        }
    }
}
