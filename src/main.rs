
#[allow(dead_code)]
fn get_full_name(first: &str, last: &str)->String{
    let full_name = format!("{0} {1}", first, last);
    full_name
}

#[allow(dead_code)]
fn some_examples_touples_array_and_box(){
    let atom_type = ("He", 2, 2_f64, 4_f64); // Touple
    println!("Atom, {:?}", atom_type);

    let atomic_charge = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]; // static array
    println!("Atomic Charge, {:?}", atomic_charge);

    let some_atoms = &atomic_charge[5..7]; // slice
    println!("Atomic Charge, {:?}", some_atoms);
}

fn main() {
    println!("{}", get_full_name("Hello", "World!"));
    some_examples_touples_array_and_box();
}
