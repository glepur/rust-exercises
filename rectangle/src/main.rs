use std::io;

struct Rectangle {
  width: f32,
  height: f32,
  error: String,
}

impl Rectangle {
  fn area(&self) -> f32 {
    self.width * self.height
  }
  fn diagonal(&self) -> f32 {
    (self.width.powf(2.0) + self.height.powf(2.0)).sqrt()
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
      println!("Values have to be separated by comma.");
      continue;
    } else {
      let vec: Vec<&str> = input.trim().split(",").collect();
      let mut error = String::new();
      Rectangle {
        width: match vec[0].parse() {
          Ok(num) => num,
          Err(_) => {
            error.push_str("Cannot parse width. ");
            0.0
          },
        },
        height: match vec[1].parse() {
          Ok(num) => num,
          Err(_) => {
            error.push_str("Cannot parse height. ");
            0.0
          },
        },
        error,
      }
    };
    if rectangle.error.len() > 0 {
      println!("{}", rectangle.error)
    } else {
      println!("Area is {:.2}, diagonal is {:.2}", rectangle.area(), rectangle.diagonal());
    }
  }
}
