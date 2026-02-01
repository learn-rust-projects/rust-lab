#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}
// 辅助函数 city_population_descending 会接受 City
// 型记录并提取其键，该键是我们对数据进行排序时要依据的字段。
// ​（它会返回一个负数，因为 sort
// 会按升序排列数值，而我们想要按降序排列：让人口最多的城市在前。​）sort_by_key
// 方法会将这个取键函数作为参数。

/// 按照人口数量对城市进行排序的辅助函数
fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut [City]) {
    //  // cities.sort(); // 出错：你到底想怎么排序？
    cities.sort_by_key(city_population_descending); // 正确
}
// 它会接受一个参数 city 并返回 -city.population。Rust
// 会从闭包的使用方式中推断出其参数类型和返回类型。
fn sort_cities_by_closure(cities: &mut [City]) {
    cities.sort_by_key(|city| city.population);
}
fn main() {
    let mut cities = vec![
        City {
            name: "Delhi".to_string(),
            population: 30_290_936,
            country: "India".to_string(),
        },
        City {
            name: "Tokyo".to_string(),
            population: 37_400_068,
            country: "Japan".to_string(),
        },
        City {
            name: "Shanghai".to_string(),
            population: 27_058_480,
            country: "China".to_string(),
        },
    ];
    println!("{:#?}", cities);
    sort_cities(&mut cities);
    println!("{:#?}", cities);
    sort_cities_by_closure(&mut cities);
    println!("{:#?}", cities);
    println!("Hello, world!");
    fn apply_mut<F>(x: i32, f: &mut F) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        f(x)
    }

    let mut accum = 0;
    let mut add_to_accum = |n| {
        accum += n;
        accum
    };

    let result = apply_mut(5, &mut add_to_accum);
    println!("result: {}", result);
    let result2 = apply_mut(3, &mut add_to_accum);
    println!("result2: {}", result2);
}
