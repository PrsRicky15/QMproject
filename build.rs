fn main() {
    println!("cargo:rustc-link-lib=dylib=blas"); // Link blas
    println!("cargo:rustc-link-lib=fftw3f"); // Link the FFTW3 library
    println!("cargo:rustc-link-search=/path/to/fftw3/lib"); // Adjust to the path of your FFTW3 library
}

