fn main() {
  let number_names = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelveth",
  ];
  let verses = [
    "12 drummers drumming",
    "11 pipers piping",
    "10 lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three french hens",
    "Two turtle doves, and",
    "A partridge in a pear tree",
  ];
  for num in 0..12 {
    println!("On the {} day of Christmas my true love sent to me", number_names[num]);
    for verse_num in (11-num)..12 {
      println!("{}", verses[verse_num]);
    }
    println!("\n");
  }
}

