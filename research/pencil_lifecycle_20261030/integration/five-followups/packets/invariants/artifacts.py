import pathlib,json,hashlib,subprocess
p=pathlib.Path(__file__).parent
root=pathlib.Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
old=pathlib.Path('/home/ec2-user/crypto-pr2304-persist')
paths=['research/projective_law_followup_20261022/src/bin/projective_law_gate.rs','research/projective_law_followup_20261022/src/bin/projective_state_collector.rs','src/cryptanalysis/three_leads.rs']
sources=[]
for name in paths:
 b=subprocess.check_output(['git','-C',str(root),'show','HEAD:'+name]);local=(old/name if name.startswith('research/') else root/name).read_bytes()
 sources.append(dict(path=name,sha256=hashlib.sha256(b).hexdigest(),historical_copy_matches_head=b==local))
 assert b==local,(name,'source drift')
(p/'sources.json').write_text(json.dumps(dict(commit=subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),sources=sources),indent=2))
rows=[('add11','511–530','same x; inverse first, equal second, otherwise distinct','field ordinate alternatives and validity still to connect'),('add12_generic','232–290','K!=0','output nonzero weights follow from factors; numerator validity open'),('add12_complete','828–878','K=0; inverse/equal; repeated support or distinct','extracted point validity, tripling and secondary K guard open'),('same_u_inverse','921–924','q1c=q2c,r1c=r2c; v1c=-v2c','identity invariant direct; class semantics separate'),('same_u_equal','925–928','same support and equal v after inverse excluded','delegates doubling'),('same_u_mixed','929–953','same support, neither equal nor inverse','existing weighted theorem excludes zero slope; extraction validity open'),('shared_zero_q','961–967','different support; B=0,dq=0','new Lean field theorem excludes guard'),('shared_cancel','968–993','B=0,dq!=0,opposite shared ordinate','extracted point validity and add11 closure open'),('shared_positive','994–1007','equal shared ordinate, compose dbl and two additions','mismatch and secondary None exclusions remain'),('distinct_drop','1009–1030','B!=0,C=0','weights nonzero algebraically; output validity open'),('distinct_common','1031','B!=0,C!=0','weights nonzero algebraically; full source/validity bridge open'),('double','727–827','B=0 split S; B!=0 split C','extracted-point branch needs validity/nonzero ordinate; remaining weights factor')]
(p/'branches.json').write_text(json.dumps([dict(branch=a,lines=b,guard=c,status=d,source=paths[0]) for a,b,c,d in rows],indent=2))
status=dict(schema='research.component-result/v1',scope='field-level constructor and exceptional-guard lemmas',scientific_trials=0,claims=[dict(id=x,status='producer_compiled',independent_review='pending') for x in ['weight_decode','weight_validity','negation_validity','shared_zero_q_implies_same_support','shared_zero_q_excluded']],implementation_correspondence='open',collector_inductive_closure='open',cost_ratio=None,failed_compile_attempts=1)
(p/'status.json').write_text(json.dumps(status,indent=2))
