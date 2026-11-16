//! Independent ground-truth control for anchored prime31 six-summand support.
//!
//! IMPORTANT: this explicitly enumerates generic three-sum support. It is
//! NOT a non-generic IC oracle, an F4 input, or an ECDLP solver. The result
//! only checks whether an algebraic F4 call marked "undetermined" actually
//! had some full-point signed witnesses on the 16 frozen natural public R.
//!
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_anchor2_m6_support_validator
use prime31_algebraic_ic::{Arithmetic, Base, Point, PRIME31, check_curve};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;

const Q:Point=Some((1_547_953_785,1_667_575_595));
const TRIPLES:u64=62*62*62;
#[derive(Deserialize)]
struct Panel { rows:Vec<Row> }
#[derive(Deserialize)]
struct Row { index:usize, a:u64, b:u64, target:[u64;2] }
fn count_six(target:Point,signed:&[Point],table:&HashMap<Point,u64>,
    ar:&mut Arithmetic)->u64 {
    let mut total=0u64;
    for &a in signed {
        for &b in signed {
            let first=ar.add(PRIME31,a,b);
            for &c in signed {
                let triple=ar.add(PRIME31,first,c);
                let complement=ar.add(PRIME31,target,ar.neg(PRIME31,triple));
                total+=table.get(&complement).copied().unwrap_or(0);
            }
        }
    }
    total
}
fn run()->Result<serde_json::Value,String> {
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Q){return Err("public Q is not on prime31 curve".into());}
    let base=Base::build(PRIME31,6,&mut ar)?;
    if base.points.len()!=31{return Err("canonical prime31 B31 changed".into());}
    let a=Some(base.points[0]);
    let b=Some(base.points[1]);
    let anchor=ar.add(PRIME31,a,b).ok_or("fixed anchors cancel")?;
    let signed:Vec<Point>=base.points.iter().copied().flat_map(|pt|
        [Some(pt),ar.neg(PRIME31,Some(pt))]).collect();
    if signed.len()!=62{return Err("expected 62 signed base points".into());}
    let build_start=Instant::now();
    let mut table:HashMap<Point,u64>=HashMap::with_capacity(TRIPLES as usize);
    for &x in &signed {
        for &y in &signed {
            let first=ar.add(PRIME31,x,y);
            for &z in &signed {
                let triple=ar.add(PRIME31,first,z);
                *table.entry(triple).or_insert(0)+=1;
            }
        }
    }
    if table.values().copied().sum::<u64>()!=TRIPLES{
        return Err("lost ordered signed triples".into());
    }
    let build_ns=build_start.elapsed().as_nanos();
    let ps=&base.points[2..8];
    let mut six=None;
    for &pt in ps {six=ar.add(PRIME31,six,Some(pt));}
    let six=six.ok_or("positive six-summand witness cancels")?;
    let known_count=count_six(Some(six),&signed,&table,&mut ar);
    if known_count==0{return Err("known positive six-point witness missed".into());}
    let panel:Panel=serde_json::from_str(
        include_str!("../M8_NATURAL_F4_PANEL_20261115.json"))
        .map_err(|err|err.to_string())?;
    if panel.rows.len()!=16{return Err("frozen natural panel changed".into());}
    let mut rows=Vec::with_capacity(panel.rows.len());
    let mut nonzero=0usize;
    let mut count_total=0u64;
    for (i,row) in panel.rows.iter().enumerate(){
        if row.index!=i || row.a>=PRIME31.n || row.b==0 || row.b>=PRIME31.n {
            return Err(format!("bad public natural mask at {i}"));
        }
        let ag=ar.scalar(PRIME31,row.a,Some(PRIME31.g));
        let bq=ar.scalar(PRIME31,row.b,Q);
        let point=ar.add(PRIME31,ag,bq)
            .ok_or("public natural mask is identity")?;
        if [point.0,point.1]!=row.target{
            return Err(format!("natural mask public target mismatch at {i}"));
        }
        let residual=ar.add(PRIME31,Some(point),ar.neg(PRIME31,Some(anchor)));
        let begin=Instant::now();
        let witnesses=count_six(residual,&signed,&table,&mut ar);
        let query_ns=begin.elapsed().as_nanos();
        if witnesses>0{nonzero+=1;}
        count_total+=witnesses;
        rows.push(json!({"index":i,"original_public_target":point,
            "anchored_residual":residual,
            "ordered_signed_six_witnesses":witnesses,
            "complete_three_sum_probe_ns":query_ns}));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "classification":"GENERIC exhaustive group-support validator only; not index calculus discovery or scalar recovery",
        "curve_prime":PRIME31.p,"curve_order":PRIME31.n,"cofactor":1,
        "anchor_points":[a,b],
        "base_size":31,"signed_base_size":signed.len(),
        "ordered_signed_triple_sequences":TRIPLES,
        "unique_triple_group_points":table.len(),
        "table_group_construction_ns":build_ns,
        "positive_known_six_point_control_count":known_count,
        "natural_public_target_count":rows.len(),
        "natural_targets_with_at_least_one_six_witness":nonzero,
        "natural_signed_six_witnesses_total":count_total,
        "uniform_average_combinatorial_witnesses_per_target":(62u64.pow(6) as f64)/(PRIME31.n as f64),
        "total_charged_wall_ns":start.elapsed().as_nanos(),
        "rows":rows,
        "warning":"These group-hash witnesses and counts MUST NEVER feed algebraic F4 or count as a non-generic IC relation"
    }))
}
fn main()->Result<(),String>{
    println!("{}",serde_json::to_string_pretty(&run()?).map_err(|e|e.to_string())?);
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn triple_domain_cardinality(){
        assert_eq!(super::TRIPLES,238_328);
        assert_eq!(62u64.pow(6),56_800_235_584);
    }
}
