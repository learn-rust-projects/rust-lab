use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")?;

    let haystack = "In the beginning, there was 1.0.0. \
                    For a while, we used 1.0.1-beta, \
                    but in the end, we settled on 1.2.4.";

    // 多次匹配，收集所有语义化版本号
    let matches: Vec<&str> = semver
        .find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();

    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    Ok(())
}
