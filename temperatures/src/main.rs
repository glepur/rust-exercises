use std::io;

fn main() {
  loop {
    println!("Input temperature in Fahrenheit or Celsius:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
    let output = if input.trim() == "exit" {
      break;
    } else if !input.trim().ends_with("°C") && !input.trim().ends_with("°F") {
      "Input must end with °C or °F".to_string()
    } else {
      let mut number_str = input.trim().to_string();
      number_str.truncate(number_str.chars().count() - 2);
      let number: f32 = number_str.parse()
        .expect("Cannot parse string!");
      if input.trim().ends_with("°C") {
        round(number * 1.8 + 32.0).to_string() + "°F"
      } else {
        round((number - 32.0) / 1.8).to_string() + "°C"
      }
    };
    println!("{}", output);
  }
}

fn round (number: f32) -> f32 {
  (number * 100.0).round() / 100.0
}
