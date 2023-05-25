use anyhow::Result; 
use opencv::core::{Point, Scalar};
use opencv::{
    prelude::*,
    imgproc,
    types as opencvTypes
};
use enigo::{Enigo, KeyboardControllable};

const SLEEP_STREAK_THRESH: usize = 100; // how many times did we not detect eyes before it's considered sleeping
const SLEEP_EYE_NUM_THRESH: usize = 0; // if 0 eyes are detected - you're sleeping

pub fn detect_sleep(frame: &mut Mat, eyes: &opencvTypes::VectorOfRect, blink_streak: &mut usize) -> Result<()> {

    let is_blinking = !(eyes.len() > SLEEP_EYE_NUM_THRESH);

    draw_sleep_counter(frame, is_blinking, blink_streak).unwrap();
    
    let is_sleeping = *blink_streak == SLEEP_STREAK_THRESH;

    if is_sleeping {
        on_sleep_detected().unwrap();
    }
    
    Ok(())
}

const SUCCESS_COLOR: Scalar =  Scalar::new(0f64, 255f64, 0f64, -1f64); // it's BGR - really weird
const WARN_COLOR: Scalar = Scalar::new(0f64, 225f64, 225f64, -1f64);

fn draw_sleep_counter(frame: &mut Mat, is_blinking: bool, blink_streak: &mut usize) -> Result<()> {
    let org : Point = Point {
        x: 10,
        y: 30,
    };

    let color: Scalar;
    let label: String;
    if is_blinking {
        *blink_streak += 1;
        if *blink_streak <= SLEEP_STREAK_THRESH {
            color = WARN_COLOR;
            label = String::from(format!("YOU ARE BLINKING!, {} left to pause", SLEEP_STREAK_THRESH - *blink_streak ));
        } else {
            color = SUCCESS_COLOR;
            label = String::from(format!("Media paused! good night!"))
        }
    } else {
        color = SUCCESS_COLOR;
        label = String::from("you're finally awake :)");
        *blink_streak = 0;
    }

    imgproc::put_text(
        frame, 
        &label, 
        org,
        1,
        2f64, 
        color,
        2,
        1, 
        false).unwrap();

    Ok(())
}

const WINDOWS_MEDIA_KEY: u16 = 0xB3;
fn on_sleep_detected() -> Result<()> {
    let mut enigo = Enigo::new();

    enigo.key_down(enigo::Key::Raw(WINDOWS_MEDIA_KEY));    
    
    Ok(())
}