use std::sync::Arc;
use gstreamer::{Element, ElementFactory, Pad, Pipeline};
use gstreamer::prelude::{ ElementExt, GstBinExtManual, GstObjectExt, ObjectExt, PadExt};
use gstreamer_app::gst;

pub struct DynamicPipelines {
    pipeline: Pipeline,
    source: Element,
    video_converter: Arc<Element>,
    audio_converter: Arc<Element>,
    audio_resample: Element,
    video_sink: Element,
    audio_sink: Element,
}

impl DynamicPipelines {
    pub fn new() -> Self {
        let pipeline = gst::Pipeline::with_name("my_dynamic_pipeline");
        let source = ElementFactory::make("uridecodebin").build().unwrap();
        let audio_converter = Arc::new(ElementFactory::make("audioconvert").build().unwrap());
        let video_converter = Arc::new(ElementFactory::make("videoconvert").build().unwrap());
        let audio_resample = ElementFactory::make("audioresample").build().unwrap();
        let video_sink = ElementFactory::make("autovideosink").build().unwrap();
        let audio_sink = ElementFactory::make("autoaudiosink").build().unwrap();

        pipeline.add_many(&[
            &source,
            &audio_converter,
            &audio_resample,
            &audio_sink,
            &video_converter,
            &video_sink,
        ]).unwrap();

        if let Err(err) = Element::link_many(&[
            &audio_converter,
            &audio_resample,
            &audio_sink
        ]) {
            eprintln!("Failed to link elements: {}", err);
            pipeline.set_state(gst::State::Null).unwrap();
            panic!("Failed to link elements");
        }
        
        if let Err(err) = Element::link_many(&[
            &video_converter,
            &video_sink
        ]) {
            eprintln!("Failed to link elements: {}", err);
            pipeline.set_state(gst::State::Null).unwrap();
            panic!("Failed to link elements");
        }

        Self {
            pipeline,
            source,
            audio_converter,
            audio_resample,
            audio_sink,
            video_converter,
            video_sink,
        }
    }
    pub fn run(&mut self) {
        println!("Running dynamic pipeline");
        self.source.set_property("uri", "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm");
        let audio_converter_week = Arc::downgrade(&self.audio_converter);
        let video_converter_week = Arc::downgrade(&self.video_converter);
        self.source.connect_pad_added(move |source, new_pad| {
            let audio_converter = match audio_converter_week.upgrade() {
                Some(converter) => converter,
                None => return,
            };
            let video_converter = match video_converter_week.upgrade() {
                Some(converter) => converter,
                None => return,
            };
            pad_added_handler(source, new_pad, &audio_converter, &video_converter);
        });

        self.pipeline.set_state(gst::State::Playing).unwrap();
        let bus = self.pipeline.bus().unwrap();
        loop {
            let message = bus.timed_pop_filtered(gst::ClockTime::NONE, &[
                gst::MessageType::Eos,
                gst::MessageType::Error,
            ]).unwrap();

            match message.view() {
                gst::MessageView::Eos(..) => {
                    println!("End-Of-Stream reached.");
                }
                gst::MessageView::Error(err) => {
                    eprintln!("Error from {:?}: {}", err.src().map(|s| s.path_string()), err.error());
                }
                gst::MessageView::StateChanged(state_changed) => {
                    println!("State changed from {:?} to {:?}", state_changed.old(), state_changed.current());
                    if state_changed.src() == Some((&self.pipeline).as_ref()) {
                        println!("Pipeline state changed from {:?} to {:?}", state_changed.old(), state_changed.current());
                    }
                }
                _ => {
                    eprintln!("Unexpected message: {:?}", message);
                }
            }
        }

        self.pipeline.set_state(gst::State::Null).unwrap();
    }
}

// fn pad_added_handler(source: &Element, new_pad: &Pad, data: &DynamicPipelines) {
fn pad_added_handler(source: &Element, new_pad: &Pad, audio_converter: &Element, video_converter: &Element) {
    let audio_sink_pad = audio_converter.static_pad("sink").unwrap();
    let video_sink_pad = video_converter.static_pad("sink").unwrap();

    println!("Received new pad '{}' from '{}'", new_pad.name(), source.name());

    if audio_sink_pad.is_linked() {
        println!("We are already linked. Ignoring.");
        return;
    }

    let new_pad_caps = new_pad.current_caps().unwrap();
    let new_pad_struct = new_pad_caps.structure(0).unwrap();
    let new_pad_type = new_pad_struct.name();

    if !new_pad_type.starts_with("audio/x-raw") {
        if let Err(err) = Pad::link(new_pad, &video_sink_pad) {
            eprintln!("Failed to link video pads: {}", err);
        }
        // println!("It has type {} which is not raw audio. Ignoring.", new_pad_type);
        return;
    }

    if let Err(err) = Pad::link(new_pad, &audio_sink_pad) {
        eprintln!("Failed to link pads: {}", err);
    }
}