1. Install AssemblyScript compiler

```
npm install -g assemblyscript
```

2. Compile your AssemblyScript code

In `zkWasm-AssemblyScript-Demo`, run:
```
asc src/add.ts -O --noAssert -o add.wasm
asc src/equal.ts -O --noAssert -o equal.wasm
asc src/ticTacToe/zk_main.ts -O --noAssert -o ticTacToe.wasm --disable bulk-memory  --runtime stub --use abort=src/ticTacToe/zk_main/abort
```

3. Fetch and Compile zkWasm

In `zkWasm-AssemblyScript-Demo`, run:
```
git clone git@github.com:DelphinusLab/zkWasm.git
cd zkWasm
git submodule update --init
cargo build --release
```

4. Setup, Create proof and Verify proof

For add:
```
cd zkWasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm setup --host standard -k 18 --wasm ../add.wasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm prove --output ./output --ctxout ctxout --wasm ../add.wasm --public 3:i64 --private 1:i64 --private 2:i64
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm verify --output ./output
```

For equal:
```
cd zkWasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm setup --host standard -k 18 --wasm ../equal.wasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm prove --output ./output --ctxout ctxout --wasm ../equal.wasm --public 0:i64 --private 0:i64
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm verify --output ./output
```

For ticTacToe:
```
cd zkWasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm setup --host standard -k 18 --wasm ../ticTacToe.wasm
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm prove --output ./output --ctxout ctxout --wasm ../ticTacToe.wasm --public 1:i64,0:i64 --private 0:i64
RUST_LOG=info cargo run --release --features cuda -- --params ./params testwasm verify --output ./output
```