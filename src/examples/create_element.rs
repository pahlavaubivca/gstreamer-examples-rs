use gstreamer::ffi::gst_element_factory_make;
use gstreamer::glib::property::PropertySet;
use gstreamer::prelude::{ElementExt, GObjectExtManualGst, GstBinExtManual, GstObjectExt, ObjectExt};
use gstreamer_app::gst;

pub fn create_element_example() {
    /// videotestsrc its element for producing test video data in a variety of formats. Its not a real video source, but rather generates a test pattern.
    let source = gst::ElementFactory::make("videotestsrc").build().unwrap();
    let blender = gst::ElementFactory::make("vertigotv").build().unwrap();
    let videoconvert = gst::ElementFactory::make("videoconvert").build().unwrap();
    let sink = gst::ElementFactory::make("autovideosink").build().unwrap();
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    let bin_many = gst::Bin::add_many(
        (&pipeline).as_ref(),
        [&source, &blender, &videoconvert,&sink],
    );
    if let Err(err) = bin_many {
        eprintln!("Failed to add elements to bin: {}", err);
        panic!("Failed to add elements to bin");
    }

    if let Err(err) = gst::Element::link_many(&[&source, &blender,&videoconvert, &sink]) {
        eprintln!("Failed to link elements: {}", err);
        pipeline.set_state(gst::State::Null).unwrap();
        panic!("Failed to link elements");
    }

    // let prop_list = source.list_properties();
    // for prop in prop_list {
    //     println!("Property: {:?}", prop.name());
    //     println!("Value type: {:?}", prop.value_type());
    //     
    // }
    // 
    // let prop_sink_list = sink.list_properties();
    // for prop in prop_sink_list {
    //     println!("Property: {:?}", prop.name());
    //     println!("Value type: {:?}", prop.value_type());
    //     
    // }


    source.set_property_from_str("pattern", "smpte");
    // source.set_property_from_str("pattern", "snow");
    // GstVideoTestSrcPattern
    // 
    // source.set_property("pattern", VideoTestSrcPattern"smpte");

    // let value_for_source_pattern = GstVideoTestSrcPattern
    // source.set_property("pattern", 0);

    let play_state_result = pipeline.set_state(gst::State::Playing);
    if let Err(err) = play_state_result {
        eprintln!("Failed to set pipeline to playing: {}", err);
        pipeline.set_state(gst::State::Null).unwrap();
        panic!("Failed to set pipeline to playing");
    }
    let bus = pipeline.bus().unwrap();
    let message = bus.timed_pop_filtered(gst::ClockTime::NONE, &[
        gst::MessageType::Eos,
        gst::MessageType::Error,
    ]);

    if let Some(message) = message {
        match message.view() {
            gst::MessageView::Eos(..) => {
                println!("End of stream");
            }
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from {}: {} ({:?})",
                    message.src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| gstreamer::glib::GString::from("None")),
                    err.error(),
                    err.debug()
                );
            }
            err =>
                eprintln!("Unexpected message: {:?}", err),
        }
    }
    pipeline.set_state(gst::State::Null).unwrap();
}