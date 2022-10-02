//! ✔️ XAudio 2.8 - Windows 8+ via preinstall
//!
//! Introduced in the [Windows 8 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#earlier-releases)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.8 (Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)
//! *   [XAudio2 and Windows 8](https://walbourn.github.io/xaudio2-and-windows-8/)

// don't use prev
use abibool::bool32;
use abistr::*;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::um::unknwnbase::IUnknown;
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

/// Maximum [XAUDIO2_FILTER_PARAMETERS::OneOverQ]
pub const XAUDIO2_MAX_FILTER_ONEOVERQ : f32 = 1.5;

/// Maximum [XAUDIO2_FILTER_PARAMETERS::Frequency]
pub const XAUDIO2_MAX_FILTER_FREQUENCY : f32 = 1.0;

/// Maximum non-infinite [XAUDIO2_BUFFER::LoopCount]
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

/// Used in [XAUDIO2_BUFFER::LoopCount]
pub const XAUDIO2_NO_LOOP_REGION : u32 = 0;

/// Used in [XAUDIO2_BUFFER::LoopCount]
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

/// Used in [XAUDIO2_BUFFER::Flags]
pub const XAUDIO2_END_OF_STREAM : u32 = 0x0040;

/// Used in [XAUDIO2_SEND_DESCRIPTOR::Flags]
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



// Forward declarations for the XAudio2 interfaces.

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2)\]
#[repr(C)] pub struct IXAudio2                  { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voice)\]
#[repr(C)] pub struct IXAudio2Voice             { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\]
#[repr(C)] pub struct IXAudio2SourceVoice       { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\]
#[repr(C)] pub struct IXAudio2SubmixVoice       { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\]
#[repr(C)] pub struct IXAudio2MasteringVoice    { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2enginecallback)\]
#[repr(C)] pub struct IXAudio2EngineCallback    { lpVtbl: *const c_void }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voicecallback)\]
#[repr(C)] pub struct IXAudio2VoiceCallback     { lpVtbl: *const c_void }



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



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_voice_details)\]
#[repr(C, packed(1))] pub struct XAUDIO2_VOICE_DETAILS {
    /// Flags the voice was created with.
    pub CreationFlags: u32,

    /// Flags currently active.
    pub ActiveFlags: u32,

    /// Channels in the voice's input audio.
    pub InputChannels: u32,

    /// Sample rate of the voice's input audio.
    pub InputSampleRate: u32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_send_descriptor)\]
#[repr(C, packed(1))] pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags:          u32,
    pub pOutputVoice:   *const IXAudio2Voice,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_voice_sends)\]
#[repr(C, packed(1))] pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount:      u32,
    pub pSends:         *mut XAUDIO2_SEND_DESCRIPTOR,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_effect_descriptor)\]
#[repr(C, packed(1))] pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect:        *const IUnknown,
    pub InitialState:   bool32,
    pub OutputChannels: u32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_effect_chain)\]
#[repr(C, packed(1))] pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount:        u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ne-xaudio2-xaudio2_filter_type)\]
/// Used in [XAUDIO2_FILTER_PARAMETERS]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct XAUDIO2_FILTER_TYPE(u32); // TODO: check type

/// Attenuates frequencies above the cutoff frequency (state-variable filter).
pub const LowPassFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(0);

/// Attenuates frequencies outside a given range      (state-variable filter).
pub const BandPassFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(1);

/// Attenuates frequencies below the cutoff frequency (state-variable filter).
pub const HighPassFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(2);

/// Attenuates frequencies inside a given range       (state-variable filter).
pub const NotchFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(3);

/// Attenuates frequencies above the cutoff frequency (one-pole filter, [XAUDIO2_FILTER_PARAMETERS::OneOverQ] has no effect)
pub const LowPassOnePoleFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(4);

/// Attenuates frequencies below the cutoff frequency (one-pole filter, [XAUDIO2_FILTER_PARAMETERS::OneOverQ] has no effect)
pub const HighPassOnePoleFilter : XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(5);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_filter_parameters)\]
/// Used in IXAudio2Voice::Set/GetFilterParameters and Set/GetOutputFilterParameters
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_FILTER_PARAMETERS {
    /// Filter type
    pub Type: XAUDIO2_FILTER_TYPE,

    /// Filter coefficient.  Must be within 0 ..= [xaudio2_9::MAX_FILTER_FREQUENCY].
    /// See XAudio2CutoffFrequencyToRadians() for state-variable filter types and
    /// XAudio2CutoffFrequencyToOnePoleCoefficient() for one-pole filter types.
    pub Frequency: f32,

    /// Reciprocal of the filter's quality factor Q; must be within 0 ..= [xaudio2_9::MAX_FILTER_ONEOVERQ].
    /// Has no effect for one-pole filters.
    pub OneOverQ: f32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\]
/// Used in IXAudio2SourceVoice::SubmitSourceBuffer
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_BUFFER {
    /// Either 0 or [xaudio2_9::END_OF_STREAM].
    pub Flags: u32,

    /// Size of the audio data buffer in bytes.
    pub AudioBytes: u32,

    /// Pointer to the audio data buffer.
    pub pAudioData: *const u8,

    /// First sample in this buffer to be played.
    pub PlayBegin: u32,

    /// Length of the region to be played in samples, or 0 to play the whole buffer.
    pub PlayLength: u32,

    /// First sample of the region to be looped.
    pub LoopBegin: u32,

    /// Length of the desired loop region in samples, or 0 to loop the entire buffer.
    pub LoopLength: u32,

    /// Number of times to repeat the loop region, or XAUDIO2_LOOP_INFINITE to loop forever.
    pub LoopCount: u32,

    /// Context value to be passed back in callbacks.
    pub pContext: *mut c_void,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer_wma)\]
/// Used in IXAudio2SourceVoice::SubmitSourceBuffer when submitting XWMA data.
///
/// NOTE: If an XWMA sound is submitted in more than one buffer, each buffer's
/// pDecodedPacketCumulativeBytes[PacketCount-1] value must be subtracted from
/// all the entries in the next buffer's pDecodedPacketCumulativeBytes array.
/// And whether a sound is submitted in more than one buffer or not, the final
/// buffer of the sound should use the [XAUDIO2_END_OF_STREAM] flag, or else the
/// client must call IXAudio2SourceVoice::Discontinuity after submitting it.
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_BUFFER_WMA {
    /// Decoded packet's cumulative size array.
    /// Each element is the number of bytes accumulated when the corresponding XWMA packet is decoded in order.
    /// The array must have PacketCount elements.
    pDecodedPacketCumulativeBytes: *const u32,

    /// Number of XWMA packets submitted.
    /// Must be >= 1 and divide evenly into [Buffer::AudioBytes].
    PacketCount: u32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_voice_state)\]
/// Returned by IXAudio2SourceVoice::GetState
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_VOICE_STATE {
    /// The pContext value provided in the [Buffer]
    ///  that is currently being processed, or NULL if
    ///  there are no buffers in the queue.
    pub pCurrentBufferContext: *mut c_void,

    /// Number of buffers currently queued on the voice
    ///  (including the one that is being processed).
    pub BuffersQueued: u32,

    /// Total number of samples produced by the voice since
    ///  it began processing the current audio stream.
    ///  If [VOICE_NOSAMPLESPLAYED] is specified
    ///  in the call to IXAudio2SourceVoice::GetState,
    ///  this member will not be calculated, saving CPU.
    pub SamplesPlayed: u64,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_performance_data)\]
/// Returned by IXAudio2::GetPerformanceData
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_PERFORMANCE_DATA {
    // CPU usage information

    /// CPU cycles spent on audio processing since the
    ///  last call to StartEngine or GetPerformanceData.
    pub AudioCyclesSinceLastQuery: u64,

    /// Total CPU cycles elapsed since the last call
    ///  (only counts the CPU XAudio2 is running on).
    pub TotalCyclesSinceLastQuery: u64,

    /// Fewest CPU cycles spent processing any one
    ///  audio quantum since the last call.
    pub MinimumCyclesPerQuantum: u32,

    /// Most CPU cycles spent processing any one
    ///  audio quantum since the last call.
    pub MaximumCyclesPerQuantum: u32,

    // Memory usage information

    /// Total heap space currently in use.
    pub MemoryUsageInBytes: u32,

    // Audio latency and glitching information

    /// Minimum delay from when a sample is read from a
    ///  source buffer to when it reaches the speakers.
    pub CurrentLatencyInSamples: u32,

    /// Audio dropouts since the engine was started.
    pub GlitchesSinceEngineStarted: u32,

    // Data about XAudio2's current workload

    /// Source voices currently playing.
    pub ActiveSourceVoiceCount: u32,

    /// Source voices currently existing.
    pub TotalSourceVoiceCount: u32,

    /// Submix voices currently playing/existing.
    pub ActiveSubmixVoiceCount: u32,

    /// Resample xAPOs currently active.
    pub ActiveResamplerCount: u32,

    /// MatrixMix xAPOs currently active.
    pub ActiveMatrixMixCount: u32,

    // Usage of the hardware XMA decoder (Xbox 360 only)

    /// Number of source voices decoding XMA data.
    pub ActiveXmaSourceVoices: u32,

    /// A voice can use more than one XMA stream.
    pub ActiveXmaStreams: u32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_debug_configuration)\]
/// Used in IXAudio2::SetDebugConfiguration
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_DEBUG_CONFIGURATION {
    /// Bitmap of enabled debug message types.
    pub TraceMask: u32,

    /// Message types that will break into the debugger.
    pub BreakMask: u32,

    /// Whether to log the thread ID with each message.
    pub LogThreadID: bool32,

    /// Whether to log the source file and line number.
    pub LogFileline: bool32,

    /// Whether to log the function name.
    pub LogFunctionName: bool32,

    /// Whether to log message timestamps.
    pub LogTiming: bool32,
}



/// For handled errors with serious effects.
pub const XAUDIO2_LOG_ERRORS : u32 = 0x0001;

/// For handled errors that may be recoverable.
pub const XAUDIO2_LOG_WARNINGS : u32 = 0x0002;

/// Informational chit-chat (e.g. state changes).
pub const XAUDIO2_LOG_INFO : u32 = 0x0004;

/// More detailed chit-chat.
pub const XAUDIO2_LOG_DETAIL : u32 = 0x0008;

/// Public API function entries and exits.
pub const XAUDIO2_LOG_API_CALLS : u32 = 0x0010;

/// Internal function entries and exits.
pub const XAUDIO2_LOG_FUNC_CALLS : u32 = 0x0020;

/// Delays detected and other timing data.
pub const XAUDIO2_LOG_TIMING : u32 = 0x0040;

/// Usage of critical sections and mutexes.
pub const XAUDIO2_LOG_LOCKS : u32 = 0x0080;

/// Memory heap usage information.
pub const XAUDIO2_LOG_MEMORY : u32 = 0x0100;

/// Audio streaming information.
pub const XAUDIO2_LOG_STREAMING  : u32 = 0x1000;
