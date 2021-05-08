# 基于 `rust` 编译出来的 `gcid` 文件计算

## 步骤

1. `wasm-pack build`
2. 在 `www` 目录下加载模块, `npm install --save file:../pkg`
3. 在 `www` 目录下, `npm run start`

## 相关命令

rustup target add wasm32-unknown-unknown

cargo install wasm-gc

cargo build --target wasm32-unknown-unknown --release

wasm-gc target/wasm32-unknown-unknown/release/crypto_gcid.wasm

使用 wasm-pack

wasm-pack build
wasm-pack build --release --target web
