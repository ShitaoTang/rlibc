use crate::include::ctype::*;

pub type Elf32_Half = uint16_t;
pub type Elf64_Half = uint16_t;

pub type Elf32_Word = uint32_t;
pub type Elf64_Word = uint32_t;
pub type Elf32_Sword = int32_t;
pub type Elf64_Sword = int32_t;

pub type Elf32_Xword = uint64_t;
pub type Elf64_Xword = uint64_t;
pub type Elf32_Sxword = int64_t;
pub type Elf64_Sxword = int64_t;

pub type Elf32_Addr = uint32_t;
pub type Elf64_Addr = uint64_t;

pub type Elf32_Off = uint32_t;
pub type Elf64_Off = uint64_t;

pub type Elf32_Section = uint32_t;
pub type Elf64_Section = uint32_t;

pub type Elf32_Versym = Elf32_Half;
pub type Elf64_Versym = Elf32_Half;

pub const EI_NIDENT: size_t = 16;

#[repr(C)]
pub struct Elf32_Ehdr {
    pub e_ident:        [c_uchar; EI_NIDENT],
    pub e_type:         Elf32_Half,
    pub e_machine:      Elf32_Half,
    pub e_version:      Elf32_Word,
    pub e_entry:        Elf32_Addr,
    pub e_phoff:        Elf32_Off,
    pub e_shoff:        Elf32_Off,
    pub e_flags:        Elf32_Word,
    pub e_ehsize:       Elf32_Half,
    pub e_phentsize:    Elf32_Half,
    pub e_phnum:        Elf32_Half,
    pub e_shentsize:    Elf32_Half,
    pub e_shnum:        Elf32_Half,
    pub e_shstrndx:     Elf32_Half,
}

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident:        [c_uchar; EI_NIDENT],
    pub e_type:         Elf64_Half,
    pub e_machine:      Elf64_Half,
    pub e_version:      Elf64_Word,
    pub e_entry:        Elf64_Addr,
    pub e_phoff:        Elf64_Off,
    pub e_shoff:        Elf64_Off,
    pub e_flags:        Elf64_Word,
    pub e_ehsize:       Elf64_Half,
    pub e_phentsize:    Elf64_Half,
    pub e_phnum:        Elf64_Half,
    pub e_shentsize:    Elf64_Half,
    pub e_shnum:        Elf64_Half,
    pub e_shstrndx:     Elf64_Half,
}

#[repr(C)]
pub struct Elf32_Phdr {
    pub p_type:     Elf32_Word,
    pub p_offset:   Elf32_Off,
    pub p_vaddr:    Elf32_Addr,
    pub p_paddr:    Elf32_Addr,
    pub p_filesz:   Elf32_Word,
    pub p_memsz:    Elf32_Word,
    pub p_flags:    Elf32_Word,
    pub p_align:    Elf32_Word,
}

#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type:     Elf64_Word,
    pub p_flags:    Elf64_Word,
    pub p_offset:   Elf64_Off,
    pub p_vaddr:    Elf64_Addr,
    pub p_paddr:    Elf64_Addr,
    pub p_filesz:   Elf64_Word,
    pub p_memsz:    Elf64_Word,
    pub p_align:    Elf64_Xword,
}

#[repr(C)]
pub struct Elf32_Sym {
    pub st_name:     Elf32_Word,
    pub st_value:    Elf32_Addr,
    pub st_size:     Elf32_Word,
    pub st_info:     c_uchar,
    pub st_other:    c_uchar,
    pub st_shndx:    Elf32_Half,
}

#[repr(C)]
pub struct Elf64_Sym {
    pub st_name:     Elf32_Word,
    pub st_info:     c_uchar,
    pub st_other:    c_uchar,
    pub st_shndx:    Elf32_Half,
    pub st_value:    Elf64_Addr,
    pub st_size:     Elf64_Xword,
}

pub const PT_NULL:    c_uint = 0;
pub const PT_LOAD:    c_uint = 1;
pub const PT_DYNAMIC: c_uint = 2;
pub const PT_INTERP:  c_uint = 3;
pub const PT_NOTE:    c_uint = 4;
pub const PT_SHLIB:   c_uint = 5;
pub const PT_PHDR:    c_uint = 6;
pub const PT_TLS:     c_uint = 7;
pub const PT_NUM:     c_uint = 8;
pub const PT_LOOS:    c_uint = 0x60000000;
pub const PT_GNU_EH_FRAME:  c_uint = 0x6474e550;
pub const PT_GNU_STACK:     c_uint = 0x6474e551;
pub const PT_GNU_RELRO:     c_uint = 0x6474e552;
pub const PT_GNU_PROPERTY:  c_uint = 0x6474e553;
pub const PT_LOSUNW:  c_uint = 0x6ffffffa;
pub const PT_SUNWBSS: c_uint = 0x6ffffffa;
pub const PT_SUNWSTACK: c_uint = 0x6ffffffb;
pub const PT_HISUNW:  c_uint = 0x6fffffff;
pub const PT_HIOS:    c_uint = 0x6fffffff;
pub const PT_LOPROC:  c_uint = 0x70000000;
pub const PT_HIPROC:  c_uint = 0x7fffffff;

pub const AT_NULL:      size_t = 0;
pub const AT_IGNORE:    size_t = 1;
pub const AT_EXECFD:    size_t = 2;
pub const AT_PHDR:      size_t = 3;
pub const AT_PHENT:     size_t = 4;
pub const AT_PHNUM:     size_t = 5;
pub const AT_PAGESZ:    size_t = 6;
pub const AT_BASE:      size_t = 7;
pub const AT_FLAGS:     size_t = 8;
pub const AT_ENTRY:     size_t = 9;
pub const AT_NOTELF:    size_t = 10;
pub const AT_UID:       size_t = 11;
pub const AT_EUID:      size_t = 12;
pub const AT_GID:       size_t = 13;
pub const AT_EGID:      size_t = 14;
pub const AT_CLKTCK:    size_t = 17;


pub const AT_PLATFORM:  size_t = 15;
pub const AT_HWCAP:     size_t = 16;


pub const AT_FPUCW:     size_t = 18;


pub const AT_DCACHEBSIZE:   size_t = 19;
pub const AT_ICACHEBSIZE:   size_t = 20;
pub const AT_UCACHEBSIZE:   size_t = 21;


pub const AT_IGNOREPPC:     size_t = 22;

pub const AT_SECURE:        size_t = 23;

pub const AT_BASE_PLATFORM: size_t = 24;

pub const AT_RANDOM:        size_t = 25;

pub const AT_HWCAP2:        size_t = 26;

pub const AT_EXECFN:        size_t = 31;


pub const AT_SYSINFO:       size_t = 32;
pub const AT_SYSINFO_EHDR:  size_t = 33;


pub const AT_L1I_CACHESHAPE:  size_t = 34;
pub const AT_L1D_CACHESHAPE:  size_t = 35;
pub const AT_L2_CACHESHAPE:   size_t = 36;
pub const AT_L3_CACHESHAPE:   size_t = 37;

pub const AT_L1I_CACHESIZE:      size_t = 40;
pub const AT_L1I_CACHEGEOMETRY:  size_t = 41;
pub const AT_L1D_CACHESIZE:      size_t = 42;
pub const AT_L1D_CACHEGEOMETRY:  size_t = 43;
pub const AT_L2_CACHESIZE:       size_t = 44;
pub const AT_L2_CACHEGEOMETRY:   size_t = 45;
pub const AT_L3_CACHESIZE:       size_t = 46;
pub const AT_L3_CACHEGEOMETRY:   size_t = 47;

pub const AT_MINSIGSTKSZ:    size_t = 51;