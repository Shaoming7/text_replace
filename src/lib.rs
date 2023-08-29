mod text_processing;
mod translate;
mod xml_utils;

// src/lib.rs

mod db;

// 使用导入的模块
pub use db::get_translations;
pub use db::insert_translations;
pub use xml_utils::XmlParsingMode;
pub use xml_utils::parse_xml_file;
pub use xml_utils::get_chinese_strings;
pub use text_processing::replace_android_text;
pub use translate::Translate;



// 测试功能的代码
// pub use regex_lib::use_reg;
