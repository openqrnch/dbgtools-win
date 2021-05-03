// Synopsis:
//   Minidumps generation can be configured to add a sequence number into their
//   file names.  Configure a context with no specific dumps directory or base
//   file name, and request sequence numbered dumps.
//
// Expected results:
//   First time the program is run a minidump_seq-0000.dmp will be created in
//   the current directory.  A subsequent run will generate a
//   minidump_seq-0001.dmp, and so on.  The number sequence number is a
//   four-digit hexadeximal number.  If one or more sequence numbers are
//   deleted any following runs will fill those gaps.  If sequence number ffff
//   is reached it will be overwritten by subsequent runs.

use dbgtools_win::minidump;

fn main() {
  let di = minidump::DumpInfo {
    seq: true,
    ..Default::default()
  };

  minidump::create(di).expect("Unable to create minidump");
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
