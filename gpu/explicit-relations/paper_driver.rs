//! Standalone adapter: CPU paper reduction and independent certificate replay,
//! with GPU signed hash construction and persistent per-trial relation tables.
#![allow(dead_code)] // The included reference also exposes its CPU-only entry.
#[path = "../../src/cryptanalysis/hashed_ecdl.rs"]
mod paper;
// Preserve the included reference's public export in this standalone crate.
pub use paper::tensor;

use paper::{Curve, DecompositionOracle, Error, HashBase, Point};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

fn encode_certificate(
    c: &Curve,
    p: Point,
    q: Point,
    cert: &paper::Certificate,
) -> Result<String, Error> {
    let (a, b) = c.coefficients();
    let (px, py) = p.ok_or(Error::Verification)?;
    let (qx, qy) = q.ok_or(Error::Verification)?;
    let mut out = format!("hashed-ecdl-gpu-certificate/v1\ncurve {} {a} {b} {}\npublic {px} {py} {qx} {qy}\nsummary {} {} {}\nhashes {}\n",
        c.modulus(),c.order(),cert.scalar,cert.trials,cert.attempts,cert.hashes.len());
    for h in &cert.hashes {
        writeln!(out, "{} {} {} {}", h.s, h.alpha, h.beta, h.tau).unwrap();
    }
    writeln!(out, "relations {}", cert.relations.len()).unwrap();
    for rel in &cert.relations {
        write!(out, "{} {} {}", rel.u, rel.v, rel.points.len()).unwrap();
        for point in &rel.points {
            let (x, y) = point.ok_or(Error::Verification)?;
            write!(out, " {x} {y}").unwrap();
        }
        out.push('\n');
    }
    write!(out, "dependency {}", cert.dependency.len()).unwrap();
    for c in &cert.dependency {
        write!(out, " {c}").unwrap();
    }
    out.push('\n');
    Ok(out)
}
fn decode_certificate(text: &str) -> Result<(Curve, Point, Point, paper::Certificate), Error> {
    let mut words = text.split_whitespace();
    fn tag(words: &mut std::str::SplitWhitespace<'_>, expected: &str) -> Result<(), Error> {
        if words.next() == Some(expected) {
            Ok(())
        } else {
            Err(Error::Verification)
        }
    }
    fn num(words: &mut std::str::SplitWhitespace<'_>) -> Result<u64, Error> {
        integer(words.next().ok_or(Error::Verification)?)
    }
    fn count(words: &mut std::str::SplitWhitespace<'_>, cap: usize) -> Result<usize, Error> {
        let n = usize::try_from(num(words)?).map_err(|_| Error::Verification)?;
        if n <= cap {
            Ok(n)
        } else {
            Err(Error::Verification)
        }
    }
    tag(&mut words, "hashed-ecdl-gpu-certificate/v1")?;
    tag(&mut words, "curve")?;
    let (modulus, a, b, order) = (
        num(&mut words)?,
        num(&mut words)?,
        num(&mut words)?,
        num(&mut words)?,
    );
    tag(&mut words, "public")?;
    let p = Some((num(&mut words)?, num(&mut words)?));
    let q = Some((num(&mut words)?, num(&mut words)?));
    let c = Curve::with_order_witness(modulus, a, b, order, p)?;
    tag(&mut words, "summary")?;
    let scalar = num(&mut words)?;
    let trials = count(&mut words, 1000)?;
    let attempts = count(&mut words, 20 * (paper::MAX_COLUMNS + 1) * 1000)?;
    tag(&mut words, "hashes")?;
    let m = count(&mut words, paper::MAX_SLOTS)?;
    let hashes = (0..m)
        .map(|_| {
            Ok(HashBase {
                s: num(&mut words)?,
                alpha: num(&mut words)?,
                beta: num(&mut words)?,
                tau: num(&mut words)?,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    tag(&mut words, "relations")?;
    let n = count(&mut words, paper::MAX_COLUMNS + 1)?;
    let mut relations = Vec::with_capacity(n);
    for _ in 0..n {
        let u = num(&mut words)?;
        let v = num(&mut words)?;
        let k = count(&mut words, paper::MAX_SLOTS)?;
        let points = (0..k)
            .map(|_| Ok(Some((num(&mut words)?, num(&mut words)?))))
            .collect::<Result<Vec<_>, Error>>()?;
        relations.push(paper::Relation { u, v, points });
    }
    tag(&mut words, "dependency")?;
    let n = count(&mut words, paper::MAX_COLUMNS + 1)?;
    let dependency = (0..n)
        .map(|_| num(&mut words))
        .collect::<Result<Vec<_>, _>>()?;
    if words.next().is_some() {
        return Err(Error::Verification);
    }
    Ok((
        c,
        p,
        q,
        paper::Certificate {
            scalar,
            hashes,
            relations,
            dependency,
            trials,
            attempts,
        },
    ))
}

struct GpuOracle {
    child: Child,
    input: BufWriter<ChildStdin>,
    output: BufReader<ChildStdout>,
    cached: Vec<Vec<Point>>,
    curve: Curve,
    cap: u64,
}
impl GpuOracle {
    fn new(executable: &str, curve: &Curve, cap: u64, chunk: u64) -> Result<Self, Error> {
        let mut child = Command::new(executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|_| Error::Invalid("cannot start relation service"))?;
        let input = BufWriter::new(child.stdin.take().ok_or(Error::Verification)?);
        let output = BufReader::new(child.stdout.take().ok_or(Error::Verification)?);
        let mut oracle = Self {
            child,
            input,
            output,
            cached: Vec::new(),
            curve: curve.clone(),
            cap,
        };
        let (a, b) = curve.coefficients();
        let response =
            oracle.request(&format!("curve {} {a} {b} {cap} {chunk}", curve.modulus()))?;
        if response != ["READY", "1"] {
            return Err(Error::Verification);
        }
        Ok(oracle)
    }
    fn request(&mut self, request: &str) -> Result<Vec<String>, Error> {
        writeln!(self.input, "{request}")
            .and_then(|()| self.input.flush())
            .map_err(|_| Error::Invalid("relation service write failed"))?;
        let mut line = String::new();
        if self
            .output
            .read_line(&mut line)
            .map_err(|_| Error::Verification)?
            == 0
        {
            return Err(Error::Invalid("relation service exited without a result"));
        }
        let fields: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        if fields.first().is_some_and(|s| s == "ERROR") {
            eprintln!("GPU service: {}", line.trim());
            return Err(if fields.get(1).is_some_and(|s| s == "LIMIT") {
                Error::Limit("GPU service capacity or work bound")
            } else {
                Error::Invalid("GPU service error; see stderr")
            });
        }
        Ok(fields)
    }
    fn parse_base(&self, fields: &[String]) -> Result<Vec<Point>, Error> {
        if fields.first().map(String::as_str) != Some("BASE") || fields.len() < 2 {
            return Err(Error::Verification);
        }
        let n = integer(&fields[1])? as usize;
        if n as u64 > self.cap
            || n.checked_mul(2).and_then(|x| x.checked_add(2)) != Some(fields.len())
        {
            return Err(Error::Verification);
        }
        let points = fields[2..]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| Ok(Some((integer(&p[0])?, integer(&p[1])?))))
            .collect::<Result<Vec<_>, Error>>()?;
        if points.iter().any(|&p| !self.curve.contains(p))
            || points.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(Error::Verification);
        }
        Ok(points)
    }
}
fn integer(s: &str) -> Result<u64, Error> {
    s.parse().map_err(|_| Error::Verification)
}
impl Drop for GpuOracle {
    fn drop(&mut self) {
        // Wait/reap even when a CUDA error or malformed response interrupted IO.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
impl DecompositionOracle for GpuOracle {
    fn enumerate_bases(
        &mut self,
        curve: &Curve,
        hashes: &[HashBase],
    ) -> Result<Vec<Vec<Point>>, Error> {
        if curve != &self.curve {
            return Err(Error::Invalid("GPU curve changed"));
        }
        hashes
            .iter()
            .map(|h| {
                let response =
                    self.request(&format!("hash {} {} {} {}", h.s, h.alpha, h.beta, h.tau))?;
                self.parse_base(&response)
            })
            .collect()
    }
    fn find(
        &mut self,
        curve: &Curve,
        bases: &[Vec<Point>],
        target: Point,
        max_work: u64,
    ) -> Result<Option<Vec<Point>>, Error> {
        if curve != &self.curve || !curve.contains(target) {
            return Err(Error::Invalid("GPU curve or target"));
        }
        if self.cached != bases {
            let mut request = format!("bases {}", bases.len());
            for base in bases {
                request.push_str(&format!(" {}", base.len()));
                for &point in base {
                    let (x, y) = point.ok_or(Error::Invalid("identity in base"))?;
                    request.push_str(&format!(" {x} {y}"));
                }
            }
            let response = self.request(&request)?;
            if response.len() != 4 || response[0] != "PREPARED" {
                return Err(Error::Verification);
            }
            self.cached = bases.to_vec();
        }
        let (inf, x, y) = target.map_or((1, 0, 0), |(x, y)| (0, x, y));
        let response = self.request(&format!("find {inf} {x} {y} {max_work}"))?;
        if response == ["NONE"] {
            return Ok(None);
        }
        if response.len() != bases.len() + 2
            || response[0] != "FOUND"
            || integer(&response[1])? as usize != bases.len()
        {
            return Err(Error::Verification);
        }
        let tuple = response[2..]
            .iter()
            .zip(bases)
            .map(|(index, base)| {
                base.get(usize::try_from(integer(index)?).map_err(|_| Error::Verification)?)
                    .copied()
                    .ok_or(Error::Verification)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let sum = tuple.iter().try_fold(None, |acc, &p| curve.add(acc, p))?;
        if sum != target {
            return Err(Error::Verification);
        }
        Ok(Some(tuple))
    }
}

fn selftest(executable: &str) -> Result<(), Error> {
    let c = Curve::new(211, 0, 7, 199)?;
    let mut gpu = GpuOracle::new(executable, &c, 100_000, 257)?;
    let mut reference = paper::Enumerate;
    let hashes = [
        HashBase {
            s: 431,
            alpha: 17,
            beta: 23,
            tau: 180,
        },
        HashBase {
            s: 431,
            alpha: 31,
            beta: 7,
            tau: 160,
        },
    ];
    let bases = gpu.enumerate_bases(&c, &hashes)?;
    if bases != reference.enumerate_bases(&c, &hashes)? {
        return Err(Error::Verification);
    }
    let p = Some((3, 33));
    for k in 0..c.order() {
        let target = c.mul(p, k)?;
        let a = gpu.find(&c, &bases, target, 100_000)?;
        let b = reference.find(&c, &bases, target, 100_000)?;
        if a.is_some() != b.is_some() {
            return Err(Error::Verification);
        }
        if a != gpu.find(&c, &bases, target, 100_000)? {
            return Err(Error::Verification);
        }
    }
    for m in [2, 3] {
        let config = paper::Config {
            m,
            tau: Some(160),
            seed: 1,
            max_trials: 64,
            max_work: 100_000,
            ..Default::default()
        };
        let target = c.mul(p, 42)?; // Public known-answer software fixture only.
        let cert = paper::solve_with_oracle(&c, p, target, &config, &mut gpu)?;
        paper::verify(&c, p, target, &cert)?;
        if cert.scalar != 42 {
            return Err(Error::Verification);
        }
        let archive = encode_certificate(&c, p, target, &cert)?;
        let expanded = paper::solve_with_oracle_profile_budget(
            &c,
            p,
            target,
            &config,
            &mut gpu,
            &mut paper::ReductionStats::default(),
            128,
        )?;
        if encode_certificate(&c, p, target, &expanded)? != archive {
            return Err(Error::Verification);
        }
        let (cc, pp, qq, replayed) = decode_certificate(&archive)?;
        paper::verify(&cc, pp, qq, &replayed)?;
        if replayed != cert {
            return Err(Error::Verification);
        }
        let mut corrupt = replayed;
        corrupt.scalar = (corrupt.scalar + 1) % cc.order();
        if paper::verify(&cc, pp, qq, &corrupt).is_ok()
            || decode_certificate(&(archive + "trailing")).is_ok()
        {
            return Err(Error::Verification);
        }
    }
    println!("PASS service hash bases, all-target CPU oracle differential, deterministic replay, complete reduction certificates");
    Ok(())
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--verify" {
        let text = std::fs::read_to_string(&args[2])?;
        let (c, p, q, cert) = decode_certificate(&text)?;
        paper::verify(&c, p, q, &cert)?;
        println!("VERIFIED scalar={}", cert.scalar);
        return Ok(());
    }
    if args.len() == 3 && args[2] == "--self-test" {
        selftest(&args[1])?;
        return Ok(());
    }
    if args.len() != 15 && args.len() != 18 && args.len() != 19 {
        return Err("usage: paper-driver SERVICE p a b r Px Py Qx Qy m tau seed max_rows chunk_rows [max_columns max_work max_trials [attempts_per_column]]; or SERVICE --self-test".into());
    }
    let n = |i: usize| -> Result<u64, Box<dyn std::error::Error>> { Ok(args[i].parse()?) };
    let started = std::time::Instant::now();
    let p = Some((n(6)?, n(7)?));
    let q = Some((n(8)?, n(9)?));
    let c = Curve::with_order_witness(n(2)?, n(3)?, n(4)?, n(5)?, p)?;
    let validated = std::time::Instant::now();
    let config = paper::Config {
        max_columns: if args.len() >= 18 {
            usize::try_from(n(15)?)?
        } else {
            512
        },
        max_work: if args.len() >= 18 { n(16)? } else { 1_000_000 },
        max_trials: if args.len() >= 18 {
            usize::try_from(n(17)?)?
        } else {
            16
        },
        m: usize::try_from(n(10)?)?,
        tau: Some(n(11)?),
        seed: n(12)?,
    };
    let mut oracle = GpuOracle::new(&args[1], &c, n(13)?, n(14)?)?;
    let ready = std::time::Instant::now();
    let mut stats = paper::ReductionStats::default();
    let attempts_per_column = if args.len() == 19 {
        usize::try_from(n(18)?)?
    } else {
        20
    };
    eprintln!("QUERY_BUDGET attempts_per_column={attempts_per_column}");
    let cert = paper::solve_with_oracle_profile_budget(
        &c,
        p,
        q,
        &config,
        &mut oracle,
        &mut stats,
        attempts_per_column,
    )?;
    let archive = encode_certificate(&c, p, q, &cert)?;
    print!("{archive}");
    std::io::stdout().flush()?;
    eprintln!("E2E total_ms={:.6} validate_ms={:.6} service_ms={:.6} bases_ms={:.6} targets_ms={:.6} oracle_ms={:.6} linear_ms={:.6} replay_ms={:.6} trials={} attempts={} relations={} certificate_bytes={}",
        started.elapsed().as_secs_f64()*1000.0,
        validated.duration_since(started).as_secs_f64()*1000.0,
        ready.duration_since(validated).as_secs_f64()*1000.0,
        stats.bases.as_secs_f64()*1000.0,stats.targets.as_secs_f64()*1000.0,
        stats.oracle.as_secs_f64()*1000.0,stats.linear.as_secs_f64()*1000.0,
        stats.replay.as_secs_f64()*1000.0,cert.trials,cert.attempts,cert.relations.len(),archive.len());
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod certificate_tests {
    use super::*;

    #[test]
    fn twelve_slot_wire_roundtrip_and_excess_count_rejection() {
        let c = Curve::new(211, 0, 7, 199).unwrap();
        let p = Some((3, 33));
        let q = c.mul(p, 42).unwrap();
        let m = paper::MAX_SLOTS;
        let mut relations = vec![
            paper::Relation {
                u: m as u64,
                v: 0,
                points: vec![p; m]
            };
            m + 1
        ];
        relations[m].u = (m as u64 + c.order() - 42) % c.order();
        relations[m].v = 1;
        let mut dependency = vec![0; m + 1];
        dependency[0] = 1;
        dependency[m] = c.order() - 1;
        let cert = paper::Certificate {
            scalar: 42,
            hashes: vec![
                HashBase {
                    s: 431,
                    alpha: 1,
                    beta: 424,
                    tau: 1
                };
                m
            ],
            relations,
            dependency,
            trials: 1,
            attempts: m + 1,
        };
        let text = encode_certificate(&c, p, q, &cert).unwrap();
        let (replayed_curve, rp, rq, decoded) = decode_certificate(&text).unwrap();
        assert_eq!(decoded, cert);
        paper::verify(&replayed_curve, rp, rq, &decoded).unwrap();
        assert!(decode_certificate(
            &text.replace(&format!("hashes {m}\n"), &format!("hashes {}\n", m + 1))
        )
        .is_err());
        let mut excessive = cert;
        excessive.relations[0].points.push(p);
        assert!(decode_certificate(&encode_certificate(&c, p, q, &excessive).unwrap()).is_err());
    }
}
