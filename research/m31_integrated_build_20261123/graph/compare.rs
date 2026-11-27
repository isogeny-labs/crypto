use crate::{add, admission::{validate_rows, Admitted}, inv, jv_bitops,
    large_prime_filter::{filter_large_primes, filter_large_primes_mst, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions, MstLimits},
    large_prime_graph::{gain_dsu_rank, projected_rank::audit_projected_rank, witness::{collect_witnesses, collect_witnesses_lazy, materialize_complete_relations}, LpIndex},
    mul, sha256, sub, tape::{ProjectionTape, TapeLimits}, ELL, SOURCE_MANIFEST_SHA256, TARGET};
use prime31_algebraic_ic::{Arithmetic, PRIME31};
use serde::{Deserialize, Serialize};
use std::{collections::{BTreeMap, BTreeSet}, time::Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm { Eager, Lazy, Pivot, Mst, MatrixFree }
impl Arm {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s { "eager" => Ok(Self::Eager), "lazy" => Ok(Self::Lazy), "pivot" => Ok(Self::Pivot), "mst" => Ok(Self::Mst), "matrix-free" => Ok(Self::MatrixFree), _ => Err("unknown arm".into()) }
    }
    pub fn name(self) -> &'static str { match self { Self::Eager=>"eager", Self::Lazy=>"lazy", Self::Pivot=>"pivot", Self::Mst=>"mst", Self::MatrixFree=>"matrix-free" } }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub max_small_weight: usize,
    pub max_sources: usize,
    pub mst_max_column_degree: usize,
    pub mst_max_edge_evaluations: usize,
    pub max_active_coefficients: usize,
    pub tape: TapeLimits,
}
impl Default for Limits {
    fn default() -> Self { Self { max_small_weight: 4096, max_sources: 4096, mst_max_column_degree: 4096, mst_max_edge_evaluations: 1_000_000, max_active_coefficients: 1_000_000, tape: TapeLimits::default() } }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Phases {
    pub index_and_lp_rank_ns: u128,
    pub extraction_ns: u128,
    pub materialization_ns: u128,
    pub exact_rank_ns: u128,
    pub source_replay_ns: u128,
    pub target_recovery_and_group_check_ns: u128,
    pub forward_product_ns: u128,
    pub transpose_product_ns: u128,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub schema: String,
    pub arm: Arm,
    pub coordinate_prime: u64,
    pub relation_prime: u64,
    pub input_sha256: String,
    pub canonical_prefix_sha256: String,
    pub source_manifest_sha256: String,
    pub rows: usize,
    pub lp_rank: usize,
    pub lp_left_nullity: usize,
    pub status: String,
    pub refusal: Option<String>,
    pub output_rows: Option<usize>,
    pub useful_small_rank: Option<usize>,
    pub augmented_rank: Option<usize>,
    pub residual_columns: Option<usize>,
    pub residual_nonzeros: Option<usize>,
    pub source_terms: Option<usize>,
    pub replayed: Option<usize>,
    pub output_sha256: Option<String>,
    pub target_scalar: Option<u64>,
    pub target_group_verified: bool,
    pub phases: Phases,
    pub filter_hook_counts: [u64; 4],
    pub algorithm_counters: serde_json::Value,
    pub limits: Limits,
    pub complete_cold_solving_cost: Option<u64>,
    pub measurement_scope: String,
}

/// Separate-process callers must keep this exact prefix and limit object equal.
pub fn run(admitted: &Admitted, prefix: usize, arm: Arm, limits: &Limits) -> Result<Record, String> {
    if prefix == 0 || prefix > admitted.rows.len() { return Err("prefix is outside admitted input".into()); }
    let rows = &admitted.rows[..prefix];
    validate_rows(rows)?;
    let start = Instant::now();
    let index = LpIndex::build(rows, ELL).map_err(|e| format!("index: {e:?}"))?;
    let rank = gain_dsu_rank(&index).map_err(|e| format!("LP rank: {e:?}"))?;
    let mut phases = Phases { index_and_lp_rank_ns: start.elapsed().as_nanos(), ..Default::default() };
    let mut record = Record {
        schema: "m31-graph-arm/v1".into(), arm, coordinate_prime: PRIME31.p, relation_prime: ELL,
        input_sha256: admitted.manifest.input_sha256.clone(),
        canonical_prefix_sha256: sha256(&serde_json::to_vec(rows).map_err(|e| e.to_string())?),
        source_manifest_sha256: SOURCE_MANIFEST_SHA256.into(), rows: prefix, lp_rank: rank.rank,
        lp_left_nullity: prefix - rank.rank, status: "accepted".into(), refusal: None,
        output_rows: None, useful_small_rank: None, augmented_rank: None, residual_columns: None,
        residual_nonzeros: None, source_terms: None, replayed: None, output_sha256: None,
        target_scalar: None, target_group_verified: false, phases: phases.clone(), filter_hook_counts: [0;4],
        algorithm_counters: serde_json::Value::Null, limits: limits.clone(), complete_cold_solving_cost: None,
        measurement_scope: "supplied-group-equation component replay; collection/setup/cold solve unmeasured".into(),
    };
    if arm == Arm::MatrixFree {
        let start = Instant::now();
        let tape = match ProjectionTape::build(rows, &limits.tape) {
            Ok(t) => t,
            Err(e) => { record.status = "refused".into(); record.refusal = Some(e); record.phases.extraction_ns = start.elapsed().as_nanos(); return Ok(record); }
        };
        phases.extraction_ns = start.elapsed().as_nanos();
        if tape.stats().lp_rank != rank.rank { return Err("tape rank disagrees with original gain DSU".into()); }
        let x: Vec<u64> = (0..tape.columns().len()).map(|i| (i as u64 + 1) % ELL).collect();
        let y: Vec<u64> = (0..tape.stats().output_rows).map(|i| (2 * i as u64 + 3) % ELL).collect();
        let start = Instant::now();
        let hx = tape.apply(&x)?;
        phases.forward_product_ns = start.elapsed().as_nanos();
        let start = Instant::now();
        let hty = tape.apply_transpose(&y)?;
        phases.transpose_product_ns = start.elapsed().as_nanos();
        if hx.iter().zip(&y).fold(0, |s, (&a,&b)| add(s,mul(a,b))) != x.iter().zip(&hty).fold(0, |s, (&a,&b)| add(s,mul(a,b))) {
            return Err("forward/transpose bilinear identity failed".into());
        }
        record.status = "operator-only".into();
        record.output_rows = Some(tape.stats().output_rows);
        record.output_sha256 = Some(sha256(&serde_json::to_vec(&(hx,hty)).unwrap()));
        record.algorithm_counters = serde_json::to_value(tape.stats()).unwrap();
        record.phases = phases;
        return Ok(record);
    }
    let options = LargePrimeFilterOptions { modulus: ELL, max_small_weight: limits.max_small_weight, max_sources: limits.max_sources };
    jv_bitops::reset();
    let start = Instant::now();
    let output: Option<Vec<CompleteRelation>> = match arm {
        Arm::Eager | Arm::Lazy => {
            let batch = if arm == Arm::Eager { collect_witnesses(&index) } else { collect_witnesses_lazy(&index) }.map_err(|e| format!("forest: {e:?}"))?;
            phases.extraction_ns = start.elapsed().as_nanos();
            record.algorithm_counters = serde_json::to_value(&batch.stats).unwrap();
            let start = Instant::now();
            let out = materialize_complete_relations(&index, &batch, rows, limits.max_small_weight, limits.max_sources);
            phases.materialization_ns = start.elapsed().as_nanos();
            out
        },
        Arm::Pivot => {
            let (out, report) = filter_large_primes(rows.to_vec(), &options).map_err(|e| format!("pivot: {e:?}"))?;
            phases.extraction_ns = start.elapsed().as_nanos();
            if report.rejected_fill > 0 { record.status = "partial-output".into(); record.refusal = Some("original pivot filter rejected fill; retained rows are valid but full projection is not asserted".into()); }
            record.algorithm_counters = serde_json::to_value(report).unwrap();
            Some(out)
        },
        Arm::Mst => {
            let ml = MstLimits { max_rows: crate::MAX_ROWS, max_column_degree: limits.mst_max_column_degree,
                max_edge_evaluations: limits.mst_max_edge_evaluations, max_active_coefficients: limits.max_active_coefficients };
            let out = filter_large_primes_mst(rows.to_vec(), &options, &ml).map_err(|e| format!("MST: {e:?}"))?;
            phases.extraction_ns = start.elapsed().as_nanos();
            if out.report.rejected_fill > 0 { record.status = "partial-output".into(); record.refusal = Some("original MST filter rejected fill".into()); }
            record.algorithm_counters = serde_json::json!({"report":out.report,"stats":out.stats});
            out.relations
        },
        Arm::MatrixFree => unreachable!(),
    };
    record.filter_hook_counts = jv_bitops::snapshot();
    let Some(output) = output else {
        record.status = "refused".into(); record.refusal = Some("original filter/materialization refused atomically".into()); record.phases=phases; return Ok(record);
    };
    let start = Instant::now();
    let audit = audit_projected_rank(&output, ELL).map_err(|e| format!("projected rank: {e:?}"))?;
    phases.exact_rank_ns = start.elapsed().as_nanos();
    if !audit.inconsistent_row_indices.is_empty() { return Err("inconsistent projected RHS despite admitted group equations".into()); }
    let start = Instant::now();
    for r in &output {
        let replay = replay_complete_relation(r, rows, ELL).ok_or("source replay failed")?;
        if replay.small != r.small || replay.rhs != r.rhs { return Err("source replay output mismatch".into()); }
    }
    phases.source_replay_ns = start.elapsed().as_nanos();
    record.output_rows = Some(output.len()); record.useful_small_rank=Some(audit.useful_small_rank);
    record.augmented_rank=Some(audit.augmented_rank); record.replayed=Some(output.len());
    record.residual_columns=Some(output.iter().flat_map(|r| r.small.iter().map(|t|t.0)).collect::<BTreeSet<_>>().len());
    record.residual_nonzeros=Some(output.iter().map(|r|r.small.len()).sum());
    record.source_terms=Some(output.iter().map(|r|r.sources.len()).sum());
    record.output_sha256=Some(sha256(&serde_json::to_vec(&output).unwrap()));
    let start=Instant::now();
    if let Some(scalar)=recover_target(&output, limits.max_active_coefficients)? {
        let mut ar=Arithmetic::default();
        if ar.scalar(PRIME31,scalar,Some(PRIME31.g)) != Some(admitted.manifest.target) { return Err("recovered scalar fails original group check".into()); }
        record.target_scalar=Some(scalar); record.target_group_verified=true;
    }
    phases.target_recovery_and_group_check_ns=start.elapsed().as_nanos();
    record.phases=phases;
    Ok(record)
}

/// The protected target is the largest small-column ID. It is identifiable iff
/// canonical ascending echelon has a pivot at that ID, yielding an isolated row.
fn recover_target(rows: &[CompleteRelation], cap: usize) -> Result<Option<u64>,String> {
    let mut pivots:BTreeMap<u32,(BTreeMap<u32,u64>,u64)>=BTreeMap::new();
    let mut stored=0usize;
    for row in rows {
        let mut work:BTreeMap<u32,u64>=row.small.iter().copied().collect();
        let mut rhs=row.rhs;
        while let Some((&column,&coefficient))=work.iter().next() {
            if stored+work.len()>cap { return Err("target audit fill cap".into()); }
            if let Some((pivot,b))=pivots.get(&column) {
                for (&c,&a) in pivot { let v=sub(work.get(&c).copied().unwrap_or(0),mul(coefficient,a)); if v==0{work.remove(&c);}else{work.insert(c,v);} }
                rhs=sub(rhs,mul(coefficient,*b));
            } else {
                let inverse=inv(coefficient);
                for v in work.values_mut(){*v=mul(*v,inverse);}
                stored+=work.len(); pivots.insert(column,(work,mul(rhs,inverse))); break;
            }
        }
    }
    Ok(pivots.get(&TARGET).map(|(_,rhs)|*rhs))
}
