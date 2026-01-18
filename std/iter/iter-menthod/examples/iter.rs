#![feature(iter_intersperse)]
use std::collections::{HashMap, HashSet};
fn main() {
    println!("=== Rust 迭代器方法测试演示 ===");

    // 基础迭代器创建和转换方法
    test_basic_iterators();

    // 过滤和查找方法
    test_filtering_methods();

    // 转换和映射方法
    test_transformation_methods();

    // 聚合和归约方法
    test_aggregation_methods();

    // 组合和分组方法
    test_combination_methods();

    // 高级迭代器方法
    test_advanced_methods();

    println!("\n=== 所有测试完成 ===");
}

fn test_basic_iterators() {
    println!("\n--- 基础迭代器方法测试 ---");

    // 1. iter() - 创建迭代器
    let vec = [1, 2, 3, 4, 5];
    let iter = vec.iter();
    println!("1. iter(): {:?}", iter.collect::<Vec<_>>());

    // 2. into_iter() - 消费迭代器
    let vec2 = [1, 2, 3];
    let sum: i32 = vec2.into_iter().sum();
    println!("2. into_iter().sum(): {}", sum);

    // 3. iter_mut() - 可变迭代器
    let mut vec3 = vec![1, 2, 3];
    for item in vec3.iter_mut() {
        *item *= 2;
    }
    println!("3. iter_mut() 修改后: {:?}", vec3);

    // 4. enumerate() - 带索引的迭代器
    let items = ["a", "b", "c"];
    let enumerated: Vec<_> = items.iter().enumerate().collect();
    println!("4. enumerate(): {:?}", enumerated);

    // 5. rev() - 反向迭代器
    let binding = [1, 2, 3];
    let reversed: Vec<_> = binding.iter().rev().collect();
    println!("5. rev(): {:?}", reversed);

    // 6. chain() - 连接迭代器
    let binding = [1, 2];
    let binding2 = [3, 4];
    let chain_result: Vec<_> = binding.iter().chain(binding2.iter()).collect();
    println!("6. chain(): {:?}", chain_result);

    // 7. zip() - 并行迭代
    let binding = [1, 2, 3];
    let binding2 = ["a", "b", "c"];
    let zipped: Vec<_> = binding.iter().zip(binding2.iter()).collect();
    println!("7. zip(): {:?}", zipped);

    // 8. cycle() - 循环迭代器（取前几个）
    let binding = [1, 2];
    let cycled: Vec<_> = binding.iter().cycle().take(5).collect();
    println!("8. cycle().take(5): {:?}", cycled);
}

fn test_filtering_methods() {
    println!("\n--- 过滤和查找方法测试 ---");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 9. filter() - 过滤
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("9. filter(偶数): {:?}", evens);

    // 10. filter_map() - 过滤并映射
    let filtered_map: Vec<_> = numbers
        .iter()
        // 过滤内容保留some里面的内容
        .filter_map(|&x| if x % 3 == 0 { Some(x * 2) } else { None })
        .collect();
    println!("10. filter_map(3的倍数×2): {:?}", filtered_map);

    // 11. take() - 取前n个
    let first_three: Vec<_> = numbers.iter().take(3).collect();
    println!("11. take(3): {:?}", first_three);

    // 12. take_while() - 条件取前
    // 返回满足条件的元素，一旦返回false，后续元素就会被忽略
    let take_while: Vec<_> = numbers.iter().take_while(|&&x| x < 5).collect();
    println!("12. take_while(<5): {:?}", take_while);

    // 13. skip() - 跳过前n个
    let skip_three: Vec<_> = numbers.iter().skip(3).collect();
    println!("13. skip(3): {:?}", skip_three);

    // 14. skip_while() - 条件跳过前n个
    // 返回满足条件的元素，一旦返回false，后续元素就会被保留
    let skip_while: Vec<_> = numbers.iter().skip_while(|&&x| x < 5).collect();
    println!("14. skip_while(<5): {:?}", skip_while);

    // 15. find() - 查找第一个匹配项
    // 返回第一个满足条件的元素，否则返回None
    let found = numbers.iter().find(|&&x| x > 7);
    println!("15. find(>7): {:?}", found);

    // 16. position() - 查找位置
    // 返回第一个满足条件的元素的索引，否则返回None
    let pos = numbers.iter().position(|&x| x == 5);
    println!("16. position(==5): {:?}", pos);

    // 17. rposition() - 反向查找位置
    // 返回从后往前第一个满足条件的元素的索引，否则返回None
    let rpos = numbers.iter().rposition(|&x| x == 5);
    println!("17. rposition(==5): {:?}", rpos);

    // 18. any() - 是否存在匹配项
    // 返回是否存在至少一个满足条件的元素
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("18. any(偶数): {}", has_even);

    // 19. all() - 是否全部匹配
    // 返回是否所有元素都满足条件
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("19. all(>0): {}", all_positive);
}

fn test_transformation_methods() {
    println!("\n--- 转换和映射方法测试 ---");

    let numbers = [1, 2, 3, 4, 5];

    // 20. map() - 映射
    // 对每个元素应用函数
    let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
    println!("20. map(×2): {:?}", doubled);

    // 21. flat_map() - 扁平映射
    // 对每个元素应用函数，然后将结果连接起来
    let nested = [vec![1, 2], vec![3, 4], vec![5]];
    let flattened: Vec<_> = nested.iter().flat_map(|v| v.iter()).collect();
    println!("21. flat_map(): {:?}", flattened);

    // 22. flatten() - 扁平化嵌套迭代器
    // 将嵌套的迭代器展开为一个扁平的迭代器
    let flattened2: Vec<_> = nested.iter().flatten().collect();
    println!("22. flatten(): {:?}", flattened2);

    // 23. inspect() - 查看中间值
    // 对每个元素应用函数，但是不改变元素本身
    // “在元素流过时，偷偷看一眼，顺便做点副作用，但把元素原样放行。”
    let inspected: Vec<_> = numbers
        .iter()
        .inspect(|&&x| print!("检查:{} ", x))
        .map(|&x| x * 3)
        .collect();
    println!("\n23. inspect() + map: {:?}", inspected);

    // 24. scan() - 带状态的转换
    let scanned: Vec<_> = numbers
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("24. scan(累加): {:?}", scanned);

    // 25. fuse() - 熔断迭代器
    //  ∀ n ≥ k, if iter.next() == None at k,
    // then iter.next() == None forever
    let mut iter = numbers.iter().fuse();
    println!(
        "25. fuse() 前几个: {:?}, {:?}, {:?}",
        iter.next(),
        iter.next(),
        iter.next()
    );

    // 26. cloned() - 克隆元素
    #[allow(clippy::iter_cloned_collect)]
    let cloned: Vec<_> = numbers.iter().cloned().collect();
    println!("26. cloned(): {:?}", cloned);
    #[allow(clippy::iter_cloned_collect)]
    // 27. copied() - 复制元素
    let copied: Vec<_> = numbers.iter().copied().collect();
    println!("27. copied(): {:?}", copied);
}

fn test_aggregation_methods() {
    println!("\n--- 聚合和归约方法测试 ---");

    let numbers = [1, 2, 3, 4, 5];

    // 28. fold() - 折叠 折叠（Fold）

    // 含义：从一个“初始状态”开始，把一串元素逐个“折进去”

    // 关键词：初始值、状态演化、过程性

    // 本质比喻：

    // 把一排纸条，一张一张按规则折成一个最终形状
    #[allow(clippy::unnecessary_fold)]
    let sum_fold = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("28. fold(求和): {}", sum_fold);

    // 29. reduce() - 归约 规约（Reduce）

    // 含义：把多个同类元素不断合并，最终缩减成一个元素

    // 关键词：同类型合并、规模缩小、代数结构

    // 本质比喻：

    // 把一堆石头，两两合并，直到只剩一块
    let product_reduce = numbers.iter().copied().reduce(|acc, x| acc * x);
    println!("29. reduce(乘积): {:?}", product_reduce);
    // | 维度     | `fold`    | `reduce`  |
    // | ------ | --------- | --------- |
    // | 初始状态   | 显式给定      | 取第一个元素    |
    // | 空迭代器   | 返回 `init` | 返回 `None` |
    // | 结合律假设  | 不要求       | **隐含要求**  |
    // | 是否偏序安全 | 是         | 否（依赖顺序）   |
    // | 表达意图   | 通用折叠      | 数学归约      |

    // 30. sum() - 求和
    let sum_total: i32 = numbers.iter().sum();
    println!("30. sum(): {}", sum_total);

    // 31. product() - 求积
    let product_total: i32 = numbers.iter().product();
    println!("31. product(): {}", product_total);

    // 32. count() - 计数
    let count = numbers.iter().count();
    println!("32. count(): {}", count);

    // 33. min() - 最小值
    let min_val = numbers.iter().min();
    println!("33. min(): {:?}", min_val);

    // 34. max() - 最大值
    let max_val = numbers.iter().max();
    println!("34. max(): {:?}", max_val);

    // 35. min_by() - 自定义比较最小值
    let binding = ["apple", "banana", "cherry"];
    let min_by_len = binding.iter().min_by(|a, b| a.len().cmp(&b.len()));
    println!("35. min_by(长度): {:?}", min_by_len);

    // 36. max_by() - 自定义比较最大值
    let binding = ["apple", "banana", "cherry"];
    let max_by_len = binding.iter().max_by(|a, b| a.len().cmp(&b.len()));
    println!("36. max_by(长度): {:?}", max_by_len);

    // 37. min_by_key() - 按键最小值
    let binding = ["apple", "banana", "cherry"];
    let min_by_key = binding.iter().min_by_key(|s| s.len());
    println!("37. min_by_key(长度): {:?}", min_by_key);

    // 38. max_by_key() - 按键最大值
    let binding = ["apple", "banana", "cherry"];

    let max_by_key = binding.iter().max_by_key(|s| s.len());
    println!("38. max_by_key(长度): {:?}", max_by_key);
}

fn test_combination_methods() {
    println!("\n--- 组合和分组方法测试 ---");

    // 39. collect() - 收集到集合
    let collected_vec: Vec<_> = (1..6).collect();
    println!("39. collect(Vec): {:?}", collected_vec);

    // 40. collect::<HashSet>()
    let collected_set: HashSet<_> = vec![1, 2, 2, 3, 3].into_iter().collect();
    println!("40. collect(HashSet): {:?}", collected_set);

    // 41. collect::<HashMap>()
    let collected_map: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();
    println!("41. collect(HashMap): {:?}", collected_map);

    // 42. partition() - 分区
    let (even, odd): (Vec<_>, Vec<_>) = (1..6).partition(|&x| x % 2 == 0);
    println!("42. partition(奇偶): 偶数{:?}, 奇数{:?}", even, odd);

    // 43. unzip() - 解压缩
    // 不能使用&引用，因为unzip()需要所有权
    let pairs = vec![(1, "a"), (2, "b"), (3, "c")];
    let (numbers, letters): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    println!("43. unzip(): 数字{:?}, 字母{:?}", numbers, letters);

    // 44. chunks() - 分块（需要数组）
    let array = [1, 2, 3, 4, 5, 6];
    let chunks: Vec<_> = array.chunks(4).collect();
    println!("44. chunks(2): {:?}", chunks);

    // 45. windows() - 滑动窗口
    let windows: Vec<_> = array.windows(4).collect();
    println!("45. windows(3): {:?}", windows);
    let windows: Vec<_> = array.windows(7).collect();
    println!("45. windows(7): {:?}", windows);
}

fn test_advanced_methods() {
    println!("\n--- 高级迭代器方法测试 ---");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 46. step_by() - 步进迭代
    let step_by: Vec<_> = numbers.iter().step_by(2).collect();
    println!("46. step_by(2): {:?}", step_by);

    // 47. peekable() - 可窥视迭代器
    let mut peekable = numbers.iter().peekable();
    println!(
        "47. peekable(): 下一个{:?}, 窥视{:?}",
        peekable.next(),
        peekable.peek()
    );

    // 48. nth() - 获取第n个元素
    #[allow(clippy::iter_nth)]
    let third = numbers.iter().nth(2);
    println!("48. nth(2): {:?}", third);

    // 49. last() - 最后一个元素
    let last = numbers.iter().last();
    println!("49. last(): {:?}", last);

    // 50. for_each() - 遍历执行
    // for_each 在语义上等价于 for 循环，但能力更受限；for 更通用、更符合 Rust
    // 习惯；在长迭代器链尾部，for_each 有时更清晰，极少数情况下也可能更快。
    // 闭包里不能 break / continue
    // 可读性优势 —— 作为“链式末端操作”
    print!("50. for_each(): ");
    numbers.iter().for_each(|&x| print!("{} ", x));
    println!();

    // 51. try_fold() - 可失败的折叠
    let try_fold_result: Result<i32, &str> = numbers.iter().try_fold(0, |acc, &x| {
        if x > 10 {
            Err("值太大")
        } else {
            Ok(acc + x)
        }
    });
    println!("51. try_fold(): {:?}", try_fold_result);

    // 52. try_for_each() - 可失败的遍历
    let try_foreach_result: Result<(), &str> = numbers
        .iter()
        .try_for_each(|&x| if x > 10 { Err("值太大") } else { Ok(()) });
    println!("52. try_for_each(): {:?}", try_foreach_result);

    // 更多方法演示...
    println!("53. 更多方法包括: cmp(), partial_cmp(), eq(), ne(), lt(), le(), gt(), ge()");
    println!("54. 比较方法: cmp_by(), partial_cmp_by()");
    println!("55. 数学方法: sum(), product()");
    println!("56. 字符串方法: as_str(), to_string()");

    // 测试一些不常用的方法
    test_less_common_methods();
}

fn test_less_common_methods() {
    println!("\n--- 不常用方法测试 ---");

    let numbers = [1, 2, 3, 4, 5];

    // 57. by_ref() - 借用迭代器
    let mut iter = numbers.iter();
    let first_two: Vec<_> = iter.by_ref().take(2).collect();
    println!("57. by_ref().take(2): {:?}", first_two);
    println!("   剩余: {:?}", iter.collect::<Vec<_>>());

    // 58. intersperse() - 插入分隔符
    let interspersed: Vec<_> = numbers.iter().intersperse(&0).collect();
    println!("58. intersperse(0): {:?}", interspersed);

    // 59. intersperse_with() - 自定义分隔符
    let mut counter = 0;
    let interspersed_with: Vec<_> = numbers
        .into_iter()
        .intersperse_with(|| {
            counter += 1;
            counter
        })
        .collect();
    println!("59. intersperse_with(计数器): {:?}", interspersed_with);

    // 60. map_while() - 条件映射
    let binding = [1, 2, 3, 4, 5];
    let map_while: Vec<_> = binding
        .iter()
        .map_while(|&x| if x < 4 { Some(x * 10) } else { None })
        .collect();
    println!("60. map_while(<4 → ×10): {:?}", map_while);

    // 61. take_while() 和 map() 组合
    let binding = [1, 2, 3, 4, 5];
    let complex: Vec<_> = binding
        .iter()
        .take_while(|&&x| x < 4)
        .map(|&x| x * 2)
        .collect();
    println!("61. take_while(<4) + map(×2): {:?}", complex);

    // 62. skip_while() 和 filter() 组合
    let binding = [1, 2, 3, 4, 5];
    let complex2: Vec<_> = binding
        .iter()
        .skip_while(|&&x| x < 3)
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("62. skip_while(<3) + filter(偶数): {:?}", complex2);

    // 演示更多组合方法
    println!("63-76. 其他方法包括:");
    println!("   - map_windows() (需要nightly)");
    println!("   - array_chunks() (需要nightly)");
    println!("   - array_windows() (需要nightly)");
    println!("   - next_chunk()");
    println!("   - advance_by()");
    println!("   - rfold()");
    println!("   - rfind()");
    println!("   - try_collect()");
    println!("   - try_reduce()");
    println!("   - cmp() 系列比较方法");
    println!("   - 各种适配器组合");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_iterators() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
        assert_eq!(vec.into_iter().sum::<i32>(), 6);
    }

    #[test]
    fn test_filtering() {
        let numbers = [1, 2, 3, 4, 5];
        let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(evens, vec![&2, &4]);

        let found = numbers.iter().find(|&&x| x == 3);
        assert_eq!(found, Some(&3));
    }

    #[test]
    fn test_transformation() {
        let numbers = [1, 2, 3];
        let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_aggregation() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(numbers.iter().sum::<i32>(), 15);
        assert_eq!(numbers.iter().product::<i32>(), 120);
        assert_eq!(numbers.iter().count(), 5);
    }

    #[test]
    fn test_combination() {
        let numbers = [1, 2, 3, 4, 5];

        let (even, odd): (Vec<&i32>, Vec<&i32>) = numbers.iter().partition(|&&x| x % 2 == 0);
        assert_eq!(even, vec![&2, &4]);
        assert_eq!(odd, vec![&1, &3, &5]);
    }

    #[test]
    fn test_advanced_methods() {
        let numbers = [1, 2, 3, 4, 5];
        let mut peekable = numbers.iter().peekable();
        assert_eq!(peekable.next(), Some(&1));
        assert_eq!(peekable.peek(), Some(&&2));
    }

    #[test]
    fn test_enumerate() {
        let items = ["a", "b", "c"];
        let enumerated: Vec<_> = items.iter().enumerate().collect();
        assert_eq!(enumerated, vec![(0, &"a"), (1, &"b"), (2, &"c")]);
    }

    #[test]
    fn test_zip() {
        let numbers = [1, 2, 3];
        let letters = ["a", "b", "c"];
        let zipped: Vec<_> = numbers.iter().zip(letters).collect();
        assert_eq!(zipped, vec![(&1, "a"), (&2, "b"), (&3, "c")]);
    }

    #[test]
    fn test_chain() {
        let vec1 = [1, 2];
        let vec2 = [3, 4];
        let chained: Vec<_> = vec1.iter().chain(vec2.iter()).collect();
        assert_eq!(chained, vec![&1, &2, &3, &4]);
    }

    #[test]
    fn test_fold() {
        let numbers = [1, 2, 3, 4, 5];
        let sum = numbers.iter().fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 15);
    }

    // 添加更多测试来覆盖76种方法...
    #[test]
    fn test_all_common_methods() {
        // 测试所有常用方法
        let vec = [1, 2, 3, 4, 5];

        // iter() 相关
        assert_eq!(vec.iter().next(), Some(&1));
        assert_eq!(vec.iter().last(), Some(&5));
        assert_eq!(vec.iter().nth(2), Some(&3));

        // 过滤相关
        assert!(vec.iter().any(|&x| x == 3));
        assert!(vec.iter().all(|&x| x > 0));

        // 转换相关
        let mapped: Vec<_> = vec.iter().map(|&x| x * 2).collect();
        assert_eq!(mapped, vec![2, 4, 6, 8, 10]);

        // 聚合相关
        assert_eq!(vec.iter().min(), Some(&1));
        assert_eq!(vec.iter().max(), Some(&5));
    }
}
