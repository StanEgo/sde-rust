pub trait Schema {
    fn to_schema();
}

pub struct SchemaProperty {
    pub name: String,
    pub kind: String,
}

pub struct SchemaNode {
    pub name: String,
    pub props: std::vec::Vec<SchemaProperty>,
}
