import {
  Unit,
  mutez,
  address,
  Option,
  Contract,
  Operation,
  Pair,
  getAmount,
  makeList,
  getSource,
  getContract,
  assertSome,
  transferTokens,
  append,
  makePair,
} from "./michelson";

function smartContract(storage: Unit, param: Unit): Pair<Operation[], Unit> {
  const amount: mutez = getAmount();
  const nil: Operation[] = makeList();
  const address: address = getSource();
  const someContract: Option<Contract<Unit>> = getContract(address);
  const contract: Contract<Unit> = assertSome(someContract);
  const operation: Operation = transferTokens(param, amount, contract);
  const operations: Operation[] = append(nil, operation);

  const p: Pair<Operation[], Unit> = makePair(operations, param);
  return p;
}

const storage: Unit = new Unit();
const param: Unit = new Unit();
const result: Pair<Operation[], Unit> = smartContract(storage, param);
console.log(result);
