mod field{pub fn p()->u64{631}}
mod setup{pub fn qf()->[u64;15]{std::fs::read_to_string(std::env::var("HEC_SETUP_DIR").unwrap()+"/QF.txt").unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()}pub fn verify_model(){assert!(crate::target::is_smooth(&qf(),631));}}
#[allow(dead_code)]mod target{include!("../hec_alignment13_20261115/atlas/target.rs");include!("../hec_calibration631_20261115/target_api631.rs");}
mod config{include!("../hec_calibration631_20261115/field_config.rs");}
mod public{include!("public_input.rs");}
#[allow(dead_code)]mod conic{include!("conic.rs");include!("descent_api.rs");}
fn main(){use std::fs;config::install(config::discover(setup::qf()));let pts:Vec<[u64;3]>=fs::read_to_string("../hec_pointlogs631_20261116/POINTS.txt").unwrap().lines().map(|s|s.split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()).collect();let logs:Vec<u64>=fs::read_to_string("../hec_pointlogs631_20261116/CANDIDATE-POINT-LOGS.txt").unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect();assert_eq!(pts.len(),597);assert_eq!(pts.len(),logs.len());conic::run(&pts,&logs);}
