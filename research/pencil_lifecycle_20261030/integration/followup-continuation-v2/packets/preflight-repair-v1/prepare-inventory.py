from pathlib import Path
import json,hashlib
p=Path(__file__).parent;r=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_large_prime_20261029');v=r/'validation/disjoint_enrollment_20261030';inputs=json.loads(Path('/var/tmp/pencil-preflight-proposal-v1/inputs.json').read_text());rows=[]
for x in inputs:
 spec=v/'outputs/cohort-16-remainder.json' if x['name']=='cohort-16-remainder-plan.json' else r/'prepared_specs'/x['name']
 plan=json.loads(Path(x['path']).read_text());rows.append({**x,'spec_path':str(spec),'spec_sha256':hashlib.sha256(spec.read_bytes()).hexdigest(),'spec_id':plan['spec_id']})
pins={name:{'path':str(v/path),'sha256':hashlib.sha256((v/path).read_bytes()).hexdigest()} for name,path in [('certificate','outputs/disjoint-enrollment.json'),('request','request.json'),('receipt','receipt.json'),('review','independent-review.json')]}
(p/'inventory-inputs.json').write_text(json.dumps({'routes':rows,'artifacts':pins},indent=2));const='const ROUTES:&[(&str,&str,&str,&str,usize)]=&[\n'+''.join('('+','.join(json.dumps(x[k]) for k in ['name','sha256','spec_sha256','spec_id'])+','+str(x['workloads'])+'),\n' for x in rows)+'];\n';const+='const ARTIFACTS:&[(&str,&str)]=&[\n'+''.join('('+json.dumps(k)+','+json.dumps(x['sha256'])+'),\n' for k,x in pins.items())+'];\n';(p/'src/inventory_pins.rs').write_text(const)
