pub mod closure{
    struct Person{
        first_name:String,
        last_name:String
    }

    #[allow(dead_code)]
    pub fn run(){
        let add : fn(f64, i32) = |x, y| println!("Sum: {}",x + y as f64);
        add(2., 3);

        let add2 : fn(f64, i32) -> f64 = |x, y|{
            println!("The value of x is: {}",x);
            println!("the value of y is: {}",y);
            x + y as f64
        };
        println!("Sum: {}", add2(8.7327, 10));

        let result = add2(5., 3);
        let print_result = || println!("The result: {}", result);
        print_result();

        let print_result = |x| println!("The result: {}", result + x);
        print_result(3.);
    }

    #[allow(dead_code)]
    pub fn mutable_run(){
        let mut person1 = Person{ first_name: "Prashant".to_string(), last_name:"Raj".to_string()};
        let mut change_name = |new_last_name:&str| person1.last_name = new_last_name.to_string();
        change_name("Singh");
        println!("Last Name: {} {}", person1.first_name, person1.last_name);
    }
}

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
    use std::process::exit;

    #[allow(dead_code)]
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

    #[derive(Debug)]
    pub struct ShellValue {
        l:u8, m:u8, n:u8
    }

    #[derive(Debug)]
    pub enum Shelltype {
        S {s: ShellValue},

        P {px: ShellValue,
            py: ShellValue,
            pz: ShellValue},

        D {dxx: ShellValue,
            dyy: ShellValue,
            dzz: ShellValue,
            dxy: ShellValue,
            dxz: ShellValue,
            dyz: ShellValue},

        F {fxxx: ShellValue,
            fyyy: ShellValue,
            fzzz: ShellValue,
            fxxy: ShellValue,
            fxxz: ShellValue,
            fyyx: ShellValue,
            fyyz: ShellValue,
            fzzx: ShellValue,
            fzzy: ShellValue,
            fxyz: ShellValue,
        },
    }

    // write a code which takes a character and generate the shell
    // with l, m, n value
    #[allow(dead_code)]
    pub fn shell(shell_type:char)->Shelltype{
        match shell_type {
            'S' => Shelltype::S {s:ShellValue {l:0,m:0,n:0}},
            _ => {println!("Shell type is not defines");exit(10)}
        }
    }

    #[allow(dead_code)]
    pub fn test_match(){
        let shell_type ='S';
        let mut sval = ShellValue{l:0, m:0, n:0};
        match shell_type {
            'S' => {sval.l = 0; sval.m = 0; sval.n =0}
            'P' => {sval.l = 1; sval.m = 0; sval.n =0}
            'D' => {sval.l = 2; sval.m = 0; sval.n =0}
            'F' => {sval.l = 3; sval.m = 0; sval.n =0}
            'G' => {sval.l = 4; sval.m = 0; sval.n =0}
            _ => {println!("Shell type is not defines");exit(10)}
        }
        println!("Shell: {:?}", sval);

        // different type of match
        let number = 101;
        match number {
            1..=9 => println!("Single digit number"),
            10..=99 => println!("Double digit number"),
            _ => println!("somthing else")
        }
    }

    #[allow(dead_code)]
    pub fn test_while_loop(){
        let mut x = 1;
        while x < 10{
            x *= 2;
            println!("Even number {}", x);
        }

        let mut x = 0;
        while x < 10{
            x += 1;
            println!("Number increment {}", x);
        }
    }

    #[allow(dead_code)]
    pub fn test_for_loop(){
        let ages:[i32; 5] = [2,4,6,8,10];
        for x in ages {
            println!("Number {}", x);
        }
    }

    #[allow(dead_code)]
    pub fn test_loop(){
        let mut x = 0;
        loop {
            println!("Number {}", x);
            x += 1;
            if x > 10 { break }
        }
    }
}