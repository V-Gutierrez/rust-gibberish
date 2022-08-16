use std::collections::HashMap;

use hello_rust::{do_stuff, variables_explaining};

fn main() {
    do_stuff(1, 2.3);
    variables_explaining();
}




/*


struct MyFirstStruct {
    name: str,
    enemy: bool,
}

trait Beginner {
    fn get_name(&self) -> &str;
}

impl Beginner for MyFirstStruct {
    fn get_name(&self) -> &str {
        self::name
    }
}

impl MyFirstStruct {
    fn new(name: &str) -> Self {
        Self {
            name: name as str,
            enemy: false,
        }
    }
}


fn inst() {
    let my_struct = MyFirstStruct::new("Me");

    print!(my_struct.name, "Me");
}


fn create_my_first_vector() {
    let mut v: Vec<i32> = vec![1, 2, 3];
}

fn create_my_first_hashMap() -> Option<bool> {
    let mut v: HashMap<i32, bool> = HashMap::new();
    v.insert(0, false)
}


enum Color {
    Red,
    Green,
    Blue,
};

match my_variable {
	2 => Color::Red,
    3 => Color::Green,
    4 => Color::Blue,
    None => None
};


let closure = | x , y | { x + y }


*/

// let red = Color::Red