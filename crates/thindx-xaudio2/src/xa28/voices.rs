use super::*;
#[cfg(doc)] use xaudio2::*;
#[cfg(doc)] use sys::*;

use core::marker::PhantomData;
use core::ops::*;
use core::ptr::NonNull;



macro_rules! voices {
    ($(
        $(#[doc = $voice_doc:literal])*
        pub struct $voice:ident <'xa2> ( NonNull< $ivoice:ty > );
    )*) => {$(
        $(#[doc = $voice_doc])*
        #[repr(transparent)] pub struct $voice <'xa2> {
            factory:    PhantomData<&'xa2 IXAudio2>,
            voice:      NonNull< $ivoice >,
        }

        impl<'xa2> $voice <'xa2> {
            /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
            /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
            ///
            /// (Dropping the voice also implicitly stops/removes it.)
            pub fn destroy_voice(self) {}
            // "It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback)."
            // Currently this cannot happen as Self : !Send.
            // Consider using https://docs.rs/static_assertions/latest/static_assertions/macro.assert_not_impl_any.html to verify this.
            // See also: util::xaudio2_thread_guard

            /// Create a voice wrapper from a raw pointer.
            ///
            /// If `raw` is null, will return [None].
            ///
            /// ### Safety
            /// *   `raw` must be a valid interface pointer if not null.
            /// *   `Self` takes ownership of `raw`.
            pub unsafe fn from_raw_opt(_xa2: &'xa2 IXAudio2, raw: *const $ivoice) -> Option<Self> { Some(Self {
                factory:    PhantomData,
                voice:      NonNull::new(raw as *mut _)?,
            })}

            /// Create a voice wrapper from a raw pointer.
            ///
            /// If `raw` is null, will [panic!].
            ///
            /// ### Safety
            /// *   `raw` must be a valid interface pointer if not null.
            /// *   `Self` takes ownership of `raw`.
            #[track_caller] pub unsafe fn from_raw(xa2: &'xa2 IXAudio2, raw: *const $ivoice) -> Self { unsafe { Self::from_raw_opt(xa2, raw) }.unwrap() }

            /// Convert `self` back into a raw pointer, relinquishing ownership.
            pub fn into_raw(self) -> *const $ivoice {
                let ptr = self.voice.as_ptr();
                core::mem::forget(self);
                ptr
            }

            pub fn as_raw(&self) -> *const $ivoice { self.voice.as_ptr() }
        }

        impl<'xa2> Deref      for $voice <'xa2> { fn deref    (&    self) -> &    Self::Target { unsafe { self.voice.as_ref() } } type Target = $ivoice; }
        impl<'xa2> DerefMut   for $voice <'xa2> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.voice.as_mut() } } }
        impl<'xa2> Drop       for $voice <'xa2> { fn drop(&mut self) { unsafe { (*self.voice.as_ptr()).DestroyVoice() } } }
    )*};
}

voices! {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voice)\] [IXAudio2Voice]
    ///
    /// Deref chain: [`Voice`] → [`IXAudio2Voice`]\([`Ext`](IXAudio2VoiceExt)\)
    ///
    /// ### Methods (via `IXAudio2VoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](IXAudio2VoiceExt::get_voice_details)                        | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](IXAudio2VoiceExt::set_output_voices)                        | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](IXAudio2VoiceExt::set_effect_chain)                          | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](IXAudio2VoiceExt::enable_effect)                                | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](IXAudio2VoiceExt::disable_effect)                              | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](IXAudio2VoiceExt::get_effect_state)                          | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](IXAudio2VoiceExt::set_effect_parameters_raw)        | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](IXAudio2VoiceExt::get_effect_parameters_raw)        | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](IXAudio2VoiceExt::set_filter_parameters)                | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](IXAudio2VoiceExt::get_filter_parameters)                | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](IXAudio2VoiceExt::set_output_filter_parameters)  | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](IXAudio2VoiceExt::get_output_filter_parameters)  | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](IXAudio2VoiceExt::set_volume)                                      | Sets this voice’s overall volume level.
    /// | [`get_volume`](IXAudio2VoiceExt::get_volume)                                      | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](IXAudio2VoiceExt::set_channel_volumes)                    | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](IXAudio2VoiceExt::get_channel_volumes)                    | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](IXAudio2VoiceExt::set_output_matrix)                        | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](IXAudio2VoiceExt::get_output_matrix)                        | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct Voice<'xa2>(NonNull<IXAudio2Voice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\] [IXAudio2MasteringVoice]
    ///
    /// Deref chain: [`MasteringVoice`] → [`IXAudio2MasteringVoice`]\([`Ext`](IXAudio2MasteringVoiceExt)\) → [`IXAudio2Voice`]\([`Ext`](IXAudio2VoiceExt)\)
    ///
    /// ### Methods (via `IXAudio2MasteringVoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_channel_mask`](IXAudio2MasteringVoiceExt::get_channel_mask)                 | Returns the channel mask for this voice.
    ///
    /// ### Methods (via `IXAudio2VoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](IXAudio2VoiceExt::get_voice_details)                        | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](IXAudio2VoiceExt::set_output_voices)                        | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](IXAudio2VoiceExt::set_effect_chain)                          | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](IXAudio2VoiceExt::enable_effect)                                | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](IXAudio2VoiceExt::disable_effect)                              | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](IXAudio2VoiceExt::get_effect_state)                          | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](IXAudio2VoiceExt::set_effect_parameters_raw)        | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](IXAudio2VoiceExt::get_effect_parameters_raw)        | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](IXAudio2VoiceExt::set_filter_parameters)                | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](IXAudio2VoiceExt::get_filter_parameters)                | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](IXAudio2VoiceExt::set_output_filter_parameters)  | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](IXAudio2VoiceExt::get_output_filter_parameters)  | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](IXAudio2VoiceExt::set_volume)                                      | Sets this voice’s overall volume level.
    /// | [`get_volume`](IXAudio2VoiceExt::get_volume)                                      | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](IXAudio2VoiceExt::set_channel_volumes)                    | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](IXAudio2VoiceExt::get_channel_volumes)                    | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](IXAudio2VoiceExt::set_output_matrix)                        | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](IXAudio2VoiceExt::get_output_matrix)                        | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    ///
    pub struct MasteringVoice<'xa2>(NonNull<IXAudio2MasteringVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\] [IXAudio2SubmixVoice]
    ///
    /// Deref chain: [`SubmixVoice`] → [`IXAudio2SubmixVoice`]\(~~`Ext`~~\) → [`IXAudio2Voice`]\([`Ext`](IXAudio2VoiceExt)\)
    ///
    /// ### Methods (via `IXAudio2VoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](IXAudio2VoiceExt::get_voice_details)                        | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](IXAudio2VoiceExt::set_output_voices)                        | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](IXAudio2VoiceExt::set_effect_chain)                          | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](IXAudio2VoiceExt::enable_effect)                                | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](IXAudio2VoiceExt::disable_effect)                              | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](IXAudio2VoiceExt::get_effect_state)                          | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](IXAudio2VoiceExt::set_effect_parameters_raw)        | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](IXAudio2VoiceExt::get_effect_parameters_raw)        | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](IXAudio2VoiceExt::set_filter_parameters)                | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](IXAudio2VoiceExt::get_filter_parameters)                | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](IXAudio2VoiceExt::set_output_filter_parameters)  | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](IXAudio2VoiceExt::get_output_filter_parameters)  | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](IXAudio2VoiceExt::set_volume)                                      | Sets this voice’s overall volume level.
    /// | [`get_volume`](IXAudio2VoiceExt::get_volume)                                      | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](IXAudio2VoiceExt::set_channel_volumes)                    | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](IXAudio2VoiceExt::get_channel_volumes)                    | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](IXAudio2VoiceExt::set_output_matrix)                        | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](IXAudio2VoiceExt::get_output_matrix)                        | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct SubmixVoice<'xa2>(NonNull<IXAudio2SubmixVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
    ///
    /// Deref chain: [`SourceVoiceUntyped`] → [`IXAudio2SourceVoiceExt`]\([`Ext`](IXAudio2SourceVoiceExt)\) → [`IXAudio2Voice`]\([`Ext`](IXAudio2VoiceExt)\)
    ///
    /// ### Methods (via `IXAudio2SourceVoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`start`](IXAudio2SourceVoiceExt::start)                                          | Makes this voice start consuming and processing audio.
    /// | [`stop`](IXAudio2SourceVoiceExt::stop)                                            | Makes this voice stop consuming audio.
    /// | [`flush_source_buffers`](IXAudio2SourceVoiceExt::flush_source_buffers)            | Removes all pending audio buffers from this voice's queue.
    /// | [`discontinuity`](IXAudio2SourceVoiceExt::discontinuity)                          | Notifies the voice of an intentional break in the stream of audio buffers (e.g. the end of a sound), to prevent XAudio2 from interpreting an empty buffer queue as a glitch.
    /// | [`exit_loop`](IXAudio2SourceVoiceExt::exit_loop)                                  | Breaks out of the current loop when its end is reached.
    /// | [`get_state`](IXAudio2SourceVoiceExt::get_state)                                  | Returns the number of buffers currently queued on this voice, the pContext value associated with the currently processing buffer (if any), and other voice state information.
    /// | [`set_frequency_ratio`](IXAudio2SourceVoiceExt::set_frequency_ratio)              | Sets this voice's frequency adjustment, i.e. its pitch.
    /// | [`get_frequency_ratio`](IXAudio2SourceVoiceExt::get_frequency_ratio)              | Returns this voice's current frequency adjustment ratio.
    /// | [`set_source_sample_rate`](IXAudio2SourceVoiceExt::set_source_sample_rate)        | Reconfigures this voice to treat its source data as being at a different sample rate than the original one specified in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
    ///
    /// ### Methods (via `IXAudio2VoiceExt` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](IXAudio2VoiceExt::get_voice_details)                        | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](IXAudio2VoiceExt::set_output_voices)                        | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](IXAudio2VoiceExt::set_effect_chain)                          | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](IXAudio2VoiceExt::enable_effect)                                | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](IXAudio2VoiceExt::disable_effect)                              | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](IXAudio2VoiceExt::get_effect_state)                          | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](IXAudio2VoiceExt::set_effect_parameters_raw)        | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](IXAudio2VoiceExt::get_effect_parameters_raw)        | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](IXAudio2VoiceExt::set_filter_parameters)                | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](IXAudio2VoiceExt::get_filter_parameters)                | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](IXAudio2VoiceExt::set_output_filter_parameters)  | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](IXAudio2VoiceExt::get_output_filter_parameters)  | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](IXAudio2VoiceExt::set_volume)                                      | Sets this voice’s overall volume level.
    /// | [`get_volume`](IXAudio2VoiceExt::get_volume)                                      | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](IXAudio2VoiceExt::set_channel_volumes)                    | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](IXAudio2VoiceExt::get_channel_volumes)                    | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](IXAudio2VoiceExt::set_output_matrix)                        | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](IXAudio2VoiceExt::get_output_matrix)                        | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct SourceVoiceUntyped<'xa2>(NonNull<IXAudio2SourceVoice>);
}
