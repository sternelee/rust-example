import * as wasm from "crypto-gcid";

console.log(wasm);
const { greet, calc_block_size, calculate, Gcid } = wasm
const request = async function (url) {
  return new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.onload = () => {
      resolve(request.response)
    }
    request.open('GET', url)
    request.responseType = 'arraybuffer'
    request.send()
  })
}
greet("sternelee");
// console.log(calc_block_size(2921921292111))

async function main () {
  const buffers = await request('/720P.mp4')
  const segment = new Uint8Array(buffers);
  console.log('buffers', segment)
  const gcid = Gcid.new(segment.byteLength);
  console.log(gcid.block_size())
  gcid.calculate(segment);
  const result = gcid.finalize();
  console.log(result);
  gcid.free()
  /* const data = calculate(segment)
  console.log(data) */
}

window.onload = main()
