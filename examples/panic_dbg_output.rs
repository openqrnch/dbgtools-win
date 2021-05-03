// Synopsis:
//   Set up the panic handler that writes the backtrace to the debug output
//   using OutputDebugStringW(), wait for a debugger to attach and then
//   voluntarily panic.
//
// Expected results:
//   The backtrace will be sent to the debuggers debug output terminal.

use dbgtools_win::debugger::{self, OnPanic};

fn main() {
  println!("Set up debug output panic handler ..");
  // Do not trigger a breakpoint in the panic handler
  debugger::set_panic_handler(OnPanic::NoBreak);

  println!("Waiting for a debugger to attach ..");
  debugger::wait_for_then_break();

  debugger::output("Hello, debugger!\n");

  println!("Presumably a debugger attached -- trigger a panic");
  a_function("panic message");
}

fn a_function(msg: &'static str) {
  another_function(msg);
}

fn another_function(msg: &'static str) {
  final_function(msg);
}

fn final_function(msg: &'static str) {
  panic!("{}", msg);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
