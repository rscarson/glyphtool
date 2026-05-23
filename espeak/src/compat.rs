//# Compatibility bridge for MSVC / POSIX naming discrepancies
#![cfg(target_env = "msvc")]

use std::os::raw::{c_char, c_int};

// Link the standard internal MSVC runtime functions
extern "C" {
    fn _strdup(s1: *const c_char) -> *mut c_char;
    fn _stricmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

// Export the exact symbol 'strdup' that espeak_mini.lib expects
#[no_mangle]
pub unsafe extern "C" fn strdup(s1: *const c_char) -> *mut c_char {
    _strdup(s1)
}

// Export the exact symbol 'stricmp' that espeak_mini.lib expects
#[no_mangle]
pub unsafe extern "C" fn stricmp(s1: *const c_char, s2: *const c_char) -> c_int {
    _stricmp(s1, s2)
}
