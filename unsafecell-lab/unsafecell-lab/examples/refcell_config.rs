use std::cell::*;
use std::collections::*;
use std::rc::*;

/// 从配置文件读取配置
fn load_configs(config: Rc<RefCell<HashMap<String, String>>>) {
    // 假设我们从文件里面读取了配置，这里为了模拟演示需要就不实际读取了
    let mut map = config.borrow_mut();
    map.insert("host".to_owned(), "0.0.0.0".to_owned());
    map.insert("port".to_owned(), "8080".to_owned());
    map.insert("db_url".to_owned(), "mysql://localhost:3306".to_owned());
    map.insert("db_username".to_owned(), "root".to_owned());
}

/// 用于处理Http服务器的类
struct HttpServer {
    config: Rc<RefCell<HashMap<String, String>>>,
}

impl HttpServer {
    /// 在配置表中为缺失的配置项填充默认值
    fn fill_defaults(config: &Rc<RefCell<HashMap<String, String>>>) {
        // 这里为了举例方便，只放一条
        let mut map = config.borrow_mut();
        if !map.contains_key("port") {
            map.insert("port".to_owned(), "8080".to_owned());
        }
    }

    fn new(config: Rc<RefCell<HashMap<String, String>>>) -> Self {
        // 填充默认值
        Self::fill_defaults(&config);

        Self { config }
    }

    fn listen(&self) {
        let map = self.config.borrow();
        println!(
            "Listening on {}:{}",
            map.get("host").unwrap(),
            map.get("port").unwrap()
        )
    }
}

/// 用于处理数据库连接的类
struct Database {
    config: Rc<RefCell<HashMap<String, String>>>,
}

impl Database {
    /// 在配置表中为缺失的配置项填充默认值
    fn fill_defaults(config: &Rc<RefCell<HashMap<String, String>>>) {
        // 这里为了举例方便，只放一条
        let mut map = config.borrow_mut();
        if !map.contains_key("db_password") {
            map.insert("db_password".to_owned(), "admin".to_owned());
        }
    }

    fn new(config: Rc<RefCell<HashMap<String, String>>>) -> Self {
        // 填充默认值
        Self::fill_defaults(&config);

        Self { config }
    }

    fn connect(&self) {
        let map = self.config.borrow();
        println!(
            "Connected to Database: {}, user:{}, password:{}",
            map.get("db_url").unwrap(),
            map.get("db_username").unwrap(),
            map.get("db_password").unwrap()
        )
    }
}

fn main() {
    let config = Rc::new(RefCell::new(HashMap::new()));

    // 读取配置
    load_configs(config.clone());

    let db = Database::new(config.clone());
    let http = HttpServer::new(config.clone());

    db.connect();
    http.listen();
}
