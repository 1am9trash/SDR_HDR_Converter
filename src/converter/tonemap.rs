#![allow(dead_code)]

pub fn tonemap(signal: &Vec<f32>) -> Vec<f32> {
    let peak: f32 = 2.0;

    let origin_tone = signal.iter().fold(1e-6, |mx, &x| f32::max(mx, x));
    let new_tone = hable(origin_tone) / hable(peak);

    signal.iter().map(|x| x * new_tone / origin_tone).collect()
}
 
pub fn inverse_tonemap(signal: &Vec<f32>) -> Vec<f32> {
    signal.iter().map(|x| (x * 2.0).min(1.0)).collect()
}

fn hable(value: f32) -> f32 {
    let a: f32 = 0.15;
    let b: f32 = 0.50;
    let c: f32 = 0.10;
    let d: f32 = 0.20;
    let e: f32 = 0.02;
    let f: f32 = 0.30;

    (value * (value * a + b * c) + d * e) / (value * (value * a + b) + d * f) - e / f
}

pub fn untrival_tonemap(signal: &Vec<f32>) -> Vec<f32> {
    let l_hdr: f32 = 1000.0;
    let l_sdr: f32 = 100.0;

    fn color_correction(signal: &Vec<f32>, y_sdr: f32) -> Vec<f32> {
        let mut ycbcr_signal = rgb_to_ycbcr_bt2020(signal);
        let scale = y_sdr / (ycbcr_signal[0] * 1.1);

        ycbcr_signal[1] *= scale;
        ycbcr_signal[2] *= scale;
        ycbcr_signal[0] = y_sdr - 0.0f32.max(0.1 * ycbcr_signal[2]);

        ycbcr_signal
    }

    let signal: Vec<f32> = signal
        .chunks(3)
        .flat_map(|chunk| {
            let mut chunk = chunk.to_vec();

            let y = calculate_luma(&chunk);
            let p_hdr = 1.0 + 32.0 * (l_hdr / 10000.0).powf(1.0 / 2.4);
            let y_p = (1.0 + (p_hdr - 1.0) * y).ln() / p_hdr.ln();

            let y_c: f32;
            if y_p >= 0.0 && y_p <= 0.7399 {
                y_c = 1.0770 * y_p;
            } else if y_p > 0.7399 && y_p < 0.9909 {
                y_c = -1.1510 * y_p.powf(2.0) + 2.7811 * y_p - 0.6302;
            } else if y_p >= 0.9909 && y_p <= 1.0 {
                y_c = 0.5 * y_p + 0.5;
            } else {
                y_c = (0.0f32).max(y_p).min(1.0);
            }

            let p_sdr = 1.0 + 32.0 * (l_sdr / 10000.0).powf(1.0 / 2.4);
            let y_sdr = (p_sdr.powf(y_c) - 1.0) / (p_sdr - 1.0);

            chunk = color_correction(&chunk, y_sdr);
            chunk = ycbcr_to_rgb_bt2020(&chunk);

            chunk
        })
        .collect();

    signal
}

pub fn untrival_inverse_tonemap(signal: &Vec<f32>) -> Vec<f32> {
    let a1: f32 = 1.8712e-5;
    let b1: f32 = -2.7334e-3;
    let c1: f32 = 1.3141;
    let a2: f32 = 2.8305e-6;
    let b2: f32 = -7.4622e-4;
    let c2: f32 = 1.2528;

    fn clamp(a: f32, b: f32, c: f32) -> f32 {
        if a < b {
            b
        } else if a > c {
            c
        } else {
            a
        }
    }

    let signal: Vec<f32> = signal
        .chunks(3)
        .flat_map(|chunk| {
            let mut chunk = chunk.to_vec();

            chunk = rgb_to_ycbcr_bt2020(&chunk);

            let y_adapt = chunk[0] * 255.0;
            let e: f32;
            if y_adapt <= 70.0 {
                e = a1 * y_adapt.powf(2.0) + b1 * y_adapt + c1;
            } else {
                e = a2 * y_adapt.powf(2.0) + b2 * y_adapt + c2;
            }
            let y_hdr = y_adapt.powf(e);

            let s_c: f32;
            if chunk[0] > 0.0 {
                s_c = 1.075 * y_hdr / chunk[0];
            } else {
                s_c = 1.0;
            }

            let cb = chunk[1] * s_c;
            let cr = chunk[2] * s_c;

            chunk[0] = clamp(y_hdr + 1.4746 * cr, 0.0, 1000.0) / 1000.0;
            chunk[1] = clamp(y_hdr - 0.16455 * cb - 0.57135 * cr, 0.0, 1000.0) / 1000.0;
            chunk[2] = clamp(y_hdr + 1.8814 * cb, 0.0, 1000.0) / 1000.0;

            chunk
        })
        .collect();

    signal
}

pub fn calculate_luma(signal: &Vec<f32>) -> f32 {
    0.2627 * signal[0] + 0.6780 * signal[1] + 0.0593 * signal[2]
}

pub fn rgb_to_ycbcr_bt2020(signal: &Vec<f32>) -> Vec<f32> {
    let y = calculate_luma(signal);

    let mut ycbcr_signal: Vec<f32> = vec![0.0; 3];

    ycbcr_signal[0] = y;
    ycbcr_signal[1] = (signal[2] - y) / 1.8814;
    ycbcr_signal[2] = (signal[0] - y) / 1.4746;

    ycbcr_signal
}

pub fn ycbcr_to_rgb_bt2020(signal: &Vec<f32>) -> Vec<f32> {
    let a: f32 = 0.2627;
    let b: f32 = 0.6780;
    let c: f32 = 0.0593;
    let d: f32 = 1.8814;
    let e: f32 = 1.4746;

    let mut ycbcr_signal: Vec<f32> = vec![0.0; 3];
    ycbcr_signal[0] = signal[0] + e * signal[2];
    ycbcr_signal[1] = signal[0] - a * e / b * signal[2] - c * d / b * signal[1];
    ycbcr_signal[2] = signal[0] + d * signal[1];

    ycbcr_signal
}