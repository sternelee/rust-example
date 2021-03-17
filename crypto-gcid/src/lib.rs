// https://rust-guide.budshome.com//9-cryptography/9.1-hashing/9.1.1-sha-digest.html
use ring::digest::{Context, Digest, SHA256};

fn sha256_digest(mut buffers: Vec[u32], len: i32) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    let mut count = 1;
    loop {
        if count > buffer.len() {
            break
        } else {
            match count % len {
                0 => {
                    let n = count / len;
                    let start = len * (n - 1);
                    let end = len * n;
                    context.update(&buffer[..count]);
                },
                _ => count += 1,
            }
        }
    }
    Ok(context.finish())
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}