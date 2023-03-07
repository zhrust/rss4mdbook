#![allow(unused)]
use std::env;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::io::{self, Write};
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
//use std::fmt::Write as fmt_write;

use std::path::PathBuf;
use std::path::Path;

use std::collections::BTreeMap;

//use toml::de::{Deserializer, MapAccess, SeqAccess};
//use toml::de::{Deserializer, value::MapDeserializer, value::SeqDeserializer};
//use toml::Value;
//use toml::value::{Value, Table};
//use toml::de::{Deserializer, MapAccess, SeqAccess, value::TableDeserializer, value::ArrayDeserializer};
//use serde::{Serialize, Deserialize};
//use envy::Error;
//use itertools::Itertools;
use dotenv::dotenv;
//use indicatif::ProgressBar;

//use tokio::io::{self, AsyncBufReadExt};
//use tokio::fs::File as async_File;
//use tokio::io::{self as async_io, AsyncBufReadExt};
//use tokio::io::{AsyncBufReadExt, BufReader};
//use tokio::io::AsyncBufReadExt;
//use tokio::io::BufReader as TokioBufReader;
////use tokio::fs::File;
//use tokio::fs::File as TokioFile;

//pub const ENV_SRC: &str = "RSS4MDBOOK_SRC";
pub const ENV_BOOK: &str = "RSS4MDBOOK_TOML";
pub const RSS_TITLE: &str = "锈周刊 -> Weekly :: China<Rustaceans>";
pub const RSS_DESC: &str = "~ 汇集上周全球锈事儿, 由 大妈/Zoom.Quiet 根据 <Rust recap for week> 快译++吐槽而得, 共同定期追踪值得oxygenation/氧化之事儿...";

pub fn upd_denv(key: &str, val: &str) {
    match ok_denv() {
        Ok(path) => {
            let path_str = path.to_str().unwrap();
            let mut new_lines = String::new();
            let mut found_key = false;

            let file = File::open(path_str);
            let reader = BufReader::new(file.unwrap());
            for line in reader.lines() {
                if let Ok(l) = line {
                    if l.starts_with(&format!("{}=", key)) {
                        found_key = true;
                        new_lines.push_str(&format!("{}={} ", key, val));
                        println!("\n\t Updated .env item: {}={} ", key, val);
                    } else {
                        new_lines.push_str(&l);
                        new_lines.push('\n');
                    }
                }
            }

            // If the key doesn't exist in the .env file, add it
            if !found_key {
                new_lines.push_str(&format!("{}={} ", key, val));
                println!("\n\t New .env item, inserted: {}={} ", key, val);
            }

            let mut file = match File::create(path_str) {
                Ok(f) => f,
                Err(_) => {
                    println!("Failed to create .env file");
                    return;
                }
            };

            file.write_all(new_lines.as_bytes()).unwrap();
        },
        Err(e) => println!("{}", e),
    }
}


pub fn reload_denv(f2denv:&str){
    // 加载 .env 文件中的配置项
    //dotenv().ok();
    dotenv::from_path(&f2denv).ok();
    // 遍历当前进程中的所有环境变量，打印每个键值对
    //for (key, value) in std::env::vars() {
    //    println!("{}={}", key, value);
    //}
}


pub enum EnvResult {
    Success(String, String),
    Failure(String),
}

pub fn chk_denv(key: &str)-> EnvResult {
    match ok_denv() {
        Ok(f2denv) => {
            let f2denv = ok_denv().unwrap().to_str().unwrap();
            println!("\n\t load .env <-{} ", f2denv);
            dotenv::from_path(&f2denv).ok();
            //let val = std::env::var(key);

            match std::env::var(key) {
                Ok(val) => {
                    //println!("\n\t got: {}={}", key, val);
                    EnvResult::Success(key.to_owned(), val)
                },
                Err(_) => {
                    //println!("{} is not set in .env file", key);
                    EnvResult::Failure(format!("{} is not set in .env file", key))
                }
            }
            //if let Ok(env_val) = std::env::var(key){
            //    println!("{}={}", key, env_val);
            //}else{
            //    println!("{} is not set in .env file", key);
            //}
            //let f2denv = ok_denv().unwrap().to_str().unwrap();
            //list_denv(f2denv);
        },
        Err(e) => EnvResult::Failure(e.to_string()),
        //println!("{}", e),
    }
}



// fn rmitem_denv(key: &str)
fn rmitem_denv(key: &str) {
    // Step 1：打开指定的 .env 配置文件，并生成一个 BufferedReader 对象

    // 从环境变量获取 .env 文件路径
    let env_path = env::var("ENV_PATH").unwrap();
    // 打开 .env 文件
    let env_file = File::open(env_path.clone()).expect("无法打开文件");
    // 生成一个 BufferedReader 对象
    let env_buf_reader = BufReader::new(env_file);

    // Step 2：遍历 BufferedReader 对象，逐行读取配置文件的每一行，检查是否包含指定的 key

    // 将文件内容存储到 Buffer 中
    let env_buffer: Vec<String> = env_buf_reader
        .lines()
        .map(|line| line.expect("无法读取行"))
        .collect();

    // 从 Buffer 中删除包含指定 key 的行
    let mut found = false;
    let env_new_buffer: Vec<String> = env_buffer
        .into_iter()
        .filter(|line| {
            // 检查每一行是否包含指定的 key
            if line.contains(key) {
                found = true;
                false
            } else {
                true
            }
        })
        .collect();

    // Step 3：如果遍历结束后没有找到指定的 key，则输出警告信息

    if !found {
        println!("Warning: 没有找到指定的 Key");
    }

    // Step 4：关闭 BufferedReader 对象，重新写回指定的 .env 文件

    // 打开一个新的 BufWriter 对象
    let env_file = File::create(env_path).unwrap();
    let mut env_buf_writer = BufWriter::new(env_file);

    // 重新写回去
    for line in env_new_buffer {
        env_buf_writer.write(line.as_bytes()).unwrap();
        env_buf_writer.write(b"\n").unwrap();
    }

    // 刷新缓冲区
    env_buf_writer.flush().unwrap();
}


/* 
pub fn rmitem_denv(key: &str) {
    match ok_denv() {
        Ok(path) => {
            let path_str = path.to_str().unwrap();
            let file = File::open(path_str);
            match file {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

                    let new_lines = lines
                            .iter()
                            .filter(|line| !line.starts_with(
                                &format!("{}=", key)))
                            .collect::<Vec<_>>() // 这里需要重新进行一次 collect 操作
                            .join("\n"); // 使用 join 方法

                    let mut file = File::create(path_str).unwrap();
                    file.write_all(new_lines.as_bytes()).unwrap();
                    println!("\n\t from .env removed item: {}", key);

                },
                Err(_) => println!("Failed to open .env file")
            }
        },
        Err(e) => println!("{}", e),
    }
}
 */

/* 
pub fn rmitem_denv(key: &str) {
    match ok_denv() {
        Ok(path) => {
            let path_str = path.to_str().unwrap();
            let file = File::open(path_str);
            match file {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

                    let new_lines = lines
                            .iter()
                            .filter(|line| !line.starts_with(
                                &format!("{}=", key)))
                            .join(" ");

                    let mut file = File::create(path_str).unwrap();
                    file.write_all(new_lines.as_bytes()).unwrap();
                    println!("\n\t from .env removed item: {}", key);

                },
                Err(_) => println!("Failed to open .env file")
            }
        },
        Err(e) => println!("{}", e),
    }
}

 */
pub fn ok_denv() -> Result<&'static Path, String> {
    // 获取应用程序的执行路径
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    // 获取执行路径的父路径
    let exe_dir = exe_path.parent().ok_or_else(|| "Failed to get parent directory".to_string())?;
    // 构造 .env 文件路径
    let mut env_path = PathBuf::from(exe_dir);
    env_path.push(".env");

    // 如果 .env 文件不存在，创建一个空的
    if !env_path.exists() {
        let mut file = File::create(&env_path).map_err(|e| e.to_string())?;
        file.write_all(b"").map_err(|e| e.to_string())?;
    }
    // must leak for return ...
    Ok(Box::leak(env_path.into_boxed_path()))
}
