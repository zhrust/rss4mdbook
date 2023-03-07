use crate::inv::util;

pub fn set(book: String, path: String) {
    //println!("set ENV:\n\t {}={}", book, path);
    if book =="book" {
        println!("upd..env => {}={}",util::ENV_BOOK, &path);
        util::upd_denv(util::ENV_BOOK, &path);
/*         
    }else if name =="docs" {
        println!("upd..env => {}={}",util::ENV_DOCS, &path);
        util::upd_denv(util::ENV_DOCS, &path);
 */        
    }else {
        println!(r#" ALERT! cfg command only support 1 option :
$ rss4mdbook cfg book path/2/u/mdbook/book,toml

means:
    ~> point the book.toml of your local mdBook site
"#);
    }

    //log::debug!("src/inv/upd:\n\t {} \n\t{}", code, word);
}
