use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number! ");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret_number is {}", secret_number);

    // 循环获取输入并比较，如果相同就break。
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess ).expect("Failed to read line");
        
        // 获取输入并转化为数字，如果不是数字就进行提示并继续。
        let guess: u32 = match guess.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("input number please ");
                continue;
            },
        };
    
        println!("you guess: { }", guess);

        // 比较 guess 和 secret_number
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("ok");
                break;
            }
        };

    }
    
}



