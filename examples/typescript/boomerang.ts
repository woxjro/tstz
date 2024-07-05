/* boomerang.py

from dataclasses import dataclass
from typing import Final, List, TypeVar, Generic, Optional

mutez = int
addr = str


class Operation:
    pass


class Unit:
    pass


T = TypeVar('T')
U = TypeVar('U')


@dataclass
class Pair(Generic[T, U]):
    first: T
    second: U


@dataclass
class Contract(Generic[T]):
    pass


def make_pair(first: T, second: U) -> Pair[T, U]:
    return Pair(first, second)


def get_amount() -> mutez:
    DUMMY_AMOUNT: Final[mutez] = 5000
    return DUMMY_AMOUNT


def make_list() -> List:
    return []


def append(operations: List[Operation], operation: Operation) -> List[Operation]:
    operations.append(operation)
    return operations


def get_source() -> addr:
    DUMMY_SOURCE: Final[addr] = "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx"
    return DUMMY_SOURCE


def get_contract(address: addr) -> Optional[Contract[T]]:
    return Contract()


def assert_some(value: Optional[T]) -> T:
    if value is None:
        raise Exception("Expected Some but got None")
    return value


def transfer_tokens(param: T, amount: mutez, contract: Contract[T]) -> Operation:
    return Operation()


def smart_contract(storage: Unit, param: Unit) -> Pair[List[Operation], Unit]:
    amount: Final[mutez] = get_amount()
    nil: Final[List[Operation]] = make_list()
    address: Final[addr] = get_source()
    some_contract: Final[Optional[Contract[Unit]]] = get_contract(address)
    contract: Final[Contract[Unit]] = assert_some(some_contract)
    operation: Final[Operation] = transfer_tokens(param, amount, contract)
    operations: Final[List[Operation]] = append(nil, operation)

    p: Final[Pair[List[Operation], Unit]] = make_pair(operations, param)
    return p

*/

// Convert the above Python code to TypeScript
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

type Contract<T> = {};

function getContract<T>(address: address): Contract<T> | null {
  return {};
}

function assertSome<T>(value: T | null): T {
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
  return {};
}

class Operation {}

class Unit {}

function makePair<T, U>(first: T, second: U): Pair<T, U> {
  return new Pair(first, second);
}

class Pair<T, U> {
  constructor(public first: T, public second: U) {}
}

function smartContract(storage: Unit, param: Unit): Pair<Operation[], Unit> {
  const amount: mutez = getAmount();
  const nil: Operation[] = makeList();
  const address: address = getSource();
  const someContract: Contract<Unit> | null = getContract(address);
  const contract: Contract<Unit> = assertSome(someContract);
  const operation: Operation = transferTokens(param, amount, contract);
  const operations: Operation[] = append(nil, operation);

  const p: Pair<Operation[], Unit> = makePair(operations, param);
  return p;
}
