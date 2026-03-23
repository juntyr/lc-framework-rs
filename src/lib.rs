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

    if components.is_empty() || components.len() > (lc_framework_sys::max_stages as _) {
        return Err(());
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

    if components.is_empty() || components.len() > (lc_framework_sys::max_stages as _) {
        return Err(());
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Component {
    Noop,
    // mutators
    TwosComplementToSignMagnitude { size: ElemSize },
    TwosComplementToNegaBinary { size: ElemSize },
    DebiasedExponentFractionSign { size: FloatSize },
    DebiasedExponentSignFraction { size: FloatSize },
    // shufflers
    BitShuffle { size: ElemSize },
    Tuple { size: TupleSize },
    // predictors
    Delta { size: ElemSize },
    DeltaAsSignMagnitude { size: ElemSize },
    DeltaAsNegaBinary { size: ElemSize },
    // reducers
    Clog { size: ElemSize },
    HClog { size: ElemSize },
    Rare { size: ElemSize },
    Raze { size: ElemSize },
    RunLengthEncoding { size: ElemSize },
    RepetitionRunBitmapEncoding { size: ElemSize },
    ZeroRunBitmapEncoding { size: ElemSize },
}

impl Component {
    fn as_id(&self) -> lc_framework_sys::LC_CPUcomponents {
        match self {
            Self::Noop => lc_framework_sys::LC_CPUcomponents_NUL_CPUcomponents,
            // mutators
            Self::TwosComplementToSignMagnitude { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_1
            }
            Self::TwosComplementToSignMagnitude { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_2
            }
            Self::TwosComplementToSignMagnitude { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_4
            }
            Self::TwosComplementToSignMagnitude { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_TCMS_8
            }
            Self::TwosComplementToNegaBinary { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_TCNB_1
            }
            Self::TwosComplementToNegaBinary { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_TCNB_2
            }
            Self::TwosComplementToNegaBinary { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_TCNB_4
            }
            Self::TwosComplementToNegaBinary { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_TCNB_8
            }
            Self::DebiasedExponentFractionSign {
                size: FloatSize::S4,
            } => lc_framework_sys::LC_CPUcomponents_DBEFS_4,
            Self::DebiasedExponentFractionSign {
                size: FloatSize::S8,
            } => lc_framework_sys::LC_CPUcomponents_DBEFS_8,
            Self::DebiasedExponentSignFraction {
                size: FloatSize::S4,
            } => lc_framework_sys::LC_CPUcomponents_DBESF_4,
            Self::DebiasedExponentSignFraction {
                size: FloatSize::S8,
            } => lc_framework_sys::LC_CPUcomponents_DBESF_8,
            // shuffle
            Self::BitShuffle { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_BIT_1,
            Self::BitShuffle { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_BIT_2,
            Self::BitShuffle { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_BIT_4,
            Self::BitShuffle { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_BIT_8,
            Self::Tuple {
                size: TupleSize::S1x2,
            } => lc_framework_sys::LC_CPUcomponents_TUPL2_1,
            Self::Tuple {
                size: TupleSize::S1x3,
            } => lc_framework_sys::LC_CPUcomponents_TUPL3_1,
            Self::Tuple {
                size: TupleSize::S1x4,
            } => lc_framework_sys::LC_CPUcomponents_TUPL4_1,
            Self::Tuple {
                size: TupleSize::S1x6,
            } => lc_framework_sys::LC_CPUcomponents_TUPL6_1,
            Self::Tuple {
                size: TupleSize::S1x8,
            } => lc_framework_sys::LC_CPUcomponents_TUPL8_1,
            Self::Tuple {
                size: TupleSize::S1x12,
            } => lc_framework_sys::LC_CPUcomponents_TUPL12_1,
            Self::Tuple {
                size: TupleSize::S2x2,
            } => lc_framework_sys::LC_CPUcomponents_TUPL2_2,
            Self::Tuple {
                size: TupleSize::S2x3,
            } => lc_framework_sys::LC_CPUcomponents_TUPL3_2,
            Self::Tuple {
                size: TupleSize::S2x4,
            } => lc_framework_sys::LC_CPUcomponents_TUPL4_2,
            Self::Tuple {
                size: TupleSize::S2x6,
            } => lc_framework_sys::LC_CPUcomponents_TUPL6_2,
            Self::Tuple {
                size: TupleSize::S4x2,
            } => lc_framework_sys::LC_CPUcomponents_TUPL2_4,
            Self::Tuple {
                size: TupleSize::S4x6,
            } => lc_framework_sys::LC_CPUcomponents_TUPL6_4,
            Self::Tuple {
                size: TupleSize::S8x3,
            } => lc_framework_sys::LC_CPUcomponents_TUPL3_8,
            Self::Tuple {
                size: TupleSize::S8x6,
            } => lc_framework_sys::LC_CPUcomponents_TUPL6_8,
            // predictors
            Self::Delta { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_DIFF_1,
            Self::Delta { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_DIFF_2,
            Self::Delta { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_DIFF_4,
            Self::Delta { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_DIFF_8,
            Self::DeltaAsSignMagnitude { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFMS_1
            }
            Self::DeltaAsSignMagnitude { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFMS_2
            }
            Self::DeltaAsSignMagnitude { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFMS_4
            }
            Self::DeltaAsSignMagnitude { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFMS_8
            }
            Self::DeltaAsNegaBinary { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFNB_1
            }
            Self::DeltaAsNegaBinary { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFNB_2
            }
            Self::DeltaAsNegaBinary { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFNB_4
            }
            Self::DeltaAsNegaBinary { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_DIFFNB_8
            }
            // reducers
            Self::Clog { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_CLOG_1,
            Self::Clog { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_CLOG_2,
            Self::Clog { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_CLOG_4,
            Self::Clog { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_CLOG_8,
            Self::HClog { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_HCLOG_1,
            Self::HClog { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_HCLOG_2,
            Self::HClog { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_HCLOG_4,
            Self::HClog { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_HCLOG_8,
            Self::Rare { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_RARE_1,
            Self::Rare { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_RARE_2,
            Self::Rare { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_RARE_4,
            Self::Rare { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_RARE_8,
            Self::Raze { size: ElemSize::S1 } => lc_framework_sys::LC_CPUcomponents_RAZE_1,
            Self::Raze { size: ElemSize::S2 } => lc_framework_sys::LC_CPUcomponents_RAZE_2,
            Self::Raze { size: ElemSize::S4 } => lc_framework_sys::LC_CPUcomponents_RAZE_4,
            Self::Raze { size: ElemSize::S8 } => lc_framework_sys::LC_CPUcomponents_RAZE_8,
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
            Self::RepetitionRunBitmapEncoding { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_RRE_1
            }
            Self::RepetitionRunBitmapEncoding { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_RRE_2
            }
            Self::RepetitionRunBitmapEncoding { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_RRE_4
            }
            Self::RepetitionRunBitmapEncoding { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_RRE_8
            }
            Self::ZeroRunBitmapEncoding { size: ElemSize::S1 } => {
                lc_framework_sys::LC_CPUcomponents_RZE_1
            }
            Self::ZeroRunBitmapEncoding { size: ElemSize::S2 } => {
                lc_framework_sys::LC_CPUcomponents_RZE_2
            }
            Self::ZeroRunBitmapEncoding { size: ElemSize::S4 } => {
                lc_framework_sys::LC_CPUcomponents_RZE_4
            }
            Self::ZeroRunBitmapEncoding { size: ElemSize::S8 } => {
                lc_framework_sys::LC_CPUcomponents_RZE_8
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElemSize {
    S1,
    S2,
    S4,
    S8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FloatSize {
    S4,
    S8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TupleSize {
    S1x2,
    S1x3,
    S1x4,
    S1x6,
    S1x8,
    S1x12,
    S2x2,
    S2x3,
    S2x4,
    S2x6,
    S4x2,
    S4x6,
    S8x3,
    S8x6,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn bit4_rle4() {
        let preprocessors = &[];
        let components = &[
            Component::BitShuffle { size: ElemSize::S4 },
            Component::RunLengthEncoding { size: ElemSize::S4 },
        ];

        let data = b"abcd";

        let encoded = compress(preprocessors, components, data).unwrap();
        let decoded = decompress(preprocessors, components, &encoded).unwrap();

        assert_eq!(decoded, data);
    }
}
