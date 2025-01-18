# TSTZ: Compiler from TypeScript to MLIR

TSTZ is a compiler that translates TypeScript into MLIR written in the Michelson Dialect.
```sh
$ npm install
$ npx ts-node ./examples/typescript/boomerang.ts
```

```sh
$ cargo run -- --input ./examples/typescript/boomerang.ts --output boomerang.mlir
```
