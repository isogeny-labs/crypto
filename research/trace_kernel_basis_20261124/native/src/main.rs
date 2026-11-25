use std::process::ExitCode;
use trace_kernel_basis::{rank, span, Context, Field};

fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn random_basis(kernel: &[u16], seed: u64) -> Vec<u16> {
    let mut state = seed;
    let mut out = Vec::new();
    while out.len() < kernel.len() {
        let bits = next(&mut state);
        let value = kernel
            .iter()
            .enumerate()
            .filter(|(i, _)| bits & (1 << i) != 0)
            .fold(0, |a, (_, b)| a ^ b);
        let mut attempt = out.clone();
        attempt.push(value);
        if rank(&attempt) == out.len() + 1 {
            out.push(value);
        }
    }
    out
}

fn inspect(n: u32) -> Result<(), String> {
    let c = match n {
        7 => 3,
        9 => 2,
        11 => 22,
        _ => return Err("inspection panel is frozen to degrees 7, 9, 11".into()),
    };
    let f = Field::new(n)?;
    // Rationality: y=x*z gives z^2+z=x+1+c/x^2 for x!=0.
    // Both trace predicates are required here, not just curve rationality.
    let xs: Vec<_> = (1..f.size())
        .filter(|&x| {
            let inverse = f.power(x, f.size() - 2);
            f.trace(x ^ 1) == 0 && f.trace(x ^ 1 ^ f.multiply(c, f.square(inverse))) == 0
        })
        .collect();
    if xs.len() < 10 {
        return Err("frozen target split requires ten distinct coordinates".into());
    }
    let training = xs[..2].to_vec();
    let heldout = xs[2..10].to_vec();
    let ctx = Context::new(f, c, training.clone())?;
    let picked = ctx.optimize();
    let mixed = random_basis(&ctx.chart.kernel, 20261124 + u64::from(n));
    assert_eq!(span(&ctx.chart.kernel), span(&picked.basis));
    assert_eq!(span(&ctx.chart.kernel), span(&mixed));
    let standard = &ctx.chart.kernel;
    let optimized = &picked.basis;
    println!("{{\"schema\":\"trace-kernel-support/v1\",\"n\":{n},\"modulus\":{},\"a2\":1,\"a6\":{c},\"training\":{training:?},\"heldout\":{heldout:?},\"candidates_scored\":{},\"support_unit\":\"descended coefficient occurrences\",\"runtime_advantage\":null,\"relation_cost\":null,\"arms\":[", f.modulus, picked.candidates_scored);
    for (i, (name, basis)) in [
        ("standard", standard),
        ("random", &mixed),
        ("optimized", optimized),
    ]
    .into_iter()
    .enumerate()
    {
        let auxiliary_weight: u64 = basis.iter().map(|&v| ctx.column_weight(v)).sum();
        let training_support = ctx.total_support(basis, &training);
        let constant_support = ctx.total_support(&[], &training);
        if training_support != constant_support + auxiliary_weight {
            return Err("support objective is not additive for this context".into());
        }
        println!("{{\"arm\":\"{name}\",\"basis\":{basis:?},\"training_support\":{training_support},\"heldout_support\":{},\"kernel_rank\":{}}}{}",
                 ctx.total_support(basis, &heldout), rank(basis), if i == 2 { "" } else { "," });
    }
    println!("]}}");
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [flag, value] if flag == "--inspect" => value.parse::<u32>()
            .map_err(|_| "degree must be an integer".to_owned()).and_then(inspect),
        _ => Err("usage: trace-kernel-basis --inspect <7|9|11>; research execution requires repository admission".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
