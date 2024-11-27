pub mod vectors{
    use std::ops::{Add, Mul};
    #[derive(Debug, Copy, Clone)]
    struct Coord3d<U> {
        x: U,
        y: U,
        z: U,
    }

    impl<U> Coord3d<U> {
        fn new(x: U, y: U, z:U) -> Coord3d<U> {
            Coord3d {x, y, z}
        }

        fn shift_all<A>(&self, a: A) -> Coord3d<U> where U: Add<A, Output=U> + Copy, A: Copy{
            Coord3d {
                x: self.x + a,
                y: self.y + a,
                z: self.z + a
            }
        }

        fn shift_x<A>(&self, a: A) -> Coord3d<U> where U: Add<A, Output=U> + Copy, A: Copy{
            Coord3d {
                x: self.x + a,
                y: self.y,
                z: self.z
            }
        }

        fn shift_y<A>(&self, a: A) -> Coord3d<U> where U: Add<A, Output=U> + Copy,A: Copy{
            Coord3d {
                x: self.x,
                y: self.y + a,
                z: self.z
            }
        }

        fn shift_z<A>(&self, a: A) -> Coord3d<U> where U: Add<A, Output=U> + Copy, A: Copy{
            Coord3d {
                x: self.x,
                y: self.y,
                z: self.z + a
            }
        }

        fn scale<A>(&self, a: A) -> Coord3d<U> where U: Mul<A, Output=U> + Copy, A: Copy{
            Coord3d {
                x: self.x * a,
                y: self.y * a,
                z: self.z * a
            }
        }

        fn add_coord<A>(&self, coord3d: Coord3d<A>) -> Coord3d<U> where U: Add<A, Output=U> + Copy{
            Coord3d {
                x: self.x + coord3d.x,
                y: self.y + coord3d.y,
                z: self.z + coord3d.z
            }
        }

        fn origin()->Coord3d<U>{
            Coord3d{x:0, y:0, z:0}
        }
    }
}