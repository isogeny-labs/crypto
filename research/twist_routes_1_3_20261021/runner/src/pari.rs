use crate::{covers::Selected, toy_field::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::{Command, Stdio},
};
fn execute(script: &str, stem: &Path) -> Result<(Option<Value>, Option<i32>)> {
    fs::write(stem.with_extension("gp"), script)?;
    let output = stem.with_extension("stdout");
    let status = Command::new("/usr/bin/timeout")
        .args([
            "--signal=TERM",
            "--kill-after=0.2s",
            "15s",
            "/tmp/twist-cas/usr/bin/gp",
            "-f",
            "-q",
        ])
        .arg(stem.with_extension("gp"))
        .stdout(Stdio::from(fs::File::create(&output)?))
        .stderr(Stdio::from(fs::File::create(
            stem.with_extension("stderr"),
        )?))
        .status()?;
    let text = fs::read_to_string(output)?;
    let value = text
        .lines()
        .find(|s| s.starts_with('{'))
        .and_then(|s| serde_json::from_str(s).ok());
    Ok((value, status.code()))
}
fn rank(rows: &BTreeSet<Vec<u64>>, r: u64) -> usize {
    let mut basis = BTreeMap::<usize, Vec<u64>>::new();
    for row in rows {
        let mut v = row.clone();
        for (&i, b) in &basis {
            let c = v[i];
            for j in i..v.len() {
                v[j] = (v[j] + r - c * b[j] % r) % r;
            }
        }
        if let Some(i) = v.iter().position(|x| *x != 0) {
            let d = inv(v[i], r);
            for c in &mut v {
                *c = *c * d % r;
            }
            basis.insert(i, v);
        }
    }
    basis.len()
}
fn verify_relations(f: &Fixture, v: &mut Value) -> Result<()> {
    let raw: Vec<[u64; 2]> = serde_json::from_value(v["images"].clone())?;
    let relations: Vec<[usize; 3]> = serde_json::from_value(v["relations"].clone())?;
    if raw.is_empty() {
        v["original_group_relation_replay"] = json!("inapplicable or no points");
        return Ok(());
    }
    let e = f.curve();
    let points: Vec<P> = raw.iter().map(|p| Some((Q(p[0], 0), Q(p[1], 0)))).collect();
    assert!(points.iter().all(|p| e.on(*p)));
    let canonical: Vec<_> = points
        .iter()
        .map(|p| (*p).min(e.pn(*p)))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut rational = BTreeSet::new();
    for t in &relations {
        let mut sum = None;
        let mut row = vec![0; canonical.len()];
        for i in t {
            let p = points[i - 1];
            sum = e.pa(sum, p);
            let c = p.min(e.pn(p));
            let index = canonical.iter().position(|x| *x == c).unwrap();
            row[index] = (row[index] + if p == c { 1 } else { f.order - 1 }) % f.order;
        }
        assert_eq!(sum, None);
        rational.insert(row);
    }
    let mut direct = BTreeSet::new();
    let signed: Vec<_> = canonical.iter().flat_map(|p| [*p, e.pn(*p)]).collect();
    for i in 0..signed.len() {
        for j in i..signed.len() {
            for k in j..signed.len() {
                if e.pa(e.pa(signed[i], signed[j]), signed[k]).is_none() {
                    let mut row = vec![0; canonical.len()];
                    for index in [i, j, k] {
                        let col = index / 2;
                        row[col] =
                            (row[col] + if index % 2 == 0 { 1 } else { f.order - 1 }) % f.order;
                    }
                    direct.insert(row);
                }
            }
        }
    }
    assert!(rational.is_subset(&direct));
    v["original_group_relation_replay"] = json!("verified");
    v["projected_signed_columns"] = json!(canonical.len());
    v["rational_three_term_projected_rows"] = json!(rational.len());
    v["rational_three_term_rank_mod_r"] = json!(rank(&rational, f.order));
    v["same_base_direct_Fp_three_term_rows"] = json!(direct.len());
    v["same_base_direct_Fp_rank_mod_r"] = json!(rank(&direct, f.order));
    v["new_relation_outside_same_base_rational_group_census"] = json!(false);
    Ok(())
}
pub fn run(fixtures: &[Fixture], selected: &[Selected], out: &Path) -> Result<Vec<Value>> {
    fs::create_dir_all(out.join("raw"))?;
    let mut geometry = vec![];
    for s in selected {
        let f = fixtures.iter().find(|f| f.id == s.fixture).unwrap();
        let mut script = format!(
            "p={};A={};B={};c={};identifier={};\n",
            f.p,
            f.a,
            f.b,
            s.c,
            serde_json::to_string(&f.id)?
        );
        script.push_str(r#"jstr(z)=Str(Strchr(34),z,Strchr(34));
H=hyperellcharpoly(Mod(1,p)*((x^2+c)^3+A*(x^2+c)+B));
print("{",jstr("fixture"),":",jstr(identifier),",",jstr("coefficients"),":",Vec(H),",",jstr("jacobian_order"),":",subst(H,variable(H),1),"}");
quit;
"#);
        let (stuff, code) = execute(&script, &out.join("raw").join(format!("{}-cover", f.id)))?;
        if let Some(mut v) = stuff {
            let coefficients: Vec<i64> = serde_json::from_value(v["coefficients"].clone())?;
            let t = f.p as i64 + 1 - f.order as i64;
            assert_eq!(
                coefficients,
                vec![1, 0, 2 * f.p as i64 - t * t, 0, (f.p * f.p) as i64]
            );
            assert_eq!(v["jacobian_order"].as_u64(), Some(f.order * s.order));
            v["exit_code"] = json!(code);
            v["independent_CAS_characteristic_polynomial_matches"] = json!(true);
            geometry.push(v);
        } else {
            geometry.push(
                json!({"fixture":f.id,"exit_code":code,"status":"CAS failure or unparsed output"}),
            );
        }
    }
    write_json(out.join("E12-pari-independent-geometry.json"), &geometry)?;
    let mut lifts = vec![];
    for f in fixtures.iter().filter(|f| f.role == "structural") {
        for d in -64i64..=64 {
            if d == 0 || d.rem_euclid(f.p as i64) == 0 {
                continue;
            }
            let mut script = format!(
                "A={};B={};p={};d={};identifier={};\n",
                f.a,
                f.b,
                f.p,
                d,
                serde_json::to_string(&f.id)?
            );
            script.push_str(r#"default(parisize,64000000);
E=ellinit([A*d^2,B*d^3]);
R=ellrank(E,0);
P=ellratpoints(E,16);
P=Set(concat(P,apply(z->ellneg(E,z),P)));
rels=List();
for(i=1,#P,for(j=i,#P,S=ellneg(E,elladd(E,P[i],P[j]));k=setsearch(P,S);if(k>=j,listput(rels,[i,j,k]))));
images=[];
if(kronecker(d,p)==1,sd=lift(sqrt(Mod(d,p)));images=apply(z->[lift(Mod(z[1]/d,p)),lift(Mod(z[2]*sd/d^2,p))],P));
jstr(z)=Str(Strchr(34),z,Strchr(34));
print("{",jstr("fixture"),":",jstr(identifier),",",jstr("d"),":",d,",",jstr("rank_lower"),":",R[1],",",jstr("rank_upper"),":",R[2],",",jstr("rank_witnesses"),":",jstr(R[4]),",",jstr("standard_model_x_height_cap"),":16,",jstr("rational_points"),":",jstr(P),",",jstr("point_count"),":",#P,",",jstr("images"),":",images,",",jstr("relations"),":",Vec(rels),"}");
quit;
"#);
            let stem = out.join("raw").join(format!("{}-d{d}", f.id));
            let (value, code) = execute(&script, &stem)?;
            if let Some(mut v) = value {
                verify_relations(f, &mut v)?;
                v["exit_code"] = json!(code);
                lifts.push(v);
            } else {
                lifts.push(json!({"fixture":f.id,"d":d,"exit_code":code,"status":"CAS failure or unparsed output"}));
            }
        }
        println!("{}", json!({"stage":"pari-rational-lifts","fixture":f.id}));
    }
    write_json(out.join("E17-rational-height-and-rank.json"), &lifts)?;
    Ok(vec![
        json!({"experiment":"E12","independent_CAS_geometry_rows":geometry.len(),"arbitrary_divisor_norm_CAS_replay":false}),
        json!({"experiment":"E17","rational_height_rank_and_relation_rows":lifts.len(),"whole_pipeline_speedup":null}),
    ])
}
