pub enum MObj {
    Unit,
    Bool(bool),
    Byte(i8),
    Short(i16),
    Word(i32),
    String(String),
    List(Vec<MObj>),
    Object(HashMap<String, MObj>),
}
