#[derive(Debug, Clone)] // Implement the Clone trait
pub struct Translate {
    pub id: i32,
    pub english: String,
    pub chinese: String,
    pub row_num: i32,
}


// 也可使用元组结构体 这样可以节省一些代码和内存开销。可以使用索引来访问字段，而不需要使用.操作符来访问命名字段。
// #[derive(Debug, Clone)]
// pub struct Translate(pub i32, pub String, pub String, pub i32);
