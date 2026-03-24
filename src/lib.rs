//! [![CI Status]][workflow] [![MSRV]][repo] [![Latest Version]][crates.io]
//! [![Rust Doc Crate]][docs.rs] [![Rust Doc Main]][docs]
//!
//! [CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/lc-framework-rs/ci.yml?branch=main
//! [workflow]: https://github.com/juntyr/lc-framework-rs/actions/workflows/ci.yml?query=branch%3Amain
//!
//! [MSRV]: https://img.shields.io/badge/MSRV-1.85.0-blue
//! [repo]: https://github.com/juntyr/lc-framework-rs
//!
//! [Latest Version]: https://img.shields.io/crates/v/lc-framework
//! [crates.io]: https://crates.io/crates/lc-framework
//!
//! [Rust Doc Crate]: https://img.shields.io/docsrs/lc-framework
//! [docs.rs]: https://docs.rs/lc-framework/
//!
//! [Rust Doc Main]: https://img.shields.io/badge/docs-main-blue
//! [docs]: https://juntyr.github.io/lc-framework-rs/lc_framework
//!
//! # lc-framework
//!
//! High-level Rust bindigs to the [LC] compression framework.
//!
//! [LC]: https://github.com/burtscher/LC-framework

use std::ffi::c_longlong;

/// Maximum number of components
pub const MAX_COMPONENTS: usize = lc_framework_sys::MAX_STAGES;

/// Maximum number of bytes
pub const MAX_BYTES: usize = const {
    #[allow(clippy::cast_possible_truncation)]
    if std::mem::size_of::<c_longlong>() <= std::mem::size_of::<usize>() {
        c_longlong::MAX as usize
    } else {
        usize::MAX
    }
};

/// Compress the `input` data with LC using zero or more `preprocessors` and
/// one or more `components`.
///
/// # Errors
///
/// Errors with
/// - [`Error::TooFewComponents`] if no `components` are given
/// - [`Error::TooManyComponents`] if too many `components` are given
/// - [`Error::ExcessiveInputData`] if the `input` data is too large
/// - [`Error::CompressionFailed`] if compression with LC failed
/// - [`Error::ExcessiveCompressedData`] if the compressed data is too large
pub fn compress(
    preprocessors: &[Preprocessor],
    components: &[Component],
    input: &[u8],
) -> Result<Vec<u8>, Error> {
    let mut preprocessor_ids = Vec::with_capacity(preprocessors.len());
    let mut preprocessor_params = Vec::new();
    let mut preprocessor_params_num = Vec::with_capacity(preprocessors.len());
    for preprocessor in preprocessors {
        preprocessor_ids.push(preprocessor.as_id());
        let preprocessor_nparams_sum = preprocessor_params.len();
        preprocessor.push_params(&mut preprocessor_params);
        preprocessor_params_num.push(preprocessor_params.len() - preprocessor_nparams_sum);
    }

    if components.is_empty() {
        return Err(Error::TooFewComponents);
    }

    if components.len() > MAX_COMPONENTS {
        return Err(Error::TooManyComponents);
    }

    let component_ids = components
        .iter()
        .copied()
        .map(Component::as_id)
        .collect::<Vec<_>>();

    let input_size: c_longlong = input
        .len()
        .try_into()
        .map_err(|_| Error::ExcessiveInputData)?;

    let mut encoded_ptr = std::ptr::null_mut();
    let mut encoded_size = 0;

    #[expect(unsafe_code)]
    // SAFETY: all pointers and lengths are valid
    let status = unsafe {
        lc_framework_sys::lc_compress(
            preprocessor_ids.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_params_num.as_ptr(),
            preprocessor_params.as_ptr(),
            component_ids.len(),
            component_ids.as_ptr(),
            input.as_ptr(),
            input_size,
            &raw mut encoded_ptr,
            &raw mut encoded_size,
        )
    };

    if status != 0 {
        return Err(Error::CompressionFailed);
    }

    let encoded_len: usize = encoded_size
        .try_into()
        .map_err(|_| Error::ExcessiveCompressedData)?;

    #[expect(unsafe_code)]
    // SAFETY: all Vec elements are initialised via the copy
    let encoded = unsafe {
        let mut encoded = Vec::with_capacity(encoded_len);
        std::ptr::copy_nonoverlapping(encoded_ptr.cast_const(), encoded.as_mut_ptr(), encoded_len);
        encoded.set_len(encoded_len);
        encoded
    };

    #[expect(unsafe_code)]
    // SAFETY: encoded_ptr was allocated by LC
    unsafe {
        lc_framework_sys::lc_free_bytes(encoded_ptr);
    }

    Ok(encoded)
}

/// Dempress the `compressed` data with LC using zero or more `preprocessors`
/// and one or more `components`.
///
/// The `compressed` data must have been [`compress`]ed using the same
/// `preprocessors` and `components`.
///
/// # Errors
///
/// Errors with
/// - [`Error::TooFewComponents`] if no `components` are given
/// - [`Error::TooManyComponents`] if too many `components` are given
/// - [`Error::ExcessiveCompressedData`] if the `compressed` data is too large
/// - [`Error::DecompressionFailed`] if decompression with LC failed
/// - [`Error::ExcessiveDecompressedData`] if the decompressed data is too
///   large
pub fn decompress(
    preprocessors: &[Preprocessor],
    components: &[Component],
    compressed: &[u8],
) -> Result<Vec<u8>, Error> {
    let encoded = compressed;

    let mut preprocessor_ids = Vec::with_capacity(preprocessors.len());
    let mut preprocessor_params = Vec::new();
    let mut preprocessor_params_num = Vec::with_capacity(preprocessors.len());
    for preprocessor in preprocessors {
        preprocessor_ids.push(preprocessor.as_id());
        let preprocessor_nparams_sum = preprocessor_params.len();
        preprocessor.push_params(&mut preprocessor_params);
        preprocessor_params_num.push(preprocessor_params.len() - preprocessor_nparams_sum);
    }

    if components.is_empty() {
        return Err(Error::TooFewComponents);
    }

    if components.len() > MAX_COMPONENTS {
        return Err(Error::TooManyComponents);
    }

    let component_ids = components
        .iter()
        .copied()
        .map(Component::as_id)
        .collect::<Vec<_>>();

    let encoded_size: c_longlong = encoded
        .len()
        .try_into()
        .map_err(|_| Error::ExcessiveCompressedData)?;

    let mut decoded_ptr = std::ptr::null_mut();
    let mut decoded_size = 0;

    #[expect(unsafe_code)]
    // SAFETY: all pointers and lengths are valid
    let status = unsafe {
        lc_framework_sys::lc_decompress(
            preprocessor_ids.len(),
            preprocessor_ids.as_ptr(),
            preprocessor_params_num.as_ptr(),
            preprocessor_params.as_ptr(),
            component_ids.len(),
            component_ids.as_ptr(),
            encoded.as_ptr(),
            encoded_size,
            &raw mut decoded_ptr,
            &raw mut decoded_size,
        )
    };

    if status != 0 {
        return Err(Error::DecompressionFailed);
    }

    let decoded_len: usize = decoded_size
        .try_into()
        .map_err(|_| Error::ExcessiveDecompressedData)?;

    #[expect(unsafe_code)]
    // SAFETY: all Vec elements are initialised via the copy
    let decoded = unsafe {
        let mut decoded = Vec::with_capacity(decoded_len);
        std::ptr::copy_nonoverlapping(decoded_ptr.cast_const(), decoded.as_mut_ptr(), decoded_len);
        decoded.set_len(decoded_len);
        decoded
    };

    #[expect(unsafe_code)]
    // SAFETY: encoded_ptr was allocated by LC
    unsafe {
        lc_framework_sys::lc_free_bytes(decoded_ptr);
    }

    Ok(decoded)
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur during compression and decompression with LC
pub enum Error {
    /// at least one component must be given
    #[error("at least one component must be given")]
    TooFewComponents,
    /// too many components were given
    #[error("at most {MAX_COMPONENTS} components must be given")]
    TooManyComponents,
    /// input data is too large
    #[error("input data must not exceed {MAX_BYTES} bytes")]
    ExcessiveInputData,
    /// internal compression error
    #[error("internal compression error")]
    CompressionFailed,
    /// compressed data is too large
    #[error("compressed data must not exceed {MAX_BYTES} bytes")]
    ExcessiveCompressedData,
    /// internal decompression error
    #[error("internal decompression error")]
    DecompressionFailed,
    /// decompressed data is too large
    #[error("decompressed data must not exceed {MAX_BYTES} bytes")]
    ExcessiveDecompressedData,
}

#[expect(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
/// LC preprocessor
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
    const fn as_id(&self) -> lc_framework_sys::LC_CPUpreprocessor {
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
/// LC error bound kind
pub enum ErrorKind {
    /// pointwise absolute error bound
    Abs,
    /// pointwise normalised absolute / data-range-relative error bound
    Noa,
    /// pointwise relative error bound
    Rel,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC quantisation decorrelation mode
pub enum Decorrelation {
    Zero,
    Random,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC Lorenzo preprocessor dtype
pub enum LorenzoDtype {
    I32,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC quantization dtype
pub enum QuantizeDType {
    F32,
    F64,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC component
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
    #[expect(clippy::too_many_lines)]
    const fn as_id(self) -> lc_framework_sys::LC_CPUcomponents {
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

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC component element size, in bytes
pub enum ElemSize {
    S1,
    S2,
    S4,
    S8,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC component float element size, in bytes
pub enum FloatSize {
    S4,
    S8,
}

#[expect(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// LC tuple component element size, in bytes x tuple length
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
