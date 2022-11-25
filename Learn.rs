

// > rustc main.rs
// > .\main.exe

use std::cmp::Ordering;
use std::io;




struct Employee {
    name:String,
    company:String,
    age:u32
 }


fn main() {
    // Declare a variable binding
    let string_type = "hello";   // string type
    let char_type = '8';       // character type
    let int:u8 = 5;         // integer type
    let float_type =12.12;  // float type 
    let bool_type = true; // bool type 
    println!("int type - {}\nchar type -  {}\nstring type - {} \nfloat type - {}\nbool - type {} ", int, char_type, string_type, float_type , bool_type);

    

    let mut new_int = 20; // This var can be change 
    println!("Value before the change: {} ", new_int);
    new_int = 10; // only var's that have mut can be changed
    println!("Value after the change: {} ", new_int);
    const LETTER:char = 'A'; // CONST 
    println!("const var - {}", LETTER);

    if int % 2 ==0 {
        println!("int is EVEN ");

    }else{
        println!("int is ODD" );

    }

    while new_int < 20{

        new_int += 1;

    }
    println!("The While loop is over! ");

    for x in 1..5 {
        println!("X: {}" , x);
    }

    
    let mut var = plus_one(5);

    println!("The value of x is: {var}");
    var = minus_one(var);
    println!("The value of x is: {var} after minus_one");

    let mut employee1 = Employee {
        company:String::from("Big company"),
        name:String::from("Bill"),
        age:70
     };
    let employee3 = Employee{
    company:String::from("Small company "),
    name:String::from("joe"),
    age:21
    };

    println!("Name is :{} company is {} age is 
   {}",employee1.name,employee1.company,employee1.age);
   println!("Name is :{} company is {} age is 
   {}",employee3.name,employee3.company,employee3.age);


    println!("Enter a name:");
    let mut word = String::new();
 
    io::stdin().read_line(&mut word).expect("failed to readline");
 
    print!("You entered {}", word);

    print_output("hello");

    emp_chage(&mut employee1);
    println!("Name is :{} company is {} age is 
    {}",employee1.name,employee1.company,employee1.age);

    println!("Guess the number!");

    let secret_number = 78;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn minus_one(x: i32) -> i32 {
    x-1 
}

fn print_output(x: &str ) {

    println!("here is the printed line {x} ");
}

fn emp_chage(emp: &mut Employee) {

    println!("ENTER NEW NAME FOR EMPLOYEE 1: ");
    let mut word = String::new();
 
    io::stdin().read_line(&mut word).expect("failed to readline");
    emp.name = word.to_string();
}
