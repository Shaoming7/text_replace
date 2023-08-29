use xml::reader::{EventReader, XmlEvent};
use std::io::BufReader;
use std::{fs::File, error::Error};
use crate::translate::Translate;

pub enum XmlParsingMode{
    Mode1,
    Mode2,
}

pub fn parse_xml_file(file_path: &str, parsing_mode: XmlParsingMode) -> Result<Vec<Translate>, Box<dyn Error>>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);
    let mut batch_data = Vec::new();
    let mut row_num = 1;
    let mut translate = Translate{
        id: 0,
        chinese: String::from(""),
        english: String::from(""),
        row_num
    };
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "string" {
                    for attr in attributes {
                        if attr.name.local_name == "name" {
                            translate.english = attr.value;
                        }
                    }
                }
            }
            Ok(XmlEvent::Characters(value)) => {
                match parsing_mode {
                    XmlParsingMode::Mode1 => {
                        translate.chinese = value.clone();
                    }
                    XmlParsingMode::Mode2 => {
                    }
                }
                translate.row_num = row_num;
                batch_data.push(translate.clone());
                row_num += 1;

                // 插入后清空数据，反正出现奇怪的值
                translate = Translate{
                    id: 0,
                    chinese: String::from(""),
                    english: String::from(""),
                    row_num
                };
            }
            _ => {}
        }
    }

    Ok(batch_data)
}

pub fn get_chinese_strings(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);
    let mut chinese_strings = Vec::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { attributes, .. }) => {
                for attr in attributes {
                    if attr.name.local_name == "text" {
                        if has_chinese_characters(&attr.value) {
                            chinese_strings.push(attr.value.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(chinese_strings)
}

fn has_chinese_characters(s: &str) -> bool {
    s.chars().any(|c| is_chinese_character(c))
}

fn is_chinese_character(c: char) -> bool {
    // 判断字符是否在中文 Unicode 范围内（包括扩展区域B和C）
    (c >= '\u{4E00}' && c <= '\u{9FFF}') ||
        (c >= '\u{3400}' && c <= '\u{4DBF}') ||
        (c >= '\u{20000}' && c <= '\u{2A6DF}')
}

fn is_chinese_character2(c: char) -> bool {
    let c_str = c.to_string();
    c_str.matches(
        |c| matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{20000}'..='\u{2A6DF}')
    ).count() > 0
}
