// https://rust-guide.budshome.com//9-cryptography/9.1-hashing/9.1.1-sha-digest.html
use ring::digest::{Context, Digest, SHA256};

// fn create() {
//     let mut context = Context::new(&SHA256);
// }

fn calculate(buffers: Vec<u8>, len: usize) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut count: usize = 1;
    loop {
        if count > buffers.len() {
            break
        } else {
            match count % len {
                0 => {
                    let n = count / len;
                    let start = len * (n - 1);
                    let end = len * n;
                    context.update(&buffers[start..end]);
                },
                _ => count += 1,
            }
        }
    }
    context.finish()
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
