#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
  simple_logger::init().unwrap();

  // if else
  let x = 5;
  let y = if x == 5 { 10 } else { 15 };
  info!("y is : {}", y);

  // switch case
  match x {
    1 => {
      info!("one");
    },
    2 => info!("two"),
    5 => info!("five"),
    _ => info!("something else"),
  }

  // for
  for x in 0..10 {
    info!("{}", x);
  }

  // for enumerate
  for (i,j) in (5..10).enumerate() {
    info!("i is: {}, j is: {}", i, j);
  }

  // while
  test_while();

  // loop
  test_loop();
}

fn test_while() {
  let mut x = 5;
  let mut done = false;

  while !done {
    x += x - 3;
    info!("x is: {}", x);

    if x % 5 == 0 {
      done = true;
    }
  }
}

fn test_loop() {
  let mut x = 5;
  loop {
    x += x-3;
    info!("x is: {}", x);
    if x % 5 == 0 {
      break;
    }
  }
}
