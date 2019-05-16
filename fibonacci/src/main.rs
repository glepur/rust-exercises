use std::{collections::HashMap, io};

fn main() {
  loop {
    println!("Input term in Fibonacci sequence:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    } else {
      let number: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
          println!("Cannot parse string!");
          continue;
        }
      };
      if number < 0 {
        println!("{} is negative!", number);
        continue;
      } else if number == 0 {
        println!("zero is not a right argument to fibonacci!");
        continue;
      }

      let mut memo: HashMap<i32, usize> = HashMap::new();
      println!(
        "Result for provided term is: {}",
        fibonacci(number, &mut memo)
      );
    };
  }
}

fn fibonacci(num: i32, memo: &mut HashMap<i32, usize>) -> usize {
  if num <= 1 {
    1
  } else if memo.contains_key(&num) {
    memo[&num]
  } else {
    let result = fibonacci(num - 1, memo) + fibonacci(num - 2, memo);
    memo.insert(num, result);
    result
  }
}
