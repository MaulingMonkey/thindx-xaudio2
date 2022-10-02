//! ✔️ XAudio 2.8 - Windows 8+ via preinstall
//!
//! Introduced in the [Windows 8 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#earlier-releases)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.8 (Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)
//! *   [XAudio2 and Windows 8](https://walbourn.github.io/xaudio2-and-windows-8/)

// don't use prev
use abistr::*;
use winapi::shared::guiddef::GUID;
use winresult::*;

#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL    : &'static str              =           "xaudio2_8.dll";
#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL_A  : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_8.dll");
#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL_W  : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_8.dll");

// There is no debug variant of xaudio2_8.dll.
// As such, these constants weren't initially defined for xaudio2.h.
// However, these are convenient if targeting 2.8 and 2.9 in the same build.
// As such, define them, but hide them from the documentation.
#[doc(hidden)] pub const XAUDIO2D_DLL   : &'static str              =           "xaudio2_8.dll";
#[doc(hidden)] pub const XAUDIO2D_DLL_A : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_8.dll");
#[doc(hidden)] pub const XAUDIO2D_DLL_W : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_8.dll");

#[doc = "`60d8dac8-5aa1-4e8e-b597-2f5e2883d484`"] pub const IID_IXAudio2 : GUID = super::guid(0x60d8dac8, 0x5aa1, 0x4e8e, 0xb5972f5e2883d484);



// Numeric boundary values

/// Maximum bytes allowed in a source buffer
pub const XAUDIO2_MAX_BUFFER_BYTES : u32 = 0x80000000;

/// Maximum buffers allowed in a voice queue
pub const XAUDIO2_MAX_QUEUED_BUFFERS : u32 = 64;

/// Maximum buffers allowed for system threads (Xbox 360 only)
pub const XAUDIO2_MAX_BUFFERS_SYSTEM : u32 = 2;

/// Maximum channels in an audio stream
pub const XAUDIO2_MAX_AUDIO_CHANNELS : u32 = 64;

/// Minimum audio sample rate supported
pub const XAUDIO2_MIN_SAMPLE_RATE : u32 = 1000;

/// Maximum audio sample rate supported
pub const XAUDIO2_MAX_SAMPLE_RATE : u32 = 200000;

/// Maximum acceptable volume level (2^24)
pub const XAUDIO2_MAX_VOLUME_LEVEL : f32 = 16777216.0;

/// Minimum SetFrequencyRatio argument
pub const XAUDIO2_MIN_FREQ_RATIO : f32 = 1.0 / 1024.0;

/// Maximum MaxFrequencyRatio argument
pub const XAUDIO2_MAX_FREQ_RATIO : f32 = 1024.0;

/// Default MaxFrequencyRatio argument
pub const XAUDIO2_DEFAULT_FREQ_RATIO : f32 = 2.0;

/// Maximum XAUDIO2_FILTER_PARAMETERS.OneOverQ
pub const XAUDIO2_MAX_FILTER_ONEOVERQ : f32 = 1.5;

/// Maximum XAUDIO2_FILTER_PARAMETERS.Frequency
pub const XAUDIO2_MAX_FILTER_FREQUENCY : f32 = 1.0;

/// Maximum non-infinite XAUDIO2_BUFFER.LoopCount
pub const XAUDIO2_MAX_LOOP_COUNT : u32 = 254;

/// Maximum simultaneous XAudio2 objects on Xbox 360
pub const XAUDIO2_MAX_INSTANCES : u32 = 8;



// For XMA voices on Xbox 360 there is an additional restriction on the MaxFrequencyRatio
// argument and the voice's sample rate: the product of these numbers cannot exceed 600000
// for one-channel voices or 300000 for voices with more than one channel.

pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO         : u32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL : u32 = 300000;



// Numeric values with special meanings

/// Used as an OperationSet argument
pub const XAUDIO2_COMMIT_NOW : u32 = 0;

/// Used in IXAudio2::CommitChanges
pub const XAUDIO2_COMMIT_ALL : u32 = 0;

/// Not allowed for OperationSet arguments
pub const XAUDIO2_INVALID_OPSET : u32 = -1_i32 as u32;

/// Used in XAUDIO2_BUFFER.LoopCount
pub const XAUDIO2_NO_LOOP_REGION : u32 = 0;

/// Used in XAUDIO2_BUFFER.LoopCount
pub const XAUDIO2_LOOP_INFINITE : u32 = 255;

/// Used in CreateMasteringVoice
pub const XAUDIO2_DEFAULT_CHANNELS : u32 = 0;

/// Used in CreateMasteringVoice
pub const XAUDIO2_DEFAULT_SAMPLERATE : u32 = 0;



// Flags

/// Used in IXAudio2::CreateSourceVoice
pub const XAUDIO2_VOICE_NOPITCH : u32 = 0x0002;

/// Used in IXAudio2::CreateSourceVoice
pub const XAUDIO2_VOICE_NOSRC : u32 = 0x0004;

/// Used in IXAudio2::CreateSource/SubmixVoice
pub const XAUDIO2_VOICE_USEFILTER : u32 = 0x0008;

/// Used in IXAudio2SourceVoice::Stop
pub const XAUDIO2_PLAY_TAILS : u32 = 0x0020;

/// Used in XAUDIO2_BUFFER.Flags
pub const XAUDIO2_END_OF_STREAM : u32 = 0x0040;

/// Used in XAUDIO2_SEND_DESCRIPTOR.Flags
pub const XAUDIO2_SEND_USEFILTER : u32 = 0x0080;

/// Used in IXAudio2SourceVoice::GetState
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED : u32 = 0x0100;



// Default parameters for the built-in filter

//pub const XAUDIO2_DEFAULT_FILTER_TYPE       : ()  = LowPassFilter; // TODO
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY  : f32 = XAUDIO2_MAX_FILTER_FREQUENCY;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ   : f32 = 1.0;



// Internal XAudio2 constants

/// [XAUDIO2_QUANTUM_MS] numerator
pub const XAUDIO2_QUANTUM_NUMERATOR : u32 = 1;

/// [XAUDIO2_QUANTUM_MS] denominator
pub const XAUDIO2_QUANTUM_DENOMINATOR : u32 = 100;

/// On Windows, XAudio2 processes audio in `10`ms chunks (= 1/100 seconds)
pub const XAUDIO2_QUANTUM_MS : f32 = 1000.0 * (XAUDIO2_QUANTUM_NUMERATOR as f32) / (XAUDIO2_QUANTUM_DENOMINATOR as f32);



/// XAudio2 error codes
pub const FACILITY_XAUDIO2 : HResultFacilityMicrosoft = HResultFacilityMicrosoft::from_constant(0x896);

/// An API call or one of its arguments was illegal
pub const XAUDIO2_E_INVALID_CALL : HResult = HResult::from_constant(0x88960001);

/// The XMA hardware suffered an unrecoverable error
pub const XAUDIO2_E_XMA_DECODER_ERROR : HResult = HResult::from_constant(0x88960002);

/// XAudio2 failed to initialize an XAPO effect
pub const XAUDIO2_E_XAPO_CREATION_FAILED : HResult = HResult::from_constant(0x88960003);

/// An audio device became unusable (unplugged, etc)
pub const XAUDIO2_E_DEVICE_INVALIDATED : HResult = HResult::from_constant(0x88960004);



/// Used in XAudio2Create, specifies which CPU(s) to use.
pub type XAUDIO2_PROCESSOR              = u32;
pub const Processor1                    : u32 = 0x00000001;
pub const Processor2                    : u32 = 0x00000002;
#[doc(hidden)] pub const Processor3     : u32 = 0x00000004;
#[doc(hidden)] pub const Processor4     : u32 = 0x00000008;
#[doc(hidden)] pub const Processor5     : u32 = 0x00000010;
#[doc(hidden)] pub const Processor6     : u32 = 0x00000020;
#[doc(hidden)] pub const Processor7     : u32 = 0x00000040;
#[doc(hidden)] pub const Processor8     : u32 = 0x00000080;
#[doc(hidden)] pub const Processor9     : u32 = 0x00000100;
#[doc(hidden)] pub const Processor10    : u32 = 0x00000200;
#[doc(hidden)] pub const Processor11    : u32 = 0x00000400;
#[doc(hidden)] pub const Processor12    : u32 = 0x00000800;
#[doc(hidden)] pub const Processor13    : u32 = 0x00001000;
#[doc(hidden)] pub const Processor14    : u32 = 0x00002000;
#[doc(hidden)] pub const Processor15    : u32 = 0x00004000;
#[doc(hidden)] pub const Processor16    : u32 = 0x00008000;
#[doc(hidden)] pub const Processor17    : u32 = 0x00010000;
#[doc(hidden)] pub const Processor18    : u32 = 0x00020000;
#[doc(hidden)] pub const Processor19    : u32 = 0x00040000;
#[doc(hidden)] pub const Processor20    : u32 = 0x00080000;
#[doc(hidden)] pub const Processor21    : u32 = 0x00100000;
#[doc(hidden)] pub const Processor22    : u32 = 0x00200000;
#[doc(hidden)] pub const Processor23    : u32 = 0x00400000;
#[doc(hidden)] pub const Processor24    : u32 = 0x00800000;
#[doc(hidden)] pub const Processor25    : u32 = 0x01000000;
#[doc(hidden)] pub const Processor26    : u32 = 0x02000000;
#[doc(hidden)] pub const Processor27    : u32 = 0x04000000;
#[doc(hidden)] pub const Processor28    : u32 = 0x08000000;
#[doc(hidden)] pub const Processor29    : u32 = 0x10000000;
#[doc(hidden)] pub const Processor30    : u32 = 0x20000000;
pub const Processor31                   : u32 = 0x40000000;
pub const Processor32                   : u32 = 0x80000000;
pub const XAUDIO2_ANY_PROCESSOR         : u32 = 0xffffffff;
pub const XAUDIO2_DEFAULT_PROCESSOR     : u32 = Processor1;
