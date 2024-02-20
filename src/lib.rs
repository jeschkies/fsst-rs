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

            let mut inputs_b = vec![CString::new("Hello").unwrap()];
            let mut lengths_b = vec![5];

            let outsize = 128;
            // TODO: figure how out is used.
            let mut out = Vec::with_capacity(outsize);
            let mut out_lengths = Vec::with_capacity(1);
            let mut out_strs = Vec::<CString>::with_capacity(1);
            let n_strings = fsst::fsst_compress(
                encoder,
                1,
                lengths_b.as_mut_ptr(),
                inputs_b.as_mut_ptr() as *mut *mut u8,
                outsize,
                out.as_mut_ptr() as *mut u8,
                out_lengths.as_mut_ptr() as *mut usize,
                out_strs.as_mut_ptr() as *mut *mut u8,
            );

            std::mem::forget(out); // prevents double free
            assert_eq!(n_strings, 1, "not all strings have been compressed.");

            assert_eq!(out_strs.len(), 1);
            //assert_eq!(out_lengths.len(), out_strs.len());
            //for it in out_lengths.iter().zip(out_strs.iter()) {
            for it in out_strs.iter() {
                //let (len, string) = it;
                let string = it;
                assert_eq!(inputs[0], string.clone());
                let outsize = 128;
                let mut out = Vec::with_capacity(outsize);
                let s = fsst::fsst_decompress(
                    encoder,
                    //*len,
                    string.clone().into_bytes().len(),
                    string.clone().into_raw() as *mut u8,
                    outsize,
                    out.as_mut_ptr() as *mut u8,
                );
                assert!(s > 0, "no string has been decompressed");
                assert_eq!(inputs[0], CString::from_vec_unchecked(out[0..s].to_vec()));
                assert_eq!(2, 1);
            }

            fsst::fsst_destroy(encoder);
        }
    }
}
