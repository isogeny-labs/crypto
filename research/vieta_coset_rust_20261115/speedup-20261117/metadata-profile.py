#!/usr/bin/env python3
"""Control-plane metadata/hash export only; no crypto/statistical analysis.
Copies the two preserved worker records and native profiler output pointers.
The frozen Bash analysis and every original byte remain unchanged.
"""
import copy,datetime,hashlib,json,pathlib
r=pathlib.Path(__file__).parent
p=json.loads((r/'protocol-profile.json').read_text())
e=copy.deepcopy(json.loads((r/'export-qualification.json').read_text()))
def pin(path,kind):
 b=path.read_bytes();return {'path':str(path.relative_to(r)),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'kind':kind}
e.update(export_id=p['protocol_id']+'-PRESERVED-PROFILE-EXPORT',created=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),protocol={'path':'protocol-profile.json','sha256':p['freeze']['protocol_sha256'],'protocol_id':p['protocol_id'],'version':p['version']},source={'repo':'crypto','commit':p['arms'][0]['instrument']['commit'],'dirty':None})
e['arms']=[{'id':a['id'],'role':a['role'],'binary_sha256':a['instrument']['binary_sha256'],'commit':a['instrument']['commit']} for a in p['arms']]
host=pin(r/'profiles/isolation.jsonl','host');e['host'].update(path=host['path'],sha256=host['sha256'])
folder=r/'profiles/attribution-20261117T0709Z';e['files']=[host]+[pin(x,'raw') for x in sorted(folder.iterdir()) if x.is_file()];e['runs']=[]
for arm in e['arms']:
 name=arm['id'];raw=folder/(name+'.json');row=json.loads(raw.read_text());assert row['status']=='ok'
 e['runs'].append({'run_id':'preserved-profile:'+name,'arm':name,'instance_id':'0','seed':'1397751811','replicate':0,'warmup':False,'status':'succeeded','outcome_class':'completed','attempts':1,'resumed':0,'checkpoint':None,'measurements':{'counted':{'muls':row['query_work']['muls']},'wall_seconds':None,'wall_valid':False,'cpu_user_seconds':None,'cpu_sys_seconds':None,'max_rss_bytes':None,'counters':row['query_work']},'isolation':{'tier':'L0','contended':json.loads((r/'profiles/isolation.jsonl').read_text().splitlines()[-1])['run']['contended'],'blockers':['Profiler active, timer warning/undercoverage; no throughput timing','Actual original wrapper is pinned; no L2 admission']},'verification':{'status':'no_claim','verifier':'Original driver complete-set check; producer-owned, no independent signature'},'exclusion':{'excluded':False,'rule':None},'artifacts':[pin(raw,'raw')],'started_at':None,'finished_at':None,'worker':'original CPU4 profile process','backend_ref':str(raw.relative_to(r))})
e['flow']={'planned':2,'attempted':2,'completed':2,'not_completed':0,'interrupted':0,'resumed':0,'excluded':[]};e['analysis']={'script':pin(r/'profile-native.sh','analysis'),'blinded':False,'blinding_key_sha256':None,'outputs':[pin(x,'analysis') for x in sorted(folder.glob('*-functions.txt'))],'unmasked_at':None}
e['notes']=[{'kind':'observation','by':'producer','text':'Post-run metadata export of original two processes, no rerun. gprofng warned1000->0 timer and tables under-cover CPU time. No reliable percentage/hotspot or speed claim. Counts/outputs copied, profiler/native tables preserved. No independent standing, L2 admission or S3 availability claimed.'}]
out=r/'export-profile.json';assert not out.exists();out.write_text(json.dumps(e,separators=(',',':'))+'\n')
