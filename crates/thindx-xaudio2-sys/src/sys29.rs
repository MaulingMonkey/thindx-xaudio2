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
    //XAUDIO2_DEFAULT_FILTER_TYPE, // TODO
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
