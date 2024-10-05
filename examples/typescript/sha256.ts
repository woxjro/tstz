import {
  Operation,
  Pair,
  makeList,
  makePair,
  bytes,
  int,
  getBytes,
  sha256,
} from "./michelson";

function smartContract(storage: bytes, param: int): Pair<Operation[], bytes> {
  const byt: bytes = getBytes(param);
  const hash: bytes = sha256(byt);
  const nil: Operation[] = makeList();

  const p: Pair<Operation[], bytes> = makePair(nil, hash);
  return p;
}

const storage: bytes = "0x";
const param: int = 777;
const result: Pair<Operation[], bytes> = smartContract(storage, param);
console.log(result);
