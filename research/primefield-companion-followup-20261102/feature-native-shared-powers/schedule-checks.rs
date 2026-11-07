#[path="division.rs"]mod compiled;
#[path="../feature-native-normalized-division/division.rs"]mod normalized;
#[path="../feature-native-compiled-division/division.rs"]mod unshared;
fn schedule_checks(){
 use compiled::{Schedule,Mode,CostModel,Step};let(mut equalities,mut plans,mut caps,mut selections)=(0,0,0,0);
 let lists=vec![vec![],vec![0,1,2,3,4,2,0],(0..128).chain([u64::MAX-1,u64::MAX]).collect(),vec![479,480,2,481,240,22,3,480]];
 for requests in &lists{let schedule=Schedule::new(requests,4096).unwrap();let st=schedule.stats();let (sq,cu)=schedule.power_counts();assert!(cu<=sq);assert_eq!(schedule.saved_multiplications(),2*st.even as u128+4*st.odd as u128-sq as u128-cu as u128);
  let mut expected=vec![0u8;st.nodes];for step in &schedule.steps{match *step{Step::Odd{b,d,..}=>{expected[b]|=3;expected[d]|=3},Step::Even{c,e,..}=>{expected[c]|=1;expected[e]|=1}}}assert_eq!(schedule.power_flags,expected);
  let reference=unshared::Schedule::new(requests,4096).unwrap();assert_eq!(Schedule::new(requests,st.nodes-1).unwrap_err(),compiled::PlanError::Cap);assert_eq!(Schedule::new(requests,st.nodes).unwrap().stats(),st);caps+=1;
  for(i,step)in schedule.steps.iter().enumerate(){let ids=match *step{Step::Odd{a,b,c,d,..}=>vec![a,b,c,d],Step::Even{a,b,c,d,e}=>vec![a,b,c,d,e]};assert!(ids.into_iter().all(|j|j<i+5))}plans+=1;
  for m in [0u64,1,3,100,u64::MAX]{for inv in [0u64,1,5,100,u64::MAX]{let full=(6*(st.even as u128+st.odd as u128))*m as u128+inv as u128;let norm=(5*st.even as u128+7*st.odd as u128+st.even_reads as u128+1)*m as u128;assert_eq!(st.choose(CostModel{multiplication:m,inversion:inv}),if norm<full{Mode::Normalized}else{Mode::Full});selections+=1}}
  for p in [5u64,7,17,41,97,193,257,509]{let(a,b,_,_)=if p==5{(2,1,7,[0,1])}else if p==7{(0,3,13,[1,2])}else if p==509{(2,10,479,[3,68])}else{fixture(p)};
   for x in 0..p{for y in 1..p{if mul(y,y,p)!=(mul(mul(x,x,p),x,p)+mul(a,x,p)+b)%p{continue}
    let got=schedule.evaluate(p,a,b,x,y,Mode::Normalized).unwrap();let full=schedule.evaluate(p,a,b,x,y,Mode::Full).unwrap();let mut old=division::ValueDivision::new(p,a,b,x,y).unwrap();let mut prior=normalized::ValueDivision::new(p,a,b,x,y).unwrap();assert_eq!(got.len(),requests.len());assert_eq!(got,reference.evaluate(p,a,b,x,y,unshared::Mode::Normalized).unwrap());assert_eq!(full,reference.evaluate(p,a,b,x,y,unshared::Mode::Full).unwrap());
    for(i,&n)in requests.iter().enumerate(){assert_eq!(got[i],old.value(n));assert_eq!(got[i],prior.value(n));assert_eq!(got[i],full[i]);equalities+=1}
   }}
  }
 }
 let mut bound_checks=0;let mut strict=0;
 for n in 0..4096{let s=Schedule::new(&[n],4096).unwrap();let st=s.stats();let(sq,cu)=s.power_counts();assert!(sq+cu<=2*st.even+4*st.odd);if s.saved_multiplications()>0{strict+=1}bound_checks+=1;}
 assert!(strict>0);println!("shared_power_bound_checks={bound_checks} strict_saving_exists=true");
 // Cost comparison must not overflow for large declared counts/cost weights.
 assert_eq!(compiled::Stats{even:0,odd:u64::MAX,even_reads:u64::MAX,nodes:0}.choose(CostModel{multiplication:u64::MAX,inversion:u64::MAX}),Mode::Full);
 assert_eq!(compiled::Stats{even:u64::MAX,odd:0,even_reads:0,nodes:0}.choose(CostModel{multiplication:u64::MAX,inversion:0}),Mode::Normalized);
 let s=Schedule::new(&[2],5).unwrap();assert!(s.evaluate(5,1,0,0,0,Mode::Normalized).is_err());assert!(s.evaluate(509,2,10,3,69,Mode::Full).is_err());assert!(s.evaluate(3,0,0,0,0,Mode::Normalized).is_err());
 println!("compiled_value_equalities={equalities} schedule_topologies={plans} exact_node_caps={caps} cost_selections={selections} both_modes_and_u64_boundaries=true");
}
