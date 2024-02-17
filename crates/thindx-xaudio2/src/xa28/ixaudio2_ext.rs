use super::*;
use super::xaudio2::sys::*;

use abistr::AsOptCStr;
use winapi::um::audiosessiontypes::AUDIO_STREAM_CATEGORY;
use winresult::*;

use alloc::boxed::Box;
use core::ptr::{null_mut, null};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2)\]
/// IXAudio2
/// &mdash; Top-level XAudio 2.x COM interface/factory pointer.
///
#[derive(Clone)] #[repr(transparent)] pub struct XAudio2(mcom::Rc<IXAudio2>);
impl core::ops::Deref for XAudio2 { type Target = mcom::Rc<IXAudio2>; fn deref(&self) -> &Self::Target { &self.0 } }

impl XAudio2 {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-registerforcallbacks)\]
    /// Adds a new client to receive XAudio2's engine callbacks.
    pub fn register_for_callbacks(&self, callback: &'static IXAudio2EngineCallback) -> Result<HResultSuccess, HResultError> {
        // SAFETY: IXAudio2EngineCallback must outlive self - this is enforced by &'static lifetime.
        unsafe { self.RegisterForCallbacks(callback) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-registerforcallbacks)\]
    /// [Self::register_for_callbacks]\([Box::leak]\([Box::new]\(...\)\)\)
    ///
    /// Leaks memory - but let's be honest, you were going to register the engine
    /// callback for the duration of your program, and never reclaim the memory
    /// you're 'leaking' with this method anyways.  A fancier scoped-based
    /// alternative could be written if you absolutely must reclaim memory safely.
    ///
    /// The returned IXAudio2EngineCallback can be unregistered and reregistered
    /// (through the non-`_leak` version of this method) if you're into that kind of thing.
    pub fn register_for_callbacks_leak(&self, callback: impl xaudio2::EngineCallback + 'static) -> Result<&'static IXAudio2EngineCallback, HResultError> {
        let leaked = Box::leak(Box::new(xaudio2::EngineCallback::wrap(callback)));
        self.register_for_callbacks(leaked)?;
        Ok(leaked)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-unregisterforcallbacks)\]
    /// Removes an existing receiver of XAudio2 engine callbacks.
    pub fn unregister_for_callbacks(&self, callback: &IXAudio2EngineCallback) {
        // SAFETY: Since we're *un*registering `callback`, it need not be 'static.
        unsafe { self.UnregisterForCallbacks(callback) }
        // if the IXAudio2 *did* reference `callback`, it no longer does.
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    pub fn create_source_voice_typed_callback<'xa2cb, S: Send + Sync + Sized + 'static, VC: xaudio2::VoiceCallback>(
        &'xa2cb self,
        format:                 &xaudio2::TypedSourceFormat<S>,
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               &'xa2cb xaudio2::VoiceCallbackWrapper<VC>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoice<'xa2cb, S, VC::BufferContext>, HResultError> {
        let voice = unsafe { self.create_source_voice_unchecked(format, flags, max_frequency_ratio, Some(callback), send_list, effect_chain) }?;
        Ok(unsafe { xaudio2::SourceVoice::from_raw(self, voice.into_raw().cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    pub fn create_source_voice_dynamic<'xa2cb, VC: xaudio2::VoiceCallback>(
        &'xa2cb self,
        format:                 &xaudio2::SourceFormat,
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               &'xa2cb xaudio2::VoiceCallbackWrapper<VC>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoiceDynamic<'xa2cb, VC::BufferContext>, HResultError> {
        let voice = unsafe { self.create_source_voice_unchecked(format, flags, max_frequency_ratio, Some(callback), send_list, effect_chain) }?;
        Ok(unsafe { xaudio2::SourceVoiceDynamic::from_raw(self, voice.into_raw().cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice)\]
    /// Creates and configures a source voice.
    ///
    /// ### Safety
    /// *   `callback` may make demands of submitted [XAUDIO2_BUFFER::pContext]s for soundness purpouses.
    pub unsafe fn create_source_voice_unchecked<'xa2cb>(
        &'xa2cb self,
        format:                 &xaudio2::SourceFormat,
        flags:                  u32,
        max_frequency_ratio:    f32,
        callback:               Option<&'xa2cb IXAudio2VoiceCallback>,
        send_list:              Option<&[xaudio2::SendDescriptor]>,
        effect_chain:           Option<&[xaudio2::EffectDescriptor]>,
    ) -> Result<xaudio2::SourceVoiceUntyped<'xa2cb>, HResultError> {
        let mut voice = null_mut();

        let send_list = send_list.map(|sl| -> Result<XAUDIO2_VOICE_SENDS, HResultError> { Ok(XAUDIO2_VOICE_SENDS {
            SendCount:  u32::try_from(sl.len()).map_err(|_| E::INVALIDARG)?,
            pSends:     sl.as_ptr() as *mut _,
        })}).transpose()?;

        let effect_chain = effect_chain.map(|ec| -> Result<XAUDIO2_EFFECT_CHAIN, HResultError> { Ok(XAUDIO2_EFFECT_CHAIN {
            EffectCount:        u32::try_from(ec.len()).map_err(|_| E::INVALIDARG)?,
            pEffectDescriptors: ec.as_ptr() as *mut _,
        })}).transpose()?;

        let hr = unsafe { self.CreateSourceVoice(
            &mut voice,
            format.as_source_format(),
            flags,
            max_frequency_ratio,
            callback                    .map_or(null(), |c| c),
            send_list       .as_ref()   .map_or(null(), |c| c),
            effect_chain    .as_ref()   .map_or(null(), |c| c),
        )};
        let voice = unsafe { xaudio2::SourceVoiceUntyped::from_raw_opt(self, voice) };
        hr.succeeded()?;
        let voice = voice.ok_or(E::NOINTERFACE)?;
        Ok(voice)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsubmixvoice)\]
    /// Creates and configures a submix voice.
    pub fn create_submix_voice(
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

        let hr = unsafe { self.CreateSubmixVoice(
            &mut voice,
            input_channels,
            input_sample_rate,
            flags,
            processing_stage,
            send_list       .as_ref().map_or(null(), |c| c),
            effect_chain    .as_ref().map_or(null(), |c| c),
        )};
        let voice = unsafe { xaudio2::SubmixVoice::from_raw_opt(self, voice) };
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
    /// | device_id         | ()
    /// | effect_chain      | None
    /// | stream_category   | [xaudio2::DEFAULT_AUDIO_CATEGORY]
    pub fn create_mastering_voice(
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

        let hr = unsafe { self.CreateMasteringVoice(
            &mut voice,
            input_channels,
            input_sample_rate,
            flags,
            device_id       .try_into().map_err(|_| E::INVALIDARG)?.as_opt_cstr(),
            effect_chain    .as_ref().map_or(null(), |c| c),
            stream_category,
        )};
        let voice = unsafe { xaudio2::MasteringVoice::from_raw_opt(self, voice) };
        hr.succeeded()?;
        let voice = voice.ok_or(E::NOINTERFACE)?;
        Ok(voice)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-startengine)\]
    /// Creates and starts the audio processing thread.
    pub fn start_engine(&self) -> Result<HResultSuccess, HResultError> { unsafe { self.StartEngine() }.succeeded() }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-stopengine)\]
    /// Stops and destroys the audio processing thread.
    pub fn stop_engine(&self) { unsafe { self.StopEngine() } }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-commitchanges)\]
    /// Atomically applies a set of operations previously tagged with a given identifier.
    pub fn commit_changes(&self, operation_set: u32) -> Result<HResultSuccess, HResultError> { unsafe { self.CommitChanges(operation_set) }.succeeded() }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-getperformancedata)\]
    /// Returns current resource usage details: memory, CPU, etc.
    pub fn get_performance_data(&self) -> xaudio2::PerformanceData {
        let mut data = Default::default();
        unsafe { self.GetPerformanceData(&mut data) };
        data
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-setdebugconfiguration)\]
    /// Configures XAudio2's debug output (in debug builds only).
    pub fn set_debug_configuration(&self, debug_configuration: &xaudio2::DebugConfiguration, _reserved: Option<core::convert::Infallible>) {
        unsafe { self.SetDebugConfiguration(debug_configuration, null()) };
    }
}
