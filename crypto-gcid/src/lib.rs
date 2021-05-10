use sha1::{Digest, Sha1};
use std::cmp;
// use lazy_static::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
/* lazy_static! {
    static mut CONTEXT: sha1::Sha1 = Sha1::new();
} */

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

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
pub fn calc_block_size(filesize: usize) -> usize {
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

#[wasm_bindgen]
pub fn calculate(buffer: &[u8]) -> String {
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
                    log_u32(count);
                    let n = count / block_size;
                    let start = block_size * (n - 1);
                    let end = block_size * n;
                    hasher.update(&buffer[start..end]);
                    count += 1;
                }
                _ => count += 1,
            }
        }
    };

    log(&result);
    return result;
}

// #[derive(Copy, Clone)]

#[wasm_bindgen]
pub struct Gcid {
    context: sha1::Sha1,
    len: usize,
    block_size: usize,
}

#[wasm_bindgen]
impl Gcid {
    pub fn new (len: usize) -> Gcid {
        Gcid {
            context: Sha1::new(),
            len,
            block_size: calc_block_size(len)
        }
    }
    pub fn block_size (&mut self) -> usize {
        self.block_size
    }
    pub fn calculate(&mut self, buffer: &[u8]) -> String {
        log_u8array(&buffer);
        let filesize = buffer.len();
        let block_size = cmp::min(self.block_size, filesize);
        let mut count = 1;
        loop {
            if count > filesize {
                break String::from("break");
            } else {
                match count % block_size {
                    0 => {
                        let n = count / block_size;
                        let start = block_size * (n - 1);
                        let end = block_size * n;
                        self.context.update(&buffer[start..end]);
                        count += 1;
                        log_u32(count);
                    },
                    _ => count += 1,
                }
            }
        };
        return String::from("done");
    }
    pub fn finalize(&mut self) -> String {
        let result = format!("{:X}", self.context.clone().finalize());
        return result;
    }
}
