use std::io::{self, BufRead};

use lazy_static::lazy_static;
use regex::Regex;

// 延迟初始化全局正则
lazy_static! {
    static ref SEMVER: Regex =
        Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[\w.-]*)?").expect("error parsing regex");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    // 按行读取标准输入
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        // 匹配语义化版本号
        if let Some(mat) = SEMVER.find(&line) {
            println!("{}", mat.as_str());
        }
    }

    Ok(())
}
