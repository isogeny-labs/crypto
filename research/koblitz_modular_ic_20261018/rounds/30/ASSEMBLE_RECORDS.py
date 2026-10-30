#!/usr/bin/env python3
"""Admit retained exact outputs and assemble metadata; no cryptographic math."""
from pathlib import Path
import hashlib,json,re,platform,subprocess
here=Path(__file__).resolve().parent
native=[('scale30-01','SCALE30_ALL_CHECKS_PASS'),('color12-01','COLOR30_ALL_CHECKS_PASS'),('color436-01','COLOR30_ALL_CHECKS_PASS'),('enum30-01','ENUM30_ALL_CHECKS_PASS'),('descent30-01','DESCENT30_ALL_CHECKS_PASS')]
proofs=[('lean30-01','KoblitzCyclicRecovery30',7,10),('descent-lean30-01','KoblitzConstantExtension30',2,3)]
runs=[]
for name,marker in native:
 p=here/'runs'/name;s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0';assert re.search('^'+marker+'$',s,re.M);assert not re.search(r'^\s*\?',s,re.M)
 runs.append({'id':name,'stage':'native','status':'accepted','source_revision':(p/'source-revision.txt').read_text().strip(),'marker':marker})
for name,module,named,audited in proofs:
 p=here/'runs'/name;assert(p/'compile.exit').read_text().strip()=='0';assert(p/'audit.exit').read_text().strip()=='0';s=(p/'audit.log').read_text();assert f'audited {audited} theorems in [{module}]; 0 failing, 0 trusted' in s
 assert len(re.findall(r'^theorem\s',(p/(module+'.lean')).read_text(),re.M))==named
 runs.append({'id':name,'stage':'proof','status':'accepted','source_revision':(p/'source-revision.txt').read_text().strip(),'named':named,'audited':audited,'trusted_native':0,'failures':0})
scale=json.loads((here/'runs/scale30-01/certificates/results.json').read_text());assert len(scale['records'])==29
color=[]
for degree,name,expected in [(12,'color12-01',[(3,4),(4,3)]),(436,'color436-01',[(3,3)])]:
 s=(here/'runs'/name/'native.log').read_text();pairs=[tuple(map(int,m)) for m in re.findall(r'^COLOR30_VERIFIED_COLORS first,second\n(\d+)\n(\d+)',s,re.M)];assert pairs==expected
 color.append({'base_field_degree':degree,'compatible_middle_count':len(pairs),'kernel_color_pairs':pairs,'all_full49_transports_pass':True,'scalar_fiber_orbit_degree':5})
s=(here/'runs/enum30-01/native.log').read_text();enum=[{'degree':int(a),'compatible_count':int(b)}for a,b in re.findall(r'^ENUM30_COMPLETE_COMPATIBLE_MIDDLES degree=(\d+) count=(\d+)',s,re.M)];assert enum==[{'degree':12,'compatible_count':2},{'degree':436,'compatible_count':1}]
descent=json.loads((here/'runs/descent30-01/certificates/results.json').read_text());assert len(descent['records'])==6
record={'schema':'koblitz.round30-results/v1','claims_class':'exact algebra and native finite-kernel construction; written geometric boundaries','runs':runs,'scale':scale,'ordinary_kernel_colors':color,'finite_middle_enumeration':{'complete':True,'records':enum,'scope':'common-j and parameter polynomials over retained ordinary fields; original-incidence identification separate'},'constant_extension':descent,'proof':{'named_theorems':9,'audited_declarations':13,'generated_declarations':4,'failures':0,'trusted_native':0,'formal_geometry':False},'actual_geometric_transfer_at_larger_n':False,'arbitrary_Jacobian_reverse_evaluator_complete':False,'new_IC_relation_rank':None,'performance_measured':False,'native_attempts':5,'accepted_native':5,'native_failures':0,'unresolved':['F32 colored component divisor classes and Frobenius/deck descent binding','ordinary degree284/83 kernel colors','actual F2 CM-factor map and intended subgroup binding','old quotient saturation, determinant19 theta embedding and node187 original-ring certificate']}
(here/'results.json').write_text(json.dumps(record,indent=2)+'\n')
(here/'protocol.json').write_text(json.dumps({'schema':'koblitz.exact-computation-protocol/v1','round':30,'normative_record':'PROTOCOL.md and prospective amendments frozen in Git before affected runs','machine_index_created_after_runs':True,'generic_checker_available':False,'comparative_or_statistical_claim':False,'native_limits':{'virtual_memory_KiB':4194304,'timeout_seconds_each':1200},'lean_limits':{'version':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','threads':1,'managed_memory_MiB':4096,'timeout_seconds_each':1200},'new_EKS_or_Diem_or_fifth_claim':False,'merge_authorized':False},indent=2)+'\n')
binaries={'scale':'/var/tmp/koblitz-r30-scale30-01','descent':'/var/tmp/koblitz-r30-descent30-01','Singular':'/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular'}
env={'schema':'koblitz.environment/v1','boot_id':Path('/proc/sys/kernel/random/boot_id').read_text().strip(),'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'native_binary_sha256':{k:hashlib.sha256(Path(v).read_bytes()).hexdigest() for k,v in binaries.items()},'CPU_timing_claim':False,'CPU_time_recorded':False,'execution_times':'per-run started-utc.txt/finished-utc.txt where recorded; not comparative metrics'}
(here/'ENVIRONMENT.json').write_text(json.dumps(env,indent=2)+'\n')
print('Accepted5native/5attempts;9named/13auditedLean;29scale cases;3completeordinarymiddles;6constant-extensionkernelrecords.')
