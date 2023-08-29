use regex::Regex;
use std::{fs::File, error::Error};
use std::io::{self, BufRead, BufReader, Write};
use crate::translate::Translate; // Assuming you have the Translate struct defined in a separate module

// const ANDROID_TEXT_REGEX: &str = r#"android:text="([^"]+)""#;
const ANDROID_TEXT_REGEX: &str = r#"android:text="([^"@]+)""#;


pub fn replace_android_text(input_file: &str, output_file: &str, translate_vec:&[Translate]) -> Result<(), Box<dyn Error>>{
    // 打开输入文件
    let input_file = File::open(input_file)?;
    let reader = BufReader::new(input_file);
    // 打开输出文件
    let output_file = File::create(output_file)?;
    let mut writer = io::BufWriter::new(output_file);

    // 定义正则表达式
    let regex = Regex::new(ANDROID_TEXT_REGEX)?;
    // 逐行处理
    for line in reader.lines() {
        let line = line?;
        let mut modified_line = line.clone();

        // 在匹配位置新增内容
        if regex.is_match(&line) {
            if let Some(captures) = regex.captures(&line) {
                let extracted_text = captures.get(1).unwrap().as_str();
                let english = translate(extracted_text, translate_vec);
                println!("line: {0}, value: {1}, english: {2}", &line, extracted_text, &english);
                let new_content = format!("                android:text=\"{0}\"", &english);
                modified_line = new_content.to_string();
            }
            // modified_line.insert_str(line.find("android:text=").unwrap(), new_content);  // 这种是写到替代内容的位置
        }
        // 写入修改后的行到输出文件
        writer.write_all(modified_line.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn translate(chinese: &str, translate_vec:&[Translate]) -> String{
    for v in translate_vec{
        if v.chinese == chinese{
            return format!("@string/{}", v.english);
        }
    }
    chinese.to_string()
}

// pub fn use_reg(){
//     let input = r#"
//         <string name="app_name">Measure</string>
//         <string name="start_point">起始点</string>
//         <string name="fast_back_point">快退</string>
//         <string name="fast_forward_point">快进</string>
//     "#;
//     let re = Regex::new(r#"<string name="([^"]+)">([^<]+)</string>"#).unwrap();
//     for cap in re.captures_iter(input) {
//         let name = &cap[1];
//         let value = &cap[2];
//         println!("Name: {}, Value: {}", name, value);
//     }
// }