use std::{ffi::CStr, sync::OnceLock};

pub fn available_preprocessors() -> &'static [&'static str] {
    static PREPROCESSORS: OnceLock<Vec<&'static str>> = OnceLock::new();

    PREPROCESSORS
        .get_or_init(|| {
            let preprocessors =
                unsafe { CStr::from_ptr(lc_framework_sys::lc_available_preprocessors()) }
                    .to_str()
                    .unwrap();

            preprocessors
                .split(' ')
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect()
        })
        .as_slice()
}

pub fn available_components() -> &'static [&'static str] {
    static COMPONENTS: OnceLock<Vec<&'static str>> = OnceLock::new();

    COMPONENTS
        .get_or_init(|| {
            let components = unsafe { CStr::from_ptr(lc_framework_sys::lc_available_components()) }
                .to_str()
                .unwrap();

            components
                .split(' ')
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect()
        })
        .as_slice()
}

pub fn compress(preprocessors: &CStr, compressors: &CStr, input: &[u8]) -> Result<Vec<u8>, ()> {
    let mut encoded_ptr = std::ptr::null_mut();
    let mut encoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_compress(
            preprocessors.as_ptr(),
            compressors.as_ptr(),
            input.as_ptr(),
            input.len() as _,
            &raw mut encoded_ptr,
            &raw mut encoded_size,
        )
    };

    if status != 0 {
        return Err(());
    }

    let mut encoded = Vec::with_capacity(encoded_size as _);
    unsafe {
        std::ptr::copy_nonoverlapping(
            encoded_ptr.cast_const(),
            encoded.as_mut_ptr(),
            encoded_size as _,
        );
    }
    unsafe {
        encoded.set_len(encoded_size as _);
    }
    unsafe {
        lc_framework_sys::lc_free_bytes(encoded_ptr);
    }

    Ok(encoded)
}

pub fn decompress(preprocessors: &CStr, compressors: &CStr, encoded: &[u8]) -> Result<Vec<u8>, ()> {
    let mut decoded_ptr = std::ptr::null_mut();
    let mut decoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_decompress(
            preprocessors.as_ptr(),
            compressors.as_ptr(),
            encoded.as_ptr(),
            encoded.len() as _,
            &raw mut decoded_ptr,
            &raw mut decoded_size,
        )
    };

    if status != 0 {
        return Err(());
    }

    let mut decoded = Vec::with_capacity(decoded_size as _);
    unsafe {
        std::ptr::copy_nonoverlapping(
            decoded_ptr.cast_const(),
            decoded.as_mut_ptr(),
            decoded_size as _,
        );
    }
    unsafe {
        decoded.set_len(decoded_size as _);
    }
    unsafe {
        lc_framework_sys::lc_free_bytes(decoded_ptr);
    }

    Ok(decoded)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        eprintln!("{:?}", available_preprocessors());
        eprintln!("{:?}", available_components());

        let preprocessors = c"";
        let compressors = c"BIT_4 RLE_4";

        let data = b"abcd";
        eprintln!("data={data:?}");

        let encoded = compress(preprocessors, compressors, b"abcd").unwrap();
        eprintln!("encoded={encoded:?}");

        let decoded = decompress(preprocessors, compressors, &encoded).unwrap();
        eprintln!("decoded={decoded:?}");
    }
}
