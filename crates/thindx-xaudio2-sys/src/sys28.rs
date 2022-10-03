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
use bytemuck::Zeroable;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::um::audiosessiontypes::AUDIO_STREAM_CATEGORY;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
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

/// Minimum [IXAudio2SourceVoice::SetFrequencyRatio] argument
pub const XAUDIO2_MIN_FREQ_RATIO : f32 = 1.0 / 1024.0;

/// Maximum [MaxFrequencyRatio](IXAudio2::CreateSourceVoice) argument
pub const XAUDIO2_MAX_FREQ_RATIO : f32 = 1024.0;

/// Default [MaxFrequencyRatio](IXAudio2::CreateSourceVoice) argument
pub const XAUDIO2_DEFAULT_FREQ_RATIO : f32 = 2.0;

/// Maximum [XAUDIO2_FILTER_PARAMETERS::OneOverQ]
pub const XAUDIO2_MAX_FILTER_ONEOVERQ : f32 = 1.5;

/// Maximum [XAUDIO2_FILTER_PARAMETERS::Frequency]
pub const XAUDIO2_MAX_FILTER_FREQUENCY : f32 = 1.0;

/// Maximum non-infinite [XAUDIO2_BUFFER::LoopCount]
pub const XAUDIO2_MAX_LOOP_COUNT : u32 = 254;

/// Maximum simultaneous XAudio2 objects on Xbox 360
pub const XAUDIO2_MAX_INSTANCES : u32 = 8;



// For XMA voices on Xbox 360 there is an additional restriction on the [MaxFrequencyRatio](IXAudio2::CreateSourceVoice)
// argument and the voice's sample rate: the product of these numbers cannot exceed 600000
// for one-channel voices or 300000 for voices with more than one channel.

pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO         : u32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL : u32 = 300000;



// Numeric values with special meanings

/// Used as an `OperationSet` argument
pub const XAUDIO2_COMMIT_NOW : u32 = 0;

/// Used in [IXAudio2::CommitChanges]
pub const XAUDIO2_COMMIT_ALL : u32 = 0;

/// Not allowed for `OperationSet` arguments
pub const XAUDIO2_INVALID_OPSET : u32 = -1_i32 as u32;

/// Used in [XAUDIO2_BUFFER::LoopCount]
pub const XAUDIO2_NO_LOOP_REGION : u32 = 0;

/// Used in [XAUDIO2_BUFFER::LoopCount]
pub const XAUDIO2_LOOP_INFINITE : u32 = 255;

/// Used in [IXAudio2::CreateMasteringVoice]
pub const XAUDIO2_DEFAULT_CHANNELS : u32 = 0;

/// Used in [IXAudio2::CreateMasteringVoice]
pub const XAUDIO2_DEFAULT_SAMPLERATE : u32 = 0;



// Flags

/// Used in [IXAudio2::CreateSourceVoice]
pub const XAUDIO2_VOICE_NOPITCH : u32 = 0x0002;

/// Used in [IXAudio2::CreateSourceVoice]
pub const XAUDIO2_VOICE_NOSRC : u32 = 0x0004;

/// Used in [IXAudio2]::[CreateSource](IXAudio2::CreateSourceVoice)/[SubmixVoice](IXAudio2::CreateSubmixVoice)
pub const XAUDIO2_VOICE_USEFILTER : u32 = 0x0008;

/// Used in [IXAudio2SourceVoice::Stop]
pub const XAUDIO2_PLAY_TAILS : u32 = 0x0020;

/// Used in [XAUDIO2_BUFFER::Flags]
pub const XAUDIO2_END_OF_STREAM : u32 = 0x0040;

/// Used in [XAUDIO2_SEND_DESCRIPTOR::Flags]
pub const XAUDIO2_SEND_USEFILTER : u32 = 0x0080;

/// Used in [IXAudio2SourceVoice::GetState]
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED : u32 = 0x0100;



// Default parameters for the built-in filter

pub const XAUDIO2_DEFAULT_FILTER_TYPE       : XAUDIO2_FILTER_TYPE = LowPassFilter;
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



/// Used in `XAudio2Create`, specifies which CPU(s) to use.
pub type XAUDIO2_PROCESSOR              = u32;
#[doc = "Processor/core no. 1"] pub const Processor1 : u32 = 0x00000001;
#[doc = "Processor/core no. 2"] pub const Processor2 : u32 = 0x00000002;
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
#[doc = "Processor/core no. 31"] pub const Processor31 : u32 = 0x40000000;
#[doc = "Processor/core no. 32"] pub const Processor32 : u32 = 0x80000000;
/// Any/all processors (might spawn a thread per core, which might be an excessive number of threads...)
pub const XAUDIO2_ANY_PROCESSOR         : u32 = 0xffffffff;
/// The "default" processor as of XAudio 2.8
pub const XAUDIO2_DEFAULT_PROCESSOR     : u32 = Processor1;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_voice_details)\]
#[derive(Clone, Copy, Debug, Default, Zeroable)] #[repr(C, packed(1))] pub struct XAUDIO2_VOICE_DETAILS {
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
#[derive(Clone, Copy, Debug, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
/// Used in [IXAudio2Voice]::[Set](IXAudio2Voice::SetFilterParameters)/[GetFilterParameters](IXAudio2Voice::GetFilterParameters)
/// and [Set](IXAudio2Voice::SetOutputFilterParameters)/[GetOutputFilterParameters](IXAudio2Voice::GetOutputFilterParameters)
#[derive(Clone, Copy, Debug, Default, Zeroable)] #[repr(C, packed(1))] pub struct XAUDIO2_FILTER_PARAMETERS {
    /// Filter type
    pub Type: XAUDIO2_FILTER_TYPE,

    /// Filter coefficient.  Must be within 0 ..= [XAUDIO2_MAX_FILTER_FREQUENCY].
    /// See [XAudio2CutoffFrequencyToRadians]\(\) for state-variable filter types and
    /// [XAudio2CutoffFrequencyToOnePoleCoefficient]\(\) for one-pole filter types.
    pub Frequency: f32,

    /// Reciprocal of the filter's quality factor Q; must be within 0 ..= [XAUDIO2_MAX_FILTER_ONEOVERQ].
    /// Has no effect for one-pole filters.
    pub OneOverQ: f32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\]
/// Used in [IXAudio2SourceVoice::SubmitSourceBuffer]
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_BUFFER {
    /// Either 0 or [XAUDIO2_END_OF_STREAM].
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
/// Used in [IXAudio2SourceVoice::SubmitSourceBuffer] when submitting XWMA data.
///
/// NOTE: If an XWMA sound is submitted in more than one buffer, each buffer's
/// pDecodedPacketCumulativeBytes[PacketCount-1] value must be subtracted from
/// all the entries in the next buffer's pDecodedPacketCumulativeBytes array.
/// And whether a sound is submitted in more than one buffer or not, the final
/// buffer of the sound should use the [XAUDIO2_END_OF_STREAM] flag, or else the
/// client must call [IXAudio2SourceVoice::Discontinuity] after submitting it.
#[derive(Clone, Copy, Debug)] #[repr(C, packed(1))] pub struct XAUDIO2_BUFFER_WMA {
    /// Decoded packet's cumulative size array.
    /// Each element is the number of bytes accumulated when the corresponding XWMA packet is decoded in order.
    /// The array must have PacketCount elements.
    pDecodedPacketCumulativeBytes: *const u32,

    /// Number of XWMA packets submitted.
    /// Must be >= 1 and divide evenly into [XAUDIO2_BUFFER::AudioBytes].
    PacketCount: u32,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_voice_state)\]
/// Returned by [IXAudio2SourceVoice::GetState]
#[derive(Clone, Copy, Debug, Zeroable)] #[repr(C, packed(1))] pub struct XAUDIO2_VOICE_STATE {
    /// The pContext value provided in the [XAUDIO2_BUFFER]
    ///  that is currently being processed, or NULL if
    ///  there are no buffers in the queue.
    pub pCurrentBufferContext: *mut c_void,

    /// Number of buffers currently queued on the voice
    ///  (including the one that is being processed).
    pub BuffersQueued: u32,

    /// Total number of samples produced by the voice since
    ///  it began processing the current audio stream.
    ///  If [XAUDIO2_VOICE_NOSAMPLESPLAYED] is specified
    ///  in the call to [IXAudio2SourceVoice::GetState],
    ///  this member will not be calculated, saving CPU.
    pub SamplesPlayed: u64,
}
impl Default for XAUDIO2_VOICE_STATE { fn default() -> Self { Self::zeroed() } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_performance_data)\]
/// Returned by [IXAudio2::GetPerformanceData]
#[derive(Clone, Copy, Debug, Default, Zeroable)] #[repr(C, packed(1))] pub struct XAUDIO2_PERFORMANCE_DATA {
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
/// Used in [IXAudio2::SetDebugConfiguration]
#[derive(Clone, Copy, Debug, Default, Zeroable)] #[repr(C, packed(1))] pub struct XAUDIO2_DEBUG_CONFIGURATION {
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



interfaces! {

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2)\]
    /// Top-level XAudio2 COM interface.
    // #[iid = ...] // XAudio 2.8 and 2.9 define this IID differently, which makes actually using it here awkward.  And pretty pointless - why would you ever type erase [IXAudio2] to [IUnknown]?
    pub interface IXAudio2(IXAudio2Vtbl) => unsafe IUnknown(IUnknownVtbl) {
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-registerforcallbacks)\]
        /// Adds a new client to receive XAudio2's engine callbacks.
        ///
        /// ### Arguments
        /// * `pCallback` - Callback interface to be called during each processing pass.
        pub unsafe fn RegisterForCallbacks(&self, pCallback: *const IXAudio2EngineCallback) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-unregisterforcallbacks)\]
        /// Removes an existing receiver of XAudio2 engine callbacks.
        ///
        /// ### Arguments
        /// * `pCallback` - Previously registered callback interface to be removed.
        pub unsafe fn UnregisterForCallbacks(&self, pCallback: *const IXAudio2EngineCallback) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
        /// Creates and configures a source voice.
        ///
        /// ### Arguments
        /// * `ppSourceVoice`       - Returns the new object's [IXAudio2SourceVoice] interface.
        /// * `pSourceFormat`       - Format of the audio that will be fed to the voice.
        /// * `Flags`               - VOICE_\* flags specifying the source voice's behavior.
        /// * `MaxFrequencyRatio`   - Maximum [IXAudio2SourceVoice::SetFrequencyRatio] argument to be allowed.
        /// * `pCallback`           - Optional pointer to a client-provided callback interface.
        /// * `pSendList`           - Optional list of voices this voice should send audio to.
        /// * `pEffectChain`        - Optional list of effects to apply to the audio data.
        pub unsafe fn CreateSourceVoice(
            &self,
            ppSourceVoice:      *mut *mut IXAudio2SourceVoice,
            pSourceFormat:      *const WAVEFORMATEX,
            MaxFrequencyRatio:  f32,
            pCallback:          *const IXAudio2VoiceCallback,
            pSendList:          *const XAUDIO2_VOICE_SENDS,
            pEffectChain:       *const XAUDIO2_EFFECT_CHAIN,
        ) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsubmixvoice)\]
        /// Creates and configures a submix voice.
        ///
        /// ### Arguments
        /// * `ppSubmixVoice`   - Returns the new object's [IXAudio2SubmixVoice] interface.
        /// * `InputChannels`   - Number of channels in this voice's input audio data.
        /// * `InputSampleRate` - Sample rate of this voice's input audio data.
        /// * `Flags`           - VOICE_\* flags specifying the submix voice's behavior.
        /// * `ProcessingStage` - Arbitrary number that determines the processing order.
        /// * `pSendList`       - Optional list of voices this voice should send audio to.
        /// * `pEffectChain`    - Optional list of effects to apply to the audio data.
        pub unsafe fn CreateSubmixVoice(
            &self,
            ppSubmixVoice:      *mut *mut IXAudio2SubmixVoice,
            InputChannels:      u32,
            InputSampleRate:    u32,
            Flags:              u32,
            ProcessingStage:    u32,
            pSendList:          *const XAUDIO2_VOICE_SENDS,
            pEffectChain:       *const XAUDIO2_EFFECT_CHAIN,
        ) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createmasteringvoice)\]
        /// Creates and configures a mastering voice.
        ///
        /// ### Arguments
        /// * `ppMasteringVoice`    - Returns the new object's [IXAudio2MasteringVoice] interface.
        /// * `InputChannels`       - Number of channels in this voice's input audio data.
        /// * `InputSampleRate`     - Sample rate of this voice's input audio data.
        /// * `Flags`               - VOICE_\* flags specifying the mastering voice's behavior.
        /// * `szDeviceId`          - Identifier of the device to receive the output audio.
        /// * `pEffectChain`        - Optional list of effects to apply to the audio data.
        /// * `StreamCategory`      - The audio stream category to use for this mastering voice
        pub unsafe fn CreateMasteringVoice(
            &self,
            ppMasteringVoice:   *mut *mut IXAudio2MasteringVoice,
            InputChannels:      u32,
            InputSampleRate:    u32,
            Flags:              u32,
            szDeviceId:         *const u16,
            pEffectChain:       *const XAUDIO2_EFFECT_CHAIN,
            StreamCategory:     AUDIO_STREAM_CATEGORY,
        ) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-startengine)\]
        /// Creates and starts the audio processing thread.
        pub unsafe fn StartEngine(&self) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-stopengine)\]
        /// Stops and destroys the audio processing thread.
        pub unsafe fn StopEngine(&self) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-commitchanges)\]
        /// Atomically applies a set of operations previously tagged with a given identifier.
        ///
        /// ### Arguments
        /// * `OperationSet` - Identifier of the set of operations to be applied.
        pub unsafe fn CommitChanges(&self, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-getperformancedata)\]
        /// Returns current resource usage details: memory, CPU, etc.
        ///
        /// ### Arguments
        /// * `pPerfData` - Returns the performance data structure.
        pub unsafe fn GetPerformanceData(&self, pPerfData: *mut XAUDIO2_PERFORMANCE_DATA) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-setdebugconfiguration)\]
        /// Configures XAudio2's debug output (in debug builds only).
        ///
        /// ### Arguments
        /// * `pDebugConfiguration` - Structure describing the debug output behavior.
        /// * `pReserved`           - Optional parameter; must be NULL.
        pub unsafe fn SetDebugConfiguration(&self, pDebugConfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, pReserved: *const c_void) -> ();
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voice)\]
    /// Base voice management interface.
    pub interface IXAudio2Voice(IXAudio2VoiceVtbl) => unsafe IUnknown(IUnknownVtbl) {
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getvoicedetails)\]
        /// Returns the basic characteristics of this voice.
        ///
        /// ### Arguments
        /// * `pVoiceDetails`   - Returns the voice's details.
        pub unsafe fn GetVoiceDetails(&self, pVoiceDetails: *mut XAUDIO2_VOICE_DETAILS) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputvoices)\]
        /// Replaces the set of submix/mastering voices that receive this voice's output.
        ///
        /// ### Arguments
        /// * `pSendList`       - Optional list of voices this voice should send audio to.
        pub unsafe fn SetOutputVoices(&self, pSendList: *const XAUDIO2_VOICE_SENDS) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-seteffectchain)\]
        /// Replaces this voice's current effect chain with a new one.
        ///
        /// ### Arguments
        /// * `pEffectChain`    - Structure describing the new effect chain to be used.
        pub unsafe fn SetEffectChain(&self, pEffectChain: *const XAUDIO2_EFFECT_CHAIN) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-enableeffect)\]
        /// Enables an effect in this voice's effect chain.
        ///
        /// ### Arguments
        /// * `EffectIndex`     - Index of an effect within this voice's effect chain.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn EnableEffect(&self, EffectIndex: u32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-disableeffect)\]
        /// Disables an effect in this voice's effect chain.
        ///
        /// ### Arguments
        /// * `EffectIndex`     - Index of an effect within this voice's effect chain.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn DisableEffect(&self, EffectIndex: u32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-geteffectstate)\]
        /// Returns the running state of an effect.
        ///
        /// ### Arguments
        /// * `EffectIndex`     - Index of an effect within this voice's effect chain.
        /// * `pEnabled`        - Returns the enabled/disabled state of the given effect.
        pub unsafe fn GetEffectState(&self, EffectIndex: u32, pEnabled: *mut bool32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-seteffectparameters)\]
        /// Sets effect-specific parameters.
        ///
        /// Unlike IXAPOParameters::SetParameters, this method may
        /// be called from any thread.  XAudio2 implements
        /// appropriate synchronization to copy the parameters to the
        /// realtime audio processing thread.
        ///
        /// ### Arguments
        /// * `EffectIndex`         - Index of an effect within this voice's effect chain.
        /// * `pParameters`         - Pointer to an effect-specific parameters block.
        /// * `ParametersByteSize`  - Size of the pParameters array  in bytes.
        /// * `OperationSet`        - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetEffectParameters(&self, EffectIndex: u32, pParameters: *const u8, ParametersByteSize: u32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-geteffectparameters)\]
        /// Obtains the current effect-specific parameters.
        ///
        /// ### Arguments
        /// * `EffectIndex`         - Index of an effect within this voice's effect chain.
        /// * `pParameters`         - Returns the current values of the effect-specific parameters.
        /// * `ParametersByteSize`  - Size of the pParameters array in bytes.
        pub unsafe fn GetEffectParameters(&self, EffectIndex: u32, pParameters: *mut u8, ParametersByteSize: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setfilterparameters)\]
        /// Sets this voice's filter parameters.
        ///
        /// ### Arguments
        /// * `pParameters`         - Pointer to the filter's parameter structure.
        /// * `OperationSet`        - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetFilterParameters(&self, pParameters: *const XAUDIO2_FILTER_PARAMETERS, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getfilterparameters)\]
        /// Returns this voice's current filter parameters.
        ///
        /// ### Arguments
        /// * `pParameters`         - Returns the filter parameters.
        pub unsafe fn GetFilterParameters(&self, pParameters: *mut XAUDIO2_FILTER_PARAMETERS) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputfilterparameters)\]
        /// Sets the filter parameters on one of this voice's sends.
        ///
        /// ### Arguments
        /// * `pDestinationVoice`   - Destination voice of the send whose filter parameters will be set.
        /// * `pParameters`         - Pointer to the filter's parameter structure.
        /// * `OperationSet`        - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetOutputFilterParameters(&self, pDestinationVoice: *const IXAudio2Voice, pParameters: *const XAUDIO2_FILTER_PARAMETERS, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getoutputfilterparameters)\]
        /// Returns the filter parameters from one of this voice's sends.
        ///
        /// ### Arguments
        /// * `pDestinationVoice`   - Destination voice of the send whose filter parameters will be read.
        /// * `pParameters`         - Returns the filter parameters.
        pub unsafe fn GetOutputFilterParameters(&self, pDestinationVoice: *const IXAudio2Voice, pParameters: *mut XAUDIO2_FILTER_PARAMETERS) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setvolume)\]
        /// Sets this voice's overall volume level.
        ///
        /// ### Arguments
        /// * `Volume`          - New overall volume level to be used, as an amplitude factor.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetVolume(&self, Volume: f32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getvolume)\]
        /// Obtains this voice's current overall volume level.
        ///
        /// ### Arguments
        /// * `pVolume`         - Returns the voice's current overall volume level.
        pub unsafe fn GetVolume(&self, pVolume: *mut f32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setchannelvolumes)\]
        /// Sets this voice's per-channel volume levels.
        ///
        /// ### Arguments
        /// * `Channels`        - Used to confirm the voice's channel count.
        /// * `pVolumes`        - Array of per-channel volume levels to be used.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetChannelVolumes(&self, Channels: u32, pVolumes: *const f32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getchannelvolumes)\]
        /// Returns this voice's current per-channel volume levels.
        ///
        /// ### Arguments
        /// * `Channels`    - Used to confirm the voice's channel count.
        /// * `pVolumes`    - Returns an array of the current per-channel volume levels.
        pub unsafe fn GetChannelVolumes(&self, Channels: u32, pVolumes: *mut f32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputmatrix)\]
        /// Sets the volume levels used to mix from each channel of this
        /// voice's output audio to each channel of a given destination
        /// voice's input audio.
        ///
        /// ### Arguments
        /// * `pDestinationVoice`   - The destination voice whose mix matrix to change.
        /// * `SourceChannels`      - Used to confirm this voice's output channel count (the number of channels produced by the last effect in the chain).
        /// * `DestinationChannels` - Confirms the destination voice's input channels.
        /// * `pLevelMatrix`        - Array of \[SourceChannels * DestinationChannels\] send levels.  The level used to send from source channel S to destination channel D should be in pLevelMatrix\[S + SourceChannels * D\].
        /// * `OperationSet`        - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetOutputMatrix(
            &self,
            pDestinationVoice:      *const IXAudio2Voice,
            SourceChannels:         u32,
            DestinationChannels:    u32,
            pLevelMatrix:           *const f32,
            OperationSet:           u32,
        ) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getoutputmatrix)\]
        /// Obtains the volume levels used to send each channel of this
        /// voice's output audio to each channel of a given destination
        /// voice's input audio.
        ///
        /// ### Arguments
        /// * `pDestinationVoice`   - The destination voice whose mix matrix to obtain.
        /// * `SourceChannels`      - Used to confirm this voice's output channel count (the number of channels produced by the last effect in the chain).
        /// * `DestinationChannels` - Confirms the destination voice's input channels.
        /// * `pLevelMatrix`        - Array of send levels, as above.
        pub unsafe fn GetOutputMatrix(
            &self,
            pDestinationVoice:      *const IXAudio2Voice,
            SourceChannels:         u32,
            DestinationChannels:    u32,
            pLevelMatrix:           *mut f32,
        ) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
        /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
        pub unsafe fn DestroyVoice(&self) -> ();
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\]
    /// Source voice management interface.
    pub interface IXAudio2SourceVoice(IXAudio2SourceVoiceVtbl) => unsafe IXAudio2Voice(IXAudio2VoiceVtbl) {
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-start)\]
        /// Makes this voice start consuming and processing audio.
        //
        /// ### Arguments
        /// *   `Flags`         - Flags controlling how the voice should be started.
        /// *   `OperationSet`  - Used to identify this call as part of a deferred batch.
        pub unsafe fn Start(&self, Flags: u32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-stop)\]
        /// Makes this voice stop consuming audio.
        ///
        /// ### Arguments
        /// * `Flags`           - Flags controlling how the voice should be stopped.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn Stop(&self, Flags: u32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer)\]
        /// Adds a new audio buffer to this voice's input queue.
        ///
        /// ### Arguments
        /// * `pBuffer`     - Pointer to the buffer structure to be queued.
        /// * `pBufferWMA`  - Additional structure used only when submitting XWMA data.
        pub unsafe fn SubmitSourceBuffer(&self, pBuffer: *const XAUDIO2_BUFFER, pBufferWMA: *const XAUDIO2_BUFFER_WMA) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-flushsourcebuffers)\]
        /// Removes all pending audio buffers from this voice's queue.
        pub unsafe fn FlushSourceBuffers(&self) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-discontinuity)\]
        /// Notifies the voice of an intentional break in the stream of
        /// audio buffers (e.g. the end of a sound), to prevent XAudio2
        /// from interpreting an empty buffer queue as a glitch.
        pub unsafe fn Discontinuity(&self) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-exitloop)\]
        /// Breaks out of the current loop when its end is reached.
        ///
        /// ### Arguments
        /// * `OperationSet` - Used to identify this call as part of a deferred batch.
        pub unsafe fn ExitLoop(&self, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getstate)\]
        /// Returns the number of buffers currently queued on this voice,
        /// the pContext value associated with the currently processing
        /// buffer (if any), and other voice state information.
        ///
        /// ### Arguments
        /// * `pVoiceState` - Returns the state information.
        /// * `Flags`       - Flags controlling what voice state is returned.
        pub unsafe fn GetState(&self, pVoiceState: *mut XAUDIO2_VOICE_STATE, Flags: u32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setfrequencyratio)\]
        /// Sets this voice's frequency adjustment, i.e. its pitch.
        ///
        /// ### Arguments
        /// * `Ratio`           - Frequency change, expressed as source frequency / target frequency.
        /// * `OperationSet`    - Used to identify this call as part of a deferred batch.
        pub unsafe fn SetFrequencyRatio(&self, Ratio: f32, OperationSet: u32) -> HResult;

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getfrequencyratio)\]
        /// Returns this voice's current frequency adjustment ratio.
        ///
        /// ### Arguments
        /// * `pRatio` - Returns the frequency adjustment.
        pub unsafe fn GetFrequencyRatio(&self, pRatio: *mut f32) -> ();

        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setsourcesamplerate)\]
        /// Reconfigures this voice to treat its source data as being
        /// at a different sample rate than the original one specified
        /// in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
        ///
        /// ### Arguments
        /// *   `NewSourceSampleRate` - The intended sample rate of further submitted source data.
        pub unsafe fn SetSourceSampleRate(&self, NewSourceSampleRate: u32) -> HResult;
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\]
    /// Submixing voice management interface.
    pub interface IXAudio2SubmixVoice(IXAudio2SubmixVoiceVtbl) => unsafe IXAudio2Voice(IXAudio2VoiceVtbl) {
        // There are currently no methods specific to submix voices.
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\]
    /// Mastering voice management interface.
    pub interface IXAudio2MasteringVoice(IXAudio2MasteringVoiceVtbl) => unsafe IXAudio2Voice(IXAudio2VoiceVtbl) {
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2masteringvoice-getchannelmask)\]
        /// Returns the channel mask for this voice
        ///
        /// ### Arguments
        /// *   `pChannelMask` - returns the channel mask for this voice.
        ///     This corresponds to the dwChannelMask member of [WAVEFORMATEXTENSIBLE].
        ///
        /// [WAVEFORMATEXTENSIBLE]: https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatextensible
        pub unsafe fn GetChannelMask(&self, pChannelmask: *mut u32) -> HResult;
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2enginecallback)\]
    /// Client notification interface for engine events.
    ///
    /// Contains methods to notify the client when certain events happen
    /// in the XAudio2 engine.  This interface should be implemented by
    /// the client.  XAudio2 will call these methods via the interface
    /// pointer provided by the client when it calls
    /// [IXAudio2::RegisterForCallbacks].
    pub interface IXAudio2EngineCallback(IXAudio2EngineCallbackVtbl) => unsafe IUnknown(IUnknownVtbl) {
        /// Called by XAudio2 just before an audio processing pass begins.
        pub unsafe fn OnProcessingPassStart(&self) -> ();

        /// Called just after an audio processing pass ends.
        pub unsafe fn OnProcessingPassEnd(&self) -> ();

        /// Called in the event of a critical system error which requires XAudio2
        /// to be closed down and restarted.  The error code is given in Error.
        pub unsafe fn OnCriticalError(&self, error: HResult) -> ();
    }



    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voicecallback)\]
    /// Client notification interface for voice events.
    ///
    /// Contains methods to notify the client when certain events happen
    /// in an XAudio2 voice.  This interface should be implemented by the
    /// client.  XAudio2 will call these methods via an interface pointer
    /// provided by the client in the [IXAudio2::CreateSourceVoice] call.
    pub interface IXAudio2VoiceCallback(IXAudio2VoiceCallbackVtbl) => unsafe IUnknown(IUnknownVtbl) {
        /// Called just before this voice's processing pass begins.
        pub unsafe fn OnVoiceProcessingPassStart(&self, BytesRequired: u32) -> ();

        /// Called just after this voice's processing pass ends.
        pub unsafe fn OnVoiceProcessingPassEnd(&self) -> ();

        /// Called when this voice has just finished playing a buffer stream
        /// (as marked with the [XAUDIO2_END_OF_STREAM] flag on the last buffer).
        pub unsafe fn OnStreamEnd(&self) -> ();

        /// Called when this voice is about to start processing a new buffer.
        pub unsafe fn OnBufferStart(&self, pBufferContext: *mut c_void) -> ();

        /// Called when this voice has just finished processing a buffer.
        /// The buffer can now be reused or destroyed.
        pub unsafe fn OnBufferEnd(&self, pBufferContext: *mut c_void) -> ();

        /// Called when this voice has just reached the end position of a loop.
        pub unsafe fn OnLoopEnd(&self, pBufferContext: *mut c_void) -> ();

        /// Called in the event of a critical error during voice processing,
        /// such as a failing xAPO or an error from the hardware XMA decoder.
        /// The voice may have to be destroyed and re-created to recover from
        /// the error.  The callback arguments report which buffer was being
        /// processed when the error occurred, and its HRESULT code.
        pub unsafe fn OnVoiceError(&self, pBufferContext: *mut c_void, error: HResult) -> ();
    }

}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2decibelstoamplituderatio)\]
/// Calculate the argument to [IXAudio2Voice::SetVolume] from a decibel value
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2DecibelsToAmplitudeRatio(Decibels: f32) -> f32 {
    f32::powf(10.0, Decibels / 20.0)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2amplituderatiotodecibels)\]
/// Recover a volume in decibels from an amplitude factor
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2AmplitudeRatioToDecibels(Volume: f32) -> f32 {
    if Volume == 0.0 {
        -3.402823466e+38 // Smallest float value (-FLT_MAX)
    } else {
        20.0 * f32::log10(Volume)
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2semitonestofrequencyratio)\]
/// Calculate the argument to [IXAudio2SourceVoice::SetFrequencyRatio] from a semitone value
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2SemitonesToFrequencyRatio(Semitones: f32) -> f32 {
    // FrequencyRatio = 2 ^ Octaves
    //                = 2 ^ (Semitones / 12)
    f32::powf(2.0, Semitones / 12.0)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2frequencyratiotosemitones)\]
/// Recover a pitch in semitones from a frequency ratio
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2FrequencyRatioToSemitones(FrequencyRatio: f32) -> f32 {
    // Semitones = 12 * log2(FrequencyRatio)
    //           = 12 * log2(10) * log10(FrequencyRatio)
    39.86313713864835 * f32::log10(FrequencyRatio)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2cutofffrequencytoradians)\]
/// Convert from filter cutoff frequencies expressed in Hertz to the radian
/// frequency values used in [XAUDIO2_FILTER_PARAMETERS::Frequency], state-variable
/// filter types only.  Use [XAudio2CutoffFrequencyToOnePoleCoefficient]\(\) for one-pole filter types.
/// Note that the highest `CutoffFrequency` supported is `SampleRate/6`.
/// Higher values of `CutoffFrequency` will return [XAUDIO2_MAX_FILTER_FREQUENCY].
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2CutoffFrequencyToRadians(CutoffFrequency: f32, SampleRate: u32) -> f32 {
    if ((CutoffFrequency * 6.0) as u32) >= SampleRate {
        XAUDIO2_MAX_FILTER_FREQUENCY
    } else {
        2.0 * f32::sin(core::f32::consts::PI * CutoffFrequency / (SampleRate as f32))
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2radianstocutofffrequency)\]
/// Convert from radian frequencies back to absolute frequencies in Hertz
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2RadiansToCutoffFrequency(Radians: f32, SampleRate: f32) -> f32{
    SampleRate * f32::asin(Radians / 2.0) / core::f32::consts::PI
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2cutofffrequencytoonepolecoefficient)\]
/// Convert from filter cutoff frequencies expressed in Hertz to the filter
/// coefficients used with [XAUDIO2_FILTER_PARAMETERS::Frequency],
/// [LowPassOnePoleFilter] and [HighPassOnePoleFilter] filter types only.
/// Use [XAudio2CutoffFrequencyToRadians] for state-variable filter types.
#[cfg(feature = "helper-functions")] #[inline] pub fn XAudio2CutoffFrequencyToOnePoleCoefficient(CutoffFrequency: f32, SampleRate: u32) -> f32 {
    if (CutoffFrequency as u32) >= SampleRate {
        XAUDIO2_MAX_FILTER_FREQUENCY
    } else {
        1.0 - f32::powf(1.0 - 2.0 * CutoffFrequency / (SampleRate as f32), 2.0)
    }
}
