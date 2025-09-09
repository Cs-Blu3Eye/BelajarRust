pub fn belajar_opration() {
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);
}

pub fn belajar_if_else() {
    if 7 > 5 {
        println!("7 is greater than 5.");
    }
}

// match merupakan pengganti switch case di bahasa" lain
pub fn belajar_match() {
  let day = 6;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}
