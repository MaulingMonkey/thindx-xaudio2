use super::xaudio2;
use super::xaudio2::sys::*;



impl IXAudio2ExtensionExt for IXAudio2Extension { fn _as_ixaudio2(&self) -> &IXAudio2Extension { self } }

/// [IXAudio2Extension] extension methods
/// ### Example
/// ```
/// use thindx_xaudio2::xaudio2_9::*;
/// let ci = std::env::var_os("CI").is_some();
///
/// let xaudio2 = unsafe { xaudio2::create(None, None) }.expect("xaudio2::create");
/// if let Some(ext) = xaudio2.try_cast::<IXAudio2Extension>() {
///     # if ci {
///     #     let _ = ext.get_processing_quantum();
///     #     let _ = ext.get_processor();
///     # } else {
///     // Not actually guaranteed
///     assert_eq!(ext.get_processing_quantum(), (1, 100));
///     assert_eq!(ext.get_processor(), xaudio2::Processor1);
///     # }
/// } else {
///     assert!(ci, "Expected cast to IXAudio2Extension to work on most non-CI boxes");
/// }
/// ```
pub trait IXAudio2ExtensionExt {
    #[doc(hidden)] fn _as_ixaudio2(&self) -> &IXAudio2Extension;

    /// \[<strike>microsoft.com</strike>\]
    /// Returns the processing quantum as a ratio of seconds (e.g. `(1, 100)` = `10ms`)
    fn get_processing_quantum(&self) -> (u32, u32) {
        let mut n = 0;
        let mut d = 0;
        unsafe { self._as_ixaudio2().GetProcessingQuantum(&mut n, &mut d) };
        (n, d)
    }

    /// \[<strike>microsoft.com</strike>\]
    /// Returns the [xaudio2::Processor] used by XAudio2.
    ///
    /// This will generally be the same as the processor passed to [xaudio2::create],
    /// unless you specified [xaudio2::USE_DEFAULT_PROCESSOR],
    /// in which case the actual processor(s) chosen by XAudio2 will be returned instead.
    fn get_processor(&self) -> xaudio2::Processor {
        let mut p = xaudio2::Processor::default();
        unsafe { self._as_ixaudio2().GetProcessor(&mut p) };
        p
    }
}
