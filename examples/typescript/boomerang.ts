type mutez = number;
type address = string;

function getAmount(): mutez {
  const DUMMY_AMOUNT: mutez = 5000;
  return DUMMY_AMOUNT;
}

function makeList<T>(): T[] {
  return [];
}

function append<T>(list: T[], elem: T): T[] {
  list.push(elem);
  return list;
}

function getSource(): address {
  const DUMMY_SOURCE: address = "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx";
  return DUMMY_SOURCE;
}

type Contract<T> = {
  param: Option<T>;
  address: address;
};

function getContract<T>(address: address): Option<Contract<T>> {
  return {
    param: null,
    address: address,
  };
}

function assertSome<T>(value: Option<T>): T {
  if (value === null) {
    throw new Error("Expected Some but got None");
  }
  return value;
}

function transferTokens<T>(
  param: T,
  amount: mutez,
  contract: Contract<T>
): Operation {
  return {
    amount: amount,
  };
}

type Operation = {
  amount: mutez;
};

class Unit {}

function makePair<T, U>(first: T, second: U): Pair<T, U> {
  return new Pair(first, second);
}

class Pair<T, U> {
  constructor(public first: T, public second: U) {}
}

type Option<T> = T | null;

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
