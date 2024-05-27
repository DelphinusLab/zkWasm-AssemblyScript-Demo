import * as ticTakToe from "./tic_tak_toe"

@external("env", "wasm_input")
declare function wasm_input(x: i32): i64

@external("env", "require")
declare function require(x: i32): void

export function abort(message: string | null, fileName: string | null, lineNumber: u32, columnNumber: u32): void {
  let a = 0;
  a++;
}

export function zkmain(): void {
  var arra: i64[] = [];
  var arrb: i64[] = [];

  let counta = wasm_input(0);
  for (var i = 0; i < counta; i++) {
    let v = wasm_input(0);
    arra.push(v);
  }

  let countb = wasm_input(1);
  for (var i2 = 0; i2 < countb; i2++) {
    let v = wasm_input(1);
    arrb.push(v);
  }

  var arrtestb = ticTakToe.ticTacToe(arra);

  require(arrtestb.length == arrb.length)

  for (var j = 0; j < arrb.length; j++) {
    require(arrtestb[j] == arrb[j])
  }
}
