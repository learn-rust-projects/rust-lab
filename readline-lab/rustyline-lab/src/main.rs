use std::{
    borrow::Cow,
    sync::atomic::{AtomicUsize, Ordering},
};

use rustyline::{
    Context, Editor, Helper,
    completion::{Completer, Pair},
    config::Configurer,
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::{ValidationContext, ValidationResult, Validator},
};
static COUNTER: AtomicUsize = AtomicUsize::new(0);
struct MyCompleter {
    commands: Vec<String>,
}

impl Completer for MyCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let start = 0; // 从行首开始补全
        let prefix = &line[..pos];
        let matches: Vec<Pair> = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: cmd.clone(),
            })
            .collect();
        Ok((0, matches))
    }
}

impl Helper for MyCompleter {} // 必须实现 Helper trait
impl Hinter for MyCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None // 不提供提示
    }
}

impl Highlighter for MyCompleter {} // 空实现

impl Validator for MyCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None)) // 始终认为输入合法
    }
}

fn main() -> Result<(), ReadlineError> {
    // 定义可补全命令
    let commands = vec![
        "help".to_string(),
        "hexp".to_string(),
        "exit".to_string(),
        "list".to_string(),
        "show".to_string(),
        "delete".to_string(),
    ];

    let completer = MyCompleter { commands };
    let mut rl = Editor::new()?;
    rl.set_completion_type(rustyline::CompletionType::List);
    rl.set_auto_add_history(true);
    rl.set_helper(Some(completer));
    // rl.load_history("../history.txt")?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                match line.as_str() {
                    "exit" => {
                        println!("Bye!");
                        break;
                    }
                    "help" => {
                        println!("Commands: exit, help, list, show, delete");
                    }
                    "history" => {
                        println!("History: {:?}", rl.history().iter().collect::<Vec<_>>());
                    }
                    "history1" => {
                        println!("History: {:?}", rl.history().iter().collect::<Vec<_>>());
                    }
                    cmd => {
                        println!("You typed: {}", cmd);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
