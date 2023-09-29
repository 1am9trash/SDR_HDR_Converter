#![allow(dead_code)]

pub fn rgb_color_space_rec2020_to_rec709(signal: &Vec<f32>) -> Vec<f32> {
    let transform_matrix: Vec<Vec<f32>> = vec![
        vec![1.660491, -0.587641, -0.072850],
        vec![-0.124550, 1.132900, -0.008349],
        vec![-0.018151, -0.100579, 1.118730],
    ];

    let mut result: Vec<f32> = vec![0.0; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[i] += transform_matrix[i][j] * signal[j];
        }
    }

    result
}

pub fn rgb_color_space_rec709_to_rec2020(signal: &Vec<f32>) -> Vec<f32> {
    let transform_matrix: Vec<Vec<f32>> = vec![
        vec![0.627404, 0.329283, 0.043313],
        vec![0.069097, 0.919540, 0.011362],
        vec![0.016391, 0.088013, 0.895595],
    ];

    let mut result: Vec<f32> = vec![0.0; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[i] += transform_matrix[i][j] * signal[j];
        }
    }

    result
}