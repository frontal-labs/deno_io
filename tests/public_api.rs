use std::io::Read;
use std::io::Write;

use deno_io::FdTable;
use deno_io::pipe;

#[test]
fn synchronous_pipe_round_trips_bytes() {
  let (mut reader, mut writer) = pipe().unwrap();
  writer.write_all(b"deno-io").unwrap();
  let mut output = [0; 7];
  reader.read_exact(&mut output).unwrap();
  assert_eq!(&output, b"deno-io");
}

#[test]
fn fd_table_tracks_uv_owned_handles_and_adoption() {
  let mut table = FdTable::new();
  assert_eq!(table.begin_uv_adopt(42), Some(false));
  assert!(table.register_uv_owned(42));
  assert!(!table.register_uv_owned(42));
  assert!(table.contains(42));
  assert_eq!(table.begin_uv_adopt(42), None);
  assert_eq!(table.remove(42), None);
  assert!(!table.contains(42));
}
