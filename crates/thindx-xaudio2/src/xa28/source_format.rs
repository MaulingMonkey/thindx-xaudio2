use winapi::shared::mmreg::*;

use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::*;



/// A [WAVEFORMATEX] or [WAVEFORMATEXTENSIBLE] suitable for passing to [IXAudio2::CreateSourceVoice].
///
/// XAudio2 supports the following PCM formats:
/// *   8-bit (unsigned) integer PCM
/// *   16-bit integer PCM (optimal format for XAudio2)
/// *   20-bit integer PCM (either in 24 or 32 bit containers)
/// *   24-bit integer PCM (either in 24 or 32 bit containers)
/// *   32-bit integer PCM
/// *   32-bit float PCM (preferred format after 16-bit integer)
///
/// [WAVEFORMATEX]:                 https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex
/// [WAVEFORMATEXTENSIBLE]:         https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible
/// [IXAudio2::CreateSourceVoice]:  https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice
#[derive(Clone, Copy)] pub struct SourceFormat(WAVEFORMATEX);

impl AsRef<SourceFormat> for SourceFormat { fn as_ref(&self) -> &SourceFormat { self } }

impl SourceFormat {
    /// Construct [SourceFormat] from a [WAVEFORMATEX](https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex).
    ///
    /// ### Safety
    /// The exact safety requirements are a bit murky.  However:
    /// *   `WAVEFORMATEX::cbSize`      in particular must be valid to avoid buffer overruns.
    /// *   `WAVEFORMATEX::wFormatTag`  should be valid for `self` to avoid buffer overruns / unhandled switch cases.
    /// *   Particularly large integers could easily lead to integer overflow related undefined behavior.
    /// *   Unexpected enum values could easily lead to exercising undefined behavior via unhandled switch cases.
    pub unsafe fn from_wave_format_ex(format: WAVEFORMATEX) -> Self {
        assert!(format.wFormatTag != WAVE_FORMAT_EXTENSIBLE, "use WAVEFORMATEXTENSIBLE instead for WAVE_FORMAT_EXTENSIBLE");
        assert!(format.cbSize == 0, "WAVEFORMATEX cannot store any trailing data");
        Self(format)
    }

    /// [IXAudio2::CreateSourceVoice]-friendly parameter.
    pub fn as_source_format(&self) -> *const WAVEFORMATEX { &self.0 }

    pub fn pcm_8bit_mono        (hz: u32) -> TypedSourceFormat<[u8;  1]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn pcm_8bit_stereo      (hz: u32) -> TypedSourceFormat<[u8;  2]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn pcm_16bit_mono       (hz: u32) -> TypedSourceFormat<[i16; 1]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn pcm_16bit_stereo     (hz: u32) -> TypedSourceFormat<[i16; 2]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn pcm_32bit_mono       (hz: u32) -> TypedSourceFormat<[i32; 1]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn pcm_32bit_stereo     (hz: u32) -> TypedSourceFormat<[i32; 2]> { Self::basic(WAVE_FORMAT_PCM, hz) }
    pub fn float_32bit_mono     (hz: u32) -> TypedSourceFormat<[f32; 1]> { Self::basic(WAVE_FORMAT_IEEE_FLOAT, hz) }
    pub fn float_32bit_stereo   (hz: u32) -> TypedSourceFormat<[f32; 2]> { Self::basic(WAVE_FORMAT_IEEE_FLOAT, hz) }

    fn basic<S: Sized, const C: usize>(fmt: u16, hz: u32) -> TypedSourceFormat<[S; C]> {
        let s_size = if let Ok(n) = u16::try_from(size_of::<S>()) { n } else { panic!("size_of::<Sample>() > u16::MAX") };
        unsafe{TypedSourceFormat::new(SourceFormat::from_wave_format_ex(WAVEFORMATEX{
            wFormatTag:         fmt,
            nChannels:          C.try_into().expect("too many channels"),
            nSamplesPerSec:     hz,
            nAvgBytesPerSec:    hz * (s_size as u32),
            nBlockAlign:        s_size,
            wBitsPerSample:     s_size * 8,
            cbSize:             0,
        }))}
    }
}

// TODO: impl Debug for SourceFormat



/// [SourceFormat], but with additional type information.
#[derive(Clone, Copy)] pub struct TypedSourceFormat<Sample>(SourceFormat, PhantomData<Sample>);

impl<S> AsRef<SourceFormat> for TypedSourceFormat<S> { fn as_ref(&self) -> &SourceFormat { &self.0 } }
impl<S> Deref for TypedSourceFormat<S> { fn deref(&self) -> &Self::Target { &self.0 } type Target = SourceFormat; }

impl<S> TypedSourceFormat<S> {
    /// Construct a [TypedSourceFormat] from a [SourceFormat]
    ///
    /// ### Safety
    /// *   `format` should match the type `S`.
    pub unsafe fn new(format: SourceFormat) -> Self {
        assert!(format.0.wBitsPerSample as usize == 8 * size_of::<S>());
        Self(format, PhantomData)
    }
}
