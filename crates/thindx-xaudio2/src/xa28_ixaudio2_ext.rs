use super::*;
use super::xaudio2::sys::*;

use abistr::AsOptCStr;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::um::audiosessiontypes::AUDIO_STREAM_CATEGORY;
use winresult::*;

use core::mem::size_of_val;
use core::ptr::{null_mut, null};



impl IXAudio2Ext for IXAudio2 { fn _as_ixaudio2(&self) -> &IXAudio2 { self } }

/// [IXAudio2] extension methods
pub trait IXAudio2Ext {
    #[doc(hidden)] fn _as_ixaudio2(&self) -> &IXAudio2;

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-registerforcallbacks)\]
    /// Adds a new client to receive XAudio2's engine callbacks.
    fn register_for_callbacks(&self, callback: &IXAudio2EngineCallback) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().RegisterForCallbacks(callback) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-unregisterforcallbacks)\]
    /// Removes an existing receiver of XAudio2 engine callbacks.
    fn unregister_for_callbacks(&self, callback: &IXAudio2EngineCallback) {
        unsafe { self._as_ixaudio2().UnregisterForCallbacks(callback) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    fn create_source_voice_no_callback(
        &self,
        format:                 &WAVEFORMATEX, // TODO: safer type?
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               Option<core::convert::Infallible>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoiceUntyped, HResultError> { // XXX: Theoretically leaks Box<()>s
        let _ = callback;
        unsafe { self.create_source_voice_unchecked(format, flags, max_frequency_ratio, None, send_list, effect_chain) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    fn create_source_voice_typed_callback<VC: xaudio2::VoiceCallback>(
        &self,
        format:                 &WAVEFORMATEX, // TODO: safer type?
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               &xaudio2::VoiceCallbackWrapper<VC>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoice<VC>, HResultError> {
        let voice = unsafe { self.create_source_voice_unchecked(format, flags, max_frequency_ratio, Some(callback.as_interface()), send_list, effect_chain) }?;
        Ok(unsafe { xaudio2::SourceVoice::from_raw(voice.into_raw().cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    ///
    /// ### Safety
    /// *   `callback` may make demands of [IXAudio2SourceVoiceExt::submit_source_buffer_unchecked]ed [XAUDIO2_BUFFER::pContext]s for soundness purpouses.
    unsafe fn create_source_voice_unchecked(
        &self,
        format:                 &WAVEFORMATEX, // TODO: safer type?
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               Option<&IXAudio2VoiceCallback>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoiceUntyped, HResultError> {
        // so much for "unchecked" - more like "less checked" amirite
        if usize::from(format.cbSize) != size_of_val(format) { return Err(E::INVALIDARG) }

        let mut voice = null_mut();

        let send_list = send_list.map(|sl| -> Result<XAUDIO2_VOICE_SENDS, HResultError> { Ok(XAUDIO2_VOICE_SENDS {
            SendCount:  u32::try_from(sl.len()).map_err(|_| E::INVALIDARG)?,
            pSends:     sl.as_ptr() as *mut _,
        })}).transpose()?;

        let effect_chain = effect_chain.map(|ec| -> Result<XAUDIO2_EFFECT_CHAIN, HResultError> { Ok(XAUDIO2_EFFECT_CHAIN {
            EffectCount:        u32::try_from(ec.len()).map_err(|_| E::INVALIDARG)?,
            pEffectDescriptors: ec.as_ptr() as *mut _,
        })}).transpose()?;

        let hr = unsafe { self._as_ixaudio2().CreateSourceVoice(
            &mut voice,
            format,
            flags,
            max_frequency_ratio,
            callback                    .map_or(null(), |c| c),
            send_list       .as_ref()   .map_or(null(), |c| c),
            effect_chain    .as_ref()   .map_or(null(), |c| c),
        )};
        let voice = unsafe { xaudio2::SourceVoiceUntyped::from_raw_opt(voice) };
        hr.succeeded()?;
        let voice = voice.ok_or(E::NOINTERFACE)?;
        Ok(voice)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsubmixvoice)\]
    /// Creates and configures a submix voice.
    fn create_submix_voice(
        &self,
        input_channels:     u32,
        input_sample_rate:  u32,
        flags:              u32,
        processing_stage:   u32,
        send_list:          Option<&[xaudio2::SendDescriptor]>,
        effect_chain:       Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SubmixVoice, HResultError> {
        let mut voice = null_mut();

        let send_list = send_list.map(|sl| -> Result<XAUDIO2_VOICE_SENDS, HResultError> { Ok(XAUDIO2_VOICE_SENDS {
            SendCount:  u32::try_from(sl.len()).map_err(|_| E::INVALIDARG)?,
            pSends:     sl.as_ptr() as *mut _,
        })}).transpose()?;

        let effect_chain = effect_chain.map(|ec| -> Result<XAUDIO2_EFFECT_CHAIN, HResultError> { Ok(XAUDIO2_EFFECT_CHAIN {
            EffectCount:        u32::try_from(ec.len()).map_err(|_| E::INVALIDARG)?,
            pEffectDescriptors: ec.as_ptr() as *mut _,
        })}).transpose()?;

        let hr = unsafe { self._as_ixaudio2().CreateSubmixVoice(
            &mut voice,
            input_channels,
            input_sample_rate,
            flags,
            processing_stage,
            send_list       .as_ref().map_or(null(), |c| c),
            effect_chain    .as_ref().map_or(null(), |c| c),
        )};
        let voice = unsafe { xaudio2::SubmixVoice::from_raw_opt(voice) };
        hr.succeeded()?;
        let voice = voice.ok_or(E::NOINTERFACE)?;
        Ok(voice)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createmasteringvoice)\]
    /// Creates and configures a mastering voice.
    ///
    /// | Argument          | Default   |
    /// | ----------------- | --------- |
    /// | input_channels    | [xaudio2::DEFAULT_CHANNELS]
    /// | input_sample_rate | [xaudio2::DEFAULT_SAMPLERATE]
    /// | flags             | 0
    /// | device_id         | None
    /// | effect_chain      | None
    /// | stream_category   | [xaudio2::DEFAULT_AUDIO_CATEGORY]
    fn create_mastering_voice(
        &self,
        input_channels:     u32,
        input_sample_rate:  u32,
        flags:              u32,
        device_id:          impl abistr::TryIntoAsOptCStr<u16>,
        effect_chain:       Option<&[xaudio2::EffectDescriptor]>,
        stream_category:    AUDIO_STREAM_CATEGORY,
    ) -> Result<xaudio2::MasteringVoice, HResultError> {
        let mut voice = null_mut();

        let effect_chain = effect_chain.map(|ec| -> Result<XAUDIO2_EFFECT_CHAIN, HResultError> { Ok(XAUDIO2_EFFECT_CHAIN {
            EffectCount:        u32::try_from(ec.len()).map_err(|_| E::INVALIDARG)?,
            pEffectDescriptors: ec.as_ptr() as *mut _,
        })}).transpose()?;

        let hr = unsafe { self._as_ixaudio2().CreateMasteringVoice(
            &mut voice,
            input_channels,
            input_sample_rate,
            flags,
            device_id       .try_into().map_err(|_| E::INVALIDARG)?.as_opt_cstr(),
            effect_chain    .as_ref().map_or(null(), |c| c),
            stream_category,
        )};
        let voice = unsafe { xaudio2::MasteringVoice::from_raw_opt(voice) };
        hr.succeeded()?;
        let voice = voice.ok_or(E::NOINTERFACE)?;
        Ok(voice)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-startengine)\]
    /// Creates and starts the audio processing thread.
    fn start_engine(&self) -> Result<HResultSuccess, HResultError> { unsafe { self._as_ixaudio2().StartEngine() }.succeeded() }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-stopengine)\]
    /// Stops and destroys the audio processing thread.
    fn stop_engine(&self) { unsafe { self._as_ixaudio2().StopEngine() } }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-commitchanges)\]
    /// Atomically applies a set of operations previously tagged with a given identifier.
    fn commit_changes(&self, operation_set: u32) -> Result<HResultSuccess, HResultError> { unsafe { self._as_ixaudio2().CommitChanges(operation_set) }.succeeded() }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-getperformancedata)\]
    /// Returns current resource usage details: memory, CPU, etc.
    fn get_performance_data(&self) -> xaudio2::PerformanceData {
        let mut data = Default::default();
        unsafe { self._as_ixaudio2().GetPerformanceData(&mut data) };
        data
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-setdebugconfiguration)\]
    /// Configures XAudio2's debug output (in debug builds only).
    fn set_debug_configuration(&self, debug_configuration: &xaudio2::DebugConfiguration, _reserved: Option<core::convert::Infallible>) {
        unsafe { self._as_ixaudio2().SetDebugConfiguration(debug_configuration, null()) };
    }
}
