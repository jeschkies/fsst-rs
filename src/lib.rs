
mod fsst;
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use crate::fsst;

    #[test]
    fn it_works() {
        let input = CString::new("foobar").unwrap();

        unsafe {
            let mut len = 6 as usize;
            let _ = fsst::fsst_create(1, &mut len as *mut usize, input.as_ptr() as *mut *mut u8, 0);
        }
        //assert_eq!(result, 4);
    }
}
