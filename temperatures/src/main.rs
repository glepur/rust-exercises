use std::io;

fn main() {
  loop {
    println!("Input temperature in Fahrenheit or Celsius:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    } else if !input.trim().ends_with("°C") && !input.trim().ends_with("°F") {
      println!("Input must end with °C or °F")
    } else {
      let mut number_str = input.trim().to_string();
      number_str.truncate(number_str.chars().count() - 2);
      let number: f32 = match number_str.parse() {
        Ok(n) => n,
        Err(_) => {
          println!("Cannot parse string!");
          continue;
        }
      };
      if input.trim().ends_with("°C") {
        println!("{:.2}°F", number * 1.8 + 32.0);
      } else {
        println!("{:.2}°C", (number - 32.0) / 1.8);
      }
    }
  }
}
