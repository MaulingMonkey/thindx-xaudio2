//! Generate an [A440](https://en.wikipedia.org/wiki/A440_(pitch_standard)) tone for a short duration.

use thindx_xaudio2::xaudio2_9::*;
use core::f32::consts::PI;



fn main() {
    mcom::init::mta().expect("mcom::init::mta");
    let xaudio2 = xaudio2::create(None, xaudio2::USE_DEFAULT_PROCESSOR);
    let xaudio2 = xaudio2.or_else(|_| xaudio2::create(None, xaudio2::DEFAULT_PROCESSOR));
    let xaudio2 = xaudio2.expect("xaudio2::create");
    xaudio2.register_for_callbacks_leak(EngineCallback).expect("register_for_callbacks_leak");

    let _master = xaudio2.create_mastering_voice(
        xaudio2::DEFAULT_CHANNELS, xaudio2::DEFAULT_SAMPLERATE,
        0, (), None, xaudio2::DEFAULT_AUDIO_CATEGORY
    ).expect("create_mastering_voice");

    let hz = 44100;
    let samples = hz / 440; // https://en.wikipedia.org/wiki/A440_(pitch_standard)

    let format = xaudio2::TypedSourceFormat::pcm(hz);
    let callback = xaudio2::VoiceCallbackWrapper::new(VoiceCallback);
    let a440 = xaudio2.create_source_voice_typed_callback(&format, 0, xaudio2::DEFAULT_FREQ_RATIO, &callback, None /* defaults to master */, None).expect("a440");

    let samples = (0 .. samples).map(|s| {
        let s = f32::sin((s as f32) * 2.0 * PI / (samples as f32));
        [s, s]
    }).collect::<Vec<_>>();
    a440.set_volume(0.2, xaudio2::COMMIT_NOW).unwrap(); // 20% pure tone is plenty loud IMO
    a440.submit_source_buffer(
        xaudio2::END_OF_STREAM,
        samples, ..,
        .., xaudio2::MAX_LOOP_COUNT,
        (),
    ).expect("a440.submit_source_buffer");
    a440.start(0, xaudio2::COMMIT_NOW).expect("a440.start()");
    std::thread::sleep(std::time::Duration::from_secs(10));
    panic!("main() end before stream end?");
}

struct EngineCallback;
impl xaudio2::EngineCallback for EngineCallback {
    // As expected, these callbacks all fire on an XAudio2 thread:
    fn on_processing_pass_start(&self) { eprintln!("on_processing_pass_start") }
    fn on_processing_pass_end(&self) { eprintln!("on_processing_pass_end") }
    fn on_critical_error(&self, error: winresult::HResult) { panic!("{error:?}") }
}

struct VoiceCallback;
impl xaudio2::VoiceCallback for VoiceCallback {
    type BufferContext = ();
    // As expected, these callbacks all fire on an XAudio2 thread:
    fn on_voice_processing_pass_start(&self, bytes_required: u32) { eprintln!("on_voice_processing_pass_start({bytes_required})") }
    fn on_voice_processing_pass_end(&self) { eprintln!("on_voice_processing_pass_end()") }
    fn on_loop_end(&self, buffer_context: &Self::BufferContext) { eprintln!("on_loop_end({buffer_context:?})") }
    fn on_buffer_start(&self, buffer_context: &Self::BufferContext) { eprintln!("on_buffer_start({buffer_context:?})") }
    fn on_buffer_end(&self, buffer_context: Self::BufferContext) { eprintln!("on_buffer_end({buffer_context:?})") }
    fn on_stream_end(&self) { eprintln!("on_stream_end"); std::process::exit(0); }
    fn on_voice_error(&self, _buffer_context: &Self::BufferContext, error: winresult::HResult) { panic!("{error:?}"); }
}
