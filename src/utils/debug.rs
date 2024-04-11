// If compatibility problem occurs, try to use ansi_term

/*
 * println a string with filename and line number
 * under blue color
 * $s: log message
 * $e: optional expression to print
 */
#[macro_export]
macro_rules! log {
  ($s:literal $(, $e:expr)*) => {
    let s = format!($s $(, $e)*);
    let l = std::panic::Location::caller();
    println!("\u{001b}[34m[{}:{}] {}\u{001b}[0m", l.file(), l.line(), s);
  };
}

/*
 * panic with a red message
 * $s: panic message
 * $e: optional expression to print
 */
#[macro_export]
macro_rules! crumble {
  ($s:literal $(, $e:expr)*) => {
    let s = format!($s $(, $e)*);
    panic!("\u{001b}[31m{}\u{001b}[0m", s);
  };
}

/*
 * println a error message with red color if expr failed
 * $e: expression
 * $s: error message
 * $x: optional expression to print
 */
#[macro_export]
macro_rules! alert {
  ($e:expr, $s:literal $(, $x:expr)*) => {
    if !$e {
      let s = format!($s $(, $x)*);
      println!("\u{001b}[31m{}\u{001b}[0m", s);
      assert!($e);
    }
  };
}
