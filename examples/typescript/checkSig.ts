import {
  Operation,
  Pair,
  String,
  bool,
  bytes,
  checkSignature,
  getFst,
  getSnd,
  key,
  makeList,
  makePair,
  pack,
  signature,
  assert,
} from "./michelson";

function smartContract(
  storage: Pair<signature, String>,
  param: key
): Pair<Operation[], Pair<signature, String>> {
  const signature: signature = getFst(storage);
  const str: String = getSnd(storage);
  const byt: bytes = pack(str);
  const result: bool = checkSignature(param, signature, byt);
  assert(result);
  const nil: Operation[] = makeList();

  const p: Pair<Operation[], Pair<signature, String>> = makePair(nil, storage);
  return p;
}

const storage = new Pair("", "");
const param = "";
const result: Pair<Operation[], Pair<signature, String>> = smartContract(
  storage,
  param
);
console.log(result);
