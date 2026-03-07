#![feature(macro_metavar_expr)]
#![recursion_limit = "512"]
macro_rules! demo_simple {
    ( $( $ident:ident ),* ) => {
        $(
            // index() 和 len() 需要在重复内部使用
            println!(
                "ident: {}, index: {}, length: {}",
                stringify!($ident),
                ${index()},
                ${len()}
            );

            // 使用 ignore 展开为空（不产生任何输出）
            ${ignore($ident)}
        )*

        // count($ident) 需要放在重复外部
        println!("Total count: {}", ${count($ident)});

        // 打印转义的 $
        println!("Escaped dollar: {}", '$');
    };
}

fn main() {
    demo_simple!(x, y, z);

    println!("\n========== 嵌套示例 ==========\n");
}
