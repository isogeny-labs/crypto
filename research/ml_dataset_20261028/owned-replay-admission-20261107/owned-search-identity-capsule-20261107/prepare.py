from pathlib import Path
import hashlib,json,difflib
ROOT=Path('/home/ec2-user/crypto-ml-ordinary-ownership-20261101');OUT=Path(__file__).resolve().parent
paths={k:ROOT/p for k,p in {'capture':'src/ml_dataset/capture.rs','execution':'src/ml_dataset/execution.rs','owner':'src/ml_dataset/sat_original_owner.rs','solvers':'src/ml_dataset/solvers.rs','koblitz':'src/ml_dataset/koblitz_m3.rs','index':'src/store/index.rs','central':'src/ml_dataset/sat_initial_central.rs'}.items()}
src={k:p.read_text() for k,p in paths.items()}
def block(s,marker):
 start=s.index(marker);a=s.index('{',start);depth=1;i=a+1
 while depth:
  depth+=(s[i]=='{')-(s[i]=='}');i+=1
 return s[start:i]
binding='#[derive(Clone,Debug,serde::Serialize,serde::Deserialize,PartialEq,Eq)]\n#[serde(deny_unknown_fields)]\n'+block(src['capture'],'pub struct AttemptBinding')
launch='#[derive(Clone,Debug,serde::Serialize,serde::Deserialize,PartialEq,Eq)]\n#[serde(deny_unknown_fields)]\n'+block(src['execution'],'pub(super) struct LaunchContext')
text='''#![allow(dead_code,unused_imports)]
mod ml_dataset {
use sha2::{Digest,Sha256};
fn digest(b:&[u8])->String{hex::encode(Sha256::digest(b))}
pub mod capture {
'''+block(src['index'],'pub fn validate_sha256')+'\nconst BINDING_SCHEMA:&str="crypto.ml-dataset.attempt-binding/v1";\n'+binding+'\nimpl AttemptBinding{'+block(src['capture'],'pub fn validate(&self)')+'\n'+block(src['capture'],'pub fn digest(&self)')+'}\n}\npub mod execution {\nuse super::capture::AttemptBinding;use serde_json::Value;\nconst SCHEMA:&str="crypto.ml-dataset.execution-context/v1";\n'+launch+'\n'+block(src['execution'],'fn positive(')+'\n'+block(src['execution'],'fn bytes(')+'\nimpl LaunchContext{'+block(src['execution'],'pub fn validate(&self, run:')+'}\n}\npub mod solvers {\nuse std::{path::Path,time::Instant};use serde_json::Value;use super::capture::AttemptBinding;\ntype CaseConfig=Value; // unused token metadata only; no CaseConfig validation claim\n'+block(src['solvers'],"pub(super) struct VerifiedWorkerOriginal<'a>")+'\n#[derive(Clone,Copy,Debug,PartialEq,Eq)]\n'+block(src['koblitz'],'pub(super) enum Stop')+'\n'+block(src['owner'],'trait DecodeBudget')+'\nmod central{use super::*;\n#[path="'+str(OUT/'owned_search_identity_capsule.rs')+'"] mod capsule;\ninclude!("'+str(OUT/'tests.rs')+'");\n}\n}\n}\n'
(OUT/'harness.rs').write_text(text)
base=src['central'];proposed=base.replace('mod search;','mod search;\n#[path="owned_search_identity_capsule.rs"]\nmod owned_identity;',1)
(OUT/'sat_initial_central.proposed.rs').write_text(proposed)
(OUT/'central-registration.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),proposed.splitlines(True),fromfile='a/src/ml_dataset/sat_initial_central.rs',tofile='b/src/ml_dataset/sat_initial_central.rs')))
(OUT/'SOURCE_HASHES.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths.values()},indent=2)+'\n')
