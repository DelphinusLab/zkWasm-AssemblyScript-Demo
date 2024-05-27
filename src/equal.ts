@external("env", "wasm_input")
declare function wasm_input(x: i32): i64

@external("env", "require")
declare function require(x: i32): void

export function zkmain(): void {
  var a= wasm_input(0);
  var b= wasm_input(1);
  require(a == b)
}
