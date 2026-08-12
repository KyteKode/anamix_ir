pub struct Variable {
    pub value: DynamicValue,
    pub cloud: bool,
}

pub struct List(pub Vec<DynamicValue>);

pub struct Broadcast(pub String);

pub enum DynamicValue {
    String(String),
    Number(f64),
    Bool(bool),
}
