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