// If compatibility problem occurs, try to use ansi_term

/*
 * println a string with blue color
 */
#[macro_export]
macro_rules! Log {
  ($s:literal) => {
    println!("\u{001b}[34m{}\u{001b}[0m", $s);
  };
}

/*
 * println a error message with red color if expr failed
 * $e: expression
 * $s: error message
 */
#[macro_export]
macro_rules! Assert {
  ($e:expr, $s:literal) => {
    if !$e {
      println!("\u{001b}[31m{}\u{001b}[0m", $s);
      assert!($e);
    }
  };
}
