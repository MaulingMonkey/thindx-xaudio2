use super::*;
#[cfg(doc)] use xaudio2::*;
#[cfg(doc)] use sys::*;

use core::marker::PhantomData;
use core::mem::transmute;
use core::ops::{Deref, DerefMut};
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
            #[allow(dead_code)] pub(crate) fn as_ref(&self) -> &$ivoice { unsafe { self.voice.as_ref() } }
        }

        impl<'xa2> Drop for $voice <'xa2> { fn drop(&mut self) { unsafe { (*self.voice.as_ptr()).DestroyVoice() } } }
    )*};
}

voices! {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voice)\] [IXAudio2Voice]
    ///
    /// ### Methods (on `Voice` itself)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](Voice::get_voice_details)                                   | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](Voice::set_output_voices)                                   | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](Voice::set_effect_chain)                                     | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](Voice::enable_effect)                                           | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](Voice::disable_effect)                                         | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](Voice::get_effect_state)                                     | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](Voice::set_effect_parameters_raw)                   | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](Voice::get_effect_parameters_raw)                   | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](Voice::set_filter_parameters)                           | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](Voice::get_filter_parameters)                           | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](Voice::set_output_filter_parameters)             | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](Voice::get_output_filter_parameters)             | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](Voice::set_volume)                                                 | Sets this voice’s overall volume level.
    /// | [`get_volume`](Voice::get_volume)                                                 | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](Voice::set_channel_volumes)                               | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](Voice::get_channel_volumes)                               | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](Voice::set_output_matrix)                                   | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](Voice::get_output_matrix)                                   | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct Voice<'xa2>(NonNull<IXAudio2Voice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\] [IXAudio2MasteringVoice]
    ///
    /// ### Methods (on `MasteringVoice` itself)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_channel_mask`](MasteringVoice::get_channel_mask)                            | Returns the channel mask for this voice.
    ///
    /// ### Methods (via `Voice` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](Voice::get_voice_details)                                   | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](Voice::set_output_voices)                                   | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](Voice::set_effect_chain)                                     | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](Voice::enable_effect)                                           | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](Voice::disable_effect)                                         | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](Voice::get_effect_state)                                     | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](Voice::set_effect_parameters_raw)                   | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](Voice::get_effect_parameters_raw)                   | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](Voice::set_filter_parameters)                           | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](Voice::get_filter_parameters)                           | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](Voice::set_output_filter_parameters)             | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](Voice::get_output_filter_parameters)             | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](Voice::set_volume)                                                 | Sets this voice’s overall volume level.
    /// | [`get_volume`](Voice::get_volume)                                                 | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](Voice::set_channel_volumes)                               | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](Voice::get_channel_volumes)                               | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](Voice::set_output_matrix)                                   | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](Voice::get_output_matrix)                                   | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    ///
    pub struct MasteringVoice<'xa2>(NonNull<IXAudio2MasteringVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\] [IXAudio2SubmixVoice]
    ///
    /// ### Methods (on `SubmixVoice` itself)
    /// No type-specific methods.
    ///
    /// ### Methods (via `Voice` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](Voice::get_voice_details)                                   | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](Voice::set_output_voices)                                   | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](Voice::set_effect_chain)                                     | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](Voice::enable_effect)                                           | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](Voice::disable_effect)                                         | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](Voice::get_effect_state)                                     | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](Voice::set_effect_parameters_raw)                   | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](Voice::get_effect_parameters_raw)                   | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](Voice::set_filter_parameters)                           | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](Voice::get_filter_parameters)                           | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](Voice::set_output_filter_parameters)             | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](Voice::get_output_filter_parameters)             | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](Voice::set_volume)                                                 | Sets this voice’s overall volume level.
    /// | [`get_volume`](Voice::get_volume)                                                 | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](Voice::set_channel_volumes)                               | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](Voice::get_channel_volumes)                               | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](Voice::set_output_matrix)                                   | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](Voice::get_output_matrix)                                   | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct SubmixVoice<'xa2>(NonNull<IXAudio2SubmixVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
    ///
    /// ### Methods (on `SourceVoiceUntyped` itself)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`start`](SourceVoiceUntyped::start)                                              | Makes this voice start consuming and processing audio.
    /// | [`stop`](SourceVoiceUntyped::stop)                                                | Makes this voice stop consuming audio.
    /// | [`flush_source_buffers`](SourceVoiceUntyped::flush_source_buffers)                | Removes all pending audio buffers from this voice's queue.
    /// | [`discontinuity`](SourceVoiceUntyped::discontinuity)                              | Notifies the voice of an intentional break in the stream of audio buffers (e.g. the end of a sound), to prevent XAudio2 from interpreting an empty buffer queue as a glitch.
    /// | [`exit_loop`](SourceVoiceUntyped::exit_loop)                                      | Breaks out of the current loop when its end is reached.
    /// | [`get_state`](SourceVoiceUntyped::get_state)                                      | Returns the number of buffers currently queued on this voice, the pContext value associated with the currently processing buffer (if any), and other voice state information.
    /// | [`set_frequency_ratio`](SourceVoiceUntyped::set_frequency_ratio)                  | Sets this voice's frequency adjustment, i.e. its pitch.
    /// | [`get_frequency_ratio`](SourceVoiceUntyped::get_frequency_ratio)                  | Returns this voice's current frequency adjustment ratio.
    /// | [`set_source_sample_rate`](SourceVoiceUntyped::set_source_sample_rate)            | Reconfigures this voice to treat its source data as being at a different sample rate than the original one specified in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
    ///
    /// ### Methods (via `Voice` after `Deref`)
    /// | Method                                                                            | Description  |
    /// | ----------------------------------------------------------------------------------| -------------|
    /// | [`get_voice_details`](Voice::get_voice_details)                                   | Get [`VoiceDetails`] (flags, channels, sample rate)
    /// | [`set_output_voices`](Voice::set_output_voices)                                   | Set submix/mastering voices that receive this voice’s output.
    /// | [`set_effect_chain`](Voice::set_effect_chain)                                     | Replaces this voice’s current effect chain with a new one.
    /// | [`enable_effect`](Voice::enable_effect)                                           | Enables an effect in this voice’s effect chain.
    /// | [`disable_effect`](Voice::disable_effect)                                         | Disables an effect in this voice’s effect chain.
    /// | [`get_effect_state`](Voice::get_effect_state)                                     | Returns the running state of an effect.
    /// | [`set_effect_parameters_raw`](Voice::set_effect_parameters_raw)                   | Sets effect-specific parameters.  Not type checked.
    /// | [`get_effect_parameters_raw`](Voice::get_effect_parameters_raw)                   | Obtains the current effect-specific parameters.  Not type checked.
    /// | [`set_filter_parameters`](Voice::set_filter_parameters)                           | Sets this voice’s [`FilterParameters`].
    /// | [`get_filter_parameters`](Voice::get_filter_parameters)                           | Returns this voice’s current [`FilterParameters`].
    /// | [`set_output_filter_parameters`](Voice::set_output_filter_parameters)             | Sets the [`FilterParameters`] on one of this voice’s sends.
    /// | [`get_output_filter_parameters`](Voice::get_output_filter_parameters)             | Returns the [`FilterParameters`] from one of this voice’s sends.
    /// | [`set_volume`](Voice::set_volume)                                                 | Sets this voice’s overall volume level.
    /// | [`get_volume`](Voice::get_volume)                                                 | Obtains this voice’s current overall volume level.
    /// | [`set_channel_volumes`](Voice::set_channel_volumes)                               | Sets this voice’s per-channel volume levels.
    /// | [`get_channel_volumes`](Voice::get_channel_volumes)                               | Returns this voice’s current per-channel volume levels.
    /// | [`set_output_matrix`](Voice::set_output_matrix)                                   | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    /// | [`get_output_matrix`](Voice::get_output_matrix)                                   | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
    pub struct SourceVoiceUntyped<'xa2>(NonNull<IXAudio2SourceVoice>);
}

impl<'xa2> Deref    for MasteringVoice<'xa2>    { fn deref    (&    self) -> &    Self::Target { unsafe { transmute(self) } } type Target = Voice<'xa2>; }
impl<'xa2> Deref    for SubmixVoice<'xa2>       { fn deref    (&    self) -> &    Self::Target { unsafe { transmute(self) } } type Target = Voice<'xa2>; }
impl<'xa2> Deref    for SourceVoiceUntyped<'xa2>{ fn deref    (&    self) -> &    Self::Target { unsafe { transmute(self) } } type Target = Voice<'xa2>; }

impl<'xa2> DerefMut for MasteringVoice<'xa2>    { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { transmute(self) } } }
impl<'xa2> DerefMut for SubmixVoice<'xa2>       { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { transmute(self) } } }
impl<'xa2> DerefMut for SourceVoiceUntyped<'xa2>{ fn deref_mut(&mut self) -> &mut Self::Target { unsafe { transmute(self) } } }
