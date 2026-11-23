use g6_large_m::{analyze, cell, curve::Curve, field::Fq, iota, sizes};

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        // the frozen cell list: n a m, instances per cell, tuple cap
        Some("cells") => {
            for (n, a, m) in cell::cell_list() {
                println!(
                    "{n}\t{a}\t{m}\t{}\t{}",
                    cell::instances_per_cell(),
                    cell::CAP_LOG2_TUPLES
                );
            }
        }
        // exact formulas, no group involved
        Some("sizes") => print!("{}", sizes::table()),
        Some("cell") => {
            let p = cell::CellParams {
                n: num(&args, "--n"),
                a: num(&args, "--a"),
                m: num(&args, "--m"),
                draws: arg(&args, "--draws")
                    .map(|s| s.parse().unwrap())
                    .unwrap_or(cell::DRAWS),
                k: arg(&args, "--k")
                    .map(|s| s.parse().unwrap())
                    .unwrap_or(cell::K_MAX),
                seed: arg(&args, "--seed")
                    .map(|s| s.parse().unwrap())
                    .unwrap_or(cell::SEED_DEFAULT),
                out: arg(&args, "--out").expect("missing --out"),
            };
            match cell::run_cell(&p) {
                Ok(s) => eprintln!("{s}"),
                Err(e) => {
                    eprintln!("CELL_NOT_RUN {e}");
                    std::process::exit(2);
                }
            }
        }
        Some("iota-audit") => {
            let (n, a, m, k): (u32, u32, usize, usize) = (
                num(&args, "--n"),
                num(&args, "--a"),
                num(&args, "--m"),
                num(&args, "--k"),
            );
            let seed: u64 = arg(&args, "--seed")
                .map(|s| s.parse().unwrap())
                .unwrap_or(cell::SEED_DEFAULT);
            let f = Fq::new(n);
            let c = Curve::koblitz(&f, a);
            match iota::iota_audit(&c, m, k, seed) {
                Ok(r) => {
                    println!("n\ta\tm\tk\ttranslate_invariant\tmultiple_of_2^(k-1)\tquotient_identity\ttuples_iota\ttuples_plain\tquery_iota\tquery_plain");
                    println!(
                        "{}\t{a}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        r.n,
                        r.m,
                        r.k,
                        r.translate_invariant,
                        r.multiple_of_2k1,
                        r.quotient_identity,
                        r.tuples_iota,
                        r.tuples_plain,
                        r.query_iota,
                        r.query_plain
                    );
                    if !(r.translate_invariant && r.multiple_of_2k1 && r.quotient_identity) {
                        eprintln!("IOTA_IDENTITY_FAILED");
                        std::process::exit(3);
                    }
                }
                Err(e) => {
                    eprintln!("CELL_NOT_RUN {e}");
                    std::process::exit(2);
                }
            }
        }
        Some("analyze") => match analyze::analyze_dir(&arg(&args, "--dir").expect("missing --dir"))
        {
            Ok(s) => print!("{s}"),
            Err(e) => {
                eprintln!("ANALYZE_FAILED {e}");
                std::process::exit(2);
            }
        },
        _ => {
            eprintln!("usage: g6census cells | sizes | cell --n N --a A --m M [--draws D --k K --seed S] --out DIR | iota-audit --n N --a A --m M --k K | analyze --dir DIR");
            std::process::exit(1);
        }
    }
}
