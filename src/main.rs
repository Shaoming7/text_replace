// #![allow(unused_variables)]
// #![allow(dead_code)]
#![allow(warnings)]
use text_replace_tool::*;
use std::collections::HashMap;
use std::{fs, io};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

// #[cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
fn main() {
    // insert_db_by_file();    // 插入数据
    let mut payment_vec= get_translations().unwrap();
    // println!("{:#?} payment_vec len is {}", payment_vec, payment_vec.len());

    compare_file_texts_with_database_values(&payment_vec);
    // process_files_and_compare_database_values(&payment_vec);
}

fn process_files_and_compare_database_values(translate_vec:&[Translate]){
    // 获取所有文件名
    let filenames = get_filenames_in_folder("input2");
    // println!("{:#?} filenames len is {}", filenames, filenames.len());
    // println!("{:#?}, {}", translate_vec, translate_vec.len());
    let output_root = "output/";

    for i in &filenames {
        let name = String::from("input2/") + i;
        println!("<!--    {}-->", i);
        replace_android_text(&name, &format!("{0}{1}", output_root, i), translate_vec).unwrap();
        println!("- - - - -");
    }
}

// 求数据库已有的值，对比文件中的text，来看是否存在
fn compare_file_texts_with_database_values(payment_vec:&[Translate]){
    // // 获取所有文件名
    let filenames = get_filenames_in_folder("input2");
    let example_vec = parse_xml_file("input/temp.xml", XmlParsingMode::Mode1).unwrap();
    let mut no_found_vec:Vec<NoFoundValue> = Vec::new();
    println!("{:?} example_vec len is {}", example_vec, example_vec.len());
    // 开始处理
    let mut my_set = HashSet::new();
    for i in &filenames{
        let name = String::from("input2/")+i;
        println!("<!--    {}-->",i);
        for i in get_chinese_strings(&name).unwrap(){
            let result = my_set.insert(i.clone());
            if result {
                let mut flag = true;
                for vec in payment_vec{  // 数据库中存在的
                    if vec.chinese == i {
                        flag = false;
                        println!("<!-- <string name=\"{0}\">{1}</string> -->",vec.english, i);
                    }
                }
                if flag { // 数据库中没有的，加入到string.xml中
                    for vec in &example_vec{
                        if vec.chinese == i {
                            flag = false;
                            println!("  <string name=\"{0}\">{1}</string> ", vec.english,vec.chinese);
                        }
                    }
                }
                if flag{
                    // let value_name_str:&'static str = i.clone().as_str();   //temporary value dropped while borrowed
                    println!("ERROR 404 VALUE : {0}", i);
                    no_found_vec.push(NoFoundValue{
                        file_path: name.clone(),
                        value_name: i.clone(),
                    })
                }
            } else {
                println!("<!-- {} already exists -->", i);
            }
        }
        println!();
    }

    for i in no_found_vec{
        println!("{:?}", i);
    }
}

fn insert_db_by_file() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let dicts = parse_xml_file("input/strings.xml", XmlParsingMode::Mode1)?;
    insert_translations(&dicts);
    Ok(())
}

fn get_filenames_in_folder(root_path: &str) -> Vec<String>{
    let mut filenames:Vec<String> = Vec::new();
    let res = get_filename_by_folder_as_result(root_path).unwrap();
    for entry in &res {
        // 将 OsStr 转换为 &str，你需要使用 OsStr 对象上的 to_str 方法。由于操作系统可能使用不同的编码，所以这个转换可能会失败。因此，返回的结果是一个 Result<&str, OsString>。
        match entry.file_name().unwrap().to_str() {
            Some(str_ref) => filenames.push(str_ref.to_string()),
            None => println!("{:?} Conversion to str failed", entry.file_name()),
        }
    }
    filenames
}

fn get_filename_by_folder_as_result(file_path: &str) -> io::Result<Vec<PathBuf>>{
    let mut entries = fs::read_dir(file_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // 若想要绝对路径地址
    // let current_dir = std::env::current_dir()?; // 获取当前工作目录的绝对路径
    // 将相对路径转换为绝对路径
    // entries.iter_mut().for_each(|entry| {
    //     *entry = current_dir.join(entry);
    // });

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();
    // The entries have now been sorted by their path.
    Ok(entries)
}
#[derive(Debug)]
struct NoFoundValue {
    file_path:String,
    value_name:String,
}