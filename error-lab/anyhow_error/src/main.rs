use std::{fs, io::Read};

use anyhow::{Context, Result, anyhow, bail, ensure};

fn auto_convert() -> Result<String> {
    let content = fs::read_to_string("nonexistent.txt")?;
    Ok(content)
}

fn return_result() -> Result<i32> {
    Ok(42)
}

fn use_bail() -> Result<i32> {
    let n = -5;
    if n < 0 {
        bail!("Number must be non-negative");
    }
    Ok(n)
}

fn use_anyhow_macro() -> Result<()> {
    Err(anyhow!("something went wrong: {}", 42))
}

fn use_context() -> Result<String> {
    let content = fs::read_to_string("config.toml").context("failed to read config.toml")?;
    Ok(content)
}

fn use_with_context() -> Result<String> {
    let path = "test.txt";
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read from {}", path))?;
    Ok(content)
}

fn use_ensure() -> Result<i32> {
    let n = 10;
    ensure!(n > 0, "Number must be positive, got {}", n);
    Ok(n)
}

fn check_error_chain() -> Result<i32> {
    let result = fs::read_to_string("missing.txt").context("Reading config");
    result.map(|_| 0)
}

fn downcast_error() -> Result<i32> {
    let err = anyhow!("Custom error: {}", 42);
    if let Some(msg) = err.downcast_ref::<&str>() {
        println!("  Caught string error: {}", msg);
    }
    if let Some(msg) = err.downcast_ref::<String>() {
        println!("  Caught string error: {}", msg);
    }
    Ok(42)
}

fn main() -> Result<()> {
    println!("=== anyhow 用法案例 ===\n");

    println!("1. 自动错误转换:");
    match auto_convert() {
        Ok(s) => println!("  {}", s),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n2. 类型 anyhow::Result:");
    match return_result() {
        Ok(n) => println!("  返回值: {}", n),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n3. bail! 宏:");
    match use_bail() {
        Ok(n) => println!("  {}", n),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n4. anyhow! 宏:");
    match use_anyhow_macro() {
        Ok(_) => println!("  Success"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n5. .context():");
    match use_context() {
        Ok(s) => println!("  {}", s),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n6. .with_context():");
    match use_with_context() {
        Ok(s) => println!("  {}", s),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n7. ensure! 宏:");
    match use_ensure() {
        Ok(n) => println!("  Number: {}", n),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n8. 错误链查看 .root_cause():");
    match check_error_chain() {
        Ok(_) => println!("  Success"),
        Err(e) => {
            println!("  Error chain: {:?}", e);
            println!("  Root cause: {}", e.root_cause());
        }
    }

    println!("\n9. downcast_ref:");
    match downcast_error() {
        Ok(n) => println!("  Result: {}", n),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n=== 完成 ===");
    Ok(())
}
