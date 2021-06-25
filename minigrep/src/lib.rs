use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        let cfg = Config {
            query,
            filename,
            case_sensitive: case_insensitive,
        };
        Ok(cfg)
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    //读取文件
    let contents = fs::read_to_string(cfg.filename)?;

    let res = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_insensitive(&cfg.query, &contents)
    };

    for line in res {
        println!("{}", line)
    }

    Ok(())
}

//search 搜索逻辑
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines()
        .filter(|x|x.contains(query))
        .collect()

}

//search_insensitive 搜索逻辑 不分大小写
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    contents.lines()
        .filter(|x|x.contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*; //todo: ??

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let s = search(query, contents);
        println!("{:#?}", s);
        assert_eq!(vec!["safe, fast, productive."], s)
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let s = search(query, contents);
        // println!("{:#?}", s);
        assert_eq!(vec!["Rust:", "Trust me."], s)
    }
}
