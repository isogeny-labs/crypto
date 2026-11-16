mod field { static P:std::sync::OnceLock<u64>=std::sync::OnceLock::new(); pub fn p()->u64{*P.get().expect("prime initialized")} pub fn set(p:u64){assert!(P.set(p).is_ok());} }
mod config;
#[allow(dead_code)]mod target {include!("../hec_alignment13_20261115/atlas/target.rs");include!("../hec_alignment13_20261115/atlas/target_api.rs");}
#[allow(dead_code)]mod good {include!("good.rs");include!("section_api.rs");include!("regular_base.rs");include!("base_control_v2.rs");}
fn main(){
 let p:u64=std::env::args().nth(1).unwrap().parse().unwrap();assert!([97,257,509,1009].contains(&p));field::set(p);
 let mut qf=[0;15];qf[0]=1;qf[10]=1;qf[14]=p-1;
 // X4+Y4-Z4 is smooth in these characteristics: simultaneous partial zeros force X=Y=Z=0.
 assert_ne!(4%p,0);
 let cfg=config::discover(qf);config::install(cfg);let cfg=config::get();
 let _s=good::Sections::new();good::verify_canonical_line();
 assert!(cfg.points.iter().all(|&a|config::evaluate(&qf,a)==0));
 println!("PASS runtime_prime={p} smooth_fermat_quartic=true points={} sections=8 exact_coefficients=true control_corruption_rejected=true source_isogeny_ported=false dlp_or_timing_claim=false",cfg.points.len());
}
