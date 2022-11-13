#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
