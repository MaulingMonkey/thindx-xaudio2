//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

mod xa29_exports;

// Might not remain pub
#[doc(hidden)] pub use xaudio2_sys::{
    IXAudio2,
    IXAudio2Extension,
};

/// Raw low level FFI bindings
///
pub use thindx_xaudio2_sys::xaudio2_9 as xaudio2_sys;

/// `XAudio2*` & `XAUDIO2_*`
pub mod xaudio2 {
    use super::*;
    use winresult::*;

    pub use xa29_exports::*;
    pub use super::xaudio2_sys as sys;

    pub use sys::XAUDIO2_DLL    as DLL;
    pub use sys::XAUDIO2_DLL_A  as DLL_A;
    pub use sys::XAUDIO2_DLL_W  as DLL_W;

    pub use sys::XAUDIO2D_DLL    as D_DLL;
    pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
    pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;

    pub use sys::{
        XAUDIO2_PROCESSOR  as Processor,
        Processor1,
        Processor2,
        Processor3,
        Processor4,
        Processor5,
        Processor6,
        Processor7,
        Processor8,
        Processor9,
        Processor10,
        Processor11,
        Processor12,
        Processor13,
        Processor14,
        Processor15,
        Processor16,
        Processor17,
        Processor18,
        Processor19,
        Processor20,
        Processor21,
        Processor22,
        Processor23,
        Processor24,
        Processor25,
        Processor26,
        Processor27,
        Processor28,
        Processor29,
        Processor30,
        Processor31,
        Processor32,
        XAUDIO2_USE_DEFAULT_PROCESSOR as USE_DEFAULT_PROCESSOR,
    };
    #[allow(deprecated)] #[doc(hidden)] pub use sys::XAUDIO2_DEFAULT_PROCESSOR as DEFAULT_PROCESSOR;

    #[cfg(feature = "helper-functions")] pub use sys::{
        XAudio2DecibelsToAmplitudeRatio             as decibels_to_amplitude_ratio,
        XAudio2AmplitudeRatioToDecibels             as amplitude_ratio_to_decibels,
        XAudio2SemitonesToFrequencyRatio            as semitones_to_frequency_ratio,
        XAudio2FrequencyRatioToSemitones            as frequency_ratio_to_semitones,
        XAudio2CutoffFrequencyToRadians             as cutoff_frequency_to_radians,
        XAudio2RadiansToCutoffFrequency             as radians_to_cutoff_frequency,
        XAudio2CutoffFrequencyToOnePoleCoefficient  as cutoff_frequency_to_one_pole_coefficient,
    };

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create)\] XAudio2Create:
    /// Creates a new [IXAudio2] instance, which you can use to start using XAudio2.
    ///
    /// | Argument  | Description                                                                                       |
    /// | --------- | ------------------------------------------------------------------------------------------------- |
    /// | flags     | Must be [None]                                                                                    |
    /// | processor | [Processor1] ..= [Processor32], <strike>[DEFAULT_PROCESSOR],</strike> or [USE_DEFAULT_PROCESSOR]  |
    ///
    /// ### Example
    /// ```
    /// use thindx_xaudio2::xaudio2_9::{xaudio2, xaudio2::sys::IXAudio2Extension};
    ///
    /// let xaudio2 = xaudio2::create(None, xaudio2::USE_DEFAULT_PROCESSOR).unwrap();
    /// let ext = xaudio2.try_cast::<IXAudio2Extension>().unwrap();
    /// ```
    ///
    /// ### Errors
    /// *   [ERROR::BAD_EXE_FORMAT]     - if `XAudio2_9.dll` had the wrong architecture (perhaps a 32-bit DLL in a 64-bit process or vicea versa?)
    /// *   [ERROR::MOD_NOT_FOUND]      - if `XAudio2_9.dll` could not be found
    /// *   [ERROR::INVALID_LIBRARY]    - if `XAudio2_9.dll` loading failed to load in a strange way
    /// *   [ERROR::PROC_NOT_FOUND]     - if `XAudio2_9.dll` failed to export `XAudio2CreateWithVersionInformation` or `XAudio2Create`
    /// *   [ERROR::NOINTERFACE]        - if [IXAudio2] was null despite the function "succeeding" (thindx specific)
    pub fn create(flags: Option<core::convert::Infallible>, processor: Processor) -> Result<mcom::Rc<sys::IXAudio2>, HResultError> {
        #![allow(non_snake_case)]

        let exports = match Exports::from_default_path_cached() {
            Ok(e) => e,
            Err(err) => {
                let code = if let Some(code) = err.raw_os_error() {
                    let code = code as u32;
                    if let Ok(code) = u16::try_from(code) {
                        ErrorCode::from(code)
                    } else {
                        return Err(HResultError::from(code));
                    }
                } else {
                    ERROR::INVALID_LIBRARY
                };
                return Err(HResultError::from_win32(code));
            }
        };

        let mut xaudio2 = core::ptr::null_mut();
        let _ = flags;
        let flags = 0;
        const NTDDI_VERSION : u32 = 0x0A00000C; // NTDDI_WIN10_NI - see C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\shared\sdkddkver.h

        let hr = if let Some(XAudio2CreateWithVersionInfo) = exports.XAudio2CreateWithVersionInfo {
            unsafe { XAudio2CreateWithVersionInfo(&mut xaudio2, flags, processor, NTDDI_VERSION) }
        } else if let Some(XAudio2Create) = exports.XAudio2Create {
            unsafe { XAudio2Create(&mut xaudio2, flags, processor) }
        } else {
            // The real SDK uses GetLastError() after GetProcAddress fails.
            // I instead hardcode the error code, since `exports` might be a cached copy of XAudio2's exports, loaded long ago.
            HResultError::from_win32(ERROR::PROC_NOT_FOUND).into()
        };

        let xaudio2 = unsafe { mcom::Rc::from_raw_opt(xaudio2) };
        hr.succeeded()?;
        let xaudio2 = xaudio2.ok_or(HResultError::from_win32(ERROR::NOINTERFACE))?; // XAudio2Create "succeeded" but gave us a null ptr?
        Ok(xaudio2)
    }
}
