mod examples;

use anyhow::Error;
use gst::prelude::*;
use std::sync::{Arc, Mutex};
use gstreamer::ffi::{gst_init, gst_parse_launch};
use gstreamer::glib;
use gstreamer::glib::GString;
use gstreamer::glib::translate::FromGlibPtrNone;
use gstreamer_app::{gst, AppSink};
use crate::examples::play_video_by_http::play_video_by_http;

fn main() {
    gst::init().unwrap();
    play_video_by_http();
    
    // Create the elements
    // let source = gst::ElementFactory::make("oct640usrc").build().unwrap();
    // let convert = gst::ElementFactory::make("videoconvert").build().unwrap();
    // let sink = gst::ElementFactory::make("appsink").build().unwrap();
    //
    // // Set the source properties
    // source.set_property("serial", &"23010000");
    //
    // // Configure the appsink
    // let appsink = sink
    //     .dynamic_cast::<AppSink>()
    //     .expect("Sink element is expected to be an appsink!");
    // appsink.set_property("emit-signals", &true);
    // appsink.set_property("max-buffers", &1u32);
    // appsink.set_property("drop", &true);
    //
    // // Create the pipeline and add elements
    // let pipeline = gst::Pipeline::new();
    // gst_parse_launch()
    // pipeline.add_many(&[&source, &convert, (&appsink).as_ref()]);
    // gst::Element::link_many(&[&source, &convert, (&appsink).as_ref()]);
    //
    // // Set up the appsink signal handler
    // // let sample_receiver = Arc::new(Mutex::new(None));
    // // let sample_receiver_clone = sample_receiver.clone();
    //
    // appsink.connect("new-sample", false, move |args| {
    //     let appsink = args[0]
    //         .get::<AppSink>()
    //         .expect("Failed to get AppSink from args");
    //     let sample = appsink.pull_sample().expect("Failed to pull sample");
    //
    //     // Access the buffer and process data here
    //     let buffer = sample.buffer().expect("Failed to get buffer from sample");
    //
    //     // Map buffer readable
    //     let map = buffer.map_readable().expect("Failed to map buffer readable");
    //
    //     // Process the data (for example, print the size of the buffer)
    //     println!("Received buffer of size {}", map.size());
    //
    //     // If needed, you can pass the data to another thread or process it here
    //
    //     // Return GST_FLOW_OK to indicate that the data was handled successfully
    //     Some(glib::value::Value::from(gst::FlowReturn::Ok))
    // });
    //
    // // Start playing the pipeline
    // pipeline.set_state(gst::State::Playing).unwrap();
    //
    // // Wait until error or EOS (End of Stream)
    // let bus = pipeline.bus().unwrap();
    // for msg in bus.iter_timed(gst::ClockTime::NONE) {
    //     match msg.view() {
    //         gst::MessageView::Eos(..) => {
    //             println!("End of Stream");
    //             break;
    //         }
    //         gst::MessageView::Error(err) => {
    //             eprintln!(
    //                 "Error from {}: {} ({:?})",
    //                 msg.src()
    //                     .map(|s| s.path_string())
    //                     .unwrap_or_else(|| GString::from(String::from("None"))),
    //                 err.error(),
    //                 err.debug()
    //             );
    //             break;
    //         }
    //         _ => (),
    //     }
    // }

    // Clean up

}
