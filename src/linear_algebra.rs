#[allow(dead_code)]
mod matrix{
    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix<T>{
        nrows:usize,
        ncols:usize,
        values:Vec<T>,
    }

    impl <T:Default+Clone> Matrix<T>{
        pub fn new(nrows:usize,ncols:usize)->Self {
            Self{nrows, ncols, values:vec![T::default();nrows*ncols]}
        }

        pub fn from_vector(nrows:usize, ncols:usize, values:Vec<T>) ->Self{
            assert_eq!(nrows * ncols, values.len(), "Dimension mismatch");
            Self{nrows, ncols, values}
        }

        pub fn nrows(&self) -> usize{
            self.nrows
        }

        pub fn ncols(&self) -> usize{
            self.ncols
        }

        pub fn get_value_at(&self, row:usize, col:usize) -> Option<&T>{
            if row < self.nrows && col < self.ncols{
                Some(&self.values[row * self.ncols + col])
            } else {
                None
            }
        }

        pub fn set_value_at(&mut self, row:usize, col:usize, value:T){
            if row < self.nrows && col < self.ncols {
                self.values[row * self.ncols + col] = value
            } else {
                panic!("Row or Column index is out of range");
            };
        }
    }
}