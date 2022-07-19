use std::path::Path;
use std::process::Command;

fn main() {
    let path = "getter_proxy/target/wasm32-unknown-unknown/release/getter_proxy.wasm";
    if Path::new(path).exists() {
        return;
    }

    Command::new("cargo")
        .current_dir("getter_proxy")
        .args(vec![
            "build",
            "--release",
            "--no-default-features",
            "--bin",
            "getter_proxy",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("Couldn't build getter proxy");

    let wasm_output = Command::new("wasm-strip").arg(path).output();

    match wasm_output {
        Ok(_) => {}
        Err(output) => {
            println!(
                "There was an error while running wasmstrip:\n{}\nContinuing anyway...",
                output
            );
        }
    }
}
