mod examples;
use gst::prelude::*;
use gstreamer::glib::translate::FromGlibPtrNone;
use gstreamer_app::{gst};
use crate::examples::bt3_dynamic_pipelines::BT3DynamicPipelines;
use crate::examples::bt7_multithreading_and_pad_availability::BT7MultithreadingAndPadAvailability;

fn main() {
    gst::init().unwrap();
    // play_video_by_http();
    // create_element_example();
    BT7MultithreadingAndPadAvailability::new().run();
    
}
