use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn inference() -> String {
    // let path_to_model = std::env::args().nth(1).unwrap();
    const PATH_TO_MODEL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../models/ggml-small.bin");
    let wav_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../recorded.wav");
    let language = "en";

    let samples: Vec<i16> = hound::WavReader::open(wav_path)
        .unwrap()
        .into_samples::<i16>()
        .map(|x| x.unwrap())
        .collect();

    // load a context and model
    let ctx = WhisperContext::new_with_params(&PATH_TO_MODEL, WhisperContextParameters::default())
        .expect("failed to load model");

    let mut state = ctx.create_state().expect("failed to create state");

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // and set the language to translate to to english
    // params.set_language(Some(&language));
    // params.set_language(Some(&language, )false);

    // params.set_detect_language(true);
    // we also explicitly disable anything that prints to stdout
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // we must convert to 16KHz mono f32 samples for the model
    // some utilities exist for this
    // note that you don't need to use these, you can do it yourself or any other way you want
    // these are just provided for convenience
    // SIMD variants of these functions are also available, but only on nightly Rust: see the docs
    let mut inter_samples = vec![Default::default(); samples.len()];

    whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
        .expect("failed to convert audio data");
    let samples = whisper_rs::convert_stereo_to_mono_audio(&inter_samples)
        .expect("failed to convert audio data");

    // let samples = inter_samples;
    // now we can run the model
    // note the key we use here is the one we created above
    state
        .full(params, &samples[..])
        .expect("failed to run model");

    let mut result = String::new(); // создаём строку для накопления результатов

    // fetch the results [Song ends] [Singing]
    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("failed to get segment start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("failed to get segment end timestamp");
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        // Добавляем данные сегмента в строку
        result.push_str(segment.as_str());
    }

    result
}
