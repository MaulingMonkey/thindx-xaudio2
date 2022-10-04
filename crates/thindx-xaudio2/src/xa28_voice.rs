use super::*;
use super::xaudio2::VoiceCallback;

use core::marker::PhantomData;
use core::ops::*;
use core::ptr::NonNull;



macro_rules! voices {
    ($(
        pub struct $voice:ident $(< $generic:ident : $constraint:ident >)? ( NonNull< $ivoice:ty > );
    )*) => {$(
        pub struct $voice $(< $generic : $constraint >)? ( NonNull< $ivoice > $(, PhantomData<$generic>)? );

        impl $(< $generic : $constraint >)? $voice $(< $generic >)? {
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
            pub unsafe fn from_raw_opt(raw: *const $ivoice) -> Option<Self> { Some(Self(NonNull::new(raw as *mut _)? $(, PhantomData::<$generic>)?)) }

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

        impl $(< $generic : $constraint >)? Deref       for $voice $(< $generic >)? { fn deref    (&    self) -> &    Self::Target { unsafe { self.0.as_ref() } } type Target = $ivoice; }
        impl $(< $generic : $constraint >)? DerefMut    for $voice $(< $generic >)? { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } } }
        impl $(< $generic : $constraint >)? Drop        for $voice $(< $generic >)? { fn drop(&mut self) { unsafe { (*self.0.as_ptr()).DestroyVoice() } } }
    )*};
}

voices! {
    pub struct Voice                            (NonNull<IXAudio2Voice>);
    pub struct MasteringVoice                   (NonNull<IXAudio2MasteringVoice>);
    pub struct SubmixVoice                      (NonNull<IXAudio2SubmixVoice>);
    pub struct SourceVoiceUntyped               (NonNull<IXAudio2SourceVoice>);
    pub struct SourceVoice<VC: VoiceCallback>   (NonNull<IXAudio2SourceVoiceTyped<VC>>);
}
