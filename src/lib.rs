
mod fsst;
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use crate::fsst;

    #[test]
    fn it_works() {

        unsafe {
            let mut inputs = vec![CString::new("Hello").unwrap()];

            let mut lengths = vec![5];

            let _ = fsst::fsst_create(1, lengths.as_mut_ptr(), inputs.as_mut_ptr() as *mut *mut u8, 1);
        }
    }
}
