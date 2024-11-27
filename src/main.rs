mod basiclearn;
mod math_vector;
use math_vector::math_vector::Coord3d;
fn main() {
   basiclearn::control_flow::gaussian_basis();
   let p1: Coord3d<u8> = Coord3d::origin();
   println!("{:?}", p1);
}
