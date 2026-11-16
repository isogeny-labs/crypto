use g5_propt_census::{analyze, run};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1).cloned())
}

fn num<T: std::str::FromStr>(args: &[String], k: &str) -> T {
    arg(args, k)
        .unwrap_or_else(|| panic!("missing {k}"))
        .parse::<T>()
        .unwrap_or_else(|_| panic!("bad value for {k}"))
}

fn opts(args: &[String]) -> run::RunOpts {
    run::RunOpts {
        seed: arg(args, "--seed")
            .map(|s| s.parse().unwrap())
            .unwrap_or(run::SEED_DEFAULT),
        reps: arg(args, "--reps").map(|s| s.parse().unwrap()),
        out: arg(args, "--out").expect("missing --out"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("cells") => {
            let cl = run::cell_list();
            for c in &cl {
                println!("{} {} {} {} {} {:.4e}", c.a, c.n, c.m, c.l, c.reps, run::cell_ops(c, c.reps));
            }
            for &(a, n) in run::CURVES.iter() {
                if !cl.iter().any(|c| c.a == a && c.n == n) {
                    eprintln!("CELL_NOT_RUN a={a} n={n}: the group gate (curve_summary) refused this curve; no cells planned");
                }
            }
        }
        Some("propt") => {
            let (n, m): (u32, usize) = (num(&args, "--n"), num(&args, "--m"));
            let trials: usize = num(&args, "--trials");
            let k: usize = num(&args, "--assigns");
            match run::run_propt(n, m, trials, k, &opts(&args)) {
                Ok(p) => eprintln!("wrote {p}"),
                Err(e) => {
                    eprintln!("CELL_NOT_RUN {e}");
                    std::process::exit(2);
                }
            }
        }
        Some("overlap") | Some("rank") => {
            let cell = run::Cell { a: num(&args, "--a"), n: num(&args, "--n"), m: num(&args, "--m"), l: num(&args, "--l"), reps: 0 };
            let o = opts(&args);
            let res = if args[0] == "overlap" {
                run::run_overlap(&run::Cell { reps: num(&args, "--reps"), ..cell }, &o)
            } else {
                let t1: usize = num(&args, "--targets");
                let cap: usize = num(&args, "--rowcap");
                let o2 = run::RunOpts { reps: Some(num(&args, "--reps")), ..o };
                run::run_rank(&cell, t1, cap, &o2)
            };
            match res {
                Ok(p) => eprintln!("wrote {p}"),
                Err(e) => {
                    eprintln!("CELL_NOT_RUN {e}");
                    std::process::exit(2);
                }
            }
        }
        Some("estimate") => {
            // planning-unit operation count of one overlap invocation (used by the pre-flight projection)
            let cell = run::Cell { a: num(&args, "--a"), n: num(&args, "--n"), m: num(&args, "--m"), l: num(&args, "--l"), reps: 0 };
            println!("{:.4e}", run::cell_ops(&cell, num(&args, "--reps")));
        }
        Some("gate") => {
            // fail-fast between phases of run_cells.sh: exit 0 only if every Proposition T cell holds and every control detects
            match analyze::propt_gate(&arg(&args, "--dir").expect("missing --dir")) {
                Ok(n) => eprintln!("propt gate passed ({n} cell-arms)"),
                Err(e) => {
                    eprintln!("PROPT GATE FAILED\n{e}");
                    std::process::exit(1);
                }
            }
        }
        Some("analyze") => {
            if let Err(e) = analyze::analyze(&arg(&args, "--dir").expect("missing --dir")) {
                eprintln!("analyze failed: {e}");
                std::process::exit(1);
            }
        }
        _ => eprintln!(
            "usage: g5census cells | propt --n N --m M --trials T --assigns K --out DIR | overlap --a A --n N --m M --l L --reps R --out DIR | rank --a A --n N --m M --l L --reps R --targets T --rowcap C --out DIR | estimate --a A --n N --m M --l L --reps R | gate --dir DIR | analyze --dir DIR"
        ),
    }
}
