//! Native exact-field/order/constructor checks for four already-run public fixtures.
use crypto_lib::cryptanalysis::ecbench::fp4::Fp4Instance;
use crypto_lib::cryptanalysis::gaudry_quartic::{E4, Pt4};
use crypto_lib::cryptanalysis::curve_id;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::io::Write;

fn prime(n:u64)->bool { if n<2{return false;} let mut d=2;while d*d<=n{if n%d==0{return false;}d+=1;}true }
fn main()->Result<(),Box<dyn std::error::Error>>{
 let args:Vec<String>=std::env::args().collect(); if args.len()!=3{return Err("PLAN OUT".into());}
 let plan:Value=serde_json::from_slice(&std::fs::read(&args[1])?)?;
 let workloads=plan["workloads"].as_array().ok_or("missing workloads")?;
 let pairs:BTreeSet<(u64,u64)>=workloads.iter().map(|w|(w["curve_spec"]["p"].as_u64().unwrap(),w["curve_spec"]["seed"].as_u64().unwrap())).collect();
 assert_eq!(pairs,BTreeSet::from([(13,1),(17,1),(29,1),(37,1)]));let mut records=Vec::new();
 for(p,seed)in pairs{
  let inst=Fp4Instance::build(p,seed)?;let curve=inst.curve();let f=&curve.f;
  assert!(prime(p)&&prime(inst.r));let t=E4([0,1,0,0]);let q=p.pow(4);
  assert_eq!(f.pow(&t,q as u128),t);
  let delta=f.sub(&f.pow(&t,p.pow(2) as u128),&t);assert!(!delta.is_zero());let inverse=f.inv(&delta);assert_eq!(f.mul(&delta,&inverse),E4::ONE);
  let disc=f.add(&f.scale(&f.mul(&f.sq(&inst.a),&inst.a),4),&f.scale(&f.sq(&inst.b),27));assert!(!disc.is_zero());
  assert!(!inst.g.inf&&curve.on_curve(&inst.g)&&curve.mul(&inst.g,inst.r).inf);
  let lower=q+1-2*p*p;let upper=q+1+2*p*p;let lower_multiple=lower.div_ceil(inst.r);let upper_multiple=upper/inst.r;
  assert_eq!(lower_multiple,upper_multiple);assert_eq!(lower_multiple,inst.cofactor);assert_eq!(inst.group_order,inst.r*inst.cofactor);
  let torsion=Pt4{x:inst.x0,y:E4::ZERO,inf:false};assert!(curve.on_curve(&torsion)&&curve.add(&torsion,&torsion).inf);
  let encoded=Fp4Instance::encode(&inst.g).unwrap();let(legacy_icv1,legacy_slug)=inst.names();let mut target_checks=0;
  for w in workloads.iter().filter(|w|w["curve_spec"]["p"]==p&&w["curve_spec"]["seed"]==seed){
   let facts=&w["curve"];assert_eq!(facts["group_order"],inst.group_order);assert_eq!(facts["r"],inst.r);assert_eq!(facts["cofactor"],inst.cofactor);assert_eq!(facts["generator"],json!(encoded));assert_eq!(facts["icv1"],legacy_icv1);assert_eq!(facts["slug"],legacy_slug);
   let target=w["target"].as_array().ok_or("target")?;let parse=|v:&Value|u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"),16).unwrap();let point=inst.decode(parse(&target[0]),parse(&target[1]))?;assert!(curve.mul(&point,inst.r).inf);target_checks+=1;
  }
  let modulus=[(p-f.c)%p,0,0,0];let canonical=curve_id::extension(p,&modulus,&inst.a.0,&inst.b.0,&BigUint::from(inst.group_order)).ok_or("canonical model identity")?;
  let field=json!({"characteristic":p,"degree":4,"representation":"polynomial_basis","defining_polynomial":[modulus[0],0,0,0,1],"element_encoding":"four ascending t-coefficients packed into little-endian 16-bit lanes","coefficient_bits":16});
  let curve_record=json!({"model":"y^2=x^3+a*x+b","a":inst.a.0,"b":inst.b.0,"curve_order":inst.group_order,"trace":q as i64+1-inst.group_order as i64,"subgroup_order":inst.r,"cofactor":inst.cofactor,"generator":encoded,"target_group":"prime_order_subgroup_generated_by_G"});
  records.push(json!({"p":p,"seed":seed,"field":field,"curve":curve_record,"icv1_identity":{"slug":canonical.slug,"full":canonical.icv1,"model_json":canonical.model_json,"status":"verified_constructor_model_not_main_registry_entry"},"legacy_constructor_identity":{"slug":legacy_slug,"full":legacy_icv1,"status":"exact_parameters_verified_against_frozen_workloads"},"checks":{"p_prime_trial_division":true,"r_prime_trial_division":true,"r_times_G_is_infinity":true,"G_nonidentity_on_curve":true,"hasse_interval":[lower,upper],"unique_subgroup_order_multiple_in_hasse_interval":lower_multiple,"rabin_t_q_equals_t":true,"rabin_t_p2_minus_t":delta.0,"rabin_unit_inverse_witness":inverse.0,"nonsingular_discriminant":disc.0,"rational_two_torsion_verified":true,"frozen_target_checks":target_checks}}));
 }
 let mut out=std::fs::OpenOptions::new().write(true).create_new(true).open(&args[2])?;writeln!(out,"{}",serde_json::to_string_pretty(&json!({"schema":"campaign-fp4-registration-native/v1","source_plan":args[1],"records":records}))?)?;println!("verified four exact Fp4 constructor records");Ok(())
}
