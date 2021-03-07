use std::panic;
use std::thread;
use std::time;

use backtrace::Backtrace;

use widestring::U16CString;

mod bindings {
  ::windows::include_bindings!();
}

use bindings::windows::win32::debug::{
  DebugBreak, IsDebuggerPresent, OutputDebugStringW
};

pub enum OnPanic {
  NoBreak,
  Break
}

/// Return the version of the crate.
pub fn version() -> &'static str {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  VERSION
}


/// Returns `true` if a debugger is attached to the process.  Returns `false`
/// otherwise.
pub fn is_debugger_present() -> bool {
  unsafe { IsDebuggerPresent() }.as_bool()
}


/// Enter a loop which waits for a debugger to attach to the process.
/// Once a debugger is detected, return to caller.
pub fn wait_for_debugger() {
  // Checking every 100ms is probably a little excessive.
  while !is_debugger_present() {
    thread::sleep(time::Duration::from_millis(100));
  }
}


/// Enter a loop which waits for a debugger to attach to the process.
/// Once a debugger is detected, trigger a hard-coded breakpoint.
pub fn wait_for_debugger_break() {
  wait_for_debugger();
  unsafe {
    DebugBreak();
  }
}

pub fn debugger_output<S: AsRef<str>>(s: S) {
  let wstr = match U16CString::from_str(s) {
    Ok(s) => s,
    Err(_) => U16CString::from_str("<bad encoding>").unwrap()
  };

  let raw = wstr.into_raw();
  unsafe { OutputDebugStringW(raw) };

  // for resource tracking
  unsafe { U16CString::from_raw(raw) };
}


/// Set up a panic hook that will write the backtrace to the Windows debugger
/// output mechanism if a debugger is attached.
///
/// If the argument `brk` is set to ``OnPanic::Break` the process will trigger
/// an explicit breakpoint after the backtrace has been dumped.
pub fn set_debug_output_panic_handler(brk: OnPanic) {
  panic::set_hook(Box::new(move |panic_info| {
    if is_debugger_present() {
      let bt = Backtrace::new();

      if let Some(location) = panic_info.location() {
        let buf = format!(
          "panic occurred in file '{}' at line {}\n",
          location.file(),
          location.line(),
        );
        debugger_output(&buf);
      }

      if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        debugger_output(s);
        debugger_output("\n");
      }

      let s = format!("{:?}\n", bt);
      debugger_output(s);

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
