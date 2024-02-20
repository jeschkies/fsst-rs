mod fsst;
#[cfg(test)]
mod tests {
    use crate::fsst;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        unsafe {
            let mut inputs = vec![CString::new("Hello").unwrap()];

            let mut lengths = vec![5];

            let encoder = fsst::fsst_create(
                1,
                lengths.as_mut_ptr(),
                inputs.as_mut_ptr() as *mut *mut u8,
                1,
            );

            let outsize = 128;
            let mut out = Vec::with_capacity(outsize);
            let mut out_lengths = Vec::with_capacity(1);
            let mut out_strs = vec![CString::new("Hello").unwrap()];
            let _ = fsst::fsst_compress(
                encoder,
                1,
                lengths.as_mut_ptr(),
                inputs.as_mut_ptr() as *mut *mut u8,
                outsize,
                out.as_mut_ptr() as *mut u8,
                out_lengths.as_mut_ptr() as *mut usize,
                out_strs.as_mut_ptr() as *mut *mut u8,
            );
        }
    }
}
