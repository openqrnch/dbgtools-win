//! Utility functions for interacting with a debugging session.

use std::panic;
use std::thread;
use std::time;

use backtrace::Backtrace;

use crate::bindings::Windows::Win32::Debug::*;


pub enum OnPanic {
  Break,
  NoBreak
}


/// Returns `true` if a debugger is attached to the process.  Returns `false`
/// otherwise.
pub fn is_present() -> bool {
  unsafe { IsDebuggerPresent() }.as_bool()
}


/// Enter a loop which waits for a debugger to attach to the process.
/// Once a debugger is detected, return to caller.
pub fn wait_for() {
  // Checking every 100ms is excessive, but we don't want the user to notice a
  // delay.
  while !is_present() {
    thread::sleep(time::Duration::from_millis(100));
  }
}


/// Convenience function which enters a loop that waits for a debugger to
/// attach to the process and trigger a hard-coded debug breakpoint once this
/// happens.
///
/// Useful to add early in programs where the startup needs to be remotely
/// debugged and the user wants to be given the opportunity to configure
/// breakpoints before continuing execution of the startup code.
pub fn wait_for_then_break() {
  wait_for();
  unsafe {
    DebugBreak();
  }
}


/// Write a string to a debugger's output terminal.
///
/// This function only serves a purpose if a debugger is attached.  To detect
/// whether this is true, an application can call [`is_present()`].
pub fn output<S: AsRef<str>>(s: S) {
  unsafe { OutputDebugStringW(s.as_ref()) };
}


/// Set up a panic hook that will write the panic message and backtrace to the
/// Windows debugger output mechanism if a debugger is attached.
///
/// If the argument `brk` is set `true` the process will trigger an explicit
/// breakpoint after the backtrace has been dumped.
///
/// This function is a diminutive version of [`crate::set_panic_handler()`]
/// which exists for convenience when a debugger is intended to ~always be
/// attached to the process.
pub fn set_panic_handler(brk: OnPanic) {
  panic::set_hook(Box::new(move |panic_info| {
    if is_present() {
      let bt = Backtrace::new();

      if let Some(location) = panic_info.location() {
        let buf = format!(
          "panic occurred in file '{}' at line {}\n",
          location.file(),
          location.line(),
        );
        output(&buf);
      }

      if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        output(s);
        output("\n");
      }

      let s = format!("{:?}\n", bt);
      output(s);

      match brk {
        OnPanic::Break => unsafe {
          DebugBreak();
        },
        _ => {}
      }
    }
  }));
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
