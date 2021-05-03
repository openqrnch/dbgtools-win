fn main() {
  windows::build!(
    Windows::Win32::Debug::{IsDebuggerPresent, DebugBreak, OutputDebugStringW,
      MiniDumpWriteDump, Beep},
    Windows::Win32::FileSystem::CreateFileW,
    Windows::Win32::SystemServices::{GetCurrentProcess},
    Windows::Win32::WindowsProgramming::CloseHandle
  );
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
