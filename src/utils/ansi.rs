/*
 * println a string with blue color
 */
#[macro_export]
macro_rules! log {
  ($s:literal) => {
    println!("{}", ansi_term::Color::Blue.paint($s));
  };
}
