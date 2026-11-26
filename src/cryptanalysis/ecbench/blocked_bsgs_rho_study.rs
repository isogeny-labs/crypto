//! Fixed non-Cartesian plan for the preregistered blocked-BSGS/rho wall study.
//!
//! This module deliberately exposes no schedule knobs. The v3 spec selects one
//! audited schedule and supplies the nonzero master seed from which every
//! public target input and rho-walk seed is derived.

use std::collections::{BTreeMap, HashSet};

use super::methods::{applicable, MethodSpec};
use super::spec::{
    AlgorithmSeedRound, Arm, CurvePolicy, Execution, Order, Role, Spec, StudyExecution,
    StudyItemIdentity, STUDY_SPEC_SCHEMA,
};
use super::workload::{CurveSpec, Instance, TargetKind, Workload};
use crate::hash::sha256::Sha256;

pub const STUDY_ID: &str = "blocked-bsgs-rho-wall/v1";
const DIAGNOSTIC_REASON: &str =
    "fixed public toy study outside the canonical scaling-panel claim ladder";

pub(crate) struct FixedExpansion {
    pub workloads: Vec<Workload>,
    pub warnings: Vec<String>,
    pub instances: Vec<Instance>,
    pub curve_of: Vec<usize>,
    pub executions: Vec<Execution>,
}

#[derive(Clone, Copy)]
struct Panel {
    name: &'static str,
    replicates: u64,
    items: u64,
    left: &'static str,
    right: &'static str,
    left_copy: &'static str,
    right_copy: &'static str,
    has_rho: bool,
    warmup: bool,
    round_base: u32,
}

const PANELS: [Panel; 5] = [
    Panel {
        name: "warmup",
        replicates: 4,
        items: 1,
        left: "bsgs-cold",
        right: "rho-cold",
        left_copy: "main",
        right_copy: "main",
        has_rho: true,
        warmup: true,
        round_base: 0,
    },
    Panel {
        name: "cold-paired",
        replicates: 512,
        items: 1,
        left: "bsgs-cold",
        right: "rho-cold",
        left_copy: "main",
        right_copy: "main",
        has_rho: true,
        warmup: false,
        round_base: 4,
    },
    Panel {
        name: "bsgs-aa",
        replicates: 64,
        items: 1,
        left: "bsgs-cold",
        right: "bsgs-aa-copy",
        left_copy: "1",
        right_copy: "2",
        has_rho: false,
        warmup: false,
        round_base: 516,
    },
    Panel {
        name: "rho-aa",
        replicates: 64,
        items: 1,
        left: "rho-cold",
        right: "rho-aa-copy",
        left_copy: "1",
        right_copy: "2",
        has_rho: true,
        warmup: false,
        round_base: 580,
    },
    Panel {
        name: "amortized-B64",
        replicates: 16,
        items: 64,
        left: "bsgs-amortized-b64",
        right: "rho-amortized-b64",
        left_copy: "main",
        right_copy: "main",
        has_rho: true,
        warmup: false,
        round_base: 644,
    },
];

pub fn curves() -> Vec<CurveSpec> {
    vec![
        CurveSpec::PrimeExplicit {
            p: 16_777_199,
            a: 16_777_196,
            b: 31,
            group_order: 16_782_299,
            r: 16_782_299,
            gx: 2,
            gy: 5_412_647,
        },
        CurveSpec::PrimeExplicit {
            p: 268_435_399,
            a: 268_435_396,
            b: 3,
            group_order: 268_407_199,
            r: 268_407_199,
            gx: 1,
            gy: 1,
        },
        CurveSpec::PrimeExplicit {
            p: 4_294_967_291,
            a: 4_294_967_288,
            b: 31,
            group_order: 4_294_959_973,
            r: 4_294_959_973,
            gx: 2,
            gy: 2_120_051_858,
        },
        CurveSpec::PrimeExplicit {
            p: 68_719_476_731,
            a: 68_719_476_728,
            b: 33,
            group_order: 68_719_022_539,
            r: 68_719_022_539,
            gx: 2,
            gy: 51_741_851_039,
        },
        CurveSpec::PrimeExplicit {
            p: 649_522_587_953,
            a: 552_810_359_288,
            b: 555_863_716_954,
            group_order: 649_523_094_257,
            r: 649_523_094_257,
            gx: 549_251_065_028,
            gy: 609_044_633_619,
        },
    ]
}

fn expected_arms() -> [(&'static str, Role, &'static str); 6] {
    [
        ("rho-cold", Role::Reference, "rho.prime_fast_v1"),
        ("bsgs-cold", Role::Candidate, "bsgs.prime_paired_v1"),
        ("bsgs-aa-copy", Role::Control, "bsgs.prime_paired_v1"),
        ("rho-aa-copy", Role::Control, "rho.prime_fast_v1"),
        (
            "bsgs-amortized-b64",
            Role::Candidate,
            "bsgs.prime_paired_batch64_v1",
        ),
        (
            "rho-amortized-b64",
            Role::Reference,
            "rho.prime_fast_batch64_v1",
        ),
    ]
}

fn validate_spec(spec: &Spec, arms: &[Arm]) -> Result<(), String> {
    if spec.schema != STUDY_SPEC_SCHEMA {
        return Err("blocked_bsgs_rho_wall_v1 requires ecbench.spec/v3".into());
    }
    if !matches!(
        &spec.curve_policy,
        CurvePolicy::Diagnostic { off_panel_reason } if off_panel_reason == DIAGNOSTIC_REASON
    ) {
        return Err("fixed study requires its exact diagnostic curve policy".into());
    }
    if spec.field_profile.is_some()
        || spec.workloads.curves != curves()
        || spec.workloads.targets_per_curve != 660
        || spec.workloads.target_seed == 0
        || spec.workloads.target_kind != TargetKind::Public
    {
        return Err("fixed study workload descriptor differs from the frozen schedule".into());
    }
    let measurement = &spec.measurement;
    if measurement.rounds != 1
        || measurement.warmup != 0
        || measurement.order != Order::AlternateMeasured
        || measurement.seed != 0
        || measurement.algorithm_seed_round != AlgorithmSeedRound::MeasuredIndex
        || measurement.isolation_required != super::isolation::Level::L2
        || measurement.timeout_seconds != 3_600
    {
        return Err("fixed study measurement descriptor differs from the frozen schedule".into());
    }
    let expected = expected_arms();
    if spec.arms.len() != expected.len() || arms.len() != expected.len() {
        return Err("fixed study requires exactly six arms".into());
    }
    for ((name, role, method), (source, resolved)) in
        expected.iter().zip(spec.arms.iter().zip(arms.iter()))
    {
        if source.name != *name
            || source.role != *role
            || source.method
                != (MethodSpec {
                    id: (*method).to_owned(),
                    params: BTreeMap::new(),
                })
            || resolved.name != *name
            || resolved.role != *role
            || resolved.method.id != *method
        {
            return Err(format!(
                "fixed study arm `{name}` differs from its frozen definition"
            ));
        }
    }
    Ok(())
}

pub fn derive_u64(
    master_seed: u64,
    curve_slug: &str,
    panel: &str,
    replicate: u64,
    item: u64,
    stream_label: &str,
) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(STUDY_ID.as_bytes());
    hasher.update(&[0]);
    hasher.update(curve_slug.as_bytes());
    hasher.update(&[0]);
    hasher.update(panel.as_bytes());
    hasher.update(&[0]);
    hasher.update(&replicate.to_le_bytes());
    hasher.update(&[0]);
    hasher.update(&item.to_le_bytes());
    hasher.update(&[0]);
    hasher.update(stream_label.as_bytes());
    hasher.update(&[0]);
    hasher.update(&master_seed.to_le_bytes());
    let digest = hasher.finalize();
    u64::from_le_bytes(digest[..8].try_into().expect("eight digest bytes"))
}

fn make_workload(
    curve_spec: &CurveSpec,
    instance: &Instance,
    first: &StudyItemIdentity,
) -> Result<Workload, String> {
    Workload::from_explicit_public_target(curve_spec, instance, first.target.clone())
}

fn insert_unique_workload(
    workloads: &mut Vec<Workload>,
    curve_of: &mut Vec<usize>,
    workload_ids: &mut BTreeMap<String, usize>,
    workload: Workload,
    curve_index: usize,
) -> Result<usize, String> {
    if workload_ids.contains_key(&workload.workload_id) {
        return Err(format!(
            "duplicate fixed-study public workload `{}`",
            workload.workload_id
        ));
    }
    let index = workloads.len();
    workload_ids.insert(workload.workload_id.clone(), index);
    workloads.push(workload);
    curve_of.push(curve_index);
    Ok(index)
}

fn insert_unique_public_target(
    public_targets: &mut HashSet<(String, [String; 2])>,
    curve_slug: &str,
    target: &[String; 2],
) -> Result<(), String> {
    if !public_targets.insert((curve_slug.to_owned(), target.clone())) {
        return Err(format!(
            "duplicate fixed-study public target on curve `{curve_slug}`: [{}, {}]",
            target[0], target[1]
        ));
    }
    Ok(())
}

pub(crate) fn expand(spec: &Spec, arms: &[Arm]) -> Result<FixedExpansion, String> {
    validate_spec(spec, arms)?;

    let curve_specs = curves();
    let mut instances = Vec::with_capacity(curve_specs.len());
    for curve_spec in &curve_specs {
        instances.push(curve_spec.build()?);
    }
    for (curve_spec, instance) in curve_specs.iter().zip(&instances) {
        let facts = instance.facts(curve_spec);
        for arm in arms {
            applicable(&arm.method, instance, &facts).map_err(|error| {
                format!(
                    "fixed arm `{}` ({}) cannot run on {}: {error}",
                    arm.name, arm.method.id, facts.slug
                )
            })?;
        }
    }

    let arm_indices: BTreeMap<_, _> = arms
        .iter()
        .enumerate()
        .map(|(index, arm)| (arm.name.as_str(), index))
        .collect();
    let mut workloads = Vec::with_capacity(3_300);
    let mut curve_of = Vec::with_capacity(3_300);
    let mut executions = Vec::with_capacity(6_600);
    let mut all_derived = HashSet::with_capacity(16_360);
    let mut public_targets = HashSet::with_capacity(8_340);
    let mut workload_ids = BTreeMap::<String, usize>::new();
    let mut logical_workloads = 0usize;
    let mut seq = 0u64;

    for (curve_index, (curve_spec, instance)) in curve_specs.iter().zip(&instances).enumerate() {
        let slug = instance.facts(curve_spec).slug;
        for panel in PANELS {
            for replicate in 0..panel.replicates {
                let mut items = Vec::with_capacity(panel.items as usize);
                for item in 0..panel.items {
                    let target_seed = derive_u64(
                        spec.workloads.target_seed,
                        &slug,
                        panel.name,
                        replicate,
                        item,
                        "target",
                    );
                    if !all_derived.insert(target_seed) {
                        return Err("duplicate fixed-study derived seed".into());
                    }
                    let rho_seed = if panel.has_rho {
                        let seed = derive_u64(
                            spec.workloads.target_seed,
                            &slug,
                            panel.name,
                            replicate,
                            item,
                            "rho-walk",
                        );
                        if !all_derived.insert(seed) {
                            return Err("duplicate fixed-study derived seed".into());
                        }
                        Some(format!("{seed:016x}"))
                    } else {
                        None
                    };
                    let public =
                        Workload::on(curve_spec, instance, target_seed, item, TargetKind::Public)?;
                    if public.planted.is_some() || public.target_counter.is_none() {
                        return Err("fixed-study public target law exposed a planted answer".into());
                    }
                    insert_unique_public_target(&mut public_targets, &slug, &public.target)?;
                    items.push(StudyItemIdentity {
                        item,
                        target_derived_u64_hex: format!("{target_seed:016x}"),
                        target: public.target,
                        rho_derived_u64_hex: rho_seed,
                    });
                }
                let workload = make_workload(curve_spec, instance, &items[0])?;
                logical_workloads += 1;
                let workload_index = insert_unique_workload(
                    &mut workloads,
                    &mut curve_of,
                    &mut workload_ids,
                    workload,
                    curve_index,
                )?;

                let left = *arm_indices
                    .get(panel.left)
                    .ok_or("fixed-study left arm is absent")?;
                let right = *arm_indices
                    .get(panel.right)
                    .ok_or("fixed-study right arm is absent")?;
                let mut pair = [
                    (left, panel.left, panel.left_copy),
                    (right, panel.right, panel.right_copy),
                ];
                if replicate % 2 == 1 {
                    pair.reverse();
                }
                let algorithm_seed = items[0]
                    .rho_derived_u64_hex
                    .as_deref()
                    .map(|seed| u64::from_str_radix(seed, 16).expect("fixed rho seed"))
                    .unwrap_or(0);
                for (schedule_position, (arm, arm_name, copy)) in pair.into_iter().enumerate() {
                    executions.push(Execution {
                        seq,
                        round: panel.round_base + replicate as u32,
                        warmup: panel.warmup,
                        arm,
                        workload: workload_index,
                        algorithm_seed,
                        study: Some(StudyExecution {
                            study: STUDY_ID.to_owned(),
                            curve_slug: slug.clone(),
                            panel: panel.name.to_owned(),
                            replicate,
                            arm: arm_name.to_owned(),
                            copy: copy.to_owned(),
                            schedule_position: schedule_position as u8,
                            items: items.clone(),
                        }),
                    });
                    seq += 1;
                }
            }
        }
    }
    if logical_workloads != 3_300
        || workloads.len() != 3_300
        || executions.len() != 6_600
        || all_derived.len() != 16_360
        || public_targets.len() != 8_340
    {
        return Err(format!(
            "fixed-study dimension mismatch: logical workloads {logical_workloads}, unique workloads {}, executions {}, derived seeds {}, unique public targets {}",
            workloads.len(),
            executions.len(),
            all_derived.len(),
            public_targets.len()
        ));
    }
    Ok(FixedExpansion {
        workloads,
        warnings: Vec::new(),
        instances,
        curve_of,
        executions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::spec::{plan, Spec};
    use serde_json::json;

    fn fixed_spec() -> Spec {
        Spec::from_json(
            &json!({
                "schema": STUDY_SPEC_SCHEMA,
                "label": "blocked BSGS/rho fixed wall study",
                "description": "fixed synthetic protocol; no production targets",
                "curve_policy": {"mode":"diagnostic", "off_panel_reason":DIAGNOSTIC_REASON},
                "workloads": {
                    "curves": curves(),
                    "targets_per_curve": 660,
                    "target_seed": 20261126,
                    "target_kind": "public"
                },
                "arms": [
                    {"name":"rho-cold","role":"reference","method":{"id":"rho.prime_fast_v1"}},
                    {"name":"bsgs-cold","role":"candidate","method":{"id":"bsgs.prime_paired_v1"}},
                    {"name":"bsgs-aa-copy","role":"control","method":{"id":"bsgs.prime_paired_v1"}},
                    {"name":"rho-aa-copy","role":"control","method":{"id":"rho.prime_fast_v1"}},
                    {"name":"bsgs-amortized-b64","role":"candidate","method":{"id":"bsgs.prime_paired_batch64_v1"}},
                    {"name":"rho-amortized-b64","role":"reference","method":{"id":"rho.prime_fast_batch64_v1"}}
                ],
                "measurement": {
                    "rounds":1,
                    "warmup":0,
                    "order":"alternate_measured",
                    "seed":0,
                    "algorithm_seed_round":"measured_index",
                    "isolation_required":"L2",
                    "timeout_seconds":3600
                }
            })
            .to_string(),
        )
        .unwrap()
    }

    #[test]
    fn fixed_plan_has_exact_population_and_nested_batches() {
        let plan = plan(fixed_spec()).unwrap();
        assert_eq!(plan.instances.len(), 5);
        assert_eq!(plan.workloads.len(), 3_300);
        assert!(plan
            .workloads
            .iter()
            .all(|workload| workload.target_seed == 0
                && workload.target_index == 0
                && workload.planted.is_none()));
        let first = plan.executions[0].study.as_ref().unwrap();
        let target_seed = u64::from_str_radix(&first.items[0].target_derived_u64_hex, 16).unwrap();
        let expected = Workload::on(
            &plan.spec.workloads.curves[0],
            &plan.instances[0],
            target_seed,
            first.items[0].item,
            TargetKind::Public,
        )
        .unwrap();
        assert!(expected.planted.is_none());
        assert!(expected.target_counter.is_some());
        assert_eq!(first.items[0].target, expected.target);
        assert_eq!(plan.executions.len(), 6_600);
        assert_eq!(plan.executions.iter().filter(|e| e.warmup).count(), 40);
        let mut panels = BTreeMap::new();
        for execution in &plan.executions {
            assert_eq!(execution.seq as usize, panels.values().sum::<usize>());
            let study = execution.study.as_ref().unwrap();
            *panels.entry(study.panel.as_str()).or_insert(0usize) += 1;
            if study.panel == "amortized-B64" {
                assert_eq!(study.items.len(), 64);
            } else {
                assert_eq!(study.items.len(), 1);
            }
        }
        assert_eq!(panels["warmup"], 40);
        assert_eq!(panels["cold-paired"], 5_120);
        assert_eq!(panels["bsgs-aa"], 640);
        assert_eq!(panels["rho-aa"], 640);
        assert_eq!(panels["amortized-B64"], 160);
        let public_targets: HashSet<_> = plan
            .executions
            .iter()
            .filter_map(|execution| {
                let study = execution.study.as_ref().unwrap();
                (study.schedule_position == 0).then_some(study)
            })
            .flat_map(|study| {
                study.items.iter().map(move |item| {
                    (
                        study.curve_slug.clone(),
                        item.target[0].clone(),
                        item.target[1].clone(),
                    )
                })
            })
            .collect();
        assert_eq!(public_targets.len(), 8_340);
    }

    #[test]
    fn fixed_plan_alternates_each_pair_and_pins_seed_vector() {
        let plan = plan(fixed_spec()).unwrap();
        let cold: Vec<_> = plan
            .executions
            .iter()
            .filter(|execution| {
                let study = execution.study.as_ref().unwrap();
                study.curve_slug == "icv1-fp24-tm5099-a6f137c6"
                    && study.panel == "cold-paired"
                    && study.replicate < 2
            })
            .collect();
        assert_eq!(cold.len(), 4);
        assert_eq!(cold[0].study.as_ref().unwrap().arm, "bsgs-cold");
        assert_eq!(cold[1].study.as_ref().unwrap().arm, "rho-cold");
        assert_eq!(cold[2].study.as_ref().unwrap().arm, "rho-cold");
        assert_eq!(cold[3].study.as_ref().unwrap().arm, "bsgs-cold");
        assert_eq!(
            cold[0].study.as_ref().unwrap().items[0].target_derived_u64_hex,
            "391199f02ba67067"
        );
    }

    #[test]
    fn fixed_plan_rejects_dimension_tuning() {
        let mut bad = fixed_spec();
        bad.workloads.targets_per_curve = 659;
        assert!(plan(bad).err().unwrap().contains("descriptor"));
        let mut zero_master = fixed_spec();
        zero_master.workloads.target_seed = 0;
        assert!(plan(zero_master).err().unwrap().contains("descriptor"));
    }

    #[test]
    fn duplicate_public_workload_is_rejected() {
        let curve_spec = curves().remove(0);
        let instance = curve_spec.build().unwrap();
        let public = Workload::on(&curve_spec, &instance, 20261126, 0, TargetKind::Public).unwrap();
        let workload =
            Workload::from_explicit_public_target(&curve_spec, &instance, public.target).unwrap();
        let workload_id = workload.workload_id.clone();
        let mut workloads = Vec::new();
        let mut curve_of = Vec::new();
        let mut ids = BTreeMap::new();
        assert_eq!(
            insert_unique_workload(&mut workloads, &mut curve_of, &mut ids, workload.clone(), 0,)
                .unwrap(),
            0
        );
        let error = insert_unique_workload(&mut workloads, &mut curve_of, &mut ids, workload, 0)
            .unwrap_err();
        assert_eq!(
            error,
            format!("duplicate fixed-study public workload `{workload_id}`")
        );
    }

    #[test]
    fn duplicate_public_target_is_rejected() {
        let target = ["0x1".to_owned(), "0x2".to_owned()];
        let mut targets = HashSet::new();
        insert_unique_public_target(&mut targets, "curve-a", &target).unwrap();
        assert_eq!(
            insert_unique_public_target(&mut targets, "curve-a", &target).unwrap_err(),
            "duplicate fixed-study public target on curve `curve-a`: [0x1, 0x2]"
        );
        insert_unique_public_target(&mut targets, "curve-b", &target).unwrap();
    }
}
