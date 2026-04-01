use rgb::RGBA8;

pub enum ArrayBlock {
    Number(f64),                        // 4
    PositiveNumber(f64),                // 5
    WholeNumber(u64),                   // 6
    Integer(i64),                       // 7
    Angle(f64),                         // 8
    Color(RGBA8),                       // 9
    String(String),                     // 10
    Broadcast(String, String),          // 11
    Variable(String, String, f64, f64), // 12
    List(String, String, f64, f64),     // 13
}
