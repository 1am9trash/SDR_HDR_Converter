mod converter {
    pub mod image;
    pub mod tonemap;
    pub mod gamma;
    pub mod color_space;
}
use converter::image::*;

mod codec_handler;

fn main() {
    let sample_in = "sample/origin_field_hdr_linear_16.png";
    let in_format = ImageFormat {
        dynamic_range: DynamicRange::HDR,
        norm: NormFormat::NonNorm,
        signal_status: SignalStatus::Linear,
        color_depth: ColorDepth::SixteenBit,
        color_space: ColorSpace::Rec2020,
        color_format: ColorFormat::RGB
    };
    let image_data = codec_handler::read_png(sample_in, &in_format);

    match image_data {
        Ok(mut image) => {
            let sample_out = "sample/output_field_sdr_709_8.png";
            let out_format = ImageFormat {
                dynamic_range: DynamicRange::SDR,
                norm: NormFormat::NonNorm,
                signal_status: SignalStatus::Gamma709,
                color_depth: ColorDepth::EightBit,
                color_space: ColorSpace::Rec709,
                color_format: ColorFormat::RGB
            };
            image.convert(&out_format);

            codec_handler::write_png(sample_out, &image.buf, image.width, image.height, image.format.color_depth as usize);
        }
        Err(err) => println!{"Read png image error. {:?}", err}
    }
} 