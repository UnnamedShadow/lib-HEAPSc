pub struct FunctionContent {
    memoized: bool,
    args: Vec<String>,
    body: Vec<Line>,
}

pub struct Function {
    name: String,
    content: FunctionContent,
}

pub struct Macro {
    left: String,
    name: String,
    arg: String,
    right: String,
}

pub enum Line {
    Function(Function),
    Macro(Macro),
}

pub struct Dependency {
    name: String,
    path: String,
}

pub enum Type {
    Enum {
        name: String,
        variants: Vec<(String, Type)>,
    },
    Function {
        name: String,
        args: Vec<Type>,
    },
    Tuple(Vec<Type>),
    Primitive(String),
}

pub type FunctionTyping = fn(Vec<crate::Type>) -> Option<crate::Type>;

pub mod frontend {
    use std::iter::Map;

    pub struct File {
        functions: Map<String, crate::FunctionContent>,
        macros: Vec<crate::Macro>,
    }

    pub mod parser;

    pub struct TypedFunctionContent {
        raw: crate::FunctionContent,
        typing: crate::FunctionTyping,
    }

    pub mod linter;

    pub mod initial_optimizer;

    pub struct Header {
        dependencies: Vec<crate::Dependency>,
        functions: Map<String, crate::FunctionTyping>,
    }
}

pub mod backend {
    pub mod optimizer;

    pub mod hlasm_generator;

    pub mod hlasm_optimizer;

    pub mod packager;

    pub mod repackager;
}

pub mod processor;
