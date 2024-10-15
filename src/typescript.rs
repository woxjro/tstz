use crate::mlir::{Type, Value};
use swc_ecma_ast::{BindingIdent, TsType};
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationToken {
    Address,
    Bool,
    Bytes,
    Contract,
    Int,
    Key,
    List,
    Mutez,
    Nat,
    Operation,
    Option,
    Pair,
    Signature,
    String,
    Unit,
}

impl std::fmt::Display for AnnotationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => write!(f, "address"),
            Self::Bool => write!(f, "bool"),
            Self::Bytes => write!(f, "bytes"),
            Self::Contract => write!(f, "Contract"),
            Self::Int => write!(f, "int"),
            Self::Key => write!(f, "key"),
            Self::List => todo!(),
            Self::Mutez => write!(f, "mutez"),
            Self::Nat => write!(f, "nat"),
            Self::Operation => write!(f, "Operation"),
            Self::Option => write!(f, "Option"),
            Self::Pair => write!(f, "Pair"),
            Self::Signature => write!(f, "signature"),
            Self::String => write!(f, "string"),
            Self::Unit => write!(f, "Unit"),
        }
    }
}

pub fn get_value(bident: BindingIdent) -> Value {
    let BindingIdent { id: _, type_ann } = bident.to_owned();

    let ty = get_mlir_type(*type_ann.unwrap().type_ann);

    Value {
        id: format!("%{}", bident.id.sym),
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
            } else if ty_sym == AnnotationToken::Bytes.to_string() {
                Type::Bytes
            } else if ty_sym == AnnotationToken::Bool.to_string() {
                Type::Bool
            } else if ty_sym == AnnotationToken::Key.to_string() {
                Type::Key
            } else if ty_sym == AnnotationToken::Signature.to_string() {
                Type::Signature
            } else if ty_sym == AnnotationToken::String.to_string() {
                Type::String
            } else if ty_sym == AnnotationToken::Int.to_string() {
                Type::Int
            } else if ty_sym == AnnotationToken::Nat.to_string() {
                Type::Nat
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
