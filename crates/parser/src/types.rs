use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TensorShapeType {
    Any,
    Shape(Vec<u64>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,

    Unit,
    F32,
    F64,
    I32,
    U32,
    I64,
    U64,
    Char,
    Bool,

    List(Box<Type>),
    Tuple(Vec<Type>),

    Tensor {
        dtype: Box<Type>,
        shape: TensorShapeType,
    },

    Function {
        params: Vec<Type>,
        ret: Box<Type>,
    },

    Ext(String),
    Unknown,
}

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "any" => Type::Any,
            "unit" => Type::Unit,

            "f32" => Type::F32,
            "f64" => Type::F64,
            "i32" => Type::I32,
            "u32" => Type::U32,
            "i64" => Type::I64,
            "u64" => Type::U64,
            "char" => Type::Char,
            "bool" => Type::Bool,

            "tensor" | "list" | "tuple" => panic!("single {} str cannot convert to a type", s),

            _ => Type::Ext(s.to_string()),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Type::Any => "any".to_string(),
            Type::Unit => "unit".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Char => "char".to_string(),
            Type::Bool => "bool".to_string(),

            Type::List(typ) => format!("List<{}>", typ.to_string()),

            Type::Tensor { dtype, shape } => {
                format!("tensor<{}, {:?}>", dtype.to_string(), shape)
            }

            Type::Ext(name) => format!("{}", name),
            Type::Unknown => "?".to_string(),
            _ => panic!("unknown type {}", self),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
