from pathlib import Path
import hashlib,json
r=Path('/home/ec2-user/crypto-ml-checkpoint-integration-20261107/src/ml_dataset'); b=Path(__file__).parent
s=(r/'owned_search_central.rs').read_text(); start=s.index('fn check_snapshot_binding(');end=s.index('\n/// Fixed-size report join',start)
(b/'helpers.rs').write_text(s[start:end])
def struct(file,name):
 s=(r/file).read_text();a=s.index('pub(super) struct '+name);z=s.index('\n}',a)+2;return s[a:z]
(b/'types.rs').write_text('mod owned_identity { '+struct('owned_search_identity_capsule.rs','Capsule')+' }\nmod capture { '+struct('capture.rs','CommittedEventPosition')+' }\nstruct Snapshot { index:u64, decoded:Option<wire::Attempt> }\n')
paths=[r/'owned_search_central.rs',r/'owned_search_wire_decode.rs',r/'owned_search_wire_prefix.rs',r/'owned_search_identity_capsule.rs',r/'capture.rs']
(b/'SOURCE_BINDINGS.json').write_text(json.dumps({str(p):{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in paths},indent=2)+'\n')
