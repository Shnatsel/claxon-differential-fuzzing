#[macro_use]
extern crate afl;
extern crate claxon;
extern crate itertools;

use std::io::Cursor;
use itertools::Itertools;

// Use system allocator so we can substitute it with a custom one via LD_PRELOAD
use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

fn main() {
    fuzz!(|data: &[u8]| {
        // Decode the data from fuzzer twice
        let cursor1 = Cursor::new(data);
        let mut reader1 = match claxon::FlacReader::new(cursor1) {
            Ok(r) => r,
            Err(..) => return,
        };

        let cursor2 = Cursor::new(data);
        let mut reader2 = match claxon::FlacReader::new(cursor2) {
            Ok(r) => r,
            // if decoding succeeded the first time, it should succeed always
            Err(..) => unreachable!(),
        };

        // Check that tags have been decoded identically
        assert!(reader1.tags().eq(reader2.tags()));

        // Check that audio data has been decoded identically
        for same_sample_decoded_twice in reader1.samples().zip_eq(reader2.samples()) {
            match same_sample_decoded_twice {
                (Ok(content1), Ok(content2)) => {assert_eq!(content1, content2);},
                (Err(..), Err(..)) => break,
                _ => unreachable!()
            }
        }
    });
}
