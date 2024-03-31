// Adapted from here: https://stackoverflow.com/questions/34981144/split-text-lines-in-scanned-document

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

    // Find all white pixels
    let mut points: opencv::core::Vector<opencv::core::Point> = opencv::core::Vector::new();
    opencv::core::find_non_zero(&bin, &mut points)?;

    // Get rotated rect of white pixels
    let mut bounding = opencv::imgproc::min_area_rect(&points)?;
    if bounding.size.width < bounding.size.height {
        std::mem::swap(&mut bounding.size.width, &mut bounding.size.height);
        bounding.angle -= 90f32;
    }

    // Draw the bounding box
    let mut vertices: [opencv::core::Point2f; 4] = [opencv::core::Point2f::default(); 4];
    bounding.points(&mut vertices)?;

    if !vertices.is_empty() {
        for i in 0..4 {
            opencv::imgproc::line(
                &mut img,
                opencv::core::Point2i::new(vertices[i].x as i32, vertices[i].y as i32),
                opencv::core::Point2i::new(
                    vertices[(i + 1) % 4].x as i32,
                    vertices[(i + 1) % 4].y as i32,
                ),
                opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                1,
                opencv::imgproc::LineTypes::LINE_8.into(),
                0,
            )?;
        }
    }

    // Rotate the image according to the found angle
    let mut rotated = opencv::core::Mat::default();
    let m = opencv::imgproc::get_rotation_matrix_2d(bounding.center, bounding.angle as f64, 1.0)?;
    opencv::imgproc::warp_affine(
        &bin,
        &mut rotated,
        &m,
        bin.size()?,
        opencv::imgproc::InterpolationFlags::INTER_LINEAR.into(),
        opencv::core::BorderTypes::BORDER_CONSTANT.into(),
        opencv::core::Scalar::default(),
    )?;

    // Compute horizontal projections
    let mut hor_proj = opencv::core::Mat::default();
    opencv::core::reduce(&rotated, &mut hor_proj, 1, opencv::core::REDUCE_AVG, -1)?;

    // Remove noise in histogram. White bins identify space lines, black bins identify text lines
    let th = 0.0;
    let mut hist = opencv::core::Mat::default();
    opencv::core::compare(
        &hor_proj,
        &th.input_array()?,
        &mut hist,
        opencv::core::CmpTypes::CMP_LE.into(),
    )?;

    // Get mean coordinate of white white pixels groups
    let mut ycoords = vec![];
    let mut y = 0;
    let mut count = 0;
    let mut is_space = false;
    for i in 0..rotated.rows() {
        if !is_space {
            if hist.at::<u8>(i).unwrap() != &0 {
                is_space = true;
                count = 1;
                y = i;
            }
        } else if hist.at::<u8>(i).unwrap() == &0 {
            is_space = false;
            ycoords.push(y / count);
        } else {
            y += i;
            count += 1;
        }
    }

    let mut result = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&rotated, &mut result, opencv::imgproc::COLOR_GRAY2BGR, 0)?;
    ycoords.push(result.rows());
    for i in 0..(ycoords.len() - 1) {
        // Put each line in one image
        // let cols = result.cols();
        // opencv::imgproc::line(
        //     &mut result,
        //     opencv::core::Point2i::new(0, ycoords[i]),
        //     opencv::core::Point2i::new(cols, ycoords[i]),
        //     opencv::core::Scalar::new(0.0, 255.0, 0.0, 1.0),
        //     1,
        //     opencv::imgproc::LineTypes::LINE_8.into(),
        //     0,
        // )?;

        // Write each line in it's own image
        let tmp = result.row_range(&opencv::core::Range::new(ycoords[i], ycoords[i + 1])?)?;
        opencv::imgcodecs::imwrite(
            &format!("./output/hw_multiline_lines_{}.png", i),
            &tmp,
            &opencv::core::Vector::new(),
        )?;
    }

    // Write all the lines in one image
    // opencv::imgcodecs::imwrite(
    //     "./assets/hw_multiline_lines.png",
    //     &result,
    //     &opencv::core::Vector::new(),
    // )?;

    Ok(())
}
