// Synopsis:
//   Set up the panic handler that writes the backtrace to the debug output
//   using OutputDebugStringW(), wait for a debugger to attach and then
//   voluntarily panic.
//
// Expected results:
//   The backtrace will be sent to the debuggers debug output terminal.

use dbgtools_win::{
  debugger_output, set_debug_output_panic_handler, wait_for_debugger_break,
  OnPanic
};

fn main() {
  println!("Set up debug output panic handler ..");
  // Do not trigger a breakpoint in the panic handler
  set_debug_output_panic_handler(OnPanic::NoBreak);

  println!("Waiting for a debugger to attach ..");
  wait_for_debugger_break();

  debugger_output("Hello, debugger!\n");

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
  panic!(msg);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :