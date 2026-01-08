use std::time::Instant;

use enum_dispatch::enum_dispatch;
use rand::Rng;

#[enum_dispatch]
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Clone)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[derive(Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

#[derive(Clone)]
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // 使用海伦公式计算面积
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}

#[enum_dispatch(Shape)]
#[derive(Clone)]
enum Shapes {
    Circle,
    Rectangle,
    Triangle,
}

fn main() {
    println!("=== enum_dispatch 性能测试与动态类型创建演示 ===\n");

    // 1. 基本计算演示
    println!("1. 基本形状计算演示:");
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    let triangle = Triangle {
        side_a: 3.0,
        side_b: 4.0,
        side_c: 5.0,
    };

    println!(
        "   圆形 (半径: {:.2}): 面积 = {:.2}, 周长 = {:.2}",
        circle.radius,
        circle.area(),
        circle.perimeter()
    );
    println!(
        "   矩形 (宽: {:.2}, 高: {:.2}): 面积 = {:.2}, 周长 = {:.2}",
        rectangle.width,
        rectangle.height,
        rectangle.area(),
        rectangle.perimeter()
    );
    println!(
        "   三角形 (边长: {:.2}, {:.2}, {:.2}): 面积 = {:.2}, 周长 = {:.2}",
        triangle.side_a,
        triangle.side_b,
        triangle.side_c,
        triangle.area(),
        triangle.perimeter()
    );
    println!();

    // 2. 直接枚举创建演示
    println!("2. 直接枚举创建演示:");
    let shapes_enum = [
        Shapes::Circle(Circle { radius: 3.0 }),
        Shapes::Rectangle(Rectangle {
            width: 2.0,
            height: 8.0,
        }),
        Shapes::Triangle(Triangle {
            side_a: 5.0,
            side_b: 5.0,
            side_c: 5.0,
        }),
    ];

    for shape in &shapes_enum {
        println!(
            "   形状: {}, 面积: {:.2}, 周长: {:.2}",
            get_shape_name(shape),
            shape.area(),
            shape.perimeter()
        );
    }
    println!();

    // 3. 动态随机类型创建和性能比较
    println!("3. 动态随机类型创建和性能比较:");

    // 生成随机数量的形状
    let mut rng = rand::thread_rng();
    let num_shapes = 5000000; // 随机长度

    println!("   生成 {} 个随机形状进行性能测试", num_shapes);

    // 3a. 使用 enum_dispatch 的性能测试
    let start_time = Instant::now();
    let mut enum_dispatch_shapes = Vec::new();

    for _ in 0..num_shapes {
        let shape_type = rng.gen_range(0..=2);
        let random_param1 = rng.gen_range(1.0..=10.0);
        let random_param2 = rng.gen_range(1.0..=10.0);
        let random_param3 = rng.gen_range(1.0..=10.0);

        let shape: Shapes = match shape_type {
            0 => Circle {
                radius: random_param1,
            }
            .into(),
            1 => Rectangle {
                width: random_param1,
                height: random_param2,
            }
            .into(),
            2 => Triangle {
                side_a: random_param1,
                side_b: random_param2,
                side_c: random_param3,
            }
            .into(),
            _ => unreachable!(),
        };

        enum_dispatch_shapes.push(shape);
    }

    let enum_dispatch_creation_time = start_time.elapsed();

    // 计算所有形状的面积总和
    let start_calc_time = Instant::now();
    let total_area_enum = enum_dispatch_shapes
        .iter()
        .map(|shape| shape.area())
        .sum::<f64>();
    let enum_dispatch_calculation_time = start_calc_time.elapsed();

    let total_enum_dispatch_time = enum_dispatch_creation_time + enum_dispatch_calculation_time;

    println!("   enum_dispatch 总时间: {:?}", total_enum_dispatch_time);
    println!("   - 创建时间: {:?}", enum_dispatch_creation_time);
    println!("   - 计算时间: {:?}", enum_dispatch_calculation_time);
    println!("   - 总面积: {:.2}", total_area_enum);
    println!();

    // 3b. 使用 Box<dyn> 的性能测试
    let start_time = Instant::now();
    let mut box_dyn_shapes: Vec<Box<dyn Shape>> = Vec::new();

    for _ in 0..num_shapes {
        let shape_type = rng.gen_range(0..=2);
        let random_param1 = rng.gen_range(1.0..=10.0);
        let random_param2 = rng.gen_range(1.0..=10.0);
        let random_param3 = rng.gen_range(1.0..=10.0);

        let shape: Box<dyn Shape> = match shape_type {
            0 => Box::new(Circle {
                radius: random_param1,
            }),
            1 => Box::new(Rectangle {
                width: random_param1,
                height: random_param2,
            }),
            2 => Box::new(Triangle {
                side_a: random_param1,
                side_b: random_param2,
                side_c: random_param3,
            }),
            _ => unreachable!(),
        };

        box_dyn_shapes.push(shape);
    }

    let box_dyn_creation_time = start_time.elapsed();

    // 计算所有形状的面积总和
    let start_calc_time = Instant::now();
    let total_area_box = box_dyn_shapes.iter().map(|shape| shape.area()).sum::<f64>();
    let box_dyn_calculation_time = start_calc_time.elapsed();

    let total_box_dyn_time = box_dyn_creation_time + box_dyn_calculation_time;

    println!("   Box<dyn> 总时间: {:?}", total_box_dyn_time);
    println!("   - 创建时间: {:?}", box_dyn_creation_time);
    println!("   - 计算时间: {:?}", box_dyn_calculation_time);
    println!("   - 总面积: {:.2}", total_area_box);
    println!();

    // 3c. 性能比较
    println!("   性能比较结果:");
    if total_enum_dispatch_time < total_box_dyn_time {
        let improvement = (total_box_dyn_time.as_nanos() as f64
            - total_enum_dispatch_time.as_nanos() as f64)
            / total_box_dyn_time.as_nanos() as f64
            * 100.0;
        println!("   enum_dispatch 比 Box<dyn> 快 {:.2}%!", improvement);
    } else {
        let slowdown = (total_enum_dispatch_time.as_nanos() as f64
            - total_box_dyn_time.as_nanos() as f64)
            / total_box_dyn_time.as_nanos() as f64
            * 100.0;
        println!("   Box<dyn> 比 enum_dispatch 快 {:.2}%!", slowdown);
    }
    println!();

    // 4. 类型转换演示
    println!("4. 类型转换演示:");
    let circle_shape = Shapes::Circle(Circle { radius: 7.0 });
    if let Shapes::Circle(c) = circle_shape {
        println!("   转换为圆形: 半径 = {}", c.radius);
    }
    println!();

    // 5. 方法重载演示
    println!("5. 方法重载演示:");
    let shapes = [
        Shapes::Circle(Circle { radius: 2.0 }),
        Shapes::Rectangle(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
        Shapes::Triangle(Triangle {
            side_a: 3.0,
            side_b: 4.0,
            side_c: 5.0,
        }),
    ];

    for shape in &shapes {
        println!(
            "   形状: {}, 面积: {:.2}, 周长: {:.2}",
            get_shape_name(shape),
            shape.area(),
            shape.perimeter()
        );
    }
}

// 辅助函数：获取形状名称
fn get_shape_name(shape: &Shapes) -> &'static str {
    match shape {
        Shapes::Circle(_) => "圆形",
        Shapes::Rectangle(_) => "矩形",
        Shapes::Triangle(_) => "三角形",
    }
}

// 辅助函数：判断是否为正多边形
fn is_regular_shape(shape: &Shapes) -> bool {
    match shape {
        Shapes::Circle(_) => true,
        Shapes::Rectangle(rect) => rect.width == rect.height, // 正方形
        Shapes::Triangle(tri) => tri.side_a == tri.side_b && tri.side_b == tri.side_c, // 等边三角形
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert!((circle.area() - 78.54).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        assert_eq!(rectangle.area(), 24.0);
    }

    #[test]
    fn test_triangle_area() {
        // 直角三角形 3-4-5
        let triangle = Triangle {
            side_a: 3.0,
            side_b: 4.0,
            side_c: 5.0,
        };
        assert!((triangle.area() - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_dispatch_functionality() {
        let shapes = vec![
            Shapes::Circle(Circle { radius: 1.0 }),
            Shapes::Rectangle(Rectangle {
                width: 1.0,
                height: 1.0,
            }),
        ];

        for shape in shapes {
            let area = shape.area();
            assert!(area > 0.0);
        }
    }

    #[test]
    fn test_type_conversion() {
        let circle = Shapes::Circle(Circle { radius: 5.0 });
        if let Shapes::Circle(c) = circle {
            assert_eq!(c.radius, 5.0);
        } else {
            panic!("Type conversion failed");
        }
    }

    #[test]
    fn test_shape_classification() {
        let square = Shapes::Rectangle(Rectangle {
            width: 5.0,
            height: 5.0,
        });
        assert!(is_regular_shape(&square));

        let rect = Shapes::Rectangle(Rectangle {
            width: 5.0,
            height: 3.0,
        });
        assert!(!is_regular_shape(&rect));
    }

    #[test]
    fn test_dynamic_random_creation() {
        let mut rng = rand::thread_rng();
        let num_shapes = rng.gen_range(10..=50);

        let mut shapes = Vec::new();
        for _ in 0..num_shapes {
            let shape_type = rng.gen_range(0..=2);
            let random_param1 = rng.gen_range(1.0..=10.0);
            let random_param2 = rng.gen_range(1.0..=10.0);
            let random_param3 = rng.gen_range(1.0..=10.0);

            let shape = match shape_type {
                0 => Shapes::Circle(Circle {
                    radius: random_param1,
                }),
                1 => Shapes::Rectangle(Rectangle {
                    width: random_param1,
                    height: random_param2,
                }),
                2 => Shapes::Triangle(Triangle {
                    side_a: random_param1,
                    side_b: random_param2,
                    side_c: random_param3,
                }),
                _ => unreachable!(),
            };

            shapes.push(shape);
        }

        assert_eq!(shapes.len(), num_shapes);

        // 验证所有形状都有正面积
        for shape in &shapes {
            assert!(shape.area() > 0.0);
        }
    }

    #[test]
    fn test_performance_comparison() {
        let mut rng = rand::thread_rng();
        let num_shapes = 100; // 小一点以便测试

        // enum_dispatch 性能测试
        let start_time = Instant::now();
        let mut enum_shapes = Vec::new();

        for _ in 0..num_shapes {
            let shape_type = rng.gen_range(0..=2);
            let random_param = rng.gen_range(1.0..=10.0);

            let shape: Shapes = match shape_type {
                0 => Circle {
                    radius: random_param,
                }
                .into(),
                1 => Rectangle {
                    width: random_param,
                    height: random_param,
                }
                .into(),
                2 => Triangle {
                    side_a: random_param,
                    side_b: random_param,
                    side_c: random_param,
                }
                .into(),
                _ => unreachable!(),
            };

            enum_shapes.push(shape);
        }

        let total_area_enum = enum_shapes.iter().map(|shape| shape.area()).sum::<f64>();

        let enum_dispatch_time = start_time.elapsed();

        // Box<dyn> 性能测试
        let start_time = Instant::now();
        let mut box_shapes: Vec<Box<dyn Shape>> = Vec::new();

        for _ in 0..num_shapes {
            let shape_type = rng.gen_range(0..=2);
            let random_param = rng.gen_range(1.0..=10.0);

            let shape: Box<dyn Shape> = match shape_type {
                0 => Box::new(Circle {
                    radius: random_param,
                }),
                1 => Box::new(Rectangle {
                    width: random_param,
                    height: random_param,
                }),
                2 => Box::new(Triangle {
                    side_a: random_param,
                    side_b: random_param,
                    side_c: random_param,
                }),
                _ => unreachable!(),
            };

            box_shapes.push(shape);
        }

        let total_area_box = box_shapes.iter().map(|shape| shape.area()).sum::<f64>();

        let box_dyn_time = start_time.elapsed();

        println!(
            "性能测试 - enum_dispatch: {:?}, Box<dyn>: {:?}",
            enum_dispatch_time, box_dyn_time
        );
    }
}
