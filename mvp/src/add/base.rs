//! AddStrategy trait 定义
//!
//! 策略模式的核心 trait，每个策略实现一个特定的功能。

use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use tera::{Context, Tera};

use crate::error::MvpError;

/// 策略 trait
///
/// 每个策略实现必须：
/// 1. 实现 `handle` 方法执行具体逻辑
/// 2. 实现 `command` 方法返回对应的 `AddCommand` 枚举
#[enum_dispatch]
pub trait ExcuteStrategy: Sync + Send + Debug {
    /// 执行策略
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError>;
}
