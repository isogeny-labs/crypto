//! Reuse the immutable reviewed enrollment; never generate plans or approve execution.
use crate::{binding::parse, canonical::sha256_hex};
use std::collections::BTreeSet;
include!("inventory_pins.rs");
pub struct Input<'a> {
    pub route: &'a str,
    pub plan: &'a [u8],
    pub spec: &'a [u8],
}
#[derive(Debug)]
pub struct Inventory {
    pub stage: &'static str,
    pub software_bindings_complete: bool,
    pub scientific_execution_admitted: bool,
    pub workloads: usize,
    pub slots: usize,
}
pub fn check(inputs: &[Input<'_>], artifacts: &[(&str, &[u8])]) -> Result<Inventory, String> {
    if inputs.len() != 9 || artifacts.len() != ARTIFACTS.len() {
        return Err("inventory incomplete".into());
    }
    let mut seen = BTreeSet::new();
    for (name, raw) in artifacts {
        if !seen.insert(*name) {
            return Err("duplicate enrollment artifact".into());
        }
        let (_, h) = ARTIFACTS
            .iter()
            .find(|(n, _)| n == name)
            .ok_or("unknown enrollment artifact")?;
        if sha256_hex(raw) != *h {
            return Err(format!("{name}: artifact hash"));
        }
    }
    let cert = parse(
        artifacts
            .iter()
            .find(|(n, _)| *n == "certificate")
            .ok_or("certificate")?
            .1,
        1024 * 1024,
    )?;
    let enrolled = cert["enrolled_plans"]
        .as_array()
        .ok_or("certificate enrollment")?;
    seen.clear();
    let (mut targets, mut slots) = (0, 0);
    for input in inputs {
        if !seen.insert(input.route) {
            return Err("duplicate route".into());
        }
        let (_, ph, sh, sid, n) = ROUTES
            .iter()
            .find(|(name, ..)| *name == input.route)
            .ok_or("unknown route")?;
        if sha256_hex(input.plan) != *ph || sha256_hex(input.spec) != *sh {
            return Err("plan/spec hash".into());
        }
        let plan = parse(input.plan, 32 * 1024 * 1024)?;
        let spec = parse(input.spec, 32 * 1024 * 1024)?;
        if plan["schema"] != "ecbench.plan/v1" || plan["spec_id"] != *sid || !spec.is_object() {
            return Err("plan/spec identity".into());
        }
        if plan["workloads"].as_array().ok_or("workloads")?.len() != *n
            || plan["executions"].as_array().ok_or("executions")?.len() != n * 18
        {
            return Err("inventory count".into());
        }
        if input.route != "gate-p186379-target0.json" {
            if enrolled
                .iter()
                .filter(|e| e["name"] == input.route && e["sha256"] == *ph && e["spec_id"] == *sid)
                .count()
                != 1
            {
                return Err("certificate route join".into());
            }
            targets += n;
            slots += n * 18;
        } else if enrolled.iter().any(|e| e["name"] == input.route) {
            return Err("upper gate entered cohort".into());
        }
    }
    if (targets, slots) != (1260, 22680) {
        return Err("cohort denominator".into());
    }
    Ok(Inventory {
        stage: "static_inventory_only",
        software_bindings_complete: false,
        scientific_execution_admitted: false,
        workloads: targets,
        slots,
    })
}
/// Called by the actual-session adapter; it binds just this already-enrolled route.
pub fn verify_route(route: &str, plan: &[u8], spec: &[u8]) -> Result<(), String> {
    let (_, ph, sh, ..) = ROUTES
        .iter()
        .find(|(n, ..)| *n == route)
        .ok_or("unknown route")?;
    if sha256_hex(plan) != *ph || sha256_hex(spec) != *sh {
        return Err("actual plan/spec not enrolled".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    #[test]
    fn full_nine_inventory_and_corruptions() {
        let v: Value = serde_json::from_str(include_str!("../inventory-inputs.json")).unwrap();
        let rows: Vec<_> = v["routes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                (
                    x["name"].as_str().unwrap().to_owned(),
                    std::fs::read(x["path"].as_str().unwrap()).unwrap(),
                    std::fs::read(x["spec_path"].as_str().unwrap()).unwrap(),
                )
            })
            .collect();
        let arts: Vec<_> = v["artifacts"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, x)| {
                (
                    k.clone(),
                    std::fs::read(x["path"].as_str().unwrap()).unwrap(),
                )
            })
            .collect();
        let mut input: Vec<_> = rows
            .iter()
            .map(|(n, p, s)| Input {
                route: n,
                plan: p,
                spec: s,
            })
            .collect();
        let mut artifacts: Vec<_> = arts
            .iter()
            .map(|(n, b)| (n.as_str(), b.as_slice()))
            .collect();
        let result = check(&input, &artifacts).unwrap();
        assert!(!result.software_bindings_complete);
        assert!(!result.scientific_execution_admitted);
        assert_eq!(result.workloads, 1260);
        assert_eq!(result.slots, 22680);
        assert!(check(&[], &artifacts).is_err());
        assert!(check(&input, &[]).is_err());
        artifacts[0].1 = b"{}";
        assert!(check(&input, &artifacts).is_err());
        artifacts = arts
            .iter()
            .map(|(n, b)| (n.as_str(), b.as_slice()))
            .collect();
        input[8].route = "gate-p739.json";
        assert!(check(&input, &artifacts).is_err());
        assert!(verify_route("gate-p739.json", b"{}", rows[0].2.as_slice()).is_err());
    }
}
