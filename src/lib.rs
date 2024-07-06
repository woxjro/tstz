pub mod mlir {

    #[derive(Debug, Clone, PartialEq)]
    pub struct Value {
        pub id: String, // "%0" "%a" "%b" etc
        pub ty: Type,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Address,
        Unit,
        Mutez,
        Operation,
        Contract { param: Box<Type> },
        Option { elem: Box<Type> },
        List { elem: Box<Type> },
        Pair { fst: Box<Type>, snd: Box<Type> },
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum OperationKind {
        MakePair,
        MakeList,
        GetAmount,
        GetSource,
        GetContract,
        AssertSome,
        TransferTokens,
        Append,
        Return,
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
                Self::Unit => write!(f, "!michelson.unit"),
                Self::Mutez => write!(f, "!michelson.mutez"),
                Self::Contract { param } => write!(f, "!michelson.contract<{}>", param),
                Self::Operation => write!(f, "!michelson.operation"),
                Self::Option { elem } => write!(f, "!michelson.option<{}>", elem),
                Self::List { elem } => write!(f, "!michelson.list<{}>", elem),
                Self::Pair { fst, snd } => write!(f, "!michelson.pair<{}, {}>", fst, snd),
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
}

pub mod typescript {
    use crate::mlir::{Type, Value};
    use swc_ecma_ast::{BindingIdent, TsType};
    #[derive(Debug, Clone, PartialEq)]
    pub enum AnnotationToken {
        Mutez,
        Unit,
        Address,
        Contract,
        Operation,
        Option,
        List,
        Pair,
    }

    impl std::fmt::Display for AnnotationToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Mutez => write!(f, "mutez"),
                Self::Unit => write!(f, "Unit"),
                Self::Address => write!(f, "address"),
                Self::Contract => write!(f, "Contract"),
                Self::Operation => write!(f, "Operation"),
                Self::Option => write!(f, "Option"),
                Self::List => todo!(),
                Self::Pair => write!(f, "Pair"),
            }
        }
    }

    pub fn get_value(bident: BindingIdent) -> Value {
        let BindingIdent { id: _, type_ann } = bident.to_owned();

        let ty = get_mlir_type(*type_ann.unwrap().type_ann);

        Value {
            id: bident.id.sym.to_string(),
            ty,
        }
    }

    pub fn get_mlir_type(ty_ann: TsType) -> Type {
        match ty_ann {
            TsType::TsTypeRef(ty_ref) => {
                let ty_sym = ty_ref.type_name.expect_ident().sym.to_string();
                if ty_sym == AnnotationToken::Mutez.to_string() {
                    Type::Mutez
                } else if ty_sym == AnnotationToken::Unit.to_string() {
                    Type::Unit
                } else if ty_sym == AnnotationToken::Address.to_string() {
                    Type::Address
                } else if ty_sym == AnnotationToken::Operation.to_string() {
                    Type::Operation
                } else if ty_sym == AnnotationToken::Option.to_string() {
                    Type::Option {
                        elem: Box::new(get_mlir_type(
                            *ty_ref.type_params.unwrap().params[0].to_owned(),
                        )),
                    }
                } else if ty_sym == AnnotationToken::Contract.to_string() {
                    Type::Contract {
                        param: Box::new(get_mlir_type(
                            *ty_ref.type_params.unwrap().params[0].to_owned(),
                        )),
                    }
                } else if ty_sym == AnnotationToken::Pair.to_string() {
                    Type::Pair {
                        fst: Box::new(get_mlir_type(
                            *ty_ref.type_params.to_owned().unwrap().params[0].to_owned(),
                        )),
                        snd: Box::new(get_mlir_type(
                            *ty_ref.type_params.unwrap().params[1].to_owned(),
                        )),
                    }
                } else {
                    panic!("Unknown type: {}", ty_sym);
                }
            }
            TsType::TsArrayType(ty_arr) => {
                let elem = get_mlir_type(*ty_arr.elem_type.to_owned());
                Type::List {
                    elem: Box::new(elem),
                }
            }
            _ => {
                unreachable!("Unsupported type annotation: {:?}", ty_ann);
            }
        }
    }
}
