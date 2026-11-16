//! Reuse the immutable reviewed enrollment; never generate plans or approve execution.
use super::{canonical::sha256_hex, diagnostic_binding_core::parse};
use std::collections::BTreeSet;
const ROUTES: &[(&str, &str, &str, &str, usize)] = &[
    (
        "gate-p739.json",
        "bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776",
        "a2c25a203ad4c39aaea57287e1705dd4b3db394cd10507fff009980ec8634c18",
        "ECS1h1bab8ffa33b6",
        60,
    ),
    (
        "cohort-16-remainder-plan.json",
        "39b73d2a2ffbab3deec9dbdf9857af15a66607ca8843b8f29c64cd94c7d2d9c9",
        "330c2017234961023a30675cf6056fb7bf4c1bee1c8dabbaa4610f6ca4ba63c9",
        "ECS1h9a5aafd3a403",
        120,
    ),
    (
        "cohort-20.json",
        "51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f",
        "33104eb19ee7a8995bccb43a3768824643b6ec6dae45bca4da809f88503fb639",
        "ECS1h2bd191253bbd",
        180,
    ),
    (
        "cohort-24.json",
        "7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4",
        "cbc91d501e44fb101a96ff0ce9a7d96cbd37d2761e910fa62cbd47dfd56ac315",
        "ECS1heaff76e306f7",
        180,
    ),
    (
        "cohort-28.json",
        "87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001",
        "04c87c844c9216993cfa90aecbb1606080338232ebfa6ab91f12b6881d4b477c",
        "ECS1he4c8727d2fc2",
        180,
    ),
    (
        "cohort-32.json",
        "a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96",
        "0bcb2f5f8811a370997724bf0186330fba14a896346d9206d3f67d8368cb22a3",
        "ECS1hdfe262979cdb",
        180,
    ),
    (
        "cohort-36.json",
        "6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2",
        "39cac1b85cb9672e5e5f3418957c994a005b59432dd2b6ca74226bed60d832fe",
        "ECS1hcd32ffa92b64",
        180,
    ),
    (
        "cohort-40.json",
        "8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5",
        "fee38c5c381107272d65a89daa8151f75aa249082d0e9c82f1014211fdee2587",
        "ECS1hf76a63f60392",
        180,
    ),
    (
        "gate-p186379-target0.json",
        "481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf",
        "d4ec180d5a2cf65617a5821b565ec0d4952435a77669da65ca77852d2723c32c",
        "ECS1hea0daedd0dff",
        1,
    ),
];
const ARTIFACTS: &[(&str, &str)] = &[
    (
        "certificate",
        "de0003271c4f49898dd38f261c279338bfe756779d405d0753d0401d58b83e72",
    ),
    (
        "request",
        "6b99db317edac83a401126ec1d427739b2e786591e4fbaffd0003ce882c846f2",
    ),
    (
        "receipt",
        "3a48cda05911f1274b467590a33bddf97e91976155b3c62e78a52948148072d3",
    ),
    (
        "review",
        "8e748e9bbcc34a6a8aa09e874df14972c3b67e60d074293a91d31b8113fb9b76",
    ),
];

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
