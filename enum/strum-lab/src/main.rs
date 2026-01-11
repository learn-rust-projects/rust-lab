use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumMessage, EnumString, IntoEnumIterator};

/// 颜色枚举 - 展示strum的各种功能
#[derive(
    Debug, Clone, Copy, PartialEq, Display, EnumString, EnumIter, EnumCount, EnumMessage, AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
/// 【Strum serialize_all 风格转换对照表】
///
/// | 配置值 (strum value) | 原始变体名 (Enum Variant) | 输出示例 (Output) | 说明 (Description) |
/// |:-------------------|:-----------------------|:-----------------|:------------------|
/// | "lowercase"        | BlueLight              | bluelight        | 全部小写，不保留分隔 |
/// | "UPPERCASE"        | BlueLight              | BLUELIGHT        | 全部大写，不保留分隔 |
/// | "snake_case"       | BlueLight              | blue_light       | 蛇形命名，以下划线分隔 |
/// | "kebab-case"       | BlueLight              | blue-light       | 短横线命名，常用于 URL |
/// | "camelCase"        | Blue_Light             | blueLight        | 小驼峰命名，首字母小写 |
/// | "PascalCase"       | blue_light             | BlueLight        | 大驼峰命名，首字母大写 |
/// | "SCREAMING_SNAKE_CASE" | BlueLight          | BLUE_LIGHT       | 常用于常量或环境变量定义 |
/// | "shouty-kebab-case"| BlueLight              | BLUE-LIGHT       | 全大写短横线命名 |
pub enum Color {
    #[strum(serialize = "红色2", serialize = "红色1", message = "热情的颜色")]
    Red,
    #[strum(serialize = "绿色", message = "自然的颜色")]
    Green,
    #[strum(serialize = "蓝色", to_string = "Blue", message = "冷静的颜色")]
    Blue,
    #[strum(serialize = "黄色", to_string = "Yellow", message = "明亮的颜色")]
    Yellow,
}

fn main() {
    println!("=== Strum库功能演示 ===\n");

    // 1. Display trait - 对应 Display 宏
    // to_string = "Red" 会影响Display的实现 也会影响EnumString的实现
    // 序列化优先级（决定转成什么字符串）
    // (1) 显式指定 (to_string)： 如果你写了 #[strum(to_string =
    // "Red")]，就用这个值。 (2) 如果定义了多个
    // serialize：选择最后面的一个来序列化， 同时字段上的优先级比全局高
    // (3) 默认名称： 如果没写属性，直接用代码里定义的成员名（如 Red 变 "Red"）。
    // 建议:使用全局宏来统一序列化风格#[strum(serialize_all = "lowercase")]
    println!("1. Display trait (对应 Display 宏):");
    let color = Color::Red;
    println!("   Color::Red 显示为: {}", color);
    println!();

    // 2. EnumString trait - 对应 EnumString 宏
    // EnumString 提供 FromStr实现
    println!("2. EnumString trait (对应 EnumString 宏):");

    // 默认实现就是"Green",提供了serialize = "绿色"之后，默认实现失效
    // 但凡有serialize = "绿色"的或to_string = "绿色"的一个，都不能用"Green"解析
    // 解析"绿色"失败，因为默认实现是基于to_string()的
    match "Green".parse::<Color>() {
        Ok(parsed_color) => println!("   解析'Green'成功: {:?}", parsed_color),
        Err(e) => println!("   解析失败: {}", e),
    }
    // serialize = "绿色"
    match "绿色".parse::<Color>() {
        Ok(parsed_color) => println!("   解析'绿色'成功: {:?}", parsed_color),
        Err(e) => println!("   解析失败: {}", e),
    }
    println!();

    // 3. EnumIter trait - 对应 EnumIter 宏
    println!("3. EnumIter trait (对应 EnumIter 宏):");
    println!("   遍历所有颜色:");
    for color in Color::iter() {
        println!("     - {}", color);
    }
    println!();

    // 4. EnumCount trait - 对应 EnumCount 宏
    println!("4. EnumCount trait (对应 EnumCount 宏):");
    println!("   颜色总数: {}", Color::COUNT);
    println!();

    // 5. EnumMessage trait - 对应 EnumMessage 宏
    println!("5. EnumMessage trait (对应 EnumMessage 宏):");
    if let Some(message) = Color::Red.get_message() {
        println!("   Color::Red 的消息: {}", message);
    }
    println!();

    // 6. AsRefStr trait - 对应 AsRefStr 宏
    println!("6. AsRefStr trait (对应 AsRefStr 宏):");
    let color = Color::Blue;
    println!("   Color::Blue 作为引用字符串: {}", color.as_ref());
    println!();

    // 7. 综合演示 - 展示所有trait协同工作
    println!("7. 综合演示 - 所有trait协同工作:");
    println!("   遍历所有颜色并展示所有属性:");
    for color in Color::iter() {
        println!("     - 颜色: {}", color);
        println!("       引用字符串: {}", color.as_ref());
        if let Some(msg) = color.get_message() {
            println!("       消息: {}", msg);
        }
        println!("       重新解析: {:?}", color.to_string().parse::<Color>());
    }
    println!();

    // 8. 错误处理演示 - EnumString的错误处理
    println!("8. 错误处理演示 (对应 EnumString 宏):");
    match "无效颜色".parse::<Color>() {
        Ok(c) => println!("   解析成功: {:?}", c),
        Err(e) => println!("   解析失败: {}", e),
    }
    println!();

    // 9. 比较操作 - 展示衍生的trait
    println!("9. 比较操作 (对应 Debug, Clone, Copy, PartialEq 宏):");
    let red1 = Color::Red;
    let red2 = Color::Red;
    let blue = Color::Blue;

    println!("   Color::Red == Color::Red: {}", red1 == red2);
    println!("   Color::Red == Color::Blue: {}", red1 == blue);
    println!("   Debug输出: {:?}", red1);
    println!();

    println!("=== 所有功能演示完成 ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Red.to_string(), "Red");
        assert_eq!(Color::Green.to_string(), "Green");
        assert_eq!(Color::Blue.to_string(), "Blue");
        assert_eq!(Color::Yellow.to_string(), "Yellow");
    }

    #[test]
    fn test_color_from_string() {
        assert_eq!("红色".parse::<Color>().unwrap(), Color::Red);
        assert_eq!("绿色".parse::<Color>().unwrap(), Color::Green);
        assert_eq!("蓝色".parse::<Color>().unwrap(), Color::Blue);
        assert_eq!("黄色".parse::<Color>().unwrap(), Color::Yellow);

        // 测试英文别名
        assert_eq!("Red".parse::<Color>().unwrap(), Color::Red);
        assert_eq!("Green".parse::<Color>().unwrap(), Color::Green);
    }

    #[test]
    fn test_color_iteration() {
        let colors: Vec<Color> = Color::iter().collect();
        assert_eq!(colors.len(), 4);
        assert!(colors.contains(&Color::Red));
        assert!(colors.contains(&Color::Green));
        assert!(colors.contains(&Color::Blue));
        assert!(colors.contains(&Color::Yellow));
    }

    #[test]
    fn test_color_count() {
        assert_eq!(Color::COUNT, 4);
    }

    #[test]
    fn test_color_message() {
        assert_eq!(Color::Red.get_message(), Some("热情的颜色"));
        assert_eq!(Color::Green.get_message(), Some("自然的颜色"));
        assert_eq!(Color::Blue.get_message(), Some("冷静的颜色"));
        assert_eq!(Color::Yellow.get_message(), Some("明亮的颜色"));
    }

    #[test]
    fn test_status_as_ref_str() {
        let color = Color::Red;
        assert_eq!(color.as_ref(), "Red");
    }

    #[test]
    fn test_invalid_parsing() {
        assert!("无效值".parse::<Color>().is_err());
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(Color::Red, Color::Red);
        assert_ne!(Color::Red, Color::Green);
    }

    #[test]
    fn test_comprehensive_functionality() {
        // 综合测试所有功能
        let color = Color::Blue;

        // 测试Display
        assert_eq!(color.to_string(), "Blue");

        // 测试从字符串解析
        let parsed_color = "Blue".parse::<Color>().unwrap();
        assert_eq!(color, parsed_color);

        // 测试消息
        assert_eq!(color.get_message(), Some("冷静的颜色"));

        // 测试迭代包含
        let colors: Vec<Color> = Color::iter().collect();
        assert!(colors.contains(&color));

        // 测试引用字符串
        assert_eq!(color.as_ref(), "Blue");
    }
}
