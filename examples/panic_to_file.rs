// Synopsis:
//   The panic handler can append the panic information and backtrace to a
//   file.  Set up the panic handler that appends the panic information and
//   backtrace to a file called "panic.log" and then trigger a panic.
//
// Expected results:
//   The panic information and backtrace will be appended to the file
//   panic.log (the file is created if needed).

use std::path::PathBuf;

use dbgtools_win::*;

fn main() {
  set_panic_handler(WinPicnic {
    output: PanicOutput::FileAppend(PathBuf::from("panic.log")),
    ..Default::default()
  });

  panic!("Don't panic.");
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
