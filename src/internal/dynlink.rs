use crate::include::ctype::*;
use crate::include::elf::*;

#[cfg(target_pointer_width = "64")] 
pub type Ehdr = Elf64_Ehdr;
#[cfg(target_pointer_width = "64")] 
pub type Phdr = Elf64_Phdr;
#[cfg(target_pointer_width = "64")] 
pub type Sym = Elf64_Sym;
#[cfg(target_pointer_width = "64")]
pub fn R_TYPE(x:size_t) -> c_int {
    (x & 0x7fffffff) as c_int
}
#[cfg(target_pointer_width = "64")]
pub fn R_SYM(x: size_t) -> c_int {
    (x >> 32) as c_int
}

#[cfg(target_pointer_width = "32")]
pub type Ehdr = Elf32_Ehdr;
#[cfg(target_pointer_width = "32")]
pub type Phdr = Elf32_Phdr;
#[cfg(target_pointer_width = "32")]
pub type Sym = Elf32_Sym;
#[cfg(target_pointer_width = "32")]
pub fn R_TYPE(x:c_uint) -> c_uint {
    x & 0xff
}
#[cfg(target_pointer_width = "32")]
pub fn R_SYM(x:c_uint) -> c_uint {
    x >> 8
}

pub enum REL_VAL{
    REl_NONE = 0,
    REL_SYMBOLIC = -100,
    REL_USYMBOLIC,
    REL_GOT,
    REL_PLT,
    REL_RELATIVE,
    REL_OFFSET,
    REL_OFFSET32,
    REL_COPY,
    REL_SYM_OR_REL,
    REL_DTPMOD,
    REL_DTPOFF,
    REL_TPOFF,
    REL_TPOFF_NEG,
    REL_TLSDSEC,
    REL_FUNCDESC,
    REL_FUNCDESC_VAL,
}

#[repr(C)]
pub struct fdpic_loadseg {
    pub addr: uintptr_t,
    pub p_vaddr: uintptr_t,
    pub p_memsz: uintptr_t,
}

#[repr(C)]
pub struct fdpic_loadmap {
    pub version: c_ushort,
    pub nesgs: c_ushort,
    pub segs: [fdpic_loadseg; 0],
}

#[allow(dead_code)]
const AUX_CNT: size_t = 32;
pub const DYN_CNT: size_t = 37;