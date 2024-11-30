#[allow(dead_code)]

mod vectors{
    use std::alloc::{alloc, dealloc, realloc, Layout};
    use std::ptr;

    struct MyVecUnsafe<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> MyVecUnsafe<T> {
        fn new(capacity: usize) -> Self {
            assert!(capacity > 0, "Capacity must be greater than 0");

            let layout = Layout::array::<T>(capacity).expect("Layout creation failed");
            let ptr = unsafe { alloc(layout) as *mut T };

            if ptr.is_null() {
                panic!("Memory allocation failed");
            }

            Self {
                ptr,
                len: 0,
                capacity,
            }
        }

        fn push(&mut self, value: T) {
            if self.len == self.capacity {
                self.resize();
            }

            unsafe {
                self.ptr.add(self.len).write(value);
            }

            self.len += 1;
        }

        fn resize(&mut self) {
            let new_capacity = self.capacity * 2;
            let layout = Layout::array::<T>(self.capacity).unwrap();
            let new_layout = Layout::array::<T>(new_capacity).unwrap();

            let new_ptr = unsafe {
                realloc(self.ptr as *mut u8, layout, new_layout.size()) as *mut T
            };

            if new_ptr.is_null() {
                panic!("Memory reallocation failed");
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }

        fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }

        fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }

            self.len -= 1;

            unsafe { Some(self.ptr.add(self.len).read()) }
        }

        fn drop(&mut self) {
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    for i in 0..self.len {
                        ptr::drop_in_place(self.ptr.add(i));
                    }
                    dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }

    impl<T> Drop for MyVecUnsafe<T> {
        fn drop(&mut self) {
            self.drop();
        }
    }

}

mod matrix{
    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix<T>{
        nrows:usize,
        ncols:usize,
        values:Vec<T>,
    }

    impl <T:Default + Clone> Matrix<T>{
        pub fn new(nrows:usize, ncols:usize)->Self {
            Self { nrows, ncols, values: vec![T::default(); nrows * ncols] }
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
                panic!("Row or Column index is out of range");
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