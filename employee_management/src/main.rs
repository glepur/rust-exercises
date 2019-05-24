use std::{
  collections::{hash_map::Entry, HashMap},
  io,
};

fn main() {
  let mut departments: HashMap<String, Vec<String>> = HashMap::new();
  loop {
    println!("What do you want to do with employees:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    } else {
      let mut words = input.trim().split_whitespace();
      let command = match words.next() {
        Some(val) => val,
        None => {
          println!("Input is not proper command");
          continue;
        }
      };
      if command == "add" || command == "Add" {
        let employee = match words.next() {
          Some(val) => val,
          None => {
            println!("You must provide employee name after add command");
            continue;
          }
        };
        let to = words
          .next()
          .unwrap_or("You must specify where to add employee");
        if to != "to" {
          println!("{}", to);
          continue;
        }
        let department = match words.next() {
          Some(val) => val,
          None => {
            println!("You must specify department");
            continue;
          }
        };
        match departments.entry(String::from(department)) {
          Entry::Vacant(e) => {
            e.insert(vec![String::from(employee)]);
          }
          Entry::Occupied(mut e) => {
            e.get_mut().push(String::from(employee));
          }
        }
        println!("Added {} to {}", employee, department);
      }
    };
  }
  println!("{:?}", departments)
}
