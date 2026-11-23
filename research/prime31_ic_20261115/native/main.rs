use prime31_algebraic_ic::{solve, Limits, PRIME31};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() != 5 {
        return Err("usage: prime31-algebraic-ic TARGET_X TARGET_Y DEPTH SEED MAX_CALLS; accepts public affine target only".into());
    }
    let q = Some((a[0].parse()?, a[1].parse()?));
    let result = solve(
        PRIME31,
        q,
        a[2].parse()?,
        a[3].parse()?,
        a[4].parse()?,
        Limits::default(),
    )?;
    println!("{}", serde_json::to_string(&result)?);
    if result.scalar.is_none() {
        std::process::exit(2);
    }
    Ok(())
}
