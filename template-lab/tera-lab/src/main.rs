use std::{collections::HashMap, sync::LazyLock};

use tera::{Context, Result as TeraResult, Tera, Value};

// Custom filter example: does nothing
fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    Ok(value.clone())
}

// Global template singleton
pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    tera.autoescape_on(vec![".html", ".sql"]);
    tera.register_filter("do_nothing", do_nothing_filter);

    tera
});

fn main() {
    // Build template context
    let mut context = Context::new();
    context.insert("name", "World");

    // Render template
    match TEMPLATES.render("hello.html", &context) {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("Template error: {}", err),
    }
}
