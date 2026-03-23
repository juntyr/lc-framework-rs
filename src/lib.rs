use std::{ffi::CStr, sync::OnceLock};

#[derive(Clone, Debug, PartialEq)]
pub enum Preprocessor {
    Noop,
    Lorenzo1D {
        dtype: LorenzoDtype,
    },
    QuantizeErrorBound {
        dtype: QuantizeDType,
        kind: ErrorKind,
        error_bound: f64,
        threshold: Option<f64>,
        decorrelation: Decorrelation,
    },
}

impl Preprocessor {
    fn as_id(&self) -> lc_framework_sys::LC_CPUpreprocessor {
        match self {
            Self::Noop => lc_framework_sys::LC_CPUpreprocessor_NUL_CPUpreprocessor,
            Self::Lorenzo1D {
                dtype: LorenzoDtype::I32,
            } => lc_framework_sys::LC_CPUpreprocessor_LOR1D_i32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Abs,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_ABS_0_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Abs,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_ABS_R_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Noa,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_NOA_0_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Noa,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_NOA_R_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Rel,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_REL_0_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F32,
                kind: ErrorKind::Rel,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_REL_R_f32,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Abs,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_ABS_0_f64,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Abs,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_ABS_R_f64,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Noa,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_NOA_0_f64,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Noa,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_NOA_R_f64,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Rel,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Zero,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_REL_0_f64,
            Self::QuantizeErrorBound {
                dtype: QuantizeDType::F64,
                kind: ErrorKind::Rel,
                error_bound: _,
                threshold: _,
                decorrelation: Decorrelation::Random,
            } => lc_framework_sys::LC_CPUpreprocessor_QUANT_REL_R_f64,
        }
    }

    fn push_params(&self, params: &mut Vec<f64>) {
        match self {
            Self::Noop | Self::Lorenzo1D { dtype: _ } => (),
            Self::QuantizeErrorBound {
                dtype: _,
                kind: _,
                error_bound,
                threshold,
                decorrelation: _,
            } => {
                params.push(*error_bound);
                if let Some(threshold) = threshold {
                    params.push(*threshold);
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    Abs,
    Noa,
    Rel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decorrelation {
    Zero,
    Random,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LorenzoDtype {
    I32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantizeDType {
    F32,
    F64,
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

pub fn compress(
    preprocessors: &[Preprocessor],
    components: &CStr,
    input: &[u8],
) -> Result<Vec<u8>, ()> {
    let mut preprocessor_ids = Vec::with_capacity(preprocessors.len());
    let mut preprocessor_params = Vec::new();
    let mut preprocessor_nparams = Vec::with_capacity(preprocessors.len());
    for preprocessor in preprocessors {
        preprocessor_ids.push(preprocessor.as_id());
        let preprocessor_nparams_sum = preprocessor_nparams.len();
        preprocessor.push_params(&mut preprocessor_params);
        preprocessor_nparams.push(preprocessor_nparams.len() - preprocessor_nparams_sum);
    }

    let mut encoded_ptr = std::ptr::null_mut();
    let mut encoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_compress(
            preprocessors.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_nparams.as_ptr(),
            preprocessor_params.as_ptr(),
            components.as_ptr(),
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

pub fn decompress(
    preprocessors: &[Preprocessor],
    components: &CStr,
    encoded: &[u8],
) -> Result<Vec<u8>, ()> {
    let mut preprocessor_ids = Vec::with_capacity(preprocessors.len());
    let mut preprocessor_params = Vec::new();
    let mut preprocessor_nparams = Vec::with_capacity(preprocessors.len());
    for preprocessor in preprocessors {
        preprocessor_ids.push(preprocessor.as_id());
        let preprocessor_nparams_sum = preprocessor_nparams.len();
        preprocessor.push_params(&mut preprocessor_params);
        preprocessor_nparams.push(preprocessor_nparams.len() - preprocessor_nparams_sum);
    }

    let mut decoded_ptr = std::ptr::null_mut();
    let mut decoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_decompress(
            preprocessors.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_nparams.as_ptr(),
            preprocessor_params.as_ptr(),
            components.as_ptr(),
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
        eprintln!("{:?}", available_components());

        let preprocessors = &[];
        let components = c"BIT_4 RLE_4";

        let data = b"abcd";
        eprintln!("data={data:?}");

        let encoded = compress(preprocessors, components, data).unwrap();
        eprintln!("encoded={encoded:?}");

        let decoded = decompress(preprocessors, components, &encoded).unwrap();
        eprintln!("decoded={decoded:?}");
    }
}
