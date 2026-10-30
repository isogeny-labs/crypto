use hashed_ecdl_cpu::{paper, CachedMitm};
use paper::{Curve, Point};
use serde::{Deserialize, Serialize};
use std::{error::Error, time::Instant};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Archive {
    schema: String,
    curve: [u64; 4],
    generator: Point,
    target: Point,
    scalar: u64,
    hashes: Vec<[u64; 4]>,
    relations: Vec<(u64, u64, Vec<Point>)>,
    dependency: Vec<u64>,
    trials: usize,
    attempts: usize,
}
impl Archive {
    fn new(curve: [u64; 4], generator: Point, target: Point, cert: paper::Certificate) -> Self {
        Self {
            schema: "hashed-ecdl-cpu-certificate/v1".into(),
            curve,
            generator,
            target,
            scalar: cert.scalar,
            hashes: cert
                .hashes
                .iter()
                .map(|h| [h.s, h.alpha, h.beta, h.tau])
                .collect(),
            relations: cert
                .relations
                .into_iter()
                .map(|r| (r.u, r.v, r.points))
                .collect(),
            dependency: cert.dependency,
            trials: cert.trials,
            attempts: cert.attempts,
        }
    }
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.schema != "hashed-ecdl-cpu-certificate/v1"
            || self.relations.len() > 16385
            || self.hashes.len() > 8
        {
            return Err("certificate format/bounds".into());
        }
        let [p, a, b, r] = self.curve;
        let curve = Curve::new(p, a, b, r)?;
        let cert = paper::Certificate {
            scalar: self.scalar,
            hashes: self
                .hashes
                .iter()
                .map(|h| paper::HashBase {
                    s: h[0],
                    alpha: h[1],
                    beta: h[2],
                    tau: h[3],
                })
                .collect(),
            relations: self
                .relations
                .iter()
                .map(|(u, v, points)| paper::Relation {
                    u: *u,
                    v: *v,
                    points: points.clone(),
                })
                .collect(),
            dependency: self.dependency.clone(),
            trials: self.trials,
            attempts: self.attempts,
        };
        paper::verify(&curve, self.generator, self.target, &cert)?;
        Ok(())
    }
}
fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--verify" {
        let archive: Archive = serde_json::from_slice(&std::fs::read(&args[2])?)?;
        archive.verify()?;
        println!("VERIFIED scalar={}", archive.scalar);
        return Ok(());
    }
    if args.len() != 15 {
        return Err("usage: hashed-ecdl-cpu MODE p a b r Px Py Qx Qy m tau seed max_rows batch; MODE=cached|reference; or --verify FILE".into());
    }
    let start = Instant::now();
    let n = |i: usize| -> Result<u64, Box<dyn Error>> { Ok(args[i].parse()?) };
    let params = [n(2)?, n(3)?, n(4)?, n(5)?];
    let curve = Curve::new(params[0], params[1], params[2], params[3])?;
    let p = Some((n(6)?, n(7)?));
    let q = Some((n(8)?, n(9)?));
    let config = paper::Config {
        m: usize::try_from(n(10)?)?,
        tau: Some(n(11)?),
        seed: n(12)?,
        max_work: n(13)?,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn archived_public_recovery_replays_and_rejects_corruption() {
        let params = [211, 0, 7, 199];
        let curve = Curve::new(211, 0, 7, 199).unwrap();
        let p = Some((3, 33));
        let q = curve.mul(p, 42).unwrap();
        let config = paper::Config {
            m: 3,
            tau: Some(60),
            seed: 7,
            ..Default::default()
        };
        let mut oracle = CachedMitm::new(100_000, 64).unwrap();
        let cert = paper::solve_with_oracle(&curve, p, q, &config, &mut oracle).unwrap();
        let archive = Archive::new(params, p, q, cert);
        let bytes = serde_json::to_vec(&archive).unwrap();
        let mut replay: Archive = serde_json::from_slice(&bytes).unwrap();
        replay.verify().unwrap();
        replay.scalar = 43;
        assert!(replay.verify().is_err());
        replay.scalar = 42;
        replay.relations[0].2[0] = None;
        assert!(replay.verify().is_err());
        let mut json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        json["undeclared"] = true.into();
        assert!(serde_json::from_value::<Archive>(json).is_err());
        let mut replay: Archive = serde_json::from_slice(&bytes).unwrap();
        replay.schema.push('x');
        assert!(replay.verify().is_err());
    }
}
