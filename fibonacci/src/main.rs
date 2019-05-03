use std::io;

fn main() {
  loop {
    println!("Input term in Fibonacci sequence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
    let output = if input.trim() == "exit" {
      break;
    } else {
      let number: i32 = input.trim().parse()
        .expect("Cannot parse string!");
      if number < 0 {
        println!("{} is negative!", number);
        continue;
      } else if number == 0 {
        println!("zero is not a right argument to fibonacci!");
        continue;
      }

      if number == 1 {
        1
      } else {
        let mut sum: isize = 0;
        let mut last: isize = 0;
        let mut curr: isize = 1;
        for _i in 1..number {
          sum = last + curr;
          last = curr;
          curr = sum;
        }
        sum
      }
    };
    println!("Result for provided term is: {}", output);
  }
}
