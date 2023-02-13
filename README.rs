1. Install AssemblyScript compiler

```
npm install -g assemblyscript
```

2. Compile your AssemblyScript code

```
asc src/add.ts -O --noAssert -o demo.wasm
```

3. Fetch and Compile zkWasm

```
git clone git@github.com:DelphinusLab/zkWasm.git
cd zkWasm
git submodule update --init
cargo build --release
```

4. Setup, Create proof and Verify proof

```
cd zkWasm
RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm ../demo.wasm setup
RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm ../demo.wasm single-prove --public 3:i64 --private 1:i64 --private 2:i64
RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm ../demo.wasm single-verify --public 3:i64 --proof output/zkwasm.0.transcript.data
```
