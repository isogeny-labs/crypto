//! Stable JSON archive and independent checked replay.
use crate::paper;
use paper::{Curve, Point};
use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Archive {
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
    pub fn scalar(&self) -> u64 {
        self.scalar
    }
    pub fn new(curve: [u64; 4], generator: Point, target: Point, cert: paper::Certificate) -> Self {
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
    pub fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.schema != "hashed-ecdl-cpu-certificate/v1"
            || self.relations.len() > 16385
            || self.hashes.len() > 8
        {
            return Err("certificate format/bounds".into());
        }
        let [p, a, b, r] = self.curve;
        let curve = Curve::with_order_witness(p, a, b, r, self.generator)?;
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::CachedMitm;
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
