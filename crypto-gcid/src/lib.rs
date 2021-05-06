use sha1::{Digest, Sha1};
use std::cmp;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[no_mangle]
#[warn(unused_comparisons)]
pub extern "C" fn calc_block_size (filesize: usize) -> usize {
    if filesize >= 0 && filesize <= (128 << 20) {
        256 << 10
    } else if filesize > (128 << 20) && filesize <= (256 << 20) {
        512 << 10
    } else if filesize > (256 << 20) && filesize <= (512 << 20) {
        1024 << 10
    } else {
        2048 << 10
    }
}

#[no_mangle]
pub extern "C" fn calculate(buffers: &[u8]) -> String {
    let filesize = buffers.len();
    let mut block_size = calc_block_size(filesize);
    block_size = cmp::min(filesize, block_size);
    let mut hasher = Sha1::new();
    let mut count = 1;
    let result: String = loop {
      if count > filesize {
        break format!("{:X}", hasher.finalize());
      } else {
          match count % block_size {
            0 => {
              let n = count / block_size;
              let start = block_size * (n - 1);
              let end = block_size * n;
              hasher.update(&buffers[start..end]);
              count += 1;
            },
            _ => count += 1,
          }
      }
    };
  return result;
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

