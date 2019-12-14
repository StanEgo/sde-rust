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

/*
TODO:0

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//WORK: Sort out
pub struct Timestamp(u64);

pub trait Id<T> {
    fn id() -> T;
}

pub trait Lifetime {
    fn start() -> Timestamp;
    fn finish() -> Option<Timestamp>;
}
*/
