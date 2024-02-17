use super::xaudio2;
use super::xaudio2::sys::*;

use winresult::*;

use core::ptr::null;



impl xaudio2::Voice<'_> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getvoicedetails)\]
    /// Returns the basic characteristics of this voice.
    pub fn get_voice_details(&self) -> xaudio2::VoiceDetails {
        let mut details = Default::default();
        unsafe { self.as_ref().GetVoiceDetails(&mut details) };
        details
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputvoices)\]
    /// Replaces the set of submix/mastering voices that receive this voice's output.
    pub fn set_output_voices(&self, send_list: Option<&[xaudio2::SendDescriptor]>) -> Result<HResultSuccess, HResultError> {
        let send_list = send_list.map(|sl| -> Result<XAUDIO2_VOICE_SENDS, HResultError> { Ok(XAUDIO2_VOICE_SENDS {
            SendCount:  u32::try_from(sl.len()).map_err(|_| E::INVALIDARG)?,
            pSends:     sl.as_ptr() as *mut _,
        })}).transpose()?;
        unsafe { self.as_ref().SetOutputVoices(send_list.as_ref().map_or(null(), |r| r)) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-seteffectchain)\]
    /// Replaces this voice's current effect chain with a new one.
    pub fn set_effect_chain(&self, effect_chain: Option<&[xaudio2::EffectDescriptor]>) -> Result<HResultSuccess, HResultError> {
        let effect_chain = effect_chain.map(|ec| -> Result<XAUDIO2_EFFECT_CHAIN, HResultError> { Ok(XAUDIO2_EFFECT_CHAIN {
            EffectCount:        u32::try_from(ec.len()).map_err(|_| E::INVALIDARG)?,
            pEffectDescriptors: ec.as_ptr() as *mut _,
        })}).transpose()?;
        unsafe { self.as_ref().SetEffectChain(effect_chain.as_ref().map_or(null(), |r| r)) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-enableeffect)\]
    /// Enables an effect in this voice's effect chain.
    pub fn enable_effect(&self, effect_index: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().EnableEffect(effect_index, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-disableeffect)\]
    /// Disables an effect in this voice's effect chain.
    pub fn disable_effect(&self, effect_index: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().DisableEffect(effect_index, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-geteffectstate)\]
    /// Returns the running state of an effect.
    pub fn get_effect_state(&self, effect_index: u32) -> bool {
        let mut state = Default::default();
        unsafe { self.as_ref().GetEffectState(effect_index, &mut state) };
        state.into()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-seteffectparameters)\]
    /// Sets effect-specific parameters.
    ///
    /// Unlike IXAPOParameters::SetParameters, this method may be called from any thread.
    /// XAudio2 implements appropriate synchronization to copy the parameters to the realtime audio processing thread.
    pub fn set_effect_parameters_raw<P: bytemuck::Pod>(&self, effect_index: u32, parameters: &P, operation_set: u32) -> Result<HResultSuccess, HResultError> { // TODO: constraint P to "EffectParameters" trait?
        let parameters = bytemuck::bytes_of(parameters);
        let parameter_bytes = u32::try_from(core::mem::size_of_val(parameters)).map_err(|_| E::INVALIDARG)?;
        unsafe { self.as_ref().SetEffectParameters(effect_index, parameters.as_ptr(), parameter_bytes, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-geteffectparameters)\]
    /// Obtains the current effect-specific parameters.
    pub fn get_effect_parameters_raw<P: bytemuck::Pod + bytemuck::Zeroable>(&self, effect_index: u32) -> Result<P, HResultError> { // TODO: constraint P to "EffectParameters" trait?
        let mut parameters = P::zeroed();
        let bytes = bytemuck::bytes_of_mut(&mut parameters);
        let n_bytes = u32::try_from(core::mem::size_of_val(bytes)).map_err(|_| E::INVALIDARG)?;
        unsafe { self.as_ref().GetEffectParameters(effect_index, bytes.as_mut_ptr(), n_bytes) }.succeeded()?;
        Ok(parameters)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setfilterparameters)\]
    /// Sets this voice's filter parameters.
    pub fn set_filter_parameters(&self, parameters: &xaudio2::FilterParameters, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().SetFilterParameters(parameters, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getfilterparameters)\]
    /// Returns this voice's current filter parameters.
    pub fn get_filter_parameters(&self) -> xaudio2::FilterParameters {
        let mut parameters = Default::default();
        unsafe { self.as_ref().GetFilterParameters(&mut parameters) };
        parameters
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputfilterparameters)\]
    /// Sets the filter parameters on one of this voice's sends.
    pub fn set_output_filter_parameters(&self, destination_voice: &IXAudio2Voice, parameters: &xaudio2::FilterParameters, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().SetOutputFilterParameters(destination_voice, parameters, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getoutputfilterparameters)\]
    /// Returns the filter parameters from one of this voice's sends.
    pub fn get_output_filter_parameters(&self, destination_voice: &IXAudio2Voice) -> xaudio2::FilterParameters {
        let mut parameters = Default::default();
        unsafe { self.as_ref().GetOutputFilterParameters(destination_voice, &mut parameters) };
        parameters
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setvolume)\]
    /// Sets this voice's overall volume level.
    pub fn set_volume(&self, volume: f32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().SetVolume(volume, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getvolume)\]
    /// Obtains this voice's current overall volume level.
    pub fn get_volume(&self) -> f32 {
        let mut v = 0.0;
        unsafe { self.as_ref().GetVolume(&mut v) };
        v
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setchannelvolumes)\]
    /// Sets this voice's per-channel volume levels.
    pub fn set_channel_volumes(&self, volumes: &[f32], operation_set: u32) -> Result<HResultSuccess, HResultError> {
        let channels = u32::try_from(volumes.len()).map_err(|_| E::INVALIDARG)?;
        unsafe { self.as_ref().SetChannelVolumes(channels, volumes.as_ptr(), operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getchannelvolumes)\]
    /// Returns this voice's current per-channel volume levels.
    pub fn get_channel_volumes(&self, volumes: &mut [f32]) {
        let channels = u32::try_from(volumes.len()).expect("BUG: tried to call IXAudio2Voice::GetChannelVolumes with > 4 billion channels? -_-");
        unsafe { self.as_ref().GetChannelVolumes(channels, volumes.as_mut_ptr()) };
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-setoutputmatrix)\]
    /// Sets the volume levels used to mix from each channel of this voice's output audio to each channel of a given destination voice's input audio.
    pub fn set_output_matrix(&self, destination_voice: &IXAudio2Voice, source_channels: u32, destination_channels: u32, level_matrix: &[f32], operation_set: u32) -> Result<HResultSuccess, HResultError> {
        let n = (source_channels as usize).checked_mul(destination_channels as usize).ok_or(E::INVALIDARG)?;
        if level_matrix.len() != n { return Err(E::INVALIDARG) }
        unsafe { self.as_ref().SetOutputMatrix(destination_voice, source_channels, destination_channels, level_matrix.as_ptr(), operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-getoutputmatrix)\]
    /// Obtains the volume levels used to send each channel of this voice's output audio to each channel of a given destination voice's input audio.
    pub fn get_output_matrix(&self, destination_voice: &IXAudio2Voice, source_channels: u32, destination_channels: u32, level_matrix: &mut [f32]) {
        let n = (source_channels as usize).checked_mul(destination_channels as usize).expect("BUG: tried to call IXAudio2Voice::GetOutputMatrix with a > 4 billion channel matrix? -_-");
        if level_matrix.len() != n { panic!("BUG: source_channels ({source_channels}) x destination_channels ({destination_channels}) != level_matrix.len() ({})", level_matrix.len()) }
        unsafe { self.as_ref().GetOutputMatrix(destination_voice, source_channels, destination_channels, level_matrix.as_mut_ptr()) }
    }
}
