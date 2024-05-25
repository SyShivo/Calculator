use std:: {io};
fn main() { 
    println!("Welcom to Calculator");
    loop { 
        let result;
        println!("Select 1 number");
        let x = input();
        println!("Select 2 number");
        let y = input();
        println!("Select an action addition = 1, subtraction = 2 , division = 3, multiplication =4, Exit = 5");
        let selected = input();
        match selected{
            1=> {
                result = x+y;
                println!("{x} + {y} = {result}")
            }
            2=>{      
                result = x-y;
                println!("{x} - {y} = {result}")   
            }
            3=>{  
                result = x/y;
                println!("{x} / {y} = {result}")       
            }
            4=>{    
                result = x*y;
                println!("{x} * {y} = {result}")     
            }
            5=>{  
                break;       
            }
            _=>println!("Choise another"),
        }
    }
}
fn input() -> u32{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return 10;
        }
    };
    return guess;
}