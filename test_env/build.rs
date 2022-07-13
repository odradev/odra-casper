use std::path::Path;
use std::process::Command;

fn main() {
    if Path::new("getter_proxy.wasm").exists() {
        return;
    }

    Command::new("cargo")
        .current_dir("getter_proxy")
        .args(vec![
            "build",
            "--release",
            "--quiet",
            "--no-default-features",
            "--bin",
            "getter_proxy",
        ])
        .output()
        .expect("Couldn't build getter proxy");

    let source = "getter_proxy/target/wasm32-unknown-unknown/release/getter_proxy.wasm";
    let target = "./getter_proxy.wasm";
    Command::new("cp")
        .args(vec![source, target])
        .output()
        .expect("Couldn't copy getter proxy");

    let wasm_output = Command::new("wasm-strip").arg("getter_proxy.wasm").output();

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
