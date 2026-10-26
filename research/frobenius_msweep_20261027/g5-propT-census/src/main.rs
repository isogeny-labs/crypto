use g5_propt_census::{analyze, run};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn num<T: std::str::FromStr>(args: &[String], k: &str) -> T {
    arg(args, k).unwrap_or_else(|| panic!("missing {k}")).parse::<T>().unwrap_or_else(|_| panic!("bad value for {k}"))
}

fn opts(args: &[String]) -> run::RunOpts {
    run::RunOpts {
        seed: arg(args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(run::SEED_DEFAULT),
        reps: arg(args, "--reps").map(|s| s.parse().unwrap()),
        out: arg(args, "--out").expect("missing --out"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("cells") => {
            for c in run::cell_list() {
                println!("{} {} {} {} {}", c.a, c.n, c.m, c.l, c.reps);
            }
        }
        Some("propt") => {
            let (n, m): (u32, usize) = (num(&args, "--n"), num(&args, "--m"));
            let trials: usize = arg(&args, "--trials").map(|s| s.parse().unwrap()).unwrap_or(20000);
            let k: usize = arg(&args, "--assigns").map(|s| s.parse().unwrap()).unwrap_or(4);
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
                run::run_overlap(&run::Cell { reps: arg(&args, "--reps").map(|s| s.parse().unwrap()).unwrap_or(40), ..cell }, &o)
            } else {
                let t1: usize = arg(&args, "--targets").map(|s| s.parse().unwrap()).unwrap_or(16);
                let cap: usize = arg(&args, "--rowcap").map(|s| s.parse().unwrap()).unwrap_or(40);
                run::run_rank(&cell, t1, cap, &o)
            };
            match res {
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
        _ => eprintln!(
            "usage: g5census cells | propt --n N --m M [--trials T] [--assigns K] --out DIR | overlap --a A --n N --m M --l L [--reps R] --out DIR | rank --a A --n N --m M --l L [--reps R] [--targets T] [--rowcap C] --out DIR | analyze --dir DIR"
        ),
    }
}
