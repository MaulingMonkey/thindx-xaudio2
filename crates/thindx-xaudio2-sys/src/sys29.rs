//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

use super::xaudio2_8 as prev;
use abistr::*;
use winapi::shared::guiddef::GUID;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL        : &'static str              =           "xaudio2_9.dll";
#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL_A      : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_9.dll");
#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL_W      : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_9.dll");

#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL      : &'static str              =           "xaudio2_9d.dll";
#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL_A    : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_9d.dll");
#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL_W    : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_9d.dll");

#[doc = "`2B02E3CF-2E0B-4ec3-BE45-1B2A3FE7210D`"] pub const IID_IXAudio2          : GUID = super::guid(0x2B02E3CF, 0x2E0B, 0x4ec3, 0xBE451B2A3FE7210D);
#[doc = "`84ac29bb-d619-44d2-b197-e4acf7df3ed6`"] pub const IID_IXAudio2Extension : GUID = super::guid(0x84ac29bb, 0xd619, 0x44d2, 0xb197e4acf7df3ed6);

// Numeric boundary values
pub use prev::{
    XAUDIO2_MAX_BUFFER_BYTES,
    XAUDIO2_MAX_QUEUED_BUFFERS,
    XAUDIO2_MAX_BUFFERS_SYSTEM,
    XAUDIO2_MAX_AUDIO_CHANNELS,
    XAUDIO2_MIN_SAMPLE_RATE,
    XAUDIO2_MAX_SAMPLE_RATE,
    XAUDIO2_MAX_VOLUME_LEVEL,
    XAUDIO2_MIN_FREQ_RATIO,
    XAUDIO2_MAX_FREQ_RATIO,
    XAUDIO2_DEFAULT_FREQ_RATIO,
    XAUDIO2_MAX_FILTER_ONEOVERQ,
    XAUDIO2_MAX_FILTER_FREQUENCY,
    XAUDIO2_MAX_LOOP_COUNT,
    XAUDIO2_MAX_INSTANCES,
};

pub use prev::{
    XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO,
    XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL,
};

// Numeric values with special meanings
pub use prev::{
    XAUDIO2_COMMIT_NOW,
    XAUDIO2_COMMIT_ALL,
    XAUDIO2_INVALID_OPSET,
    XAUDIO2_NO_LOOP_REGION,
    XAUDIO2_LOOP_INFINITE,
    XAUDIO2_DEFAULT_CHANNELS,
    XAUDIO2_DEFAULT_SAMPLERATE,
    XAUDIO2_DEFAULT_AUDIO_CATEGORY,
};



// Flags
pub use prev::{
    XAUDIO2_VOICE_NOPITCH,
    XAUDIO2_VOICE_NOSRC,
    XAUDIO2_VOICE_USEFILTER,
    XAUDIO2_PLAY_TAILS,
    XAUDIO2_END_OF_STREAM,
    XAUDIO2_SEND_USEFILTER,
    XAUDIO2_VOICE_NOSAMPLESPLAYED,
};

/// Used in XAudio2Create
pub const XAUDIO2_DEBUG_ENGINE : u32 = 0x0001;

/// Used in XAudio2Create to force the engine to Stop when no source voices are Started, and Start when a voice is Started
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE : u32 = 0x2000;

/// Used in XAudio2Create to specify nondefault processing quantum of 21.33 ms (1024 samples at 48KHz)
pub const XAUDIO2_1024_QUANTUM : u32 = 0x8000;

/// Used in CreateMasteringVoice to create a virtual audio client
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT : u32 = 0x10000;



// Default parameters for the built-in filter
pub use prev::{
    XAUDIO2_DEFAULT_FILTER_TYPE,
    XAUDIO2_DEFAULT_FILTER_FREQUENCY,
    XAUDIO2_DEFAULT_FILTER_ONEOVERQ,
};

// Internal XAudio2 constants
pub use prev::{
    XAUDIO2_QUANTUM_NUMERATOR,
    XAUDIO2_QUANTUM_DENOMINATOR,
    XAUDIO2_QUANTUM_MS,
};

// XAudio2 error codes
pub use prev::{
    FACILITY_XAUDIO2,
    XAUDIO2_E_INVALID_CALL,
    XAUDIO2_E_XMA_DECODER_ERROR,
    XAUDIO2_E_XAPO_CREATION_FAILED,
    XAUDIO2_E_DEVICE_INVALIDATED,
};



// Used in XAudio2Create, specifies which CPU(s) to use.
pub use prev::{
    XAUDIO2_PROCESSOR,
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
    XAUDIO2_ANY_PROCESSOR,
};

/// Allow XAudio2 to choose the processor.
///
/// This constant won't work on all platforms (e.g. will fail with [XAUDIO2_E_INVALID_CALL] on Windows Server 2019), so a fallback on [XAUDIO2_DEFAULT_PROCESSOR] is appropriate.
pub const XAUDIO2_USE_DEFAULT_PROCESSOR : u32 = 0x00000000;

/// A hardcoded "default" processor (e.g. [Processor1].)
///
/// Implementations targeting Games and WIN10_19H1 and later should try [XAUDIO2_USE_DEFAULT_PROCESSOR] first to let XAudio2 select the appropriate default processor for the hardware platform.
/// That won't work on all platforms (e.g. will fail with [XAUDIO2_E_INVALID_CALL] on Windows Server 2019), so a fallback on [XAUDIO2_DEFAULT_PROCESSOR] is appropriate.
pub const XAUDIO2_DEFAULT_PROCESSOR : u32 = Processor1;

pub use prev::{
    XAUDIO2_VOICE_DETAILS,
    XAUDIO2_SEND_DESCRIPTOR,
    XAUDIO2_VOICE_SENDS,
    XAUDIO2_EFFECT_DESCRIPTOR,
    XAUDIO2_EFFECT_CHAIN,
    XAUDIO2_FILTER_TYPE,
    LowPassFilter,
    BandPassFilter,
    HighPassFilter,
    NotchFilter,
    LowPassOnePoleFilter,
    HighPassOnePoleFilter,
    XAUDIO2_FILTER_PARAMETERS,
    XAUDIO2_BUFFER,
    XAUDIO2_BUFFER_WMA,
    XAUDIO2_VOICE_STATE,
    XAUDIO2_PERFORMANCE_DATA,
    XAUDIO2_DEBUG_CONFIGURATION,
};

pub use prev::{
    XAUDIO2_LOG_ERRORS,
    XAUDIO2_LOG_WARNINGS,
    XAUDIO2_LOG_INFO,
    XAUDIO2_LOG_DETAIL,
    XAUDIO2_LOG_API_CALLS,
    XAUDIO2_LOG_FUNC_CALLS,
    XAUDIO2_LOG_TIMING,
    XAUDIO2_LOG_LOCKS,
    XAUDIO2_LOG_MEMORY,
    XAUDIO2_LOG_STREAMING,
};

pub use prev::{
    IXAudio2, // Has non-semantic changes
    IXAudio2Vtbl,
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

interfaces! {
    /// \[<strike>microsoft.com</strike>\]
    /// Extends [IXAudio2] with additional 2.9+ specific functionality.
    ///
    /// Use [IUnknown]::[QueryInterface](IUnknown::QueryInterface) to obtain a pointer to this interface.
    #[iid = IID_IXAudio2Extension]
    pub interface IXAudio2Extension(IXAudio2ExtensionVtbl) => unsafe IUnknown(IUnknownVtbl) {
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable#duration-of-audio-processing-quantum)\]
        /// Returns the processing quantum
        /// quantumMilliseconds = (1000.0f * quantumNumerator / quantumDenominator)
        ///
        /// ### Arguments
        /// * `quantumNumerator`    - Quantum numerator
        /// * `quantumDenominator`  - Quantum denominator
        pub unsafe fn GetProcessingQuantum(&self, quantumNumerator: *mut u32, quantumDenominator: *mut u32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable#preferred-cpu-core)\]
        /// Returns the number of the processor used by XAudio2
        ///
        /// ### Arguments
        /// * `processor`           - Non-zero Processor number
        pub unsafe fn GetProcessor(&self, processor: *mut XAUDIO2_PROCESSOR) -> ();
    }
}

#[cfg(feature = "helper-functions")] pub use prev::{
    XAudio2DecibelsToAmplitudeRatio,
    XAudio2AmplitudeRatioToDecibels,
    XAudio2SemitonesToFrequencyRatio,
    XAudio2FrequencyRatioToSemitones,
    XAudio2CutoffFrequencyToRadians,            // Has non-semantic changes
    XAudio2RadiansToCutoffFrequency,
    XAudio2CutoffFrequencyToOnePoleCoefficient, // Has non-semantic changes
};
