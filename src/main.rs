pub mod basiclearn;
fn main() {
    println!("{}", basiclearn::basicfunc::get_full_name("Hello", "World!"));
    basiclearn::basicfunc::some_examples_touples_array_and_box();

    basiclearn::control_flow::drive_test();
}
