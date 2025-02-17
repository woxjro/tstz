#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub id: String, // "%0" "%a" "%b" etc
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Address,
    Bool,
    Bytes,
    Contract { param: Box<Type> },
    Int,
    Key,
    List { elem: Box<Type> },
    Mutez,
    Nat,
    Operation,
    Option { elem: Box<Type> },
    Pair { fst: Box<Type>, snd: Box<Type> },
    Signature,
    String,
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationKind {
    Append,
    Assert,
    AssertSome,
    CheckSignature,
    GetAmount,
    GetBytes,
    GetContract,
    GetFst,
    GetSnd,
    GetSource,
    MakeList,
    MakePair,
    Pack,
    Return,
    Sha256,
    TransferTokens,
}

impl From<&str> for OperationKind {
    fn from(s: &str) -> Self {
        match s {
            "append" => Self::Append,
            "assert" => Self::Assert,
            "assertSome" => Self::AssertSome,
            "checkSignature" => Self::CheckSignature,
            "getAmount" => Self::GetAmount,
            "getBytes" => Self::GetBytes,
            "getContract" => Self::GetContract,
            "getFst" => Self::GetFst,
            "getSnd" => Self::GetSnd,
            "getSource" => Self::GetSource,
            "makeList" => Self::MakeList,
            "makePair" => Self::MakePair,
            "pack" => Self::Pack,
            "return" => Self::Return,
            "sha256" => Self::Sha256,
            "transferTokens" => Self::TransferTokens,
            _ => panic!("Unknown operation kind: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub kind: OperationKind,
    pub args: Vec<Value>,
    pub results: Vec<Value>,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => write!(f, "!michelson.address"),
            Self::Bool => write!(f, "!michelson.bool"),
            Self::Bytes => write!(f, "!michelson.bytes"),
            Self::Contract { param } => write!(f, "!michelson.contract<{}>", param),
            Self::Int => write!(f, "!michelson.int"),
            Self::Key => write!(f, "!michelson.key"),
            Self::List { elem } => write!(f, "!michelson.list<{}>", elem),
            Self::Mutez => write!(f, "!michelson.mutez"),
            Self::Nat => write!(f, "!michelson.nat"),
            Self::Operation => write!(f, "!michelson.operation"),
            Self::Option { elem } => write!(f, "!michelson.option<{}>", elem),
            Self::Pair { fst, snd } => write!(f, "!michelson.pair<{}, {}>", fst, snd),
            Self::Signature => write!(f, "!michelson.signature"),
            Self::String => write!(f, "!michelson.string"),
            Self::Unit => write!(f, "!michelson.unit"),
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            OperationKind::MakePair => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 2);
                let result_type = Type::Pair {
                    fst: Box::new(self.args[0].ty.clone()),
                    snd: Box::new(self.args[1].ty.clone()),
                };
                write!(
                    f,
                    "{} = \"michelson.make_pair\"({}, {}): ({}, {}) -> {result_type}",
                    self.results[0].id,
                    self.args[0].id,
                    self.args[1].id,
                    self.args[0].ty,
                    self.args[1].ty,
                )
            }
            OperationKind::MakeList => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 0);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.make_list\"(): () -> {}",
                    result.id, result.ty
                )
            }
            OperationKind::GetAmount => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 0);
                assert_eq!(self.results[0].ty, Type::Mutez);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.get_amount\"(): () -> {}",
                    result.id, result.ty
                )
            }
            OperationKind::GetBytes => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results[0].ty, Type::Bytes);
                let result = &self.results[0];

                write!(
                    f,
                    "{} = \"michelson.get_bytes\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::GetSource => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 0);
                assert_eq!(self.results[0].ty, Type::Address);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.get_source\"(): () -> {}",
                    result.id, result.ty
                )
            }
            OperationKind::GetContract => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 1);
                if let Type::Option { elem } = &self.results[0].ty {
                    if let Type::Contract { .. } = &**elem {
                    } else {
                        panic!("Expected Contract<..>");
                    }
                } else {
                    panic!("Expected Option<Contract<..>>");
                }

                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.get_contract\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::Sha256 => {
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args[0].ty, Type::Bytes);
                assert_eq!(self.results[0].ty, Type::Bytes);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.sha256\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::AssertSome => {
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 1);
                if let Type::Option { .. } = &self.args[0].ty {
                } else {
                    panic!("Expected Option<..>");
                }

                let result = &self.results[0];

                write!(
                    f,
                    "{} = \"michelson.assert_some\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::TransferTokens => {
                // %operation = "michelson.transfer_tokens"(%parameter, %amount, %contract) :
                // (!michelson.unit, !michelson.mutez, !michelson.contract<!michelson.unit>) -> !michelson.operation
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 3);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.transfer_tokens\"({}, {}, {}): ({}, {}, {}) -> {}",
                    result.id,
                    self.args[0].id,
                    self.args[1].id,
                    self.args[2].id,
                    self.args[0].ty,
                    self.args[1].ty,
                    self.args[2].ty,
                    result.ty
                )
            }
            OperationKind::Append => {
                // %operations = "michelson.cons"(%nil, %operation) :
                // (!michelson.list<!michelson.operation>, !michelson.operation) -> !michelson.list<!michelson.operation>
                assert_eq!(self.results.len(), 1);
                assert_eq!(self.args.len(), 2);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.cons\"({}, {}): ({}, {}) -> {}",
                    result.id,
                    self.args[0].id,
                    self.args[1].id,
                    self.args[0].ty,
                    self.args[1].ty,
                    result.ty
                )
            }
            OperationKind::Pack => {
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results.len(), 1);
                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.pack\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::GetFst => {
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results.len(), 1);

                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.get_fst\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::GetSnd => {
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results.len(), 1);

                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.get_snd\"({}): ({}) -> {}",
                    result.id, self.args[0].id, self.args[0].ty, result.ty
                )
            }
            OperationKind::CheckSignature => {
                assert_eq!(self.args.len(), 3);
                assert_eq!(self.results.len(), 1);

                let result = &self.results[0];
                write!(
                    f,
                    "{} = \"michelson.check_signature\"({}, {}, {}): ({}, {}, {}) -> {}",
                    result.id,
                    self.args[0].id,
                    self.args[1].id,
                    self.args[2].id,
                    self.args[0].ty,
                    self.args[1].ty,
                    self.args[2].ty,
                    result.ty
                )
            }
            OperationKind::Assert => {
                assert_eq!(self.args.len(), 1);
                assert_eq!(self.results.len(), 0);

                write!(
                    f,
                    "\"michelson.assert\"({}): ({}) -> ()",
                    self.args[0].id, self.args[0].ty
                )
            }
            OperationKind::Return => {
                assert_eq!(self.results.len(), 0);
                assert_eq!(self.args.len(), 1);
                let arg = &self.args[0];
                write!(f, "return {}: {}", arg.id, arg.ty)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_list_type_to_string() {
        let t = Type::List {
            elem: Box::new(Type::Mutez),
        };
        assert_eq!(t.to_string(), "!michelson.list<!michelson.mutez>");
    }

    #[test]
    fn test_pair_type_to_string() {
        let t = Type::Pair {
            fst: Box::new(Type::Mutez),
            snd: Box::new(Type::Operation),
        };
        assert_eq!(
            t.to_string(),
            "!michelson.pair<!michelson.mutez, !michelson.operation>"
        );
    }

    #[test]
    fn test_nested_type_to_string() {
        let t = Type::List {
            elem: Box::new(Type::Pair {
                fst: Box::new(Type::Mutez),
                snd: Box::new(Type::Operation),
            }),
        };
        assert_eq!(
            t.to_string(),
            "!michelson.list<!michelson.pair<!michelson.mutez, !michelson.operation>>"
        );
    }

    #[test]
    fn test_make_pair_to_string() {
        let op = Operation {
            kind: OperationKind::MakePair,
            args: vec![
                Value {
                    id: "%a".to_string(),
                    ty: Type::Mutez,
                },
                Value {
                    id: "%b".to_string(),
                    ty: Type::Operation,
                },
            ],
            results: vec![Value {
                id: "%0".to_string(),
                ty: Type::Pair {
                    fst: Box::new(Type::Mutez),
                    snd: Box::new(Type::Operation),
                },
            }],
        };

        assert_eq!(
                op.to_string(),
                "%0 = !michelson.make_pair(%a, %b): !michelson.pair<!michelson.mutez, !michelson.operation>"
            );
    }

    #[test]
    fn test_make_list_to_string() {
        let op = Operation {
            kind: OperationKind::MakeList,
            args: vec![],
            results: vec![Value {
                id: "%0".to_string(),
                ty: Type::List {
                    elem: Box::new(Type::Mutez),
                },
            }],
        };

        assert_eq!(
            op.to_string(),
            "%0 = !michelson.make_list(): !michelson.list<!michelson.mutez>"
        );
    }

    #[test]
    fn test_get_amount_to_string() {
        let op = Operation {
            kind: OperationKind::GetAmount,
            args: vec![],
            results: vec![Value {
                id: "%0".to_string(),
                ty: Type::Mutez,
            }],
        };

        assert_eq!(
            op.to_string(),
            "%0 = !michelson.get_amount(): !michelson.mutez"
        );
    }
}
