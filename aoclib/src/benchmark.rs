
// Adapted from github:thepacketgeek/rust-macros-demo.git
macro_rules! elapsed {
    // Attempt to match function name & args
    // ```ignore
    // elapsed!(my_func(a,b));
    // ```
    // > 'my_func' took 2000 ms
    ($f:ident ( $($args:expr),*)) => {{
      let _start = std::time::Instant::now();
      let _res = $f($($args,)*);
      // Use the function name (ident) in the log
      println!("'{}' took {:.3} us", stringify!($f), _start.elapsed().as_micros());
      _res
    }};

    // Otherwise take a function by name:
    // ```ignore
    // elapsed!(my_func);
    // ```
    // > Took 2000 us
    ($e:expr) => {{
      let _start = std::time::Instant::now();
      let _res = $e();
      println!("Took {:.3} us", _start.elapsed().as_micros());
      _res
    }};
}
pub(crate) use elapsed;

pub fn thetest() {
	println!("This function exists.");
}
