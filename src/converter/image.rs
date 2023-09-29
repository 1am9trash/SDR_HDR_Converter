#![allow(dead_code)]

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum DynamicRange {
    HDR,
    SDR
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum NormFormat {
    Norm,
    NonNorm
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SignalStatus {
    Linear,
    Gamma709,
    GammaPQ,
    GammaHLG
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ColorDepth {
    EightBit = 8,
    SixteenBit = 16
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ColorSpace {
    Rec2020,
    Rec709
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ColorFormat {
    RGB
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ImageFormat {
    pub dynamic_range: DynamicRange,
    pub norm: NormFormat,
    pub signal_status: SignalStatus,
    pub color_depth: ColorDepth,
    pub color_space: ColorSpace,
    pub color_format: ColorFormat
}

pub struct ImageData {
    pub format: ImageFormat,
    pub width: usize,
    pub height: usize,
    pub buf: Vec<f32>,
}

impl ImageData {
    pub fn new(
        width: usize, 
        height: usize,
        format: &ImageFormat,
        image: Vec<f32>
    ) -> Self {
        ImageData {
            width: width,
            height: height,
            format: *format,
            buf: image
        }
    }

    pub fn convert(&mut self, format: &ImageFormat) {
        use super::gamma;
        use super::color_space;
        use super::tonemap;

        if self.format.norm == NormFormat::NonNorm {
            println!("\x1b[32mNormalizing.\x1b[0m");
            match self.format.color_depth {
                ColorDepth::EightBit => self.buf.iter_mut().for_each(|x| *x /= 255.0),
                ColorDepth::SixteenBit => self.buf.iter_mut().for_each(|x| *x /= 65535.0)
            }
        }

        let eotf_fn = match self.format.signal_status {
            SignalStatus::Linear => None,
            SignalStatus::GammaHLG => Some(gamma::inverse_oetf_hlg_arib_b67 as fn(f32) -> f32),
            SignalStatus::GammaPQ => Some(gamma::eotf_pq_st2084 as fn(f32) -> f32),
            SignalStatus::Gamma709 => Some(gamma::eotf_gamma_rec709 as fn(f32) -> f32),
        };

        if let Some(transform) = eotf_fn {
            println!("\x1b[32mEOTF transforming.\x1b[0m");
            self.buf.iter_mut().for_each(|x| {
                *x = transform(*x);
            });
        }

        let space_fn = match (&self.format.color_space, &format.color_space) {
            (ColorSpace::Rec2020, ColorSpace::Rec709) => Some(color_space::rgb_color_space_rec2020_to_rec709 as fn(&Vec<f32>) -> Vec<f32>),
            (ColorSpace::Rec709, ColorSpace::Rec2020) => Some(color_space::rgb_color_space_rec709_to_rec2020 as fn(&Vec<f32>) -> Vec<f32>),
            (in_format, out_format) => {
                if in_format != out_format { 
                    panic!("Never reached")
                }
                None
            }
        };

        if let Some(transform) = space_fn {
            println!("\x1b[32mColor space transforming.\x1b[0m");
            self.buf = self.buf
                .chunks(3)
                .flat_map(|chunk| transform(&chunk.to_vec()))
                .collect();
        };

        let range_fn =  match (&self.format.dynamic_range, &format.dynamic_range) {
            (DynamicRange::HDR, DynamicRange::SDR) => Some(tonemap::untrival_tonemap as fn(&Vec<f32>) -> Vec<f32>),
            (DynamicRange::SDR, DynamicRange::HDR) => Some(tonemap::untrival_inverse_tonemap as fn(&Vec<f32>) -> Vec<f32>),
            (in_format, out_format) => {
                if in_format != out_format { 
                    panic!("Not supported yet")
                }
                None
            }
        };

        if let Some(transform) = range_fn {
            println!("\x1b[32mTone mapping.\x1b[0m");
            // untrival tone mapping
            self.buf = transform(&self.buf);

            // trival tone mapping
            // self.buf = self.buf
            //     .chunks(3)
            //     .flat_map(|chunk| transform(&chunk.to_vec()))
            //     .collect();
        };

        let oetf_fn = match format.signal_status {
            SignalStatus::Linear => None,
            SignalStatus::GammaHLG => Some(gamma::oetf_hlg_arib_b67 as fn(f32) -> f32),
            SignalStatus::GammaPQ => Some(gamma::inverse_eotf_pq_st2084 as fn(f32) -> f32),
            SignalStatus::Gamma709 => Some(gamma::oetf_gamma_rec709 as fn(f32) -> f32),
        };

        if let Some(transform) = oetf_fn {
            println!("\x1b[32mOETF transforming.\x1b[0m");
            self.buf.iter_mut().for_each(|x| {
                *x = transform(*x);
            });
        }

        if format.norm == NormFormat::NonNorm {
            println!("\x1b[32mInversely normalizing.\x1b[0m");
            match format.color_depth {
                ColorDepth::EightBit => self.buf.iter_mut().for_each(|x| *x *= 255.0),
                ColorDepth::SixteenBit => self.buf.iter_mut().for_each(|x| *x *= 65535.0)
            }
        }

        self.format = *format;
    }
}