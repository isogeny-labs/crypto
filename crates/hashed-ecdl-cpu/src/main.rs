use hashed_ecdl_cpu::Archive;
use hashed_ecdl_cpu::{paper, CachedMitm};
use paper::Curve;
use std::{error::Error, time::Instant};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--verify" {
        let archive: Archive = serde_json::from_slice(&std::fs::read(&args[2])?)?;
        archive.verify()?;
        println!("VERIFIED scalar={}", archive.scalar());
        return Ok(());
    }
    if args.len() != 15 && args.len() != 16 {
        return Err("usage: hashed-ecdl-cpu MODE p a b r Px Py Qx Qy m tau seed max_rows batch [max_columns]; MODE=cached|reference; or --verify FILE".into());
    }
    let start = Instant::now();
    let n = |i: usize| -> Result<u64, Box<dyn Error>> { Ok(args[i].parse()?) };
    let params = [n(2)?, n(3)?, n(4)?, n(5)?];
    let p = Some((n(6)?, n(7)?));
    let q = Some((n(8)?, n(9)?));
    let curve = Curve::with_order_witness(params[0], params[1], params[2], params[3], p)?;
    let config = paper::Config {
        m: usize::try_from(n(10)?)?,
        tau: Some(n(11)?),
        seed: n(12)?,
        max_work: n(13)?,
        max_columns: if args.len() == 16 {
            usize::try_from(n(15)?)?
        } else {
            512
        },
        ..Default::default()
    };
    let mut oracle = CachedMitm::new(usize::try_from(n(13)?)?, usize::try_from(n(14)?)?)?;
    let cert = match args[1].as_str() {
        "cached" => paper::solve_with_oracle(&curve, p, q, &config, &mut oracle)?,
        "reference" => paper::solve(&curve, p, q, &config)?,
        _ => return Err("unknown CPU mode".into()),
    };
    let archive = Archive::new(params, p, q, cert);
    archive.verify()?;
    let serialized = serde_json::to_string_pretty(&archive)?;
    println!("{serialized}");
    eprintln!(
        "cpu_complete_ms={:.6} stats={:?}",
        start.elapsed().as_secs_f64() * 1000.0,
        oracle.stats()
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
