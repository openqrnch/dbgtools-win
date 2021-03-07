fn main() {
  windows::build!(
    windows::win32::debug::{IsDebuggerPresent, DebugBreak, OutputDebugStringW}
  );
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
