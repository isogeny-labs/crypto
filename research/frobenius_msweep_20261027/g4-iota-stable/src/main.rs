use g4_iota_stable::{analyze, run};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("cells") => {
            for c in run::cell_list() {
                println!("{} {} {} {} {}", c.a, c.n, c.m, c.l, run::reps_for(c.n));
            }
        }
        Some("run") => {
            let get = |k: &str| arg(&args, k).unwrap_or_else(|| panic!("missing {k}")).parse::<u32>().unwrap();
            let cell = run::Cell { a: get("--a"), n: get("--n"), m: get("--m") as usize, l: get("--l") };
            let o = run::RunOpts {
                seed: arg(&args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(20261027),
                reps: arg(&args, "--reps").map(|s| s.parse().unwrap()),
                out: arg(&args, "--out").expect("missing --out"),
            };
            match run::run_cell(&cell, &o) {
                Ok(p) => eprintln!("wrote {p}"),
                Err(e) => {
                    eprintln!("CELL_NOT_RUN {e}");
                    std::process::exit(2);
                }
            }
        }
        Some("analyze") => {
            if let Err(e) = analyze::analyze(&arg(&args, "--dir").expect("missing --dir")) {
                eprintln!("analyze failed: {e}");
                std::process::exit(1);
            }
        }
        _ => eprintln!("usage: g4iota cells | run --a A --n N --m M --l L --out DIR [--reps R] [--seed S] | analyze --dir DIR"),
    }
}
