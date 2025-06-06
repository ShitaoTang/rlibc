use crate::include::ctype::*;
use super::in_addr;
use core::fmt::Write;
use crate::stdio::slice_write::SliceWriter;
use crate::string::strcpy::strcpy;

static mut buf: [c_uchar; 16] = [0; 16];    // considering its lifetime

// Network byte order -> ASCII string
// only for IPv4
// 32-bit integer in network byte order -> string in dotted-decimal notation
// 0xC0A80101 -> "192.168.1.1"
#[no_mangle]
pub extern "C" fn inet_ntoa(_in: in_addr) -> *const c_char
{
    let a = &_in.s_addr.to_ne_bytes() as &[u8; 4];
    let buf_ptr = core::ptr::addr_of_mut!(buf) as *mut c_uchar as *mut c_char;
    let mut temp = [0 as c_char; 16];
    let mut writer = SliceWriter::new(&mut temp);
    write!(
        &mut writer,
        "{}.{}.{}.{}",
        a[0] as c_int,
        a[1] as c_int,
        a[2] as c_int,
        a[3] as c_int
    ).unwrap();
    temp[writer.pos] = 0;
    unsafe { 
        strcpy(buf_ptr, temp.as_ptr());
        buf_ptr as *const c_char
    }

    // let fmt = b"%d.%d.%d.%d\0"; // this C-string should end with '\0'
    // unsafe {
    //     let buf_ptr = core::ptr::addr_of_mut!(buf) as *mut c_uchar as *mut c_char;
    //     libc::snprintf(
    //         buf_ptr,
    //         16,
    //         fmt.as_ptr() as *const c_char,
    //         a[0] as c_int,
    //         a[1] as c_int,
    //         a[2] as c_int,
    //         a[3] as c_int,
    //     );
    //     buf_ptr as *const c_char
    // }
}

/* if not using libc::snprintf
 * it may not be safe and even not work with ld error:
 * /usr/bin/ld: /usr/lib/gcc/aarch64-linux-gnu/12/libgcc_eh.a(unwind-dw2-fde-dip.o): in function `_Unwind_Find_FDE':
 * (.text+0x1828): undefined reference to `_dl_find_object'
 * collect2: error: ld returned 1 exit status
// change one segment once
fn u8_to_str(n: u8, buf: &mut [u8], pos: usize) -> usize
{
    let mut n = n;
    let mut len = 0;
    if num == 0 {
        buf[pos] = b'0';
        len = 1;
    } else {
        while n > 0 {
            buf[pos + len] = (n%10) as u8 + b'0';
            n /= 10;
            len += 1;
        } 
    }
    len
}

#[no_mangle]
pub extern "C" fn inet_ntoa(_in: in_addr) -> *const c_char
{
    let a = &_in.s_addr.to_ne_bytes() as &[u8; 4];
    let mut pos: usize = 0;
    
    unsafe {
        for i in 0..4 {
            if i > 0 {
                buf[pos] = b'.';
                pos += 1;
            }
            pos += u8_to_str(a[i], &mut buf, pos);
        }
        buf[pos] = 0;
        buf.as_ptr() as *const c_char
    }
}
   */