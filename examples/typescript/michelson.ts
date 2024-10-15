export type address = string;
export type bool = boolean;
export type bytes = string;
export type int = number;
export type key = string;
export type mutez = number;
export type nat = number;
export type signature = string;
// export type string = string;

export function getAmount(): mutez {
  const DUMMY_AMOUNT: mutez = 5000;
  return DUMMY_AMOUNT;
}

export function makeList<T>(): T[] {
  return [];
}

export function append<T>(list: T[], elem: T): T[] {
  list.push(elem);
  return list;
}

export function getSource(): address {
  const DUMMY_SOURCE: address = "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx";
  return DUMMY_SOURCE;
}

export type Contract<T> = {
  param: Option<T>;
  address: address;
};

export function getContract<T>(address: address): Option<Contract<T>> {
  return {
    param: null,
    address: address,
  };
}

export function getBytes(number: int): bytes {
  return number.toString();
}

export function sha256(bytes: bytes): bytes {
  return bytes;
}

export function assertSome<T>(value: Option<T>): T {
  if (value === null) {
    throw new Error("Expected Some but got None");
  }
  return value;
}

export function transferTokens<T>(
  param: T,
  amount: mutez,
  contract: Contract<T>
): Operation {
  return {
    amount: amount,
  };
}

export type Operation = {
  amount: mutez;
};

export class Unit {}

export function makePair<T, U>(first: T, second: U): Pair<T, U> {
  return new Pair(first, second);
}

export class Pair<T, U> {
  constructor(
    public first: T,
    public second: U
  ) {}
}

export type Option<T> = T | null;

// The type of value should be packable to bytes
export function pack(value: string): bytes {
  return value;
}

export function getFst<T, U>(pair: Pair<T, U>): T {
  return pair.first;
}

export function getSnd<T, U>(pair: Pair<T, U>): U {
  return pair.second;
}

export function assert(condition: bool) {
  if (!condition) {
    throw new Error("Assertion failed");
  }
  return;
}

export function checkSignature(
  key: key,
  signature: signature,
  bytes: bytes
): bool {
  return false;
}
