#!/usr/bin/env bash
# Source transport only. Every slice is unchanged Rust from commit 2ca7ece552.
# Usage: bash stage.sh SOURCE_ROOT; source root is a checkout of that commit.
set -euo pipefail
cd "$(dirname "$0")"
s=$1
(cd "$s" && sha256sum -c <<'HASHES'
87c447d4f4a9c751bc316c060042214be10178b68d66bd0f3aa341dcfb977e11  src/cryptanalysis/ic_boundary.rs
79c5a3f8faa316e70c57faf86126dfcb7d378cd97fab1b0ee7cd5f3dfa3415f9  src/cryptanalysis/chained_transfer/ic_control.rs
82ecb8c6c9f7362c4a12372fb9dba9aff308a9949cbd2a92cd530a959b8d356a  src/cryptanalysis/residual_walk.rs
dcf0069a3992871d708cbc3c1779285279031768e084476c165da0e16e004b80  src/cryptanalysis/genus_ladder.rs
HASHES
)
mkdir -p generated
b=$s/src/cryptanalysis/ic_boundary.rs
c=$s/src/cryptanalysis/chained_transfer/ic_control.rs
r=$s/src/cryptanalysis/residual_walk.rs
{
  echo 'use std::collections::{BTreeMap, HashMap}; use std::hash::{BuildHasherDefault, Hasher}; use std::time::Instant; use rand::{Rng,SeedableRng}; use rand::rngs::StdRng; use serde::{Serialize,Deserialize}; use num_bigint::BigUint; use num_traits::ToPrimitive;'
  sed -n '141,203p;820,853p;923,1215p;1399,1401p;1408,1566p;1706,2023p;2420,3384p;4176,4211p' "$b"
  echo 'pub fn generic_floor_ops(r:f64,a:f64)->f64 {(std::f64::consts::PI*r/(2.0*a)).sqrt()}'
} > generated/boundary.rs
{
  echo 'use rand::Rng; use rand::rngs::StdRng; use crate::cryptanalysis::ic_boundary::pivot_inversion_steps; use crate::cryptanalysis::residual_walk::is_prime_u64;'
  sed -n '50,106p;599,646p;742,753p;923,1477p' "$c"
} > generated/control.rs
{
  sed -n '291,320p;336,341p' "$r"
  echo 'fn mul_mod(a:u64,b:u64,p:u64)->u64 {((a as u128*b as u128)%p as u128) as u64} fn pow_mod(mut a:u64,mut e:u64,p:u64)->u64 {let mut z=1;while e>0 {if e&1==1 {z=mul_mod(z,a,p);}a=mul_mod(a,a,p);e>>=1;}z}'
} > generated/residual.rs
sed -n '46,2030p' "$s/src/cryptanalysis/genus_ladder.rs" > generated/ladder.rs
sha256sum "$b" "$c" "$r" "$s/src/cryptanalysis/genus_ladder.rs" > generated/source.sha256
if [ "${G4_AUDIT_DUMP:-0}" = 1 ]; then
  # Optional audit-only observation. No RNG draws, counters or arithmetic changed.
  sed -i '/            if prm.verify_relations {/i\            if let Ok(path) = std::env::var("G4_REL_DUMP") { use std::io::Write; let mut file=std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap(); writeln!(file,"{}",serde_json::json!({"kind":"relation","p":p,"r":r,"h":h,"f":\&f,"G":[curve.u_coeffs(\&inst.g),curve.v_coeffs(\&inst.g)],"T":[curve.u_coeffs(\&inst.target),curve.v_coeffs(\&inst.target)],"cur":[\&u,\&vc],"places":\&places,"ca":ca,"cb":cb})).unwrap(); }' generated/ladder.rs
fi
