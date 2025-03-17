// 사용자가 추리한 값을 입력받아 그대로 출력하는 코드
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  

    loop {
    println!("Please input your guess.");

    let mut guess = String::new(); // let mut : mutable 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    // parse() 메서드는 Result 타입을 반환
    // Result는 OK, Err 배리언트를 가진 열거형
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
