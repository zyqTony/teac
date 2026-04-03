pub type Pos = usize;

#[derive(Debug, Clone)]
pub enum BuiltIn {
    Int,
    Float,  // 新增：浮点类型 f32
}

#[derive(Debug, Clone)]
pub enum TypeSpecifierInner {
    BuiltIn(BuiltIn),
    Composite(String),
    Reference(Box<TypeSpecifier>),
}

#[derive(Debug, Clone)]
pub struct TypeSpecifier {
    pub pos: Pos,
    pub inner: TypeSpecifierInner,
}
