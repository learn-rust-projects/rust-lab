#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::time::Instant;

use enum_dispatch::enum_dispatch;
use rand::Rng;
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Circle {
    radius: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for Circle {
    #[inline]
    fn clone(&self) -> Circle {
        Circle {
            radius: ::core::clone::Clone::clone(&self.radius),
        }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
struct Rectangle {
    width: f64,
    height: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for Rectangle {
    #[inline]
    fn clone(&self) -> Rectangle {
        Rectangle {
            width: ::core::clone::Clone::clone(&self.width),
            height: ::core::clone::Clone::clone(&self.height),
        }
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for Triangle {
    #[inline]
    fn clone(&self) -> Triangle {
        Triangle {
            side_a: ::core::clone::Clone::clone(&self.side_a),
            side_b: ::core::clone::Clone::clone(&self.side_b),
            side_c: ::core::clone::Clone::clone(&self.side_c),
        }
    }
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}
enum Shapes {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}
#[automatically_derived]
impl ::core::clone::Clone for Shapes {
    #[inline]
    fn clone(&self) -> Shapes {
        match self {
            Shapes::Circle(__self_0) => Shapes::Circle(::core::clone::Clone::clone(__self_0)),
            Shapes::Rectangle(__self_0) => Shapes::Rectangle(::core::clone::Clone::clone(__self_0)),
            Shapes::Triangle(__self_0) => Shapes::Triangle(::core::clone::Clone::clone(__self_0)),
        }
    }
}
impl ::core::convert::From<Circle> for Shapes {
    fn from(v: Circle) -> Shapes {
        Shapes::Circle(v)
    }
}
impl ::core::convert::From<Rectangle> for Shapes {
    fn from(v: Rectangle) -> Shapes {
        Shapes::Rectangle(v)
    }
}
impl ::core::convert::From<Triangle> for Shapes {
    fn from(v: Triangle) -> Shapes {
        Shapes::Triangle(v)
    }
}
impl ::core::convert::TryInto<Circle> for Shapes {
    type Error = &'static str;
    fn try_into(
        self,
    ) -> ::core::result::Result<Circle, <Self as ::core::convert::TryInto<Circle>>::Error> {
        match self {
            Shapes::Circle(v) => Ok(v),
            Shapes::Rectangle(v) => Err("Tried to convert variant Rectangle to Circle"),
            Shapes::Triangle(v) => Err("Tried to convert variant Triangle to Circle"),
        }
    }
}
impl ::core::convert::TryInto<Rectangle> for Shapes {
    type Error = &'static str;
    fn try_into(
        self,
    ) -> ::core::result::Result<Rectangle, <Self as ::core::convert::TryInto<Rectangle>>::Error>
    {
        match self {
            Shapes::Rectangle(v) => Ok(v),
            Shapes::Circle(v) => Err("Tried to convert variant Circle to Rectangle"),
            Shapes::Triangle(v) => Err("Tried to convert variant Triangle to Rectangle"),
        }
    }
}
impl ::core::convert::TryInto<Triangle> for Shapes {
    type Error = &'static str;
    fn try_into(
        self,
    ) -> ::core::result::Result<Triangle, <Self as ::core::convert::TryInto<Triangle>>::Error> {
        match self {
            Shapes::Triangle(v) => Ok(v),
            Shapes::Circle(v) => Err("Tried to convert variant Circle to Triangle"),
            Shapes::Rectangle(v) => Err("Tried to convert variant Rectangle to Triangle"),
        }
    }
}
impl Shape for Shapes {
    #[inline]
    fn area(&self) -> f64 {
        match self {
            Shapes::Circle(inner) => Shape::area(inner),
            Shapes::Rectangle(inner) => Shape::area(inner),
            Shapes::Triangle(inner) => Shape::area(inner),
        }
    }
    #[inline]
    fn perimeter(&self) -> f64 {
        match self {
            Shapes::Circle(inner) => Shape::perimeter(inner),
            Shapes::Rectangle(inner) => Shape::perimeter(inner),
            Shapes::Triangle(inner) => Shape::perimeter(inner),
        }
    }
}
fn main() {
    {
        ::std::io::_print(format_args!(
            "=== enum_dispatch 性能测试与动态类型创建演示 ===\n\n",
        ));
    };
    {
        ::std::io::_print(format_args!("1. 基本形状计算演示:\n"));
    };
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
    {
        ::std::io::_print(format_args!(
            "   圆形 (半径: {0:.2}): 面积 = {1:.2}, 周长 = {2:.2}\n",
            circle.radius,
            circle.area(),
            circle.perimeter(),
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   矩形 (宽: {0:.2}, 高: {1:.2}): 面积 = {2:.2}, 周长 = {3:.2}\n",
            rectangle.width,
            rectangle.height,
            rectangle.area(),
            rectangle.perimeter(),
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   三角形 (边长: {0:.2}, {1:.2}, {2:.2}): 面积 = {3:.2}, 周长 = {4:.2}\n",
            triangle.side_a,
            triangle.side_b,
            triangle.side_c,
            triangle.area(),
            triangle.perimeter(),
        ));
    };
    {
        ::std::io::_print(format_args!("\n"));
    };
    {
        ::std::io::_print(format_args!("2. 直接枚举创建演示:\n"));
    };
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
        {
            ::std::io::_print(format_args!(
                "   形状: {0}, 面积: {1:.2}, 周长: {2:.2}\n",
                get_shape_name(shape),
                shape.area(),
                shape.perimeter(),
            ));
        };
    }
    {
        ::std::io::_print(format_args!("\n"));
    };
    {
        ::std::io::_print(format_args!("3. 动态随机类型创建和性能比较:\n"));
    };
    let mut rng = rand::thread_rng();
    let num_shapes = 5000000;
    {
        ::std::io::_print(format_args!(
            "   生成 {0} 个随机形状进行性能测试\n",
            num_shapes
        ));
    };
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
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        enum_dispatch_shapes.push(shape);
    }
    let enum_dispatch_creation_time = start_time.elapsed();
    let start_calc_time = Instant::now();
    let total_area_enum = enum_dispatch_shapes
        .iter()
        .map(|shape| shape.area())
        .sum::<f64>();
    let enum_dispatch_calculation_time = start_calc_time.elapsed();
    let total_enum_dispatch_time = enum_dispatch_creation_time + enum_dispatch_calculation_time;
    {
        ::std::io::_print(format_args!(
            "   enum_dispatch 总时间: {0:?}\n",
            total_enum_dispatch_time
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   - 创建时间: {0:?}\n",
            enum_dispatch_creation_time
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   - 计算时间: {0:?}\n",
            enum_dispatch_calculation_time
        ));
    };
    {
        ::std::io::_print(format_args!("   - 总面积: {0:.2}\n", total_area_enum));
    };
    {
        ::std::io::_print(format_args!("\n"));
    };
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
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        box_dyn_shapes.push(shape);
    }
    let box_dyn_creation_time = start_time.elapsed();
    let start_calc_time = Instant::now();
    let total_area_box = box_dyn_shapes.iter().map(|shape| shape.area()).sum::<f64>();
    let box_dyn_calculation_time = start_calc_time.elapsed();
    let total_box_dyn_time = box_dyn_creation_time + box_dyn_calculation_time;
    {
        ::std::io::_print(format_args!(
            "   Box<dyn> 总时间: {0:?}\n",
            total_box_dyn_time
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   - 创建时间: {0:?}\n",
            box_dyn_creation_time
        ));
    };
    {
        ::std::io::_print(format_args!(
            "   - 计算时间: {0:?}\n",
            box_dyn_calculation_time
        ));
    };
    {
        ::std::io::_print(format_args!("   - 总面积: {0:.2}\n", total_area_box));
    };
    {
        ::std::io::_print(format_args!("\n"));
    };
    {
        ::std::io::_print(format_args!("   性能比较结果:\n"));
    };
    if total_enum_dispatch_time < total_box_dyn_time {
        let improvement = (total_box_dyn_time.as_nanos() as f64
            - total_enum_dispatch_time.as_nanos() as f64)
            / total_box_dyn_time.as_nanos() as f64
            * 100.0;
        {
            ::std::io::_print(format_args!(
                "   enum_dispatch 比 Box<dyn> 快 {0:.2}%!\n",
                improvement
            ));
        };
    } else {
        let slowdown = (total_enum_dispatch_time.as_nanos() as f64
            - total_box_dyn_time.as_nanos() as f64)
            / total_box_dyn_time.as_nanos() as f64
            * 100.0;
        {
            ::std::io::_print(format_args!(
                "   Box<dyn> 比 enum_dispatch 快 {0:.2}%!\n",
                slowdown
            ));
        };
    }
    {
        ::std::io::_print(format_args!("\n"));
    };
    {
        ::std::io::_print(format_args!("4. 类型转换演示:\n"));
    };
    let circle_shape = Shapes::Circle(Circle { radius: 7.0 });
    if let Shapes::Circle(c) = circle_shape {
        {
            ::std::io::_print(format_args!("   转换为圆形: 半径 = {0}\n", c.radius));
        };
    }
    {
        ::std::io::_print(format_args!("\n"));
    };
    {
        ::std::io::_print(format_args!("5. 方法重载演示:\n"));
    };
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
        {
            std::io::_print(format_args!(
                "   形状: {0}, 面积: {1:.2}, 周长: {2:.2}\n",
                get_shape_name(shape),
                shape.area(),
                shape.perimeter(),
            ));
        };
    }
}
fn get_shape_name(shape: &Shapes) -> &'static str {
    match shape {
        Shapes::Circle(_) => "圆形",
        Shapes::Rectangle(_) => "矩形",
        Shapes::Triangle(_) => "三角形",
    }
}
fn is_regular_shape(shape: &Shapes) -> bool {
    match shape {
        Shapes::Circle(_) => true,
        Shapes::Rectangle(rect) => rect.width == rect.height,
        Shapes::Triangle(tri) => tri.side_a == tri.side_b && tri.side_b == tri.side_c,
    }
}
