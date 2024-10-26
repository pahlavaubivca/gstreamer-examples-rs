use std::sync::Arc;
use gstreamer::{Element, ElementFactory, Pad, Pipeline};
use gstreamer::ffi::GstPad;
use gstreamer::glib::gobject_ffi::{g_signal_connect_closure, g_signal_connect_data, GCallback};
use gstreamer::glib::OptionArg::Callback;
use gstreamer::glib::RustClosure;
use gstreamer::glib::subclass::Signal;
use gstreamer::prelude::{ControlBindingExt, ElementExt, GstBinExtManual, GstObjectExt, ObjectExt, PadExt};
use gstreamer_app::gst;

pub struct DynamicPipelines {
    pipeline: Pipeline,
    source: Element,
    converter: Arc<Element>,
    resample: Element,
    sink: Element,
}

impl DynamicPipelines {
    pub fn new() -> Self {
        let pipeline = gst::Pipeline::with_name("my_dynamic_pipeline");
        let source = ElementFactory::make("uridecodebin").build().unwrap();
        let converter = Arc::new(ElementFactory::make("audioconvert").build().unwrap());
        let resample = ElementFactory::make("audioresample").build().unwrap();
        let sink = ElementFactory::make("autoaudiosink").build().unwrap();

        pipeline.add_many(&[
            &source,
            &converter,
            &resample,
            &sink
        ]).unwrap();

        if let Err(err) = Element::link_many(&[
            &converter,
            &resample,
            &sink
        ]) {
            eprintln!("Failed to link elements: {}", err);
            pipeline.set_state(gst::State::Null).unwrap();
            panic!("Failed to link elements");
        }

        Self {
            pipeline,
            source,
            converter,
            resample,
            sink,
        }
    }
    pub fn run(&mut self) {
        println!("Running dynamic pipeline");
        self.source.set_property("uri", "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm");
        // g_signal_connect_data(&self.source, "pad-added", Some(pad_added_handler), Some(self));
        // self.source.connect("pad-added", false, Callback( pad_added_handler), Some(self)).unwrap();
        // Signal::builder("pad-added").
        let converter_week = Arc::downgrade(&self.converter);
        self.source.connect_pad_added(move |source, new_pad| {
            let converter = match converter_week.upgrade() {
                Some(converter) => converter,
                None => return,
            };
            pad_added_handler(source, new_pad, &converter);
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
fn pad_added_handler(source: &Element, new_pad: &Pad, converter: &Element) {
    let sink_pad = converter.static_pad("sink").unwrap();

    println!("Received new pad '{}' from '{}'", new_pad.name(), source.name());

    if sink_pad.is_linked() {
        println!("We are already linked. Ignoring.");
        return;
    }

    let new_pad_caps = new_pad.current_caps().unwrap();
    let new_pad_struct = new_pad_caps.structure(0).unwrap();
    let new_pad_type = new_pad_struct.name();

    if !new_pad_type.starts_with("audio/x-raw") {
        println!("It has type {} which is not raw audio. Ignoring.", new_pad_type);
        return;
    }

    if let Err(err) = Pad::link(new_pad, &sink_pad) {
        eprintln!("Failed to link pads: {}", err);
    }
}