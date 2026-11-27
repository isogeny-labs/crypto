use m31_graph_compare::{admission::{admit, Manifest}, compare::{run, Arm, Limits}, large_prime_filter::PartialRelation,
    sha256, tape::{ProjectionTape, TapeLimits}, COORDINATE_PRIME, ELL, TARGET};
use prime31_algebraic_ic::{Arithmetic, PRIME31};
use std::{collections::{BTreeMap, BTreeSet}, fs, process::Command, sync::atomic::{AtomicU64, Ordering}};

fn mm(a:u64,b:u64)->u64{(a as u128*b as u128%ELL as u128)as u64}
fn minus(a:u64,b:u64)->u64{(a+ELL-b)%ELL}
fn inverse(a:u64)->u64{
    let(mut r0,mut r1,mut s0,mut s1)=(ELL as i128,a as i128,0i128,1i128);
    while r1!=0{let q=r0/r1;(r0,r1)=(r1,r0-q*r1);(s0,s1)=(s1,s0-q*s1);}assert_eq!(r0,1);s0.rem_euclid(ELL as i128)as u64
}
fn rref(mut a:Vec<Vec<u64>>,columns:usize)->Vec<Vec<u64>>{
    let mut pivot=0;
    for c in 0..columns{
        let Some(i)=(pivot..a.len()).find(|i|a[*i][c]!=0)else{continue};a.swap(i,pivot);
        let s=inverse(a[pivot][c]);for v in &mut a[pivot]{*v=mm(*v,s);}
        for i in 0..a.len(){if i==pivot{continue}let k=a[i][c];for j in c..columns{a[i][j]=minus(a[i][j],mm(k,a[pivot][j]));}}
        pivot+=1;if pivot==a.len(){break;}
    }
    a.retain(|r|r.iter().any(|v|*v!=0));a
}
fn raw_projected(rows:&[PartialRelation],small:&[u32])->(usize,Vec<Vec<u64>>){
    let large:Vec<u64>=rows.iter().flat_map(|r|r.large.iter().map(|x|x.0)).collect::<BTreeSet<_>>().into_iter().collect();
    let cols=large.len()+small.len()+1;
    let mut a:Vec<Vec<u64>>=rows.iter().map(|r|{
        let mut out=vec![0;cols];for&(c,v)in &r.large{out[large.binary_search(&c).unwrap()]=v;}
        for&(c,v)in &r.small{out[large.len()+small.binary_search(&c).unwrap()]=v;}out[cols-1]=r.rhs;out
    }).collect();
    let mut rank=0;
    // Independent dense block elimination, operating directly on [L|S|b].
    for c in 0..large.len(){
        let Some(i)=(rank..a.len()).find(|i|a[*i][c]!=0)else{continue};a.swap(i,rank);
        let s=inverse(a[rank][c]);for v in &mut a[rank]{*v=mm(*v,s);}
        for i in 0..a.len(){if i==rank{continue}let k=a[i][c];for j in c..cols{a[i][j]=minus(a[i][j],mm(k,a[rank][j]));}}
        rank+=1;
    }
    (rank,a[rank..].iter().map(|r|r[large.len()..].to_vec()).collect())
}
fn rows_json(rows:&[PartialRelation])->Vec<u8>{rows.iter().map(|r|format!("{}\n",serde_json::to_string(r).unwrap())).collect::<String>().into_bytes()}
fn fixture()->(Manifest,Vec<PartialRelation>){
    let small=BTreeMap::from([(0u32,13u64),(1,29),(TARGET,37)]);let large=BTreeMap::from([(100u64,41u64),(200,43),(300,47)]);
    let inputs=vec![
        (vec![(0,1)],vec![(100,2),(200,3)]),
        (vec![(1,1)],vec![(200,5),(300,7)]),
        (vec![(0,4)],vec![(100,11),(300,19)]),
        (vec![(0,2),(TARGET,1)],vec![]),
        (vec![(0,3)],vec![(100,1)]),
        (vec![(1,1)],vec![(100,2),(200,3)]),
        (vec![(TARGET,2)],vec![]),
        (vec![],vec![]),
    ];
    let rows:Vec<PartialRelation>=inputs.into_iter().enumerate().map(|(i,(s,l))|{
        let rhs=s.iter().map(|(id,a)|mm(*a,small[id])).chain(l.iter().map(|(id,a)|mm(*a,large[id]))).fold(0,|s,v|(s+v)%ELL);
        PartialRelation{id:1000+i as u64,small:s,large:l,rhs}
    }).collect();
    let mut ar=Arithmetic::default();let mut point=|k|ar.scalar(PRIME31,k,Some(PRIME31.g)).unwrap();
    let manifest=Manifest{schema:"m31-graph-input/v1".into(),coordinate_prime:COORDINATE_PRIME,relation_prime:ELL,
        curve_a:PRIME31.a,curve_b:PRIME31.b,generator:PRIME31.g,target:point(37),target_column:TARGET,
        input_sha256:sha256(&rows_json(&rows)),small_points:small.iter().map(|(&c,&k)|(c,point(k))).collect(),
        large_points:large.iter().map(|(&c,&k)|(c,point(k))).collect(),provenance_label:"planted development control, no natural capture".into()};
    (manifest,rows)
}

#[test]
fn original_four_arms_match_useful_rank_and_recover_public_target(){
    let(manifest,rows)=fixture();let admitted=admit(manifest,&rows_json(&rows)).unwrap();
    let mut ranks=Vec::new();let mut witnesses=Vec::new();
    for arm in [Arm::Eager,Arm::Lazy,Arm::Pivot,Arm::Mst]{
        let record=run(&admitted,rows.len(),arm,&Limits::default()).unwrap();
        assert_eq!(record.status,"accepted");assert_eq!(record.target_scalar,Some(37));assert!(record.target_group_verified);
        assert_eq!(record.replayed,record.output_rows);ranks.push((record.useful_small_rank,record.augmented_rank));witnesses.push(record.output_sha256);
        assert_eq!(record.complete_cold_solving_cost,None);
        let incomplete=run(&admitted,2,arm,&Limits::default()).unwrap();assert_eq!(incomplete.target_scalar,None);assert!(!incomplete.target_group_verified);
    }
    assert!(ranks.iter().all(|r|*r==ranks[0]));assert_eq!(witnesses[0],witnesses[1]);
    let op=run(&admitted,rows.len(),Arm::MatrixFree,&Limits::default()).unwrap();assert_eq!(op.status,"operator-only");assert_eq!(op.useful_small_rank,None);assert!(!op.target_group_verified);
}

fn check_operator(rows:&[PartialRelation]){
    let tape=ProjectionTape::build(rows,&TapeLimits::default()).unwrap();let width=tape.columns().len();
    let mut dense=vec![vec![0;width+1];tape.stats().output_rows];
    for c in 0..width{let mut e=vec![0;width];e[c]=1;let col=tape.apply(&e).unwrap();for(i,v)in col.into_iter().enumerate(){dense[i][c]=v;}}
    for(i,v)in tape.projected_rhs().into_iter().enumerate(){dense[i][width]=v;}
    let(rank,expected)=raw_projected(rows,tape.columns());assert_eq!(rank,tape.stats().lp_rank);assert_eq!(rows.len()-rank,tape.stats().output_rows);
    assert_eq!(rref(dense.clone(),width+1),rref(expected,width+1));
    let x:Vec<u64>=(0..width).map(|i|5*i as u64+9).collect();let y:Vec<u64>=(0..dense.len()).map(|i|7*i as u64+11).collect();
    let hx=tape.apply(&x).unwrap();let ht=tape.apply_transpose(&y).unwrap();
    for(i,row)in dense.iter().enumerate(){assert_eq!(hx[i],row[..width].iter().zip(&x).fold(0,|s,(&a,&b)|(s+mm(a,b))%ELL));}
    for c in 0..width{assert_eq!(ht[c],dense.iter().zip(&y).fold(0,|s,(r,&v)|(s+mm(r[c],v))%ELL));}
    assert_eq!(hx.iter().zip(&y).fold(0,|s,(&a,&b)|(s+mm(a,b))%ELL),x.iter().zip(&ht).fold(0,|s,(&a,&b)|(s+mm(a,b))%ELL));
}
#[test]
fn weighted_rank_deficient_anchored_and_zero_lp_projection(){
    let(_,rows)=fixture();check_operator(&rows);
    let deficient=vec![
        PartialRelation{id:1,small:vec![(7,3)],large:vec![(100,2),(200,3)],rhs:17},
        PartialRelation{id:2,small:vec![(TARGET,5)],large:vec![(100,4),(200,6)],rhs:19},
        PartialRelation{id:3,small:vec![(7,1),(TARGET,2)],large:vec![],rhs:23},
    ];check_operator(&deficient);
    let zero=vec![PartialRelation{id:9,small:vec![(TARGET,3)],large:vec![],rhs:11}];check_operator(&zero);
}
#[test]
fn seeded_dense_block_oracle_checks_128_m31_relation_systems(){
    let mut state=0x3d83_7321_a881_019bu64;
    let mut next=||{state^=state<<13;state^=state>>7;state^=state<<17;state};
    for trial in 0..128{
        let n=2+(next()%20)as usize;let mut rows=Vec::new();
        for i in 0..n{
            let mut large=BTreeMap::new();for _ in 0..next()%3{large.insert(100+next()%7,1+next()%(ELL-1));}
            let mut small=BTreeMap::new();for _ in 0..next()%5{let c=if next()%4==0{TARGET}else{(next()%5)as u32};small.insert(c,1+next()%(ELL-1));}
            rows.push(PartialRelation{id:trial*100+i as u64,small:small.into_iter().collect(),large:large.into_iter().collect(),rhs:next()%ELL});
        }check_operator(&rows);
    }
}
#[test]
fn refuses_malformed_inputs_cap_and_vector_shape_without_partial_tape(){
    let(_,mut rows)=fixture();let limits=TapeLimits{max_tape_terms:0,..Default::default()};assert!(ProjectionTape::build(&rows,&limits).is_err());
    let limits=TapeLimits{max_pivot_nonzeros:0,..Default::default()};assert!(ProjectionTape::build(&rows,&limits).is_err());
    let limits=TapeLimits{max_build_multiply_add_terms:0,..Default::default()};assert!(ProjectionTape::build(&rows,&limits).is_err());
    let tape=ProjectionTape::build(&rows,&TapeLimits::default()).unwrap();assert!(tape.apply(&[]).is_err());assert!(tape.apply_transpose(&[]).is_err());
    let mut bad=vec![0;tape.columns().len()];bad[0]=ELL;assert!(tape.apply(&bad).is_err());
    rows[1].id=rows[0].id;assert!(ProjectionTape::build(&rows,&TapeLimits::default()).is_err());
}
#[test]
fn group_admission_rejects_poison_identity_mapping_and_hash(){
    let(m,mut rows)=fixture();let bytes=rows_json(&rows);admit(m.clone(),&bytes).unwrap();
    let mut bad=m.clone();bad.coordinate_prime=ELL;assert!(admit(bad,&bytes).is_err());
    let mut bad=m.clone();bad.curve_a=COORDINATE_PRIME-3;assert!(admit(bad,&bytes).is_err());
    let mut bad=m.clone();bad.input_sha256="00".repeat(32);assert!(admit(bad,&bytes).is_err());
    let mut bad=m.clone();bad.target_column=0;assert!(admit(bad,&bytes).is_err());
    let mut bad=m.clone();bad.small_points.push(bad.small_points[0]);assert!(admit(bad,&bytes).is_err());
    rows[4].rhs=(rows[4].rhs+1)%ELL;let poison=rows_json(&rows);let mut bad=m.clone();bad.input_sha256=sha256(&poison);assert!(admit(bad,&poison).is_err());
}
#[test]
fn capped_filters_preserve_refusal_or_partial_status(){
    let(m,rows)=fixture();let admitted=admit(m,&rows_json(&rows)).unwrap();
    let tiny=Limits{max_small_weight:0,max_sources:0,mst_max_edge_evaluations:0,..Default::default()};
    for arm in [Arm::Eager,Arm::Lazy,Arm::Mst]{let r=run(&admitted,rows.len(),arm,&tiny).unwrap();assert_ne!(r.status,"accepted");}
    let r=run(&admitted,rows.len(),Arm::Pivot,&tiny).unwrap();assert!(r.status=="partial-output"||r.status=="refused");
}
static SERIAL:AtomicU64=AtomicU64::new(0);
#[test]
fn cli_coordinator_uses_four_fresh_processes_and_rejects_corruption_atomically(){
    let(m,rows)=fixture();let dir=std::env::temp_dir().join(format!("m31-graph-test-{}-{}",std::process::id(),SERIAL.fetch_add(1,Ordering::Relaxed)));fs::create_dir(&dir).unwrap();
    let input=dir.join("rows.jsonl");let manifest=dir.join("manifest.json");fs::write(&input,rows_json(&rows)).unwrap();fs::write(&manifest,serde_json::to_vec(&m).unwrap()).unwrap();
    let output=Command::new(env!("CARGO_BIN_EXE_m31-graph-compare")).arg("compare").arg("--input").arg(&input).arg("--manifest").arg(&manifest)
        .args(["--prefixes","8","--rounds","1","--out-dir"]).arg(dir.join("comparison")).output().unwrap();
    assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let summary:serde_json::Value=serde_json::from_slice(&fs::read(dir.join("comparison/summary.json")).unwrap()).unwrap();
    assert_eq!(summary["attempts"].as_array().unwrap().len(),4);assert_eq!(summary["all_arms_accepted"],true);
    assert_eq!(summary["comparison_equivalent"],true);
    assert_eq!(summary["group_admission_in_coordinator"],false);
    assert!(summary["coordinator_preflight_ns"].as_u64().unwrap()>0);
    assert!(summary["total_process_wall_ns"].as_u64().unwrap()>summary["coordinator_preflight_ns"].as_u64().unwrap());
    let pids:BTreeSet<u64>=summary["records"].as_array().unwrap().iter().map(|r|r["pid"].as_u64().unwrap()).collect();assert_eq!(pids.len(),4);
    if let Ok(destination)=std::env::var("M31_GRAPH_TEST_ARTIFACTS") {
        let destination=std::path::PathBuf::from(destination);fs::create_dir_all(&destination).unwrap();
        for path in [input.clone(),manifest.clone()] { fs::copy(&path,destination.join(path.file_name().unwrap())).unwrap(); }
        for item in fs::read_dir(dir.join("comparison")).unwrap() { let path=item.unwrap().path();fs::copy(&path,destination.join(path.file_name().unwrap())).unwrap(); }
    }
    let mut bad=m;bad.relation_prime=COORDINATE_PRIME;fs::write(&manifest,serde_json::to_vec(&bad).unwrap()).unwrap();
    let output=Command::new(env!("CARGO_BIN_EXE_m31-graph-compare")).arg("run").arg("--input").arg(&input).arg("--manifest").arg(&manifest).args(["--arm","lazy"]).output().unwrap();
    assert!(!output.status.success());assert!(output.stdout.is_empty());
    // Full identity rejection occurs inside capped children; no parent group
    // arithmetic is required to retain four explicit refusal attempts.
    let output=Command::new(env!("CARGO_BIN_EXE_m31-graph-compare")).arg("compare").arg("--input").arg(&input).arg("--manifest").arg(&manifest)
        .args(["--prefixes","8","--rounds","1","--out-dir"]).arg(dir.join("invalid-comparison")).output().unwrap();
    assert!(!output.status.success());
    let invalid:serde_json::Value=serde_json::from_slice(&fs::read(dir.join("invalid-comparison/summary.json")).unwrap()).unwrap();
    assert_eq!(invalid["attempts"].as_array().unwrap().len(),4);assert_eq!(invalid["records"].as_array().unwrap().len(),0);
    assert_eq!(invalid["comparison_equivalent"],false);assert_eq!(invalid["group_admission_in_coordinator"],false);
    assert!(invalid["attempts"].as_array().unwrap().iter().all(|a|a["exit_code"]==1));
    if let Ok(destination)=std::env::var("M31_GRAPH_TEST_ARTIFACTS") {
        let destination=std::path::PathBuf::from(destination).join("invalid-identity");fs::create_dir_all(&destination).unwrap();
        fs::copy(&manifest,destination.join("manifest.json")).unwrap();
        for item in fs::read_dir(dir.join("invalid-comparison")).unwrap(){let path=item.unwrap().path();fs::copy(&path,destination.join(path.file_name().unwrap())).unwrap();}
    }
    fs::remove_dir_all(dir).unwrap();
}
