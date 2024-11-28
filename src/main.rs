mod math_vector;
mod basiclearn;
mod linear_algebra;

use num_complex::Complex32;
use std::ffi::c_void;

extern "C" {
    fn fftwf_plan_dft_1d(
        n: i32,
        in_: *mut Complex32,
        out: *mut Complex32,
        sign: i32,
        flags: u32,
    ) -> *mut c_void;

    fn fftwf_execute(plan: *mut c_void);
    fn fftwf_destroy_plan(plan: *mut c_void);
}

const FFTW_FORWARD: i32 = -1;
const FFTW_MEASURE: u32 = 0;

fn fft(){
    let n = 8; // Size of the input array
    let mut input: Vec<Complex32> = vec![
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, 0.0),
        Complex32::new(0.0, 0.0),
        Complex32::new(0.0, 0.0),
        Complex32::new(0.0, 0.0),
        Complex32::new(0.0, 0.0),
    ];
    let mut output: Vec<Complex32> = vec![Complex32::new(0.0, 0.0); n];

    unsafe {
        // Create FFTW plan
        let plan = fftwf_plan_dft_1d(
            n as i32,
            input.as_mut_ptr(),
            output.as_mut_ptr(),
            FFTW_FORWARD,
            FFTW_MEASURE,
        );

        // Execute the plan
        fftwf_execute(plan);

        // Destroy the plan
        fftwf_destroy_plan(plan);
    }

    // Print the result
    println!("Output:");
    for (i, value) in output.iter().enumerate() {
        println!("output[{}] = {}", i, value);
    }
}

fn to_column_major(a: &[f64], rows: usize, cols: usize) -> Vec<f64> {
    let mut col_major = vec![0.0; rows * cols];
    for i in 0..rows {
        for j in 0..cols {
            col_major[j * rows + i] = a[i * cols + j];
        }
    }
    col_major
}

fn blas(){
    let a = vec![1.0, 2.0, 3.0, 4.0]; // 2x2 matrix
    let b = vec![5.0, 6.0, 7.0, 8.0]; // 2x2 matrix
    let mut c = vec![0.0; 4];         // Result 2x2 matrix

    let a_col_major = to_column_major(&a, 2, 2);
    let b_col_major = to_column_major(&b, 2, 2);

    basiclearn::blas::dgemm('N', 'N', 2, 2, 2, 1.0, &a_col_major, 2, &b_col_major, 2, 0.0, &mut c, 2);
    let c_col_major = to_column_major(&c, 2, 2);
    assert_eq!(c_col_major, vec![19.0, 22.0, 43.0, 50.0]); // Validate output
}
fn main() {
    fft();
    blas();
}
