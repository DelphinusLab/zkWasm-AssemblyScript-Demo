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
RUST_LOG=info cargo run --release -- --params params testwasm setup --host standard -k 18 --wasm ../demo.wasm
RUST_LOG=info cargo run --release -- --params ./params testwasm prove --output ./output --wasm ../demo.wasm --public 3:i64 --private 1:i64 --private 2:i64
RUST_LOG=info cargo run --release -- --params ./params testwasm verify --output ./output
```
