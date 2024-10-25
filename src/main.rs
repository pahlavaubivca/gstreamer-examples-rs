mod examples;

use anyhow::Error;
use gst::prelude::*;
use std::sync::{Arc, Mutex};
use gstreamer::ffi::{gst_init, gst_parse_launch};
use gstreamer::glib;
use gstreamer::glib::GString;
use gstreamer::glib::translate::FromGlibPtrNone;
use gstreamer_app::{gst, AppSink};
use crate::examples::create_element::create_element_example;
use crate::examples::play_video_by_http::play_video_by_http;

fn main() {
    gst::init().unwrap();
    // play_video_by_http();
    create_element_example();

    
}
