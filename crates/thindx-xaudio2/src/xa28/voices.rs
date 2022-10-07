use super::*;

use core::marker::PhantomData;
use core::ops::*;
use core::ptr::NonNull;



macro_rules! voices {
    ($(
        $(#[doc = $voice_doc:literal])*
        pub struct $voice:ident <'xa2> ( NonNull< $ivoice:ty > );
    )*) => {$(
        $(#[doc = $voice_doc])*
        pub struct $voice <'xa2> {
            factory:    PhantomData<&'xa2 IXAudio2>,
            voice:      NonNull< $ivoice >,
        }

        impl<'xa2> $voice <'xa2> {
            /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
            /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
            ///
            /// (Dropping the voice also implicitly stops/removes it.)
            pub fn destroy_voice(self) {}

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
    pub struct Voice<'xa2>(NonNull<IXAudio2Voice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\] [IXAudio2MasteringVoice]
    pub struct MasteringVoice<'xa2>(NonNull<IXAudio2MasteringVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\] [IXAudio2SubmixVoice]
    pub struct SubmixVoice<'xa2>(NonNull<IXAudio2SubmixVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
    pub struct SourceVoiceUntyped<'xa2>(NonNull<IXAudio2SourceVoice>);
}
