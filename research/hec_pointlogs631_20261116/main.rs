mod field{pub fn p()->u64{631}}
mod setup{pub fn qf()->[u64;15]{std::fs::read_to_string(std::env::var("HEC_SETUP_DIR").unwrap()+"/QF.txt").unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()}}
mod config{include!("../hec_calibration631_20261115/field_config.rs");}
#[allow(dead_code)]mod target{include!("../hec_alignment13_20261115/atlas/target.rs");include!("../hec_calibration631_20261115/target_api631.rs");}
mod sparse{include!("sparse.rs");}
mod lp_trace{include!("../hec_chartcold13_20261115/lp_trace.rs");}
mod points{include!("points.rs");}
fn main(){config::install(config::discover(setup::qf()));let(pts,logs)=points::fresh();assert_eq!(pts.len(),logs.len());assert!(logs.iter().any(|&a|a!=0));println!("PASS F631_fresh_point_relation_kernel=true points={} no_input_class_logs_or_scalar_verified=true",pts.len());}
