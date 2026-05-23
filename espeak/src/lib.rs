use std::{ffi::CString, sync::OnceLock};

use bindings::PHONEME_CONFIGS;

mod bindings;

mod compat;

//
// Error type
pub type EspeakResult<T> = Result<T, Error>;
#[derive(Debug, Clone)]
pub struct Error(pub String);
impl Error {
    pub fn from_raw(raw: i32) -> EspeakResult<()> {
        if raw >= 0 {
            Ok(())
        } else {
            Err(Error(format!("Failed with error code: {}", raw)))
        }
    }
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "eSpeak-ng Error :{}", self.0)
    }
}

//
// E-speak mini data files
const EN_DICT: &[u8] = include_bytes!("../data/en_dict");
const EN_US: &[u8] = include_bytes!("../data/en-US");
const INTONATIONS: &[u8] = include_bytes!("../data/intonations");
const PHONDATA: &[u8] = include_bytes!("../data/phondata");
const PHONINDEX: &[u8] = include_bytes!("../data/phonindex");
const PHONTAB: &[u8] = include_bytes!("../data/phontab");

fn initialize() -> EspeakResult<()> {
    // Keeps the configurations structure alive in memory globally
    static INITIALIZE_CELL: OnceLock<EspeakResult<()>> = OnceLock::new();

    let res = INITIALIZE_CELL.get_or_init(|| {
        let mut configs = PHONEME_CONFIGS {
            intonations: INTONATIONS.as_ptr() as *mut i8,
            intonation_len: INTONATIONS.len() as i32,

            data: PHONDATA.as_ptr() as *mut i8,
            data_len: PHONDATA.len() as i32,

            index: PHONINDEX.as_ptr() as *mut i8,
            index_len: PHONINDEX.len() as i32,

            tab: PHONTAB.as_ptr() as *mut i8,
            tab_len: PHONTAB.len() as i32,
        };

        let result = unsafe {
            bindings::espeak_Initialize(
                bindings::espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL,
                0,
                &mut configs,
                (bindings::espeakINITIALIZE_DONT_EXIT | bindings::espeakINITIALIZE_PHONEME_IPA)
                    as i32,
            )
        };

        Error::from_raw(result)?;

        let result = unsafe {
            bindings::espeak_SetVoiceByBuffer(
                c"en-us".as_ptr(),
                EN_US.as_ptr() as *mut i8,
                EN_US.len() as i32,
                EN_DICT.as_ptr() as *mut i8,
                EN_DICT.len() as i32,
            )
        };

        Error::from_raw(result)
    });

    res.clone()
}

pub fn text_to_phonemes(text: &str) -> EspeakResult<String> {
    initialize()?;

    let text =
        CString::new(text).map_err(|_| Error("Invalid text (contains null byte)".to_string()))?;
    let mut tptr = text.as_ptr();
    let text_c_char_ptr = std::ptr::addr_of_mut!(tptr);

    let result = unsafe {
        bindings::espeak_TextToPhonemes(
            text_c_char_ptr.cast(),
            bindings::espeakCHARS_UTF8 as i32,
            bindings::espeakINITIALIZE_PHONEME_IPA as i32,
        )
    };

    if result.is_null() {
        Err(Error("Failed to convert text to phonemes".to_string()))
    } else {
        let mut phonemes = unsafe { std::ffi::CStr::from_ptr(result) }
            .to_string_lossy()
            .into_owned();

        // Remove stress
        phonemes = phonemes.replace(['ˈ', 'ˌ', 'ː'], "");

        Ok(phonemes)
    }
}
