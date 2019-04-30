#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
  simple_logger::init().unwrap();

  debug!("this is a debug {}", "message");
  error!("this is printed by default");

  let x = 3 * 4; // expensive computation
  info!("the answer was: {}", x);

  test_sl();

  info!("Hello World");
}

fn test_sl() {
  info!("this is a message!");
}
