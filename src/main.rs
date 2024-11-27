mod basiclearn;
mod math_vector;
use math_vector::math_vector::Coord3d;
fn main() {
   basiclearn::control_flow::gaussian_basis();
   let p1: Coord3d<u8> = Coord3d::origin();
   let p2: Coord3d<u8> = Coord3d::new(1,1,1);
   println!("{:?}", p2.distance(p1));
}
