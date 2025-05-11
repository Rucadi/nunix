use clap::Parser;
use colored_json::ToColoredJson;
use pathdiff::diff_paths;
use serde_json::Value as JsonValue;
use snix_eval::{Evaluation, Value};
use std::env;
use std::process;
use std::str;
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(&["file"])
))]
struct Opt {
    #[arg(value_name = "FILE")]
    file: Option<String>
}
fn main() {
    let opt = Opt::parse();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join(opt.file.unwrap_or_else(|| exit_err("No input file")));

    let current_dir =
        env::current_dir().unwrap_or_else(|e| exit_err(&format!("Couldn't get CWD: {}", e)));

    let relative_path = diff_paths(&file_path, &current_dir)
        .unwrap_or_else(|| exit_err("Unable to compute relative path"));

    let evaluator = Evaluation::builder_impure().build();

    let to_eval = format!(
        "builtins.toJSON (import {})",
        relative_path.to_string_lossy().replace("\\", "/")
    );

    println!("Evaluating: {}", to_eval);
    let res = evaluator.evaluate(to_eval, Some(current_dir));

    for err in &res.errors {
        exit_err(&format!("Error: {}", err));
    }

    let json_string = match res.value.unwrap() {
        Value::String(s) => s,
        _ => exit_err("Evaluation must return a JSON string"),
    };

    let json_value: JsonValue = serde_json::from_str(&json_string.as_str().unwrap())
        .unwrap_or_else(|e| exit_err(&format!("invalid JSON output: {}", e)));

    print_output(&json_value);
}

fn print_output(json: &JsonValue) {
    match json {
        JsonValue::String(s) => println!("{}", s),
        _ => {
            let json_string = serde_json::to_string_pretty(json);
            match json_string {
                Ok(s) => {
                    match s.to_colored_json_auto() {
                        Ok(colored) => println!("{}", colored),
                        Err(_) => println!("{}", s), // Fallback to plain JSON string
                    }
                }
                Err(e) => eprintln!("Failed to serialize JSON: {}", e),
            }
        }
    }
}

/// Print an error and exit
fn exit_err(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
