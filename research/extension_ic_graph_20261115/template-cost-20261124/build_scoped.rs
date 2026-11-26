//! Thin native compiler orchestration; parses an existing pinned argument file
//! as data and invokes rustc directly, without shell evaluation.
use std::{
    env, fs,
    path::Path,
    process::{exit, Command},
};
fn words(source: &str) -> Vec<String> {
    let (mut out, mut word, mut quote, mut escaped, mut active) =
        (Vec::new(), String::new(), '\0', false, false);
    for c in source.chars() {
        if escaped {
            word.push(c);
            escaped = false;
            active = true;
            continue;
        }
        if c == '\\' && quote != '\'' {
            escaped = true;
            active = true;
            continue;
        }
        if quote != '\0' {
            if c == quote {
                quote = '\0';
            } else {
                word.push(c);
            }
            active = true;
            continue;
        }
        if c == '\'' || c == '"' {
            quote = c;
            active = true;
        } else if c.is_whitespace() {
            if active {
                out.push(std::mem::take(&mut word));
                active = false;
            }
        } else {
            word.push(c);
            active = true;
        }
    }
    assert!(!escaped && quote == '\0', "unterminated compiler argument");
    if active {
        out.push(word);
    }
    out
}
fn main() {
    let a: Vec<_> = env::args().collect();
    assert!(
        (7..=8).contains(&a.len()),
        "BASE_ARGV SOURCE OUTPUT MODE CERTIFIED_RLIB NEW_TARGET [EXTRA_EXTERN]"
    );
    let mut args = words(&fs::read_to_string(&a[1]).unwrap());
    assert_eq!(args.remove(0), "rustc");
    let source = args
        .iter()
        .position(|s| s == "examples/jv_sieve_action_operator_check.rs")
        .expect("known baseline source");
    args[source] = a[2].clone();
    if let Some(i) = args.iter().position(|s| s == "-o") {
        args[i + 1] = a[3].clone();
    } else {
        args.extend(["-o".into(), a[3].clone()]);
    }
    assert!(!args.iter().any(|s| s == "--test"));
    if a[4].contains("test") {
        args.push("--test".into());
    }
    if a[4].starts_with("algebra") {
        args.extend([
            "--crate-name".into(),
            "certified_relations".into(),
            "--crate-type=rlib".into(),
        ]);
        if !a[4].contains("uncounted") {
            args.extend(["--cfg".into(), "feature=\"count-field-ops\"".into()]);
        }
    } else if !a[4].contains("uncounted") {
        args.extend(["--cfg".into(), "feature=\"nagao-template-counters\"".into()]);
    }
    if !a[4].starts_with("algebra") {
        assert!(Path::new(&a[5]).is_file());
        args.extend(["--extern".into(), format!("certified_relations={}", a[5])]);
    }
    for package in if a[4].starts_with("algebra") {
        Vec::new()
    } else {
        fs::read_dir(Path::new(&a[6]).join("release/build"))
            .unwrap()
            .collect::<Vec<_>>()
    } {
        let path = package.unwrap().path();
        if !path.is_dir() {
            continue;
        }
        for version in fs::read_dir(path).unwrap() {
            let out = version.unwrap().path().join("out");
            if out.is_dir() {
                args.extend(["-L".into(), format!("dependency={}", out.display())]);
            }
        }
    }
    let program = if a[4].starts_with("clippy") {
        "/home/agent/.cargo/bin/clippy-driver"
    } else {
        "/home/agent/.cargo/bin/rustc"
    };
    if a.len() == 8 {
        args.extend(["--extern".into(), a[7].clone()]);
    }
    eprintln!("compiler={program}\nargv={args:?}");
    let status = Command::new(program)
        .args(args)
        .current_dir("/workspace/crypto-unit-descent-20261123")
        .status()
        .unwrap();
    exit(status.code().unwrap_or(1));
}
