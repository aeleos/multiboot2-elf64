use core::marker::PhantomData;

#[allow(dead_code)]
//#[non_exhaustive]
#[derive(PartialEq)]
#[repr(u32)]
pub enum TagType {
    EndTag = 0,
    CmdLine = 1,
    LoaderName = 2,
    Modules = 3,
    MemInfo = 4,
    BiosDev = 5,
    MemMap = 6,
    VbeInfo = 7,
    FbInfo = 8,
    ElfSection = 9,
    ApmTable = 10,
}

#[repr(C)]
pub struct Tag {
    pub typ: TagType,
    pub size: u32,
    // tag specific fields
}

pub struct TagIter<'a> {
    pub current: *const Tag,
    pub phantom: PhantomData<&'a Tag>,
}

impl<'a> Iterator for TagIter<'a> {
    type Item = &'a Tag;

    fn next(&mut self) -> Option<&'a Tag> {
        match unsafe { &*self.current } {
            &Tag {
                typ: TagType::EndTag,
                size: 8,
            } => None, // end tag
            tag => {
                // go to next tag
                let mut tag_addr = self.current as usize;
                tag_addr += ((tag.size + 7) & !7) as usize; //align at 8 byte
                self.current = tag_addr as *const _;

                Some(tag)
            }
        }
    }
}
