use anyhow::Result; 
use opencv::core::{Size, Scalar};
use opencv::{
    prelude::*,
    objdetect::{CascadeClassifier},
    types as opencvTypes,
    imgproc
};

pub fn detect_eyes(classifier: &mut CascadeClassifier, frame: &Mat) -> Result<opencvTypes::VectorOfRect> {
    const SCALE_FACTOR: f64 = 1.1;
    const MIN_NEIGHBORS: i32 = 5;
    const FLAGS: i32 = 0;
    const MIN_SIZE: Size = Size {
        width: 20,
        height: 20,
    };
    const MAX_SIZE: Size = Size {
        width: 0,
        height: 0,
    };


    let mut eyes = opencvTypes::VectorOfRect::new();
    classifier.detect_multi_scale(frame, &mut eyes, SCALE_FACTOR, MIN_NEIGHBORS, FLAGS, MIN_SIZE, MAX_SIZE).unwrap();

    Ok(eyes)
}

pub fn draw_eye(frame: &mut Mat, eye: opencv::core::Rect_<i32>) -> Result<()> {
    const THICKNESS: i32 = 2;
    const LINE_TYPE: i32 = 8;
    const SHIFT: i32 = 0;
    let color_red = Scalar::new(0f64, 0f64, 255f64, -1f64);

    imgproc::rectangle(frame, eye, color_red, THICKNESS, LINE_TYPE, SHIFT)?;

    Ok(())
}