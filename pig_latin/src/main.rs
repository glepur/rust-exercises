use std::io;

fn main() {
  loop {
    println!("Enter some text:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    } else {
      let words = input.trim().split_whitespace();
      let mut pig_latin: Vec<String> = Vec::new();
      for word in words {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if "aeiouAEIOU".contains(first_char) {
          pig_latin.push(format!("{}-hay", word));
        } else {
          pig_latin.push(format!("{}-{}ay", chars.collect::<String>(), first_char))
        }
      }
      println!("{}", pig_latin.join(" "));
    };
  }
}
