use std::ffi::CString;

use espeakng_sys::{
    espeakINITIALIZE_DONT_EXIT, espeakINITIALIZE_PHONEME_IPA,
    espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL, espeak_ERROR_EE_OK,
};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone)]
pub struct Error(pub String);
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "eSpeak-ng Error :{}", self.0)
    }
}

fn initialize() -> Result<()> {
    static IS_INITIALIZED: std::sync::Once = std::sync::Once::new();
    if IS_INITIALIZED.is_completed() {
        return Ok(());
    } else {
        IS_INITIALIZED.call_once(|| {});
    }

    let options = espeakINITIALIZE_DONT_EXIT as i32 | espeakINITIALIZE_PHONEME_IPA as i32;
    let result = unsafe {
        espeakng_sys::espeak_Initialize(
            espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL,
            0,
            std::ptr::null(),
            options,
        )
    };

    unsafe {
        if espeakng_sys::espeak_SetVoiceByName(c"en-us".as_ptr()) != espeak_ERROR_EE_OK {
            return Err(Error("Failed to set voice".to_string()));
        }
    }

    if result < 0 {
        Err(Error(format!("Failed to initialize eSpeak-ng: {result}")))
    } else {
        Ok(())
    }
}

pub fn text_to_phonemes(text: &str) -> Result<String> {
    initialize()?;

    let text =
        CString::new(text).map_err(|_| Error("Invalid text (contains null byte)".to_string()))?;
    let mut tptr = text.as_ptr();
    let text_c_char_ptr = std::ptr::addr_of_mut!(tptr);

    let result = unsafe {
        espeakng_sys::espeak_TextToPhonemes(
            text_c_char_ptr as *mut _,
            espeakng_sys::espeakCHARS_UTF8 as i32,
            espeakng_sys::espeakINITIALIZE_PHONEME_IPA as i32,
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
