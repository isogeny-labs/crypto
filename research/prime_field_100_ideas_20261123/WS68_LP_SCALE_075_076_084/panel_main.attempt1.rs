// Twelve frozen public EC cases: four target points x three independent coefficient streams.
include!("run.rs");
fn main(){
 let targets=[Pt::A(3280,1514),Pt::A(1419,683),Pt::A(3957,905),Pt::A(1671,799)];
 let case:usize=std::env::args().nth(1).expect("case index 0..11").parse().unwrap();assert!(case<12);
 let target_index=case/3;let replicate=case%3;let seed=SEEDS[replicate];let g=Pt::A(1,921);let q=targets[target_index];
 assert!(valid(g)&&valid(q));let mut order_cost=Cost::default();assert_eq!(cm(R,g,&mut order_cost),Pt::O);
 let base_start=Instant::now();let(base,y_tests,x_scans)=base();let base_wall=base_start.elapsed().as_micros();assert_eq!(base.len(),24);assert_eq!(base[0],g);
 let fstart=Instant::now();let mut fcost=Cost::default();let mut full=HashMap::new();for i in 0..SMALL{for j in i..SMALL{let p=ca(base[i],base[j],&mut fcost);full.entry(p).or_insert(Decomp{kind:Kind::Full,i,j});}}let full_setup_wall=fstart.elapsed().as_micros();
 let astart=Instant::now();let mut acost=Cost::default();let (_check,all,pair_counts)=tables(&base,&mut acost);let all_setup_wall=astart.elapsed().as_micros();assert_eq!(full.len(),_check.len());assert_eq!(fcost.group_adds,36);assert_eq!(acost.group_adds,300);
 let sstart=Instant::now();let (stream,candidate_cost)=stream(g,q,seed);let candidate_wall=sstart.elapsed().as_micros();
 let mut transcript=String::new();for(trial,&(a,b,r)) in stream.iter().enumerate(){transcript.push_str(&format!("trial={trial} a={a} b={b} r={r:?}\n"))}
 fs::write(format!("WS68_LP_SCALE_075_076_084/candidate{case}.stdout"),transcript).unwrap();
 println!("input case={case} target_index={target_index} replicate={replicate} curve_p={P} order={R} seed={seed} cap={CAP} generator={g:?} target_point={q:?} order_check_group_adds={} base_y_tests={y_tests} base_x_scans={x_scans} base_wall_us={base_wall} full_setup_group_adds={} full_setup_wall_us={full_setup_wall} all_setup_group_adds={} all_setup_wall_us={all_setup_wall} candidate_group_adds={} candidate_wall_us={candidate_wall} full_keys={} all_keys={} pair_work={pair_counts:?}",order_cost.group_adds,fcost.group_adds,acost.group_adds,candidate_cost.group_adds,full.len(),all.len());
 let cfstart=Instant::now();let mut flookup=Cost::default();let mut fverify=Cost::default();let full_src=collect(&stream,&full,&base,q,g,&mut flookup,&mut fverify);let full_collect_wall=cfstart.elapsed().as_micros();
 let castart=Instant::now();let mut alookup=Cost::default();let mut averify=Cost::default();let all_src=collect(&stream,&all,&base,q,g,&mut alookup,&mut averify);let all_collect_wall=castart.elapsed().as_micros();
 let mut kinds=[0;3];for s in &all_src{match s.kind{Kind::Full=>kinds[0]+=1,Kind::One=>kinds[1]+=1,Kind::Two=>kinds[2]+=1}}assert_eq!(full_src.len(),kinds[0]);
 for(i,s)in full_src.iter().enumerate(){assert_eq!(s.row,all_src.iter().filter(|r|matches!(r.kind,Kind::Full)).nth(i).unwrap().row)}
 println!("collection case={case} full_source_rows={} all_source_rows={} kind_counts={kinds:?} full_lookups={} all_lookups={} full_collect_wall_us={full_collect_wall} all_collect_wall_us={all_collect_wall} same_stream=true",full_src.len(),all_src.len(),flookup.lookups,alookup.lookups);
 let mut fcharged=flookup;fcharged.group_adds=candidate_cost.group_adds;let mut acharged=alookup;acharged.group_adds=candidate_cost.group_adds;
 analyse("full_only",&full_src,&base,q,g,fcost,fcharged,fverify);
 analyse("partial_filter",&all_src,&base,q,g,acost,acharged,averify);
 rho(g,q);
 println!("RUN_OK=1 case={case}");
}
