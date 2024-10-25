use gstreamer::glib::GString;
use gstreamer::prelude::{ElementExt, ElementExtManual, GstObjectExt};
use gstreamer::SeekFlags;
use gstreamer_app::gst;

///
/// Play a video by http
/// example [from](https://gstreamer.freedesktop.org/documentation/tutorials/basic/hello-world.html?gi-language=c) 
pub fn play_video_by_http() {
    // let gpl = gstreamer::parse::launch(
    //     "playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm"
    // ).unwrap();
    let gpl = gstreamer::parse::launch(
        "playbin uri=file:///var/tmp/video.mp4"
    ).unwrap();
    
    gpl.set_state(gstreamer::State::Playing).unwrap();
    // gstreamer::Element::set_state(&gpl, gstreamer::State::Playing).unwrap();
    
    // gstreamer::Element::seek_simple(
    //     &gpl,
    //     SeekFlags::FLUSH,
    //     gstreamer::ClockTime::from_seconds(1)
    // ).unwrap();
    // gpl.seek_simple(
    //     SeekFlags::FLUSH,
    //     gstreamer::ClockTime::from_seconds(1)
    // ).unwrap();
    let bus = gpl.bus().unwrap();
    let msg = bus.timed_pop_filtered(gst::ClockTime::NONE, &[
        gst::MessageType::Eos,
        gst::MessageType::Error,
    ]);

    
    
    if let Some(msg) = msg {
        match msg.view() {
            gst::MessageView::Eos(..) => {
                println!("End of stream");
            }
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from {}: {} ({:?})",
                    msg.src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| GString::from("None")),
                    err.error(),
                    err.debug()
                );
            }
            _ => (),
        }
    }
    gpl.set_state(gst::State::Null).unwrap();
}