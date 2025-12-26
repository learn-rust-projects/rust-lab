use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumMessage, EnumString, IntoEnumIterator};

/// 颜色枚举 - 展示strum的基本功能
#[derive(
    Debug, Clone, Copy, PartialEq, Display, EnumString, EnumIter, EnumCount, EnumMessage, AsRefStr,
)]
pub enum Color {
    #[strum(serialize = "红色", to_string = "Red", message = "热情的颜色")]
    Red,
    #[strum(serialize = "绿色", to_string = "Green", message = "自然的颜色")]
    Green,
    #[strum(serialize = "蓝色", to_string = "Blue", message = "冷静的颜色")]
    Blue,
    #[strum(serialize = "黄色", to_string = "Yellow", message = "明亮的颜色")]
    Yellow,
}

/// 状态枚举 - 展示更多strum功能
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, EnumIter, EnumCount, AsRefStr)]
pub enum Status {
    // 最后一个serialize就是AsRefStr的实现
    #[strum(serialize = "pending", serialize = "等待中", serialize = "等待中1")]
    Pending,
    #[strum(serialize = "processing", serialize = "处理中")]
    Processing,
    #[strum(serialize = "completed", serialize = "已完成")]
    Completed,
    #[strum(serialize = "failed", serialize = "失败")]
    Failed,
}

/// 方向枚举 - 展示默认字符串转换
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    println!("=== Strum库功能演示 ===\n");

    // 1. Display trait 演示 - 转换为字符串
    println!("1. Display trait 演示:");
    let color = Color::Red;
    println!("   Color::Red 显示为: {}", color);
    println!("   Color::Green 显示为: {}", Color::Green);
    println!();

    // 2. EnumString trait 演示 - 从字符串解析
    println!("2. EnumString trait 演示:");
    match "红色".parse::<Color>() {
        Ok(color) => println!("   解析'红色'成功: {:?}", color),
        Err(e) => println!("   解析失败: {}", e),
    }

    match "Green".parse::<Color>() {
        Ok(color) => println!("   解析'Green'成功: {:?}", color),
        Err(e) => println!("   解析失败: {}", e),
    }
    println!();

    // 3. EnumIter trait 演示 - 迭代枚举值
    println!("3. EnumIter trait 演示:");
    println!("   所有颜色:");
    for color in Color::iter() {
        println!("     - {}", color);
    }
    println!();

    // 4. EnumCount trait 演示 - 获取枚举变体数量
    println!("4. EnumCount trait 演示:");
    println!("   颜色变体数量: {}", Color::COUNT);
    println!("   状态变体数量: {}", Status::COUNT);
    println!();

    // 5. EnumMessage trait 演示 - 获取消息
    println!("5. EnumMessage trait 演示:");
    if let Some(message) = Color::Red.get_message() {
        println!("   Color::Red 的消息: {}", message);
    }
    if let Some(message) = Color::Green.get_message() {
        println!("   Color::Green 的消息: {}", message);
    }
    println!();

    // 6. 方向枚举演示
    println!("6. 方向枚举演示:");
    for direction in Direction::iter() {
        println!("   {} -> {}", direction, direction);
    }
    println!();

    // 7. 状态枚举演示
    println!("7. 状态枚举演示:");
    let statuses = ["等待中", "处理中", "已完成", "failed"];
    for status_str in statuses {
        match status_str.parse::<Status>() {
            Ok(status) => println!("   解析'{}'成功: {:?}", status_str, status),
            Err(_) => println!("   无法解析'{}'", status_str),
        }
    }
    println!();

    // 8. 实用功能演示
    println!("8. 实用功能演示:");
    println!("   所有状态值:");
    for status in Status::iter() {
        println!("     - {} (显示为: {})", status, status);
    }

    // 9. 错误处理演示
    println!("\n9. 错误处理演示:");
    match "无效颜色".parse::<Color>() {
        Ok(color) => println!("   解析成功: {:?}", color),
        Err(e) => println!("   解析'无效颜色'失败: {}", e),
    }
    // 10. 方向枚举解析
    println!("10. 方向枚举解析:");
    match "north".parse::<Direction>() {
        Ok(direction) => println!("   解析'north'成功: {:?}", direction),
        Err(e) => println!("   解析'north'失败: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Red.to_string(), "红色");
        assert_eq!(Color::Green.to_string(), "绿色");
        assert_eq!(Color::Blue.to_string(), "蓝色");
        assert_eq!(Color::Yellow.to_string(), "黄色");
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
    fn test_status_display() {
        assert_eq!(Status::Pending.to_string(), "pending");
        assert_eq!(Status::Processing.to_string(), "processing");
        assert_eq!(Status::Completed.to_string(), "completed");
        assert_eq!(Status::Failed.to_string(), "failed");
    }

    #[test]
    fn test_status_from_string() {
        assert_eq!("等待中".parse::<Status>().unwrap(), Status::Pending);
        assert_eq!("处理中".parse::<Status>().unwrap(), Status::Processing);
        assert_eq!("已完成".parse::<Status>().unwrap(), Status::Completed);
        assert_eq!("失败".parse::<Status>().unwrap(), Status::Failed);

        // 测试英文别名
        assert_eq!("pending".parse::<Status>().unwrap(), Status::Pending);
        assert_eq!("completed".parse::<Status>().unwrap(), Status::Completed);
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(Direction::North.to_string(), "North");
        assert_eq!(Direction::South.to_string(), "South");
        assert_eq!(Direction::East.to_string(), "East");
        assert_eq!(Direction::West.to_string(), "West");
    }

    #[test]
    fn test_direction_iteration() {
        let directions: Vec<Direction> = Direction::iter().collect();
        assert_eq!(directions.len(), 4);
        assert!(directions.contains(&Direction::North));
        assert!(directions.contains(&Direction::South));
        assert!(directions.contains(&Direction::East));
        assert!(directions.contains(&Direction::West));
    }

    #[test]
    fn test_invalid_parsing() {
        assert!("无效值".parse::<Color>().is_err());
        assert!("unknown".parse::<Status>().is_err());
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(Color::Red, Color::Red);
        assert_ne!(Color::Red, Color::Green);

        assert_eq!(Status::Pending, Status::Pending);
        assert_ne!(Status::Pending, Status::Completed);
    }

    #[test]
    fn test_comprehensive_functionality() {
        // 综合测试所有功能
        let color = Color::Blue;

        // 测试Display
        assert_eq!(color.to_string(), "蓝色");

        // 测试从字符串解析
        let parsed_color = "蓝色".parse::<Color>().unwrap();
        assert_eq!(color, parsed_color);

        // 测试消息
        assert_eq!(color.get_message(), Some("冷静的颜色"));

        // 测试迭代包含
        let colors: Vec<Color> = Color::iter().collect();
        assert!(colors.contains(&color));
    }
    #[test]
    fn test_status_as_ref_str() {
        // 测试AsRefStr
        let status = Status::Pending;
        assert_eq!(status.as_ref(), "pending");
    }
}
