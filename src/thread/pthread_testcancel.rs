use super::pthread_cancel::*;

#[no_mangle]
pub extern "C" fn pthread_testcancel() -> ()
{
    testcancel();
}