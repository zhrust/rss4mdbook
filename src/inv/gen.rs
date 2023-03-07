//#![allow(unused)]
//use std::error::Error;
use std::fs;
use std::fs::File;
//use std::io::prelude::*;
//use std::io::Cursor;
//use std::io;
use std::io::Read;
use std::io::Write;
//use std::io::BufWriter;
//use std::collections::BTreeMap;
//use std::cmp::Reverse;
use std::path::Path;
use std::path::PathBuf;
use std::path::Component;
//use std::time::SystemTime;

use rss::Channel;
use rss::Item;
use chrono::prelude::*;
//use chrono::Utc;
use chrono::DateTime;
//use chrono::NaiveDateTime;
//use chrono::NaiveDate;
use walkdir::{DirEntry as WalkDirEntry, WalkDir};
//use walkdir::{DirEntry as WalkDirEntry, WalkDir};
//use walkdir::WalkDir;
use toml::Value;

use crate::inv::util;

/* CLI for gen. RSS from mdBook
- walk the src path
- check all .md file's update date
- order pick lated 5
- export as rss.xml -> u want path
/Users/zoomq/Exercism/proj/rss4mdbook/target/debug
*/
pub fn exp() {
    //println!("src/inv/gen: {}", env!("CARGO_PKG_VERSION"));
    let pkg_name = option_env!("CARGO_PKG_NAME").unwrap_or("DAMA's Crate");
    let pkg_version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.42");
    //println!("CARGO_PKG_NAME: {}",pkg_name);
    //println!("CARGO_PKG_VERSION: {}",pkg_version);
    println!("digging and generating by\n\t~> {} v{} <~",pkg_name,pkg_version);
    //log::debug!("src/inv/gen: as {}", env!("CARGO_PKG_VERSION"));
// check .env is OK?
    match util::chk_denv(util::ENV_BOOK) {
        util::EnvResult::Success(_ekey, _p2docs) => {
            println!("let's make RSS now...");
            //log::debug!(".env:\n {}={}",_ekey, _p2docs);
// try read ENV_BOOK
    match read_file(&_p2docs) {
        Ok(contents) => {
// got path from book.toml
        let toml_value = contents.parse::<Value>().unwrap();
        let src = toml_value["book"]["src"].as_str().unwrap();
        let build_dir = toml_value["build"]["build-dir"].as_str().unwrap();
        let rss_url_base = toml_value["rss4mdbook"]["url-base"].as_str().unwrap();

        if let Some(directory_str) = get_directory(&_p2docs) {
            let src2md = format!("{}/{}",directory_str,src);
            let expath = format!("{}/{}",directory_str,build_dir);
            let exprss = format!("{}/RSS.xml",expath);

            //log::debug!("\n rss url base: {}", rss_url_base);
            //log::debug!("\n src2md: {}\n expath: {}"
            //    , src2md
            //    , expath);
// walk dir for top5 lasted .md
            let latest5files = scan_dir(src2md.clone(), 4);
            //println!("lasted5top:{:?}",mds);
            println!("will export these article into RSS.xml");
            for md in latest5files.clone() {
                println!("\t{}",md);
            }
        match rss4top5md(rss_url_base.to_owned()
                    , exprss.clone()
                    , src2md.clone()
                    , latest5files){
                Ok(_) => println!("\n Export => {}\n\n",exprss.clone()),
                Err(e) =>println!("Error: {}", e)
                }
            }//get_directory(&_p2docs)
        }, Err(e) => println!("Error: {}", e)
    }// match read_file(&_p2docs)

        },util::EnvResult::Failure(e) => println!("failed: {}", e),
    }//match util::chk_denv(util::ENV_BOOK)
//    Ok(())
}


//fn scan_dir(src2md: String) {
fn scan_dir(src2md: String, topn:usize) -> Vec<String> {
    let walker = WalkDir::new(src2md).into_iter();
    //let mut file_modified_times = Vec::new();
/* 
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        if let Some(extension) = entry.path().extension() {
            if extension == "md" {
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    if let Ok(modified_time) = metadata.modified() {
                        file_modified_times.push((entry.path().to_owned(), modified_time));
                    }
                }
            }
        }
    }
 */
    let mut file_modified_times = walker
        .filter_map(Result::ok)     // only got Ok
        .map(|e| e.into()) // for e as walkdir::DirEntry
        .filter(|e| !is_hidden(e))
        .filter(
            |e| e.path().extension().map_or(false
                                                , |ext| ext == "md"))
        .filter_map(|e| {
            fs::metadata(e.path())
                .ok()
                .and_then(
                    |m| m.modified().ok().map(
                            |t| (e.path().to_owned(), t)
                        )
                    )
        })
        .collect::<Vec<_>>();

    // 排序
    file_modified_times.sort_by_key(|(_, time)| time.clone());

    // 获取最新的5个文件，过滤掉包含 SUMMARY.md 的路径
    let newest_files: Vec<String> = file_modified_times
        .iter()
        .rev()
        .filter(|(path, _)| !path.to_string_lossy().contains("SUMMARY.md"))
        .take(topn)//.take(5)
        .map(|(path, _)| path.to_string_lossy().to_string())
        .collect();
//  return the lasted5md
    newest_files

}


fn rss4top5md(uri:String
    , rss:String
    , src2md:String
    , latest5files: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    // 创建一个 RSS channel
    let mut channel = Channel::default();
    // 设置 channel 的元数据
    channel.title = util::RSS_TITLE.to_string();//"My RSS feed".to_owned();
    channel.link = uri.clone();//"https://example.com".to_owned();
    channel.description = util::RSS_DESC.to_string();//"This is my RSS feed".to_owned();
    channel.generator = Some("my_rss_generator".to_owned());

    // 为每个文件创建 RSS item
    for file in latest5files {
        let _p4src = site_uri(file.clone(), &src2md);
        //log::debug!("\n_p4src:{}",_p4src);
        let _uri4md = &_p4src[.._p4src.len()-3];
        //log::debug!("_uri4md:{}",_uri4md);
        //println!("_uri4md: {}/{}",uri.clone(), _uri4md);

        let metadata = fs::metadata(&file)?;
        let date = DateTime::<Local>::from(metadata.modified()?)
            .to_rfc2822();
        let content = fs::read_to_string(&file)?;
        let file_path = PathBuf::from(&file);
        let file_name = file_path.file_name().unwrap().to_string_lossy().into_owned();
        
        let item = Item {
            title: Some(file_name),
            //link: None,
            link: Some(format!("{}/{}",uri.clone(), _uri4md)),
            description: None,
            author: None,
            categories: vec![],
            comments: None,
            enclosure: None,
            guid: None,
            pub_date: Some(date),
            source: None,
            content: Some(content.into()),
            ..Default::default()
        };
        channel.items.push(item);
    }
    // Write the RSS XML to the output file
    let mut output_file = File::create(rss)?;
    output_file.write_fmt(format_args!("{}", channel.to_string()))?;

    Ok(())
}


//use std::path::{Path, Component};
fn site_uri(path: String, base: &str) -> String {
    log::debug!("\n {} ~ {}",path, base);
/* 
    let parent_iter = Path::new(&path)
        .ancestors()
        .skip_while(|p| p != &Path::new(base))
        .next()
        .and_then(|p| p.strip_prefix(base))
        .and_then(|p| p.components().next())
        .map(|p| p.as_os_str())
        .and_then(|p| p.to_str())
        .map(|p| p.to_owned());

    if let Some(parent) = parent_iter {
        let mut uri = String::new();
        uri.push_str(&parent);
        uri
    } else {
        String::new()
    }
 */    
    let parent_iter = Path::new(&path)
        .ancestors()
        //.skip_while(|p| p != &Path::new(base))
        .next()
        .unwrap()
        .strip_prefix(base)
        .unwrap()
        .components()
        .rev();

//log::debug!("\n {:?}",parent_iter.clone());

    let mut uri = String::new();
    for component in parent_iter {
        match component {
            Component::Normal(normal) => {
                uri.insert_str(0, normal.to_str().unwrap());
                uri.insert(0, '/');
//log::debug!("~ {}",uri.clone());
            },
            _ => {}
        }
    }
    uri 


}


fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_directory(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    match path.parent() {
        Some(parent) => Some(parent.to_str().unwrap().to_owned()),
        None => None,
    }
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}


