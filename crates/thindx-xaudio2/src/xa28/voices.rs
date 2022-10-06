use super::*;

use core::ops::*;
use core::ptr::NonNull;



macro_rules! voices {
    ($(
        $(#[doc = $voice_doc:literal])*
        pub struct $voice:ident ( NonNull< $ivoice:ty > );
    )*) => {$(
        $(#[doc = $voice_doc])*
        pub struct $voice ( NonNull< $ivoice > );

        impl $voice {
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
            pub unsafe fn from_raw_opt(raw: *const $ivoice) -> Option<Self> { Some(Self(NonNull::new(raw as *mut _)?)) }

            /// Create a voice wrapper from a raw pointer.
            ///
            /// If `raw` is null, will [panic!].
            ///
            /// ### Safety
            /// *   `raw` must be a valid interface pointer if not null.
            /// *   `Self` takes ownership of `raw`.
            #[track_caller] pub unsafe fn from_raw(raw: *const $ivoice) -> Self { unsafe { Self::from_raw_opt(raw) }.unwrap() }

            /// Convert `self` back into a raw pointer, relinquishing ownership.
            pub fn into_raw(self) -> *const $ivoice {
                let ptr = self.0.as_ptr();
                core::mem::forget(self);
                ptr
            }

            pub fn as_raw(&self) -> *const $ivoice { self.0.as_ptr() }
        }

        impl Deref      for $voice { fn deref    (&    self) -> &    Self::Target { unsafe { self.0.as_ref() } } type Target = $ivoice; }
        impl DerefMut   for $voice { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } } }
        impl Drop       for $voice { fn drop(&mut self) { unsafe { (*self.0.as_ptr()).DestroyVoice() } } }
    )*};
}

voices! {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2voice)\] [IXAudio2Voice]
    pub struct Voice(NonNull<IXAudio2Voice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)\] [IXAudio2MasteringVoice]
    pub struct MasteringVoice(NonNull<IXAudio2MasteringVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2submixvoice)\] [IXAudio2SubmixVoice]
    pub struct SubmixVoice(NonNull<IXAudio2SubmixVoice>);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
    pub struct SourceVoiceUntyped(NonNull<IXAudio2SourceVoice>);
}
