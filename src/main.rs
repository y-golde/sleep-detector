use anyhow::Result; 
use opencv::{
    prelude::*,
    videoio,
    highgui,
    objdetect
};

mod eye_detection;
use eye_detection::{detect_eyes, draw_eye};

mod sleep_detection;
use sleep_detection::{detect_sleep};

const CASCADE_XML_FILE: &str = "haarcascade_eye_tree_eyeglasses.xml";
const QUITTING_KEY_CODE: i32 = 113; // press q to quit

fn run() -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let mut classifier = objdetect::CascadeClassifier::new(CASCADE_XML_FILE).unwrap();

    let mut blink_counter: usize = 0;

    loop {
        cam.read(&mut frame)?;

        let eyes = detect_eyes(&mut classifier, &frame).unwrap();
        detect_sleep(&mut frame, &eyes, &mut blink_counter).unwrap();
        
        for eye in eyes {
            draw_eye(&mut frame, eye).unwrap();
        } 


        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == QUITTING_KEY_CODE { 
            break;
        }
    }
    Ok(())
}

fn main() { 
    run().unwrap();
}
