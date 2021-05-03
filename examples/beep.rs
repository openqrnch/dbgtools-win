// Beeps.  :(

use std::time::Duration;

fn main() {
  dbgtools_win::beep(740, Duration::from_millis(100));
  dbgtools_win::beep(1760, Duration::from_millis(100));
  dbgtools_win::beep(988, Duration::from_millis(100));
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
