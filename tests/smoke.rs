extern crate debug_cell;

use debug_cell::RefCell;
use std::env;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 && args[1] == "child1" {
        let r = RefCell::new(3);
        let _a = r.borrow();
        let _b = r.borrow();
        r.borrow_mut();
    } else if args.len() > 1 && args[1] == "child2" {
        let r = RefCell::new(3);
        let _a = r.borrow_mut();
        r.borrow_mut();
    } else {
        runtest(
            "child1",
            &[
                "current active borrow",
                "tests/smoke.rs:11",
                "tests/smoke.rs:12",
            ],
        );
        runtest("child2", &["current active borrow", "tests/smoke.rs:16"]);
    }
}

fn runtest(name: &str, substrs: &[&str]) {
    let output = Command::new(env::current_exe().unwrap())
        .arg(name)
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    for s in substrs {
        assert!(
            stderr.contains(s) == cfg!(debug_assertions),
            "`{s}` not found in `{stderr}`"
        );
    }
    println!("ok: {name}");
}
