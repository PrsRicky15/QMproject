#[allow(dead_code)]
pub mod math_vector {
    use std::ops::{Add, Sub, Mul};

    #[derive(Debug, Copy, Clone)]
    pub struct Coord3d<U> {
        x: U,
        y: U,
        z: U,
    }

    impl<U> Coord3d<U>
    where
        U: Add<Output = U> + Sub<Output = U> + Mul<Output = U> + Copy + Default + Into<f64>,
    {
        pub fn new(x: U, y: U, z: U) -> Self {
            Self { x, y, z }
        }

        pub fn shift_all(&self, a: U) -> Self {
            Self {
                x: self.x + a,
                y: self.y + a,
                z: self.z + a,
            }
        }

        pub fn shift_x(&self, a: U) -> Self {
            Self {
                x: self.x + a,
                y: self.y,
                z: self.z,
            }
        }

        pub fn shift_y(&self, a: U) -> Self {
            Self {
                x: self.x,
                y: self.y + a,
                z: self.z,
            }
        }

        pub fn shift_z(&self, a: U) -> Self {
            Self {
                x: self.x,
                y: self.y,
                z: self.z + a,
            }
        }

        pub fn scale(&self, a: U) -> Self {
            Self {
                x: self.x * a,
                y: self.y * a,
                z: self.z * a,
            }
        }

        pub fn add_coord(&self, coord3d: Coord3d<U>) -> Self {
            Self {
                x: self.x + coord3d.x,
                y: self.y + coord3d.y,
                z: self.z + coord3d.z,
            }
        }

        pub fn origin() -> Self {
            Self {
                x: U::default(),
                y: U::default(),
                z: U::default(),
            }
        }

        pub fn inner_product(&self) -> U {
            self.x * self.x + self.y * self.y + self.z * self.z
        }

        pub fn outter_product(&self, other: &Coord3d<U>) -> [[U; 3]; 3] {
            [
                [self.x * other.x, self.x * other.y, self.x * other.z],
                [self.y * other.x, self.y * other.y, self.y * other.z],
                [self.z * other.x, self.z * other.y, self.z * other.z],
            ]
        }

        pub fn distance(&self, coord3d: Coord3d<U>) -> f64 {
            let dx = self.x - coord3d.x;
            let dy = self.y - coord3d.y;
            let dz = self.z - coord3d.z;
            (dx * dx + dy * dy + dz * dz).into().sqrt()
        }
    }
}
