use core::marker::PhantomData;

const MULTIBOOT2_MAGIC: u32 = 0xE85250D6;
const MULTIBOOT2_ARCH_I386: u32 = 0;

#[repr(C)]
struct MultibootMagic<Container> {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    _container: PhantomData<Container>,
}

impl<Container> MultibootMagic<Container> {
    const MAGIC: Self = Self {
        magic: MULTIBOOT2_MAGIC,
        architecture: MULTIBOOT2_ARCH_I386,
        header_length: core::mem::size_of::<Container>() as u32,
        checksum: -((MULTIBOOT2_MAGIC
            + MULTIBOOT2_ARCH_I386
            + core::mem::size_of::<Container>() as u32) as i32) as u32,
        _container: PhantomData::<Container>,
    };
}

#[repr(C)]
struct MultibootEndTag {
    ty: u16,
    flags: u16,
    size: u32,
}

impl MultibootEndTag {
    const TAG: MultibootEndTag = MultibootEndTag {
        ty: 0,
        flags: 0,
        size: 8,
    };
}

#[repr(C)]
struct MultibootHeader {
    magic: MultibootMagic<Self>,
    end_tag: MultibootEndTag,
}

#[link_section = ".multiboot"]
#[used]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MultibootMagic::MAGIC,
    end_tag: MultibootEndTag::TAG,
};
