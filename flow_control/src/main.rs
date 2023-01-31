
 use std::io; 
 fn main() {
    // let z = (2 as f32) <= 2.2;  //true convierto a float 
    // println!("Condition {}", z);
    // logics operators
    // &&   || ! not

    //input console
    println!("Enter your favorite food");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read line") ;
    
    if food.trim() == "cookie" {
        println!("I like the {} too ", food);

    }else if  food.trim() == "milk" {
        println!("Oh I like milk  {} too", food);

    }else{
        println!("I dont ' like the {}", food);
    }

    // match food.trim() {
    //     "cookie" => println!("I like The {} too", food),
    //     "milk" => println!("Oh i like milk too {}", food),
    //     _ => println!("I dont ' like The {}", food),
    //  }
 }
