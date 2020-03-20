use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Input some number:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = guess_func(guess);

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too Big!"),
      Ordering::Equal => {
        println!("Correct!");
        break;
      },
    }
  }
}

fn guess_func (guess: String ) -> u32 {
  match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
  }
}
