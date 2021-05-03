//! Expensive debugging utility functions for Windows.
//!
//! This crate is meant to be used in development versions of applications to
//! assist in debugging, and should normally not be included in release builds
//! used for production.
//!
//!
//! # Bloated example
//! The following is a little bit of a contrived example usage of
//! `dbgtools-win` which changes plenty of defaults just to show what options
//! are available.
//!
//! ```no_run
//! use std::path::PathBuf;
//! use dbgtools_win::*;
//!
//! fn init() {
//!   let mut wp = WinPicnic::default();
//!
//!   // If a panic occurs, append its output to a log file.  Can be useful if
//!   // process does not have a console, like a service.  If a debugger is
//!   // attached the output can be sent to PanicOutput::Debugger instead.
//!   let plog = PathBuf::from("C:\\Temp\\panic.log");
//!   wp.output = PanicOutput::FileAppend(plog);
//!
//!   // If a panic occurs, and there's a debugger attached to the process,
//!   // then trigger a debug interrupt.
//!   wp.brk = true;
//!
//!   // If a panic occurs, play three short notes.  This can be useful to
//!   // indicate a problem when running in an environment which doesn't have a
//!   // console, like a service.
//!   wp.beep = true;
//!
//!   // Instruct panic hook to generate minidumps if a panic occurs.
//!   wp.mdump = Some({
//!     let mut md = minidump::DumpInfo::default();
//!
//!     // Generate a full memory dump
//!     md.mdtype = minidump::DumpType::FullMem;
//!
//!     // Write dumps to a specific, absolute, directory.  This is useful if
//!     // you don't know what directory your process will be in, or if the
//!     // process has write access in it, when the panic handler is run.
//!     let dumpdir = PathBuf::from("C:\\Temp");
//!     md.dumpsdir = Some(dumpdir);
//!
//!     // Set a hardcoded base file name for dumps.
//!     let basename = PathBuf::from("myapp");
//!     md.name = Some(basename);
//!
//!     // Don't overwrite existing dump file; instead attach a sequence
//!     // number to the base name and skip existing sequence numbers.
//!     md.seq = true;
//!
//!     md
//!   });
//!
//!   // Set custom panic handler
//!   set_panic_handler(wp);
//!
//!   // Wait for a debugger to attach.  Once a debugger attaches, trigger a
//!   // hard-coded debug breakpoint.  This can be useful when remote
//!   // debugging services which start early during boot.
//!   debugger::wait_for_then_break();
//! }
//! ```
//!
//! # Live debugging example
//! There's a special diminutive version of `set_panic_handler()` in
//! `debugger::set_panic_handler()` which has fewer options but is especially
//! suited to situations where a program is ~always run with a debugger
//! attached.
//!
//! ```no_run
//! use std::path::PathBuf;
//! use dbgtools_win::*;
//! use dbgtools_win::debugger::OnPanic;
//!
//! fn init() {
//!   // Set up a custom panic hook which will output the panic information and
//!   // backtrace to the debugger output terminal, and trigger an explicit
//!   // debug breakpoint
//!   debugger::set_panic_handler(OnPanic::Break);
//!
//!   // Wait for a debugger to attach and trigger an explicit code breakpoint
//!   // when that happens.
//!   debugger::wait_for_then_break();
//! }
//! ```

pub mod debugger;
pub mod err;
pub mod minidump;

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::panic;
use std::path::PathBuf;
use std::time;

use backtrace::Backtrace;

mod bindings {
  ::windows::include_bindings!();
}

use bindings::Windows::Win32::Debug::*;

pub use crate::err::Error;


/// Used to select where text printed in the [panic handler](set_panic_handler)
/// is output to.
pub enum PanicOutput {
  /// Don't print anything.  Use this if the application is running without a
  /// console (like services).
  None,

  /// Output to standard output.
  Stdout,

  /// Output to standard error.
  Stderr,

  /// Output to the debuggers output channel.  If no debugger is attached this
  /// will behave like the `PanicOutput::None`.
  Debugger,

  /// Append to a file.  Errors opening or writing to this file will be
  /// silently ignored.  The file will be created if it doesn't exist.  The
  /// caller must ensure that the filename can be accessed for writing.
  FileAppend(PathBuf)
}


/// Context used to control what happens on panic.
pub struct WinPicnic {
  pub output: PanicOutput,

  /// If this is `true`, and a debugger is present, the panic hook will call
  /// `DebugBreak()`.  This is done at the very end of the hook to allow all
  /// other processing to occurr before breaking.
  ///
  /// If it is `false` the hook will not cause a debug break.
  pub brk: bool,

  /// Beep on panic.
  pub beep: bool,

  /// Control whether the hook will generate a "minidump" or not.
  pub mdump: Option<minidump::DumpInfo>
}

impl Default for WinPicnic {
  fn default() -> Self {
    WinPicnic {
      output: PanicOutput::Stderr,
      brk: false,
      beep: false,
      mdump: None
    }
  }
}


/// Return the version of the crate.
pub fn version() -> &'static str {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  VERSION
}


/// Beep.
///
/// Beep beep, Richie.
///
/// This function is mighty annoying.
pub fn beep(freq: u32, dur: time::Duration) {
  let mut ms = dur.as_millis();
  if ms > 0xffffffff {
    // If you're trying to beep for this long you should be in jail.
    ms = 0xffffffff;
  }

  unsafe { Beep(freq, ms as u32) };
}


/// Set up a panic hook which supports Windows specific features.
///
/// The [`WinPicnic`] context buffer can be used to configure:
/// - Where to output panic information and backtrace.
/// - If a minidump should be written.
/// - Optionally beep
/// - Trigger a hard-coded debug breakpoint (if a debugger is connected).
///
/// # Notes
/// - If a minidump is requested, it will be created on a separate thread (the
///   minidump writing function can't accurately produce the callstack for its
///   calling thread).  To hide this bureaucracy one can filter out the thread
///   id of the thread calling `MiniDumpWriteDump()` from the minidump output.
///   This is currently not implemented.
pub fn set_panic_handler(picnic: WinPicnic) {
  panic::set_hook(Box::new(move |panic_info| {
    let bt = Backtrace::new();

    // Output panic information
    if let Some(location) = panic_info.location() {
      let buf = format!(
        "\npanic occurred on {} in file '{}' at line {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        location.file(),
        location.line(),
      );
      panic_output(&picnic.output, &buf);
    }

    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      let buf = format!("\n{}\n\n", s);
      panic_output(&picnic.output, &buf);
    }

    // Output backtrace
    let buf = format!("{:?}\n", bt);
    panic_output(&picnic.output, &buf);

    // MiniDumpWriteDump() may not produce a valid stack trace for the calling
    // thread.  We could either trigger an exception, catch it and pass the
    // exception information to MiniDumpWriteDump(), or run MiniDumpWriteDump()
    // on a separate thread.  We'll opt for the second option.  See the Remarks
    // section in https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/nf-minidumpapiset-minidumpwritedump#remarks
    if let Some(mdump) = &picnic.mdump {
      let mdump = mdump.clone();
      let thrd = std::thread::spawn(move || minidump::create(mdump));
      let _ = thrd.join();
    }

    // If the caller really wants it, then make a litte noise.  :(
    if picnic.beep {
      beep(740, time::Duration::from_millis(100));
      beep(1760, time::Duration::from_millis(100));
      beep(988, time::Duration::from_millis(100));
    }

    // If a debugger is present, and the caller requested to trigger a
    // hard-coded debug breakpoint, then do so.
    if debugger::is_present() {
      match picnic.brk {
        true => unsafe {
          DebugBreak();
        },
        _ => {}
      }
    }
  }));
}

fn panic_output(p: &PanicOutput, s: &str) {
  match p {
    PanicOutput::None => {}
    PanicOutput::Stdout => {
      let _ = io::stdout().write_all(s.as_bytes());
    }
    PanicOutput::Stderr => {
      let _ = io::stderr().write_all(s.as_bytes());
    }
    PanicOutput::Debugger => {
      if debugger::is_present() {
        debugger::output(s);
      }
    }
    PanicOutput::FileAppend(fname) => {
      if let Ok(mut f) =
        OpenOptions::new().append(true).create(true).open(fname)
      {
        let _ = f.write_all(s.as_bytes());
      }
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
