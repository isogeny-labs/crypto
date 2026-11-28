use generalized_rootfinder::solver::{self, field::Result, Limits};
fn run() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let path=args.get(1).ok_or("usage: generalized-rootfinder INPUT.json [--max-work N --max-table N --max-solutions N --max-domain N --max-degree N --max-trials N --seed N]")?;
    let mut limits = Limits::default();
    let mut i = 2;
    while i < args.len() {
        let v = args
            .get(i + 1)
            .ok_or("missing option value")?
            .parse::<u64>()
            .map_err(|_| "option requires unsigned integer")?;
        match args[i].as_str() {
            "--max-work" => limits.max_work = v,
            "--max-table" => {
                limits.max_table = usize::try_from(v).map_err(|_| "limit too large")?
            }
            "--max-solutions" => {
                limits.max_solutions = usize::try_from(v).map_err(|_| "limit too large")?
            }
            "--max-domain" => {
                limits.max_domain = usize::try_from(v).map_err(|_| "limit too large")?
            }
            "--max-degree" => {
                limits.max_degree = usize::try_from(v).map_err(|_| "limit too large")?
            }
            "--max-trials" => limits.max_trials = v,
            "--seed" => limits.seed = v,
            _ => return Err("unknown option".into()),
        };
        i += 2;
    }
    let input = std::fs::read(path).map_err(|e| e.to_string())?;
    let problem = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&solver::solve_json(&problem, &limits)?).unwrap()
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
