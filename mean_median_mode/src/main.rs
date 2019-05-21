use std::{collections::HashMap, io};

fn main() {
  loop {
    println!("Enter integer values separated by comma:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    } else if !input.trim().contains(",") {
      println!("Values have to be separated by comma.");
      continue;
    } else {
      let result: Result<Vec<u32>, _> = input.trim().split(",").map(|s| s.parse()).collect();
      let mut integers = match result {
        Ok(num) => num,
        Err(_) => {
          println!("Could not parse all values");
          continue;
        }
      };
      let sum = integers.iter().fold(0, |acc, x| acc + x);
      integers.sort();
      let mean: f32 = sum as f32 / integers.len() as f32;
      let median: &u32 = &integers[integers.len() / 2];
      let mut map: HashMap<u32, i8> = HashMap::new();
      for num in integers.iter() {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
      }
      let mut mode_key = match map.keys().next() {
        Some(key) => *key,
        None => {
          println!("Could not find key");
          continue;
        }
      };
      for (key, val) in &map {
        let mode_val = map[&mode_key];
        if val > &mode_val {
          mode_key = *key;
        }
      }
      let mode = map[&mode_key];

      println!(
        "Mean is: {:.2}\n Median is: {}\n Mode is: {}",
        mean, median, mode
      );
    };
  }
}
