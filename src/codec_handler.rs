use std::error::Error;

use super::converter::image::*;

pub fn read_png(filename: &str, format: &ImageFormat) -> Result<ImageData, Box<dyn Error>> {
    use std::fs::File;
    use png::Decoder;
    use png::Transformations;

    let mut decoder = Decoder::new(File::open(filename)?);
    decoder.set_transformations(Transformations::IDENTITY);

    let mut reader = decoder.read_info()?;
    let info = reader.info();

    let mut cur_format = *format;
    let color_depth = match (info.bit_depth, info.color_type) {
        (png::BitDepth::Eight, png::ColorType::Rgb) => {
            cur_format.color_depth = ColorDepth::EightBit;
            cur_format.color_format = ColorFormat::RGB;
            8usize
        },
        (png::BitDepth::Sixteen, png::ColorType::Rgb) => {
            cur_format.color_depth = ColorDepth::SixteenBit;
            cur_format.color_format = ColorFormat::RGB;
            16usize
        },
        (_, _) => {
            return Err("Png format error".into());
        }
    };

    let width = info.width as usize;
    let height = info.height as usize;

    let mut byte_buf = vec![0u8; width * height * color_depth / 8 * 3];
    reader.next_frame(&mut byte_buf)?;

    let image_data = ImageData::new(width, height, &cur_format, arrayu8_to_vecf32(&byte_buf, color_depth));
    Ok(image_data)
}

pub fn write_png(filename: &str, buf: &Vec<f32>, width: usize, height: usize, depth: usize) {
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create(filename).unwrap();
    let ref mut writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
    match depth {
        8 => encoder.set_depth(png::BitDepth::Eight),
        16 => encoder.set_depth(png::BitDepth::Sixteen),
        _ => panic!("depth must be 8 or 16.")
    }
    encoder.set_color(png::ColorType::Rgb);

    let mut writer = encoder.write_header().unwrap();

    let byte_buf = vecf32_to_vecu8(buf, depth);

    writer.write_image_data(&byte_buf).unwrap();
}

fn arrayu8_to_vecf32(data: &[u8], depth: usize) -> Vec<f32> {
    if depth == 8 {
        data.iter().map(|x| *x as f32).collect()
    } else if depth == 16 {
        data.chunks(2)
            .map(|chunk| ((u16::from(chunk[0]) << 8) + u16::from(chunk[1])) as f32)
            .collect()
    } else {
        panic!("depth must be 8 or 16.");
    }
}

fn vecf32_to_vecu8(data: &Vec<f32>, depth: usize) -> Vec<u8> {
    if depth == 8 {
        data.iter().map(|x| *x as u8).collect()
    } else if depth == 16 {
        data.iter()
            .flat_map(|x| {
                vec![(*x / 256.0) as u8, (*x % 256.0) as u8] 
            })
            .collect()
    } else {
        panic!("depth must be 8 or 16.");
    }
}