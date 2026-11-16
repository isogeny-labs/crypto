import json,os
R=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','raw')
v=[json.loads(l) for l in open(os.path.join(R,'validate.jsonl'))]
r1=json.loads(open(os.path.join(R,'r1_n31_C1_s1.jsonl')).read()); r2=json.loads(open(os.path.join(R,'r2_n29_C1_s0.jsonl')).read())
l2=[json.loads(l) for l in open(os.path.join(R,'..','..','L2-d5-and-n33-control','raw','d5_n31.jsonl'))]
out=dict(experiment='L7-d5-stalled',engine='L2 gf2close5 (copied unchanged, sha256 in src/SHA256)',
 validation=v[-1]|dict(scope='d=5 vs Python tensor_closure, n=9,11 s=0..2 arms A,C1,D1; plus L2 V1 81/81 n<=13 and L2 audit 11/11 n=19/21'),
 runs=dict(n31_C1_s1_d5=r1,n29_C1_s0_d5=r2),
 imported_from_L2=dict(n31_D1_s25_d5=[x for x in l2 if x.get('arm')=='D1'][0],n31_C1_s1_d5_capped=[x for x in l2 if x.get('arm')=='C1'][0]),
 answers=dict(n31_C1_s1_one_in_W5=bool(r1['one']),n31_C1_s1_dF=5 if r1['one'] else None,
   n31_D1_s0='SAT (nsol=5, N1 raw): 1 in no W_d; not run',n29_C1_s0='SAT sanity: one=0, dim=ncols-2'),
 predictions=dict(P1='held (1 in W_5, terminated within cap)',P2_one0='held',P2_cpu_ratio=round(r2['cpu']/r1['cpu'],2),P2_ratio_range='0.3-0.6 predicted: FAILED (0.95)'))
json.dump(out,open(os.path.join(R,'..','RESULTS.json'),'w'),indent=1); print(json.dumps(out['answers']),out['predictions'])
