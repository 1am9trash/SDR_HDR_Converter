#![allow(dead_code)]

pub fn eotf_pq_st2084(value: f32) -> f32 {
    let m1:f32 = 0.1593017578125;
    let m2:f32 = 78.84375;
    let c1:f32 = 0.8359375;
    let c2:f32 = 18.8515625;
    let c3:f32 = 18.6875;

    if value < 0.0 {
        0.0
    } else {
        let m1_inv = 1.0 / m1;
        let m2_inv = 1.0 / m2;

        let value_pow = value.powf(m2_inv);
        let num = f32::max(value_pow - c1, 0.0);
        let den = f32::max(c2 - c3 * value_pow, f32::MIN);

        (num / den).powf(m1_inv)
    }
}

pub fn inverse_eotf_pq_st2084(value: f32) -> f32 {
    let m1:f32 = 0.1593017578125;
    let m2:f32 = 78.84375;
    let c1:f32 = 0.8359375;
    let c2:f32 = 18.8515625;
    let c3:f32 = 18.6875;

    if value < 0.0 {
        0.0
    } else {
        let value_pow = value.powf(m1);
        let num = c1 + c2 * value_pow;
        let den = 1.0 + c3 * value_pow;

        (num / den).powf(m2)
    }
}

pub fn eotf_gamma_rec709(value: f32) -> f32 {
    let alpha: f32 = 1.09929682680944;
    let beta: f32 = 0.018053968510807;

    let value = f32::max(value, 0.0);

    if value < 4.5 * beta {
        value / 4.5
    } else {
        ((value + alpha - 1.0) / alpha).powf(1.0 / 0.45)
    }
}

pub fn oetf_gamma_rec709(value: f32) -> f32 {
    let alpha: f32 = 1.09929682680944;
    let beta: f32 = 0.018053968510807;

    let value = f32::max(value, 0.0);

    if value <= beta {
        value * 4.5
    } else {
        alpha * value.powf(0.45) - (alpha - 1.0)
    }
}

pub fn inverse_oetf_hlg_arib_b67(value: f32) -> f32 {
    let a: f32 = 0.17883277;
    let b: f32 = 0.28466892;
    let c: f32 = 0.55991073;

    let value = f32::max(value, 0.0);

    if value <= 0.5 {
        value.powf(2.0) / 3.0
    } else {
        ((value - c) / a + b).exp() / 12.0
    }
}

pub fn oetf_hlg_arib_b67(value: f32) -> f32 {
    let a: f32 = 0.17883277;
    let b: f32 = 0.28466892;
    let c: f32 = 0.55991073;

    let value = f32::max(value, 0.0);

    if value <= 1.0 / 12.0 {
        (3.0 * value).powf(0.5)
    } else {
        a * (12.0 * value - b).ln() + c
    }
}