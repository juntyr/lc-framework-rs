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

pub enum Component {
    // mutators
    Noop,
    TwosComplementToMagnitudeSign { size: ElemSize },
    // shufflers
    BitShuffle { size: ElemSize },
    // reducers
    RunLengthEncoding { size: ElemSize },
    // CLOG_1, CLOG_2, CLOG_4, CLOG_8, DBEFS_4, DBEFS_8, DBESF_4, DBESF_8, DIFFMS_1, DIFFMS_2, DIFFMS_4, DIFFMS_8, DIFFNB_1, DIFFNB_2, DIFFNB_4, DIFFNB_8, DIFF_1, DIFF_2, DIFF_4, DIFF_8, HCLOG_1, HCLOG_2, HCLOG_4, HCLOG_8, RARE_1, RARE_2, RARE_4, RARE_8, RAZE_1, RAZE_2, RAZE_4, RAZE_8, RRE_1, RRE_2, RRE_4, RRE_8, RZE_1, RZE_2, RZE_4, RZE_8, TCNB_1, TCNB_2, TCNB_4, TCNB_8, TUPL12_1, TUPL2_1, TUPL2_2, TUPL2_4, TUPL3_1, TUPL3_2, TUPL3_8, TUPL4_1, TUPL4_2, TUPL6_1, TUPL6_2, TUPL6_4, TUPL6_8, TUPL8_1
}

pub enum ElemSize {
    S1,
    S2,
    S4,
    S8,
}

impl Component {
    fn as_id(&self) -> lc_framework_sys::LC_CPUcomponents {
        match self {
            Self::Noop => lc_framework_sys::LC_CPUcomponents_NUL_CPUcomponents,
            Self::TwosComplementToMagnitudeSign { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_1
            }
            Self::TwosComplementToMagnitudeSign { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_2
            }
            Self::TwosComplementToMagnitudeSign { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_4
            }
            Self::TwosComplementToMagnitudeSign { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_8
            }
            Self::BitShuffle { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_BIT_1,
            Self::BitShuffle { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_BIT_2,
            Self::BitShuffle { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_BIT_4,
            Self::BitShuffle { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_BIT_8,
            Self::RunLengthEncoding { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_RLE_1
            }
            Self::RunLengthEncoding { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_RLE_2
            }
            Self::RunLengthEncoding { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_RLE_4
            }
            Self::RunLengthEncoding { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_RLE_8
            }
        }
    }
}

pub fn compress(
    preprocessors: &[Preprocessor],
    components: &[Component],
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

    let component_ids = components.iter().map(Component::as_id).collect::<Vec<_>>();

    let mut encoded_ptr = std::ptr::null_mut();
    let mut encoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_compress(
            preprocessor_ids.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_nparams.as_ptr(),
            preprocessor_params.as_ptr(),
            component_ids.len(),
            component_ids.as_ptr(),
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
    components: &[Component],
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

    let component_ids = components.iter().map(Component::as_id).collect::<Vec<_>>();

    let mut decoded_ptr = std::ptr::null_mut();
    let mut decoded_size = 0;

    let status = unsafe {
        lc_framework_sys::lc_decompress(
            preprocessor_ids.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_nparams.as_ptr(),
            preprocessor_params.as_ptr(),
            component_ids.len(),
            component_ids.as_ptr(),
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
        let preprocessors = &[];
        let components = &[
            Component::BitShuffle { size: ElemSize::S4 },
            Component::RunLengthEncoding { size: ElemSize::S4 },
        ];

        let data = b"abcd";
        eprintln!("data={data:?}");

        let encoded = compress(preprocessors, components, data).unwrap();
        eprintln!("encoded={encoded:?}");

        let decoded = decompress(preprocessors, components, &encoded).unwrap();
        eprintln!("decoded={decoded:?}");
    }
}
