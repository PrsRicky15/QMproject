pub mod math_vector {
    use std::ops::{Add, Sub, Mul};

    #[derive(Debug, Copy, Clone)]
    pub struct Coord3d<U> {
        x: U,
        y: U,
        z: U,
    }

    impl<U> Coord3d<U> {
        pub fn new(x: U, y: U, z: U) -> Self {
            Self { x, y, z }
        }

        pub fn shift_all(&self, a: U) -> Self
        where
            U: Add<Output = U> + Copy,
        {
            Self {
                x: self.x + a,
                y: self.y + a,
                z: self.z + a,
            }
        }

        pub fn shift_x(&self, a: U) -> Self
        where
            U: Add<Output = U> + Copy,
        {
            Self {
                x: self.x + a,
                y: self.y,
                z: self.z,
            }
        }

        pub fn shift_y(&self, a: U) -> Self
        where
            U: Add<Output = U> + Copy,
        {
            Self {
                x: self.x,
                y: self.y + a,
                z: self.z,
            }
        }

        pub fn shift_z(&self, a: U) -> Self
        where
            U: Add<Output = U> + Copy,
        {
            Self {
                x: self.x,
                y: self.y,
                z: self.z + a,
            }
        }

        pub fn scale(&self, a: U) -> Self
        where
            U: Mul<Output = U> + Copy,
        {
            Self {
                x: self.x * a,
                y: self.y * a,
                z: self.z * a,
            }
        }

        pub fn add_coord(&self, coord3d: Coord3d<U>) -> Self
        where
            U: Add<Output = U> + Copy,
        {
            Self {
                x: self.x + coord3d.x,
                y: self.y + coord3d.y,
                z: self.z + coord3d.z,
            }
        }

        pub fn origin() -> Self
        where
            U: Default,
        {
            Self {
                x: U::default(),
                y: U::default(),
                z: U::default(),
            }
        }

        pub fn distance(&self, coord3d: Coord3d<U>) -> U
        where
            U: Sub<Output = U> + Add<Output = U> + Copy + Mul<Output = U>,
        {
            let dx = self.x - coord3d.x;
            let dy = self.y - coord3d.y;
            let dz = self.z - coord3d.z;
            dx * dx + dy * dy + dz * dz
        }
    }

    impl<U> Default for Coord3d<U>
    where
        U: Default,
    {
         fn default() -> Self {
            Self {
                x: U::default(),
                y: U::default(),
                z: U::default(),
            }
        }
    }
}