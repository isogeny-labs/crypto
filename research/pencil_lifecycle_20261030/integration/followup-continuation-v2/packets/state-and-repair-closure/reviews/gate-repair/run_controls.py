from pathlib import Path
import subprocess,json,hashlib,re
D=Path(__file__).parent;S=D/'subject';out=[]
for label,args,expected in [('success1',[],0),('success2',[],0),('false',['ReviewerFalse'],1),('missing',['ReviewerDefinitelyMissingModule20261101'],1),('empty',['ReviewerEmpty'],0)]:
 cmd=['python3',str(S/'gate.py')]+args
 r=subprocess.run(cmd,capture_output=True,text=True,timeout=390)
 paths=[line for line in r.stdout.splitlines() if line.startswith(str(S/'invocation-'))]
 receipt=json.loads((Path(paths[-1])/'receipt.json').read_text()) if paths else None
 item={'label':label,'command':cmd,'exit':r.returncode,'expected_exit':expected,'stdout':r.stdout,'stderr':r.stderr,'receipt':receipt}
 (D/(label+'.json')).write_text(json.dumps(item,indent=2)+'\n');out.append(item)
 assert r.returncode==expected and receipt and receipt['gate_exit']==expected and receipt['bindings_unchanged']
 print(label,r.returncode,paths[-1],flush=True)
assert len({x['receipt']['directory'] for x in out})==len(out)
(D/'control-summary.json').write_text(json.dumps(out,indent=2)+'\n')
