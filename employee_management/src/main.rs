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
          println!("Please enter a valid command");
          continue;
        }
      };
      let words_vec = words.collect::<Vec<&str>>();

      if command == "add" || command == "Add" {
        add_employee(&mut departments, words_vec);
      } else if command == "list" || command == "List" {
        list(&departments, words_vec);
      } else {
        println!("Command not recognized")
      }
    };
  }
  println!("{:?}", departments)
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, words: Vec<&str>) {
  if words.len() != 3 {
    println!("Wrong number of arguments supplied");
    return;
  }
  if words[1] != "to" {
    println!("You must specify where to add employee");
    return;
  }
  let employee = String::from(words[0]);
  let department = String::from(words[2]);
  match departments.entry(department) {
    Entry::Vacant(e) => {
      e.insert(vec![employee]);
    }
    Entry::Occupied(mut e) => {
      let vec = e.get_mut();
      if vec.contains(&employee) {
        println!("Employee {} already exists in {}", words[0], words[2]);
        return;
      }
      vec.push(employee);
      vec.sort();
    }
  }
  println!("Added {} to {}", words[0], words[2]);
}

fn list(departments: &HashMap<String, Vec<String>>, words: Vec<&str>) {
  let wc = words.len();
  if wc != 1 && wc != 3 {
    println!("Invalid number of arguments supplied");
  } else if wc == 1 && words[0] == "departments" {
    println!(
      "Here is the list of all departments:\n{}",
      list_departments(departments)
    );
  } else if words[0] == "employees" {
    if wc == 1 {
      let mut all_employees = Vec::new();
      for department in departments {
        all_employees.extend(department.1.iter().cloned());
      }
      all_employees.sort();
      println!(
        "Here is the list of every single employee:\n{}",
        list_employees(all_employees)
      );
    } else {
      // TODO: list employees per department
    }
  } else {
    println!("Could not parse command");
  }
}

fn list_departments(departments: &HashMap<String, Vec<String>>) -> String {
  let mut vec = departments.keys().map(|s| &**s).collect::<Vec<_>>();
  vec.sort();
  return format!("{}", vec.join("\n"));
}

fn list_employees(employees: Vec<String>) -> String {
  return format!("{}", employees.join("\n"));
}