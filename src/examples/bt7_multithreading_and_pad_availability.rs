use gstreamer::{Element, ElementFactory, Pipeline};
use gstreamer::prelude::{ElementExt, ElementExtManual, GstBinExtManual, ObjectExt};

pub fn multithreading_and_pad_availability() {
    let audio_source = ElementFactory::make_with_name(
        "audiotestsrc",
        Some("audio_source"),
    ).unwrap();
    let tee = ElementFactory::make_with_name(
        "tee",
        Some("tee"),
    ).unwrap();
    let audio_queue = ElementFactory::make_with_name(
        "queue",
        Some("audio_queue"),
    ).unwrap();
    let audio_convert = ElementFactory::make_with_name(
        "audioconvert",
        Some("audio_convert"),
    ).unwrap();
    let audio_resample = ElementFactory::make_with_name(
        "audioresample",
        Some("audio_resample"),
    ).unwrap();
    let audio_sink = ElementFactory::make_with_name(
        "autoaudiosink",
        Some("audio_sink"),
    ).unwrap();
    let video_queue = ElementFactory::make_with_name(
        "queue",
        Some("video_queue"),
    ).unwrap();
    let visual = ElementFactory::make_with_name(
        "wavescope",
        Some("visual"),
    ).unwrap();
    let video_convert = ElementFactory::make_with_name(
        "videoconvert",
        Some("video_convert"),
    ).unwrap();
    let video_sink = ElementFactory::make_with_name(
        "autovideosink",
        Some("video_sink"),
    ).unwrap();

    let pipeline = Pipeline::with_name("test-pipeline");

    audio_source.set_property("freq", &215.0);
    visual.set_property("shader", &0);
    visual.set_property("style", &1);

    pipeline.add_many(&[
        &audio_source,
        &tee,
        &audio_queue,
        &audio_convert,
        &audio_resample,
        &audio_sink,
        &video_queue,
        &visual,
        &video_convert,
        &video_sink,
    ]).unwrap();

    if let Err(err) = Element::link_many(&[&audio_source, &tee]) {
        eprintln!("Failed to link audio_source and tee: {}", err);
        pipeline.set_state(gstreamer::State::Null).unwrap();
        panic!("Failed to link audio_source and tee");
    }

    if let Err(err) = Element::link_many(&[&audio_queue, &audio_convert, &audio_resample, &audio_sink]) {
        eprintln!("Failed to link audio_queue,audio_convert,audio_resample,audio_sink: {}", err);
        pipeline.set_state(gstreamer::State::Null).unwrap();
        panic!("Failed to link audio_queue,audio_convert,audio_resample,audio_sink");
    }

    if let Err(err) = Element::link_many(&[&video_queue, &visual, &video_convert, &video_sink]) {
        eprintln!("Failed to link video_queue,visual,video_convert,video_sink: {}", err);
        pipeline.set_state(gstreamer::State::Null).unwrap();
        panic!("Failed to link video_queue,visual,video_convert,video_sink");
    }
    
    let tee_audio_pad = tee.request_pad_simple("src_%u").unwrap();
    let queue_audio_pad = audio_queue.static_pad("sink").unwrap();
    let tee_video_pad = tee.request_pad_simple("src_%u").unwrap();
    let queue_video_pad = video_queue.static_pad("sink").unwrap();
    if Err(err) = tee_audio_pad.link(&queue_audio_pad) {
        eprintln!("Failed to link tee_audio_pad and queue_audio_pad: {}", err);
        pipeline.set_state(gstreamer::State::Null).unwrap();
        panic!("Failed to link tee_audio_pad and queue_audio_pad");
    }
}