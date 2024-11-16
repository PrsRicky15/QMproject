pub mod basicfunc {
    #[allow(dead_code)]
    pub fn get_full_name(first: &str, last: &str)->String{
        let full_name = format!("{0} {1}", first, last);
        full_name
    }

    #[allow(dead_code)]
    pub fn some_examples_touples_array_and_box(){
        let atom_type = ("He", 2, 2_f64, 4_f64); // Touple
        println!("Atom, {:?}", atom_type);

        let atomic_charge = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]; // static array
        println!("Atomic Charge, {:?}", atomic_charge);

        let some_atoms = &atomic_charge[5..7]; // slice
        println!("Atomic Charge, {:?}", some_atoms);
    }
}

pub mod control_flow {
    pub fn ifstatement_test(){
        let age_to_drive:u8 = 18;

        println!("Enter the person age:");
        let myinput = &mut String::from("");
        std::io::stdin().read_line(myinput).unwrap();

        let age = myinput.replace("\n","").parse::<u8>().unwrap();
        if age > age_to_drive && age < 60 {
            println!("You are old enough drive!")
        } else if age < age_to_drive || age > 60 {
            println!("Can not drive!")
        } else {
            println!("This year you can drive!")
        }
    }

    pub fn while_loop(){}
}