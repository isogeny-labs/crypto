//! Native follow-up experiments. Every command requires a fresh output tree.
mod bigfield;
mod coupled;
mod divisors;
mod geometry;
mod gluing;
mod homogeneous;
mod partial;
mod projection;

use crate::{digest, write_json, Fixture, Result};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, ToPrimitive};
use serde_json::{json, Value};
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Source {
    pub name: String,
    pub slug: String,
    pub p: BigUint,
    pub a: BigUint,
    pub b: BigUint,
    pub n: BigUint,
    pub d: BigUint,
}
impl Source {
    pub fn nt(&self) -> BigUint {
        (&self.p + BigUint::one()) * 2u32 - &self.n
    }
    pub fn residue_trace(&self, ell: u64) -> u64 {
        ((&self.p + BigUint::one() + &self.n * BigUint::from(ell)) - &self.n)
            .mod_floor(&BigUint::from(ell))
            .to_u64()
            .unwrap()
    }
    pub fn model(&self) -> Value {
        json!({"name":self.name,"source_slug":self.slug,"p":self.p.to_string(),
          "a":self.a.to_string(),"b":self.b.to_string(),"order":self.n.to_string(),
          "twist_d":self.d.to_string(),"twist_order":self.nt().to_string()})
    }
}
pub fn sources(fixtures: &[Fixture]) -> Result<Vec<Source>> {
    let mut out = fixtures
        .iter()
        .filter(|f| crate::toy_field::prime(f.order) && f.order % 2 == 1)
        .map(|f| Source {
            name: f.id.clone(),
            slug: f.id.clone(),
            p: f.p.into(),
            a: f.a.into(),
            b: f.b.into(),
            n: f.order.into(),
            d: f.d.into(),
        })
        .collect::<Vec<_>>();
    let registry: Value = serde_json::from_slice(&fs::read("docs/curves/registry.json")?)?;
    for name in ["P-192", "P-224", "P-256", "P-384", "P-521"] {
        let r = registry["curves"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| {
                r["standard_names"]
                    .as_array()
                    .is_some_and(|ns| ns.iter().any(|n| n == name))
            })
            .ok_or("missing standard curve")?;
        let p: BigUint = r["params"]["p"].as_str().unwrap().parse()?;
        let mut d = BigUint::from(2u32);
        let exp = (&p - BigUint::one()) / 2u32;
        while d.modpow(&exp, &p).is_one() {
            d += 1u32;
        }
        out.push(Source {
            name: name.into(),
            slug: r["slug"].as_str().unwrap().into(),
            p,
            a: r["params"]["a"].as_str().unwrap().parse()?,
            b: r["params"]["b"].as_str().unwrap().parse()?,
            n: r["order"].as_str().unwrap().parse()?,
            d,
        });
    }
    Ok(out)
}
pub fn main(args: &[String]) -> Result<()> {
    let command = args.first().ok_or("missing follow-up command")?;
    let manifest = fs::read(args.get(1).ok_or("missing frozen fixture manifest")?)?;
    let v: Value = serde_json::from_slice(&manifest)?;
    let fixtures: Vec<Fixture> = serde_json::from_value(v["fixtures"].clone())?;
    let out = Path::new(args.get(2).ok_or("missing fresh output directory")?);
    if out.exists() {
        return Err("follow-up output must not exist".into());
    }
    fs::create_dir_all(out)?;
    let mut src = sources(&fixtures)?;
    if command == "torsion" {
        if let Some(name) = args.get(3) {
            src.retain(|s| &s.name == name);
            if src.is_empty() {
                return Err("selected source does not exist".into());
            }
        }
    }
    let rows = match command.as_str() {
        "filters" => gluing::filters(&src, out)?,
        "torsion" => gluing::torsion(&src, out)?,
        "geometry-input" => geometry::input(&src, out)?,
        "geometry-verify" => geometry::verify(
            &src,
            Path::new(args.get(3).ok_or("missing GP stdout")?),
            out,
        )?,
        "adjudicate-cas" => {
            adjudicate(Path::new(args.get(3).ok_or("missing CAS log prefix")?), out)?
        }
        "coupled-census" => coupled::census(&fixtures, out)?,
        "projection-replay" => projection::replay(&fixtures, out)?,
        "homogeneous-pilot" => homogeneous::run(
            &fixtures,
            Path::new(args.get(3).ok_or("missing frozen bases")?),
            out,
        )?,
        "jacobian-replay" => divisors::independent(
            &fixtures,
            Path::new(args.get(3).ok_or("missing selected covers")?),
            out,
        )?,
        "three-transfer" => divisors::transfer(
            &fixtures,
            Path::new(args.get(3).ok_or("missing verified geometry index")?),
            out,
        )?,
        "partial-replay" => partial::replay(
            &fixtures,
            Path::new(args.get(3).ok_or("missing census directory")?),
            out,
        )?,
        "coupled-relations" => coupled::relations(
            &fixtures,
            Path::new(args.get(3).ok_or("missing census directory")?),
            out,
        )?,
        "coupled-energy" => coupled::energies(
            &fixtures,
            Path::new(args.get(3).ok_or("missing census directory")?),
            out,
        )?,
        _ => return Err("followup: expected filters or torsion".into()),
    };
    write_json(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.followup-run/v1","command":command,
      "fixture_sha256":digest(&manifest),"rows":rows,"performance_claim":false,
      "novelty_established":false,"independent_host_replay":false}),
    )?;
    println!(
        "{}",
        json!({"status":"finished","command":command,"rows":rows.len(),"out":out})
    );
    Ok(())
}

fn adjudicate(prefix: &Path, out: &Path) -> Result<Vec<Value>> {
    let suffix = |s: &str| std::path::PathBuf::from(format!("{}.{s}", prefix.display()));
    let meta: Value = serde_json::from_slice(&fs::read(suffix("json"))?)?;
    let stdout = fs::read(suffix("stdout"))?;
    let stderr = fs::read(suffix("stderr"))?;
    let text = std::str::from_utf8(&stdout)?;
    let errors = std::str::from_utf8(&stderr)?;
    let allowed_warning = |line: &str| {
        let growth = line.trim().strip_prefix("***").and_then(|s| {
            let s = s.trim();
            s.strip_prefix("Warning: increasing stack size to ")
                .map(|size| ("anonymous", size))
                .or_else(|| s.split_once(": Warning: increasing stack size to "))
        });
        let known_growth = growth.is_some_and(|(op, size)| {
            !op.is_empty()
                && op
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_+-*/^".contains(c))
                && size
                    .strip_suffix('.')
                    .is_some_and(|n| n.parse::<u64>().is_ok_and(|n| n <= 4_000_002_048))
        });
        line.trim().is_empty() || line.contains("Warning: new maximum stack size =") || known_growth
    };
    let error_lines = errors
        .lines()
        .filter(|l| !allowed_warning(l))
        .collect::<Vec<_>>();
    let marker = text.contains("INDEPENDENT_MODULE_REPLAY_OK")
        || text.contains("ALL_TORSION_REPLAY_OK")
        || text.contains("ALL_GEOMETRY_SEARCH_DONE");
    let marker = marker
        || text.contains("ALL_COUPLED_REPLAY_OK")
        || text.contains("ALL_PROJECTION_REPLAY_OK")
        || text.contains("ALL_DIVISOR_REPLAY_OK");
    let hashes_match =
        meta["stdout_sha256"] == digest(&stdout) && meta["stderr_sha256"] == digest(&stderr);
    let script = meta["argv"]
        .as_array()
        .and_then(|xs| xs.last())
        .and_then(Value::as_str)
        .ok_or("missing CAS input path")?;
    let script_hash_matches = meta["script_sha256"] == digest(&fs::read(script)?);
    let accepted = meta["exit_code"] == 0
        && marker
        && hashes_match
        && script_hash_matches
        && error_lines.is_empty();
    let row = json!({"schema":"twist.native-cas-adjudication/v1","log_prefix":prefix,"transport_metadata":meta,
      "completion_marker_present":marker,"raw_hashes_match":hashes_match,"script_hash_matches":script_hash_matches,"allowed_stack_warning_present":errors.contains("Warning: new maximum stack size ="),
      "unaccepted_error_lines":error_lines,"accepted":accepted,"status":if accepted{"accepted-native-checked-CAS-replay"}else{"invalid-or-incomplete-CAS-replay"},
      "scope":"The GP script performs independent group/pairing/polynomial checks. This Rust gate rejects errors even when GP exits zero and prints a marker."});
    write_json(out.join("ADJUDICATION.json"), &row)?;
    if !accepted {
        return Err("CAS replay contains errors, missing output or hash mismatches".into());
    }
    Ok(vec![row])
}

#[cfg(test)]
mod cas_tests {
    use super::*;
    #[test]
    fn gp_zero_exit_and_marker_cannot_hide_errors_or_input_mutation() {
        let dir = std::env::temp_dir().join(format!("twist-cas-gate-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("input.gp");
        fs::write(&script, "print(1);\n").unwrap();
        for (index, stderr, input_hash, expected) in [
            (
                0,
                "  *** Warning: new maximum stack size = 4000002048\n",
                digest(&fs::read(&script).unwrap()),
                true,
            ),
            (
                1,
                "  *** user error: bad Weil pairing\n",
                digest(&fs::read(&script).unwrap()),
                false,
            ),
            (2, "", String::from("wrong-input-hash"), false),
            (3, "  *** _+_: Warning: increasing stack size to 128000000.\n", digest(&fs::read(&script).unwrap()), true),
            (4, "  *** _+_: Warning: increasing stack size to 9999999999.\n", digest(&fs::read(&script).unwrap()), false),
            (5, "  *** _+_: Warning: increasing stack size to 128000000.\n  *** user error: failed divisor replay\n", digest(&fs::read(&script).unwrap()), false),
            (6, "  *** Mod: Warning: increasing stack size to 16000000.\n", digest(&fs::read(&script).unwrap()), true),
            (7, "  ***   Warning: increasing stack size to 32000000.\n", digest(&fs::read(&script).unwrap()), true),
        ] {
            let prefix = dir.join(format!("case-{index}"));
            let stdout = "ALL_TORSION_REPLAY_OK\n";
            fs::write(format!("{}.stdout", prefix.display()), stdout).unwrap();
            fs::write(format!("{}.stderr", prefix.display()), stderr).unwrap();
            write_json(format!("{}.json",prefix.display()),&json!({"exit_code":0,"argv":["gp","-f","-q",script],
              "script_sha256":input_hash,"stdout_sha256":digest(stdout.as_bytes()),"stderr_sha256":digest(stderr.as_bytes())})).unwrap();
            let out = dir.join(format!("out-{index}"));
            fs::create_dir_all(&out).unwrap();
            assert_eq!(adjudicate(&prefix, &out).is_ok(), expected);
            let row: Value =
                serde_json::from_slice(&fs::read(out.join("ADJUDICATION.json")).unwrap()).unwrap();
            assert_eq!(row["accepted"], expected);
        }
        fs::remove_dir_all(dir).unwrap();
    }
}
