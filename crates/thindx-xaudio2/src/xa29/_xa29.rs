//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

use super::xaudio2_8 as prev;

mod exports;
mod ixaudio2extension_ext;          pub use ixaudio2extension_ext::*;

// Might not remain pub
#[doc(hidden)] pub use xaudio2::sys::{
    IXAudio2,
    IXAudio2Vtbl,
    IXAudio2Extension,
    IXAudio2ExtensionVtbl,
    IXAudio2Voice,
    IXAudio2VoiceVtbl,
    IXAudio2SourceVoice,
    IXAudio2SourceVoiceVtbl,
    IXAudio2SubmixVoice,
    IXAudio2SubmixVoiceVtbl,
    IXAudio2MasteringVoice,
    IXAudio2MasteringVoiceVtbl,
    IXAudio2EngineCallback,
    IXAudio2EngineCallbackVtbl,
    IXAudio2VoiceCallback,
    IXAudio2VoiceCallbackVtbl,
};

/// `XAudio2*` & `XAUDIO2_*`
pub mod xaudio2 {
    use super::*;
    use winresult::*;

    pub use exports::*;
    #[doc(inline)] pub use prev::xaudio2::{
        // Structs
        Context,
        DebugConfiguration,
        EffectDescriptor,
        EngineCallbackWrapper,
        FilterParameters,
        LoopCount,
        MasteringVoice,
        PerformanceData,
        SampleRange,
        SendDescriptor,
        SourceFormat,
        SourceVoice,
        SourceVoiceUntyped,
        SubmixVoice,
        TypedSourceFormat,
        Voice,
        VoiceCallbackWrapper,
        VoiceDetails,
        VoiceState,

        // Traits
        EngineCallback,
        HasPcmWaveFormat,
        VoiceCallback,
    };

    /// Raw low level FFI bindings
    ///
    pub use thindx_xaudio2_sys::xaudio2_9 as sys;

    pub use sys::XAUDIO2_DLL    as DLL;
    pub use sys::XAUDIO2_DLL_A  as DLL_A;
    pub use sys::XAUDIO2_DLL_W  as DLL_W;

    pub use sys::XAUDIO2D_DLL    as D_DLL;
    pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
    pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;

    pub use prev::xaudio2::{
        MAX_BUFFER_BYTES,
        MAX_QUEUED_BUFFERS,
        MAX_BUFFERS_SYSTEM,
        MAX_AUDIO_CHANNELS,
        MIN_SAMPLE_RATE,
        MAX_SAMPLE_RATE,
        MAX_VOLUME_LEVEL,
        MIN_FREQ_RATIO,
        MAX_FREQ_RATIO,
        DEFAULT_FREQ_RATIO,
        MAX_FILTER_ONEOVERQ,
        MAX_FILTER_FREQUENCY,
        MAX_LOOP_COUNT,
        MAX_INSTANCES,
        MAX_RATIO_TIMES_RATE_XMA_MONO,
        MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL,
        COMMIT_NOW,
        COMMIT_ALL,
        INVALID_OPSET,
        NO_LOOP_REGION,
        LOOP_INFINITE,
        DEFAULT_CHANNELS,
        DEFAULT_SAMPLERATE,
        DEFAULT_AUDIO_CATEGORY,
        VOICE_NOPITCH,
        VOICE_NOSRC,
        VOICE_USEFILTER,
        PLAY_TAILS,
        END_OF_STREAM,
        SEND_USEFILTER,
        VOICE_NOSAMPLESPLAYED,
        DEFAULT_FILTER_TYPE,
        DEFAULT_FILTER_FREQUENCY,
        DEFAULT_FILTER_ONEOVERQ,
        QUANTUM_NUMERATOR,
        QUANTUM_DENOMINATOR,
        QUANTUM_MS,
        FACILITY,
        E_INVALID_CALL,
        E_XMA_DECODER_ERROR,
        E_XAPO_CREATION_FAILED,
        E_DEVICE_INVALIDATED,
    };

    pub use prev::xaudio2::{
        Processor,
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
    };
    pub use sys::{
        XAUDIO2_ANY_PROCESSOR           as ANY_PROCESSOR,
        XAUDIO2_USE_DEFAULT_PROCESSOR   as USE_DEFAULT_PROCESSOR,
        XAUDIO2_DEFAULT_PROCESSOR       as DEFAULT_PROCESSOR,
    };

    #[cfg(feature = "helper-functions")] pub use prev::xaudio2::{
        decibels_to_amplitude_ratio,
        amplitude_ratio_to_decibels,
        semitones_to_frequency_ratio,
        frequency_ratio_to_semitones,
        cutoff_frequency_to_radians,
        radians_to_cutoff_frequency,
        cutoff_frequency_to_one_pole_coefficient,
    };

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create)\] XAudio2Create:
    /// Creates a new [IXAudio2] instance, which you can use to start using XAudio2.
    ///
    /// Initialize COM (see e.g. [mcom::init::mta]) before calling this function.
    /// In XAudio 2.7 and earlier, XAudio2Create immediately creates a COM object and will fail if COM is not initialized.
    /// In XAudio 2.8 and later, this call may succeed, but basic operations like creating voices will fail with e.g. [CO::E_NOTINITIALIZED].
    ///
    /// ### Arguments
    /// *   `flags`     - Must be [None]
    /// *   `processor` - The processor(s) to run XAudio2's worker thread(s) on.
    ///
    ///     | Value                     | Description   |
    ///     | ------------------------- | ------------- |
    ///     | [USE_DEFAULT_PROCESSOR]   | Let XAudio2 choose the core (requires WIN10_19H1+ to avoid [xaudio2::E_INVALID_CALL])  |
    ///     | [DEFAULT_PROCESSOR]       | Hardcoded SDK default processor (e.g. [Processor1])   |
    ///     | [ANY_PROCESSOR]           | ⚠️ Spawn threads for every processor/core!  (Excessive!)  |
    ///     | [Processor1]              | Run specifically on processor/core 1                  |
    ///     |   ..                      |   ..                                                  |
    ///     | [Processor32]             | Run specifically on processor/core 32                 |
    ///
    /// ### Example
    /// ```
    /// use thindx_xaudio2::xaudio2_9::*;
    ///
    /// let xaudio2 = xaudio2::create(None, xaudio2::USE_DEFAULT_PROCESSOR);
    /// let xaudio2 = xaudio2.or_else(|_| xaudio2::create(None, xaudio2::DEFAULT_PROCESSOR));
    /// let xaudio2 = xaudio2.expect("xaudio2::create");
    /// ```
    ///
    /// ### Errors
    /// *   [HResultError::from_win32]\([ERROR::BAD_EXE_FORMAT])    - if `XAudio2_9.dll` had the wrong architecture (perhaps a 32-bit DLL in a 64-bit process or vicea versa?)
    /// *   [HResultError::from_win32]\([ERROR::MOD_NOT_FOUND])     - if `XAudio2_9.dll` could not be found
    /// *   [HResultError::from_win32]\([ERROR::INVALID_LIBRARY])   - if `XAudio2_9.dll` loading failed to load in a strange way
    /// *   [HResultError::from_win32]\([ERROR::PROC_NOT_FOUND])    - if `XAudio2_9.dll` failed to export `XAudio2CreateWithVersionInformation` or `XAudio2Create`
    /// *   [HResultError::from_win32]\([ERROR::NOINTERFACE])       - if [IXAudio2] was null despite the function "succeeding" (thindx specific)
    /// *   [xaudio2::E_INVALID_CALL]                               - if `processor` is invalid (e.g. specified [xaudio2::USE_DEFAULT_PROCESSOR] on Windows Server 2019)
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

#[doc(inline)] pub use prev::{
    // Structs
    IXAudio2SourceVoiceTyped,

    // Traits
    IXAudio2Ext,
    IXAudio2MasteringVoiceExt,
    IXAudio2SourceVoiceExt,
    IXAudio2VoiceExt,
};
