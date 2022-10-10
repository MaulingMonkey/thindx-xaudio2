//! ✔️ XAudio 2.8 - Windows 8+ via preinstall
//!
//! Introduced in the [Windows 8 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#earlier-releases)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.8 (Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)
//! *   [XAudio2 and Windows 8](https://walbourn.github.io/xaudio2-and-windows-8/)

mod context;
mod engine_callback;
mod ixaudio2_ext;                   pub use ixaudio2_ext::*;
mod ixaudio2masteringvoice_ext;     pub use ixaudio2masteringvoice_ext::*;
mod ixaudio2voice_ext;              pub use ixaudio2voice_ext::*;
mod ixaudio2sourcevoice_ext;        pub use ixaudio2sourcevoice_ext::*;
mod source_buffer;                  pub(crate) use source_buffer::*;
mod loop_count;
mod sample_range;
mod source_format;
mod source_voice_dynamic;
mod source_voice;
mod voices;
mod voice_callback;


// Might not remain pub
#[doc(hidden)] pub use xaudio2::sys::{
    IXAudio2,
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

/// `XAudio2*` & `XAUDIO2_*`
pub mod xaudio2 {
    pub use super::context::*;
    pub use super::engine_callback::*;
    pub use super::loop_count::*;
    pub use super::sample_range::*;
    pub use super::source_format::*;
    pub use super::source_voice_dynamic::*;
    pub use super::source_voice::*;
    pub use super::voices::*;
    pub use super::voice_callback::*;

    /// Raw low level FFI bindings
    ///
    pub use thindx_xaudio2_sys::xaudio2_8 as sys;

    pub use sys::XAUDIO2_DLL    as DLL;
    pub use sys::XAUDIO2_DLL_A  as DLL_A;
    pub use sys::XAUDIO2_DLL_W  as DLL_W;

    pub use sys::XAUDIO2D_DLL    as D_DLL;
    pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
    pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;

    pub use sys::{
        XAUDIO2_MAX_BUFFER_BYTES                        as MAX_BUFFER_BYTES,
        XAUDIO2_MAX_QUEUED_BUFFERS                      as MAX_QUEUED_BUFFERS,
        XAUDIO2_MAX_BUFFERS_SYSTEM                      as MAX_BUFFERS_SYSTEM,
        XAUDIO2_MAX_AUDIO_CHANNELS                      as MAX_AUDIO_CHANNELS,
        XAUDIO2_MIN_SAMPLE_RATE                         as MIN_SAMPLE_RATE,
        XAUDIO2_MAX_SAMPLE_RATE                         as MAX_SAMPLE_RATE,
        XAUDIO2_MAX_VOLUME_LEVEL                        as MAX_VOLUME_LEVEL,
        XAUDIO2_MIN_FREQ_RATIO                          as MIN_FREQ_RATIO,
        XAUDIO2_MAX_FREQ_RATIO                          as MAX_FREQ_RATIO,
        XAUDIO2_DEFAULT_FREQ_RATIO                      as DEFAULT_FREQ_RATIO,
        XAUDIO2_MAX_FILTER_ONEOVERQ                     as MAX_FILTER_ONEOVERQ,
        XAUDIO2_MAX_FILTER_FREQUENCY                    as MAX_FILTER_FREQUENCY,
        XAUDIO2_MAX_INSTANCES                           as MAX_INSTANCES,
        XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO           as MAX_RATIO_TIMES_RATE_XMA_MONO,
        XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL   as MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL,
        XAUDIO2_COMMIT_NOW                              as COMMIT_NOW,
        XAUDIO2_COMMIT_ALL                              as COMMIT_ALL,
        XAUDIO2_INVALID_OPSET                           as INVALID_OPSET,
        XAUDIO2_DEFAULT_CHANNELS                        as DEFAULT_CHANNELS,
        XAUDIO2_DEFAULT_SAMPLERATE                      as DEFAULT_SAMPLERATE,
        XAUDIO2_DEFAULT_AUDIO_CATEGORY                  as DEFAULT_AUDIO_CATEGORY,
        XAUDIO2_VOICE_NOPITCH                           as VOICE_NOPITCH,
        XAUDIO2_VOICE_NOSRC                             as VOICE_NOSRC,
        XAUDIO2_VOICE_USEFILTER                         as VOICE_USEFILTER,
        XAUDIO2_PLAY_TAILS                              as PLAY_TAILS,
        XAUDIO2_END_OF_STREAM                           as END_OF_STREAM,
        XAUDIO2_SEND_USEFILTER                          as SEND_USEFILTER,
        XAUDIO2_VOICE_NOSAMPLESPLAYED                   as VOICE_NOSAMPLESPLAYED,
        XAUDIO2_DEFAULT_FILTER_TYPE                     as DEFAULT_FILTER_TYPE,
        XAUDIO2_DEFAULT_FILTER_FREQUENCY                as DEFAULT_FILTER_FREQUENCY,
        XAUDIO2_DEFAULT_FILTER_ONEOVERQ                 as DEFAULT_FILTER_ONEOVERQ,
        XAUDIO2_QUANTUM_NUMERATOR                       as QUANTUM_NUMERATOR,
        XAUDIO2_QUANTUM_DENOMINATOR                     as QUANTUM_DENOMINATOR,
        XAUDIO2_QUANTUM_MS                              as QUANTUM_MS,
        FACILITY_XAUDIO2                                as FACILITY,
        XAUDIO2_E_INVALID_CALL                          as E_INVALID_CALL,
        XAUDIO2_E_XMA_DECODER_ERROR                     as E_XMA_DECODER_ERROR,
        XAUDIO2_E_XAPO_CREATION_FAILED                  as E_XAPO_CREATION_FAILED,
        XAUDIO2_E_DEVICE_INVALIDATED                    as E_DEVICE_INVALIDATED,
    };

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
        XAUDIO2_ANY_PROCESSOR       as ANY_PROCESSOR,
        XAUDIO2_DEFAULT_PROCESSOR   as DEFAULT_PROCESSOR,
    };

    #[cfg(feature = "helper-functions")] pub use sys::{
        XAudio2DecibelsToAmplitudeRatio             as decibels_to_amplitude_ratio,
        XAudio2AmplitudeRatioToDecibels             as amplitude_ratio_to_decibels,
        XAudio2SemitonesToFrequencyRatio            as semitones_to_frequency_ratio,
        XAudio2FrequencyRatioToSemitones            as frequency_ratio_to_semitones,
        XAudio2CutoffFrequencyToRadians             as cutoff_frequency_to_radians,
        XAudio2RadiansToCutoffFrequency             as radians_to_cutoff_frequency,
        XAudio2CutoffFrequencyToOnePoleCoefficient  as cutoff_frequency_to_one_pole_coefficient,
    };

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_effect_descriptor)\]
    /// [XAUDIO2_EFFECT_DESCRIPTOR](sys::XAUDIO2_EFFECT_DESCRIPTOR): safe equivalent
    #[derive(Clone)] #[repr(C)] pub struct EffectDescriptor {
        pub effect:             mcom::Rc<winapi::um::unknwnbase::IUnknown>, // XXX: would suggest &IUnknown, but that can be constructed unsoundly from safe code
        pub initial_state:      abibool::bool32,
        pub output_channels:    u32,
    }
    impl EffectDescriptor {
        pub fn new(
            effect:             mcom::Rc<winapi::um::unknwnbase::IUnknown>,
            initial_state:      bool,
            output_channels:    u32,
        ) -> Self {
            Self { effect, initial_state: initial_state.into(), output_channels }
        }
    }
    const _ : () = assert!(core::mem::align_of::<sys::XAUDIO2_EFFECT_DESCRIPTOR>() <= core::mem::align_of::<EffectDescriptor>());
    const _ : () = assert!(core::mem::size_of ::<sys::XAUDIO2_EFFECT_DESCRIPTOR>() == core::mem::size_of ::<EffectDescriptor>());

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_send_descriptor)\]
    /// [XAUDIO2_SEND_DESCRIPTOR](sys::XAUDIO2_SEND_DESCRIPTOR): safe equivalent
    #[repr(C, packed(1))] pub struct SendDescriptor<'a> { // XXX: note that packed here means output_voice isn't 8-byte aligned on x64!
        pub flags:          u32,
        pub output_voice:   &'a sys::IXAudio2Voice, // XXX: mcom::Rc might be saner here?
    }
    impl<'a> SendDescriptor<'a> {
        pub fn new(flags: u32, output_voice: &'a sys::IXAudio2Voice) -> Self { Self { flags, output_voice } }
    }
    const _ : () = assert!(core::mem::align_of::<sys::XAUDIO2_SEND_DESCRIPTOR>() <= core::mem::align_of::<SendDescriptor>());
    const _ : () = assert!(core::mem::size_of ::<sys::XAUDIO2_SEND_DESCRIPTOR>() == core::mem::size_of ::<SendDescriptor>());

    pub use sys::{
        XAUDIO2_DEBUG_CONFIGURATION     as DebugConfiguration,
        XAUDIO2_FILTER_PARAMETERS       as FilterParameters,
        XAUDIO2_PERFORMANCE_DATA        as PerformanceData,
        XAUDIO2_VOICE_DETAILS           as VoiceDetails,
        XAUDIO2_VOICE_STATE             as VoiceState,
    };
}
