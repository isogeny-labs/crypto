from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;p=D.parent/'feature-source-divisor-bridge/bridge.rs';s=p.read_text();assert s.count('fn main(){')==1
out=s.replace('fn main(){','fn prior_main(){')+'\npub fn program(r:u64,q:i128)->(bool,Vec<(u64,i128)>){let m=projected_program(r,q);(m.negative,m.exponents.into_iter().collect())}\n'
(D/'prior.rs').write_text(out);(D/'generation.json').write_text(json.dumps({'source':'../feature-source-divisor-bridge/bridge.rs','sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(out.encode()).hexdigest(),'edits':'rename main to prior_main; append program accessor; arithmetic unchanged'},indent=2)+'\n')
