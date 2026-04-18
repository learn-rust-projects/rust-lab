use tracing::{Level, debug, error, info, instrument, span, trace, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // 初始化tracing订阅者
    tracing_subscriber::fmt::init();

    println!("=== Tracing 常见用法示例 ===\n");

    // 1. 基本日志级别
    let user_id = "user-123".to_string();
    basic_logging_levels(user_id);

    // 2. 使用span追踪执行上下文
    spans_example();

    // 3. 结构化日志记录
    structured_logging();

    // 4. 异步追踪示例
    async_tracing_example().await;

    // 5. 字段和事件
    fields_and_events();

    // 6. 数据库操作示例
    database_operation(&"user-123".to_string(), &"SELECT * FROM users".to_string());

    // 7. 高级示例
    run_advanced_examples();

    // 8. 不同级别的Spans
    spans_with_different_levels();

    // 9. 日志级别配置的影响总结
    log_level_impact_summary();

    println!("\n=== Tracing 示例完成 ===");
}
#[instrument(level = Level::INFO, name = "基本日志级别", fields(x = %user_id, y = 10))]
fn basic_logging_levels(user_id: String) {
    println!("--- 基本日志级别 ---");

    // 不同的日志级别
    tracing::trace!("这是trace级别的日志 - 用于非常详细的调试信息");
    tracing::debug!("这是debug级别的日志 - 用于调试信息");
    tracing::info!("这是info级别的日志 - 用于一般信息");
    tracing::warn!("这是warn级别的日志 - 用于警告信息");
    tracing::error!("这是error级别的日志 - 用于错误信息");
}

fn spans_example() {
    println!("\n--- Spans 示例 ---");

    // 创建一个span来追踪函数执行
    let span = span!(Level::INFO, "计算操作", x = 5, y = 10);
    let _enter = span.enter();

    info!("开始执行计算");

    // 模拟一些工作
    let result = compute_operation(5, 10);
    info!(result = result, "计算完成");

    // span会自动结束，当_enter离开作用域
}

fn compute_operation(x: i32, y: i32) -> i32 {
    let span = span!(Level::ERROR, "内部计算", x, y);
    let _enter = span.enter();

    error!("执行加法运算");
    let result = x + y;

    error!(result, "计算得到结果");
    result
}

fn structured_logging() {
    println!("\n--- 结构化日志示例 ---");

    let user_id = 12345;
    let action = "登录";
    let ip = "192.168.1.1";

    // 结构化日志 - 带有结构化字段
    info!(user_id = user_id, action = action, ip = ip, "用户执行操作");

    // 更复杂的结构化日志
    let request_id = "req-abc-123";
    let status = 200;
    let duration_ms = 150;

    info!(
        request_id,
        status,
        duration_ms,
        method = "GET",
        path = "/api/users",
        "HTTP请求完成"
    );
}

async fn async_tracing_example() {
    println!("\n--- 异步追踪示例 ---");

    // 异步span - 在异步上下文中追踪
    let async_span = span!(Level::INFO, "异步-闭包块-操作", operation = "数据库查询");
    async_span.in_scope(|| {
        info!("开始异步操作");
    });

    // 使用instrument为异步函数添加追踪
    let result = perform_async_task().await;
    info!(result, "异步任务完成");
}

#[tracing::instrument]
async fn perform_async_task() -> String {
    info!("执行异步任务");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    info!("异步任务即将完成");
    "任务结果".to_string()
}

#[tracing::instrument(name = "instrument数据库操作", skip(db_connection, query))]
fn database_operation(db_connection: &str, query: &str) -> String {
    info!("执行数据库操作");
    // 模拟数据库操作
    std::thread::sleep(std::time::Duration::from_millis(50));
    info!("查询 '{}' 的结果", query);
    format!("结果 for '{}'", query)
}

fn fields_and_events() {
    println!("\n--- 字段和事件示例 ---");

    // 创建带有字段的span
    let db_span = span!(
        Level::INFO,
        "数据库操作",
        connection_string = "postgresql://localhost:5432/test",
        query = "SELECT * FROM users LIMIT 10"
    );

    let _guard = db_span.enter();

    // 在span上下文中记录事件
    info!("连接到数据库");
    info!("执行查询");
    info!(rows_returned = 10, "查询完成");

    // 使用调试字段
    let user_data = vec!["Alice", "Bob", "Charlie"];
    debug_span_fields_example(&user_data);
}

#[tracing::instrument(name = "instrument嵌套调试")]
pub fn debug_span_fields_example(users: &[&str]) {
    // 使用?和%操作符来控制字段的显示
    info!(count = users.len(), ?users, "用户列表信息");

    // ? 用于调试格式化 (Debug)
    // % 用于显示格式化 (Display)
    let id = 12345;
    info!(?id, %id, "ID的Debug和Display格式");

    // 条件字段
    let maybe_error: Option<&str> = Some("Something went wrong");
    info!(error_msg = ?maybe_error, "可能的错误信息");
}

// 额外的实用示例
mod advanced_examples {
    use tracing::{Level, event, info, span};

    pub fn custom_span_with_values() {
        info!("--- 自定义Span值示例 ---");

        // 动态设置span字段
        let user_id = 999;
        let operation = "数据处理";

        let span = span!(Level::INFO, "用户操作", user_id, operation);
        let _enter = span.enter();

        info!("开始处理用户数据");

        // 手动记录事件
        event!(Level::INFO, processed_items = 100, "批量处理完成");

        info!("结束处理用户数据");
    }

    pub fn enter_exit_example() {
        info!("--- 进入/退出示例 ---");

        let span = span!(Level::INFO, "作用域操作");
        info!("操作前");

        {
            let _enter = span.enter();
            info!("在span内部");
            std::thread::sleep(std::time::Duration::from_millis(10));
            info!("仍在span内部");
        } // span在这里退出

        info!("操作后");
    }
}

// 运行高级示例
fn run_advanced_examples() {
    advanced_examples::custom_span_with_values();
    advanced_examples::enter_exit_example();
}

fn spans_with_different_levels() {
    println!("\n--- 不同级别的Spans ---");

    // TRACE级别span - 最详细的信息
    let trace_span = span!(Level::TRACE, "详细追踪操作", detail = true);
    let _trace_guard = trace_span.enter();
    trace!("在TRACE级别span内记录详细信息");
    debug!("调试信息");
    info!("一般信息");
    drop(_trace_guard);

    // DEBUG级别span - 调试信息
    let debug_span = span!(Level::DEBUG, "调试操作", operation = "debug_task");
    let _debug_guard = debug_span.enter();
    debug!("在DEBUG级别span内记录调试信息");
    info!("一般信息");
    warn!("警告信息");
    drop(_debug_guard);

    // INFO级别span - 一般信息
    let info_span = span!(Level::INFO, "一般操作", task = "regular_task");
    let _info_guard = info_span.enter();
    info!("在INFO级别span内记录一般信息");
    warn!("警告信息");
    error!("错误信息");
    drop(_info_guard);
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_basic_tracing() {
        info!("测试信息日志");
        assert!(logs_contain("测试信息日志"));
    }

    #[test]
    #[traced_test]
    fn test_error_tracing() {
        error!("测试错误日志");
        assert!(logs_contain("测试错误日志"));
        assert!(logs_contain("ERROR"));
    }
}

// 日志级别配置的影响总结
fn log_level_impact_summary() {
    info!("\n--- 日志级别配置的影响总结 ---");
    info!("TRACE: 最详细的日志，用于深入调试。在生产环境中通常被禁用以避免性能问题。");
    info!("DEBUG: 调试信息，用于开发和测试阶段。生产环境中通常被禁用。");
    info!("INFO: 一般信息，用于记录程序正常运行的关键步骤。生产环境中通常启用。");
    info!("WARN: 警告信息，表示可能出现的问题，但仍可继续运行。生产环境中启用。");
    info!("ERROR: 错误信息，表示发生了错误，可能导致功能失败。生产环境中必须启用。");

    info!("\n性能影响:");
    info!("- 较低级别的日志（如TRACE、DEBUG）在禁用时几乎无性能开销");
    info!("- 启用大量日志会增加I/O操作，影响性能");
    info!("- 结构化日志比普通日志稍微消耗更多资源");
    info!("- Span的创建和销毁也有轻微性能开销");

    info!("\n最佳实践:");
    info!("- 生产环境中使用INFO/WARN/ERROR级别");
    info!("- 开发环境中可以使用DEBUG/TRACE级别");
    info!("- 合理使用span来追踪操作上下文");
    info!("- 避免在热点路径上记录过多日志");
}
