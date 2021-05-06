use sha1::{Digest, Sha1};
use std::cmp;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8array(a: &[u8]);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn calc_block_size (filesize: usize) -> usize {
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

// #[wasm_bindgen]
// pub fn str2buffer (str: String) {
//   return str.as_bytes();
// }

#[wasm_bindgen]
pub fn calculate(buffer: &[u8]) -> String {
    // 不支持传送arraybuffer: https://github.com/rustwasm/wasm-bindgen/issues/1961
    // let buffer = buf.as_bytes();
    log_u8array(&buffer);
    let filesize = buffer.len();
    let mut block_size = calc_block_size(filesize);
    block_size = cmp::min(filesize, block_size);
    let mut hasher = Sha1::new();
    let mut count = 1;
    log_u32(filesize);
    log_u32(block_size);
    let result: String = loop {
      if count > filesize {
        break format!("{:X}", hasher.finalize());
      } else {
          match count % block_size {
            0 => {
              let n = count / block_size;
              let start = block_size * (n - 1);
              let end = block_size * n;
              hasher.update(&buffer[start..end]);
              count += 1;
              log_u32(count);
            },
            _ => count += 1,
          }
      }
    };

  log(&result);
  return result;
}
