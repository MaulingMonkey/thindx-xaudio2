//! Generate an [A440](https://en.wikipedia.org/wiki/A440_(pitch_standard)) tone for a short duration.

use thindx_xaudio2::xaudio2_9::*;

use winapi::shared::mmreg::{WAVEFORMATEX, WAVE_FORMAT_IEEE_FLOAT};
use core::f32::consts::PI;
use core::mem::size_of;



fn main() {
    mcom::init::mta().expect("mcom::init::mta");
    let xaudio2 = xaudio2::create(None, xaudio2::USE_DEFAULT_PROCESSOR).expect("xaudio2::create");
    xaudio2.register_for_callbacks_leak(EngineCallback).expect("register_for_callbacks_leak");

    let _master = xaudio2.create_mastering_voice(
        xaudio2::DEFAULT_CHANNELS, xaudio2::DEFAULT_SAMPLERATE,
        0, (), None, xaudio2::DEFAULT_AUDIO_CATEGORY
    ).expect("create_mastering_voice");

    //let hz = master.get_voice_details().InputSampleRate;
    let hz = 44100;
    let samples = hz / 440; // https://en.wikipedia.org/wiki/A440_(pitch_standard)

    // XXX: yes, definitely replace WAVEFORMATEX (see also WAVEFORMATEXTENSIBLE)
    let waveformatex = WAVEFORMATEX {
        wFormatTag:         WAVE_FORMAT_IEEE_FLOAT,
        nChannels:          1,  // mono
        nSamplesPerSec:     hz, // match output
        nAvgBytesPerSec:    (size_of::<f32>() as u32) * hz,
        nBlockAlign:        1 * 32 / 8, // channels * bits / bits per byte
        wBitsPerSample:     32,
        cbSize:             size_of::<WAVEFORMATEX>() as _,
    };

    let callback = xaudio2::VoiceCallbackWrapper::new(VoiceCallback);
    // TODO: XXX: soundness: ensure `callback` outlives `a440` via type system.
    let a440 = xaudio2.create_source_voice_typed_callback(&waveformatex, 0, xaudio2::DEFAULT_FREQ_RATIO, &callback, None /* defaults to master */, None).expect("a440");

    let samples = (0 .. samples).map(|s| f32::sin((s as f32) * 2.0 * PI / (samples as f32))).collect::<Vec<_>>();
    let samples = xaudio2::Buffer {
        AudioData:  bytemuck::cast_slice(&samples[..]),
        LoopCount:  xaudio2::MAX_LOOP_COUNT, // 254
        Flags:      xaudio2::END_OF_STREAM,
        ..Default::default()
    };
    a440.set_volume(0.2, xaudio2::COMMIT_NOW).unwrap(); // 20% pure tone is plenty loud IMO
    // TODO: XXX: soundness: keep `samples` alive until processed.
    a440.submit_source_buffer(samples, None).expect("a440.submit_source_buffer(samples, None)");
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
