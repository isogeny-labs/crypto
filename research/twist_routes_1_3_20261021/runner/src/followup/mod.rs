//! Native follow-up experiments. Every command requires a fresh output tree.
mod bigfield;
mod gluing;

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
