// Adapted from here: https://stackoverflow.com/questions/68986787/draw-bounding-boxding-box-around-whole-block-of-text-in-image-using-python

use anyhow::Result;
use opencv::core::ToInputArray;
use opencv::prelude::MatTraitConst;

fn main() -> Result<()> {
    // Read image
    let mut img = opencv::imgcodecs::imread(
        "./assets/hw_multiline.png",
        opencv::imgcodecs::ImreadModes::IMREAD_COLOR.into(),
    )?;

    // Binarize image. Text is white, background is black
    let mut bin_raw = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&img, &mut bin_raw, opencv::imgproc::COLOR_BGR2GRAY, 0)?;
    let mut bin = opencv::core::Mat::default();
    opencv::core::compare(
        &bin_raw,
        &100.0f64.input_array()?,
        &mut bin,
        opencv::core::CmpTypes::CMP_LT.into(),
    )?;

    opencv::imgcodecs::imwrite("./output/bin.png", &bin, &opencv::core::Vector::new())?;

    let mut morph = opencv::core::Mat::default();
    opencv::imgproc::morphology_ex(
        &bin,
        &mut morph,
        opencv::imgproc::MorphTypes::MORPH_CLOSE.into(),
        &opencv::imgproc::get_structuring_element(
            opencv::imgproc::MorphShapes::MORPH_RECT.into(),
            opencv::core::Size::new(150, 50),
            opencv::core::Point2i::new(-1, -1),
        )?,
        opencv::core::Point2i::new(-1, -1),
        1,
        opencv::core::BorderTypes::BORDER_CONSTANT.into(),
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    opencv::imgcodecs::imwrite(
        "./output/hw_morph.png",
        &morph,
        &opencv::core::Vector::new(),
    )?;

    let mut contours = opencv::types::VectorOfVectorOfPoint::new();
    opencv::imgproc::find_contours(
        &morph,
        &mut contours,
        opencv::imgproc::RetrievalModes::RETR_EXTERNAL.into(),
        opencv::imgproc::ContourApproximationModes::CHAIN_APPROX_SIMPLE.into(),
        opencv::core::Point::default(),
    )?;

    assert!(!contours.is_empty());

    for i in 0..contours.len() {
        let contour = contours.get(i)?;

        let bounding = opencv::imgproc::bounding_rect(&contour)?;
        opencv::imgproc::rectangle(
            &mut img,
            bounding,
            opencv::core::Scalar::new(0.0, 0.0, 255.0, 255.0),
            1,
            opencv::imgproc::LineTypes::LINE_8.into(),
            0,
        )?;

        // Save each cut of the image using the bounding box
        let row = img.row_bounds(bounding.y, bounding.y + bounding.height)?;
        let col = row.col_bounds(bounding.x, bounding.x + bounding.width)?;

        opencv::imgcodecs::imwrite(
            &format!("./output/hw_bounding_{}.png", i),
            &col,
            &opencv::core::Vector::new(),
        )?;
    }

    // Write all the lines in one image
    opencv::imgcodecs::imwrite(
        "./output/hw_bounding.png",
        &img,
        &opencv::core::Vector::new(),
    )?;

    Ok(())
}
