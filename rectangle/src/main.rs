use std::io;

struct Rectangle {
  width: u32,
  height: u32,
  error: String,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn diagonal(&self) -> f32 {
    ((self.width.pow(2) + self.height.pow(2)) as f32).sqrt()
  }
}

fn main() {
  loop {
    println!("Enter width and height of rectangle separated by comma:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
    let rectangle = if input.trim() == "exit" {
      break;
    } else if !input.trim().contains(",") {
      Rectangle {
        width: 0,
        height: 0,
        error: "Values have to be separated by comma.".to_string(),
      }
    } else {
      let vec: Vec<&str> = input.trim().split(",").collect();
      Rectangle {
        width: vec[0].parse().expect("Cannot parse width"),
        height: vec[1].parse().expect("Cannot parse height"),
        error: "".to_string(),
      }
    };
    if rectangle.error.len() > 0 {
      println!("{}", rectangle.error)
    } else {
      println!("Area is {}, diagonal is {:.2}", rectangle.area(), rectangle.diagonal());
    }
  }
}
