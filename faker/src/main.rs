use fake::Dummy;
use fake::Faker;

/// 简单枚举 - 使用 Dummy derive
#[derive(Debug, Clone, Dummy)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

/// 带关联值的枚举 - 使用 Dummy derive
#[derive(Debug, Clone, Dummy)]
enum Status {
    Active(String),
    Inactive,
    Pending(u32),
}

/// 带数据的枚举变体 - 使用 Dummy derive
#[derive(Debug, Clone, Dummy)]
enum Product {
    Physical { name: String, price: f64 },
    Digital { url: String, size_mb: u32 },
    Service { description: String, hours: u8 },
}

fn main() {
    println!("=== 简单枚举 (Dummy) ===");
    for _ in 0..5 {
        println!("{:?}", Color::dummy(&Faker));
    }

    println!("\n=== 带关联值的枚举 (Dummy) ===");
    for _ in 0..5 {
        println!("{:?}", Status::dummy(&Faker));
    }

    println!("\n=== 带数据的枚举变体 (Dummy) ===");
    for _ in 0..5 {
        println!("{:?}", Product::dummy(&Faker));
    }
}
