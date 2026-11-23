# Administrative fault injection only. No provider requests or solver execution.
import hashlib,json,os,pathlib,subprocess,tarfile,tempfile
source=pathlib.Path('/home/ec2-user/ecbench-e2e-20261030-v19/controller.sh').read_text()
root=pathlib.Path(tempfile.mkdtemp(prefix='ecbench-controller-test-'))
for scenario,wanted in [('healthy',0),('absent',93)]:
 d=root/scenario;d.mkdir();(d/'private').mkdir();(d/'bin').mkdir();(d/'payload').mkdir()
 for n in ['bootstrap.exit','validation.exit']:(d/'payload'/n).write_text('0\n')
 with tarfile.open(d/'fixture.tgz','w:gz') as t:
  for p in (d/'payload').iterdir():t.add(p,arcname=p.name)
 (d/'digest').write_text(hashlib.sha256((d/'fixture.tgz').read_bytes()).hexdigest())
 (d/'request.json').write_text(json.dumps({'name':'fixture'}));(d/'private/auth.curl').write_text('')
 (d/'scenario').write_text(scenario)
 (d/'controller.sh').write_text(source.replace('cd /home/ec2-user/ecbench-e2e-20261030-v19',f'cd {d}'))
 (d/'bin/sleep').write_text('#!/bin/sh\nexit 0\n')
 (d/'bin/aws').write_text('''#!/bin/sh
if [ "$1" = configure ]; then exit 0; fi
cp fixture.tgz "$4"
''')
 (d/'bin/curl').write_text('''#!/usr/bin/python3
import json,pathlib,sys
p=pathlib.Path('.');a=sys.argv[1:];url=a[-1];method=a[a.index('-X')+1] if '-X' in a else 'GET'
def emit(v):
 s=json.dumps(v)
 if '-o' in a:pathlib.Path(a[a.index('-o')+1]).write_text(s)
 else:print(s)
 if '-w' in a:print('200',end='')
pod={'id':'fixturepod','name':'fixture','status':'RUNNING','cost':0.89,'gpu':{'count':1,'id':'NVIDIA GeForce RTX 4090'}}
with (p/'calls.log').open('a') as f:f.write(method+' '+url+'\\n')
if method=='POST':emit({'id':'fixturepod'})
elif method=='DELETE':
 (p/'deleted').touch();emit({})
elif '/logs?' in url:
 n=int((p/'logcount').read_text())+1 if (p/'logcount').exists() else 1;(p/'logcount').write_text(str(n))
 lines=['waiting'] if n==1 else ['ECBENCH_ARCHIVE_SHA256 '+(p/'digest').read_text(),'ECBENCH_DONE validation=0 archive=0']
 for line in lines:print('data: '+json.dumps({'line':line}))
elif url.endswith('/pods'):
 pods=[pod] if (p/'read-failed').exists() and not (p/'deleted').exists() and (p/'scenario').read_text()=='healthy' else []
 emit({'pagination':{'hasNextPage':False},'pods':pods})
else:
 if not (p/'initial').exists():(p/'initial').touch();emit(pod)
 else:(p/'read-failed').touch();sys.exit(22)
''')
 for p in (d/'bin').iterdir():p.chmod(0o755)
 env=dict(os.environ,PATH=str(d/'bin')+':'+os.environ['PATH'])
 r=subprocess.run(['/bin/bash',str(d/'controller.sh')],env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=30)
 (d/'test-output.log').write_text(r.stdout)
 assert r.returncode==wanted,(scenario,r.returncode,r.stdout)
 assert (d/'controller.exit').read_text().strip()==str(wanted)
 assert (d/'deletion.txt').read_text().strip()=='DELETION_VERIFIED fixturepod'
 calls=(d/'calls.log').read_text().splitlines();assert sum(x.startswith('DELETE') for x in calls)==1
 if scenario=='healthy':
  assert 'VALIDATION_PASSED' in r.stdout
  assert sum('/logs?' in x for x in calls)==2
  assert calls.index(next(x for x in calls if x.startswith('DELETE')))>max(i for i,x in enumerate(calls) if '/logs?' in x)
 else:
  assert 'POD_ABSENT_TWICE' in r.stdout
  assert (d/'partial-collected.sha256').exists()
 print('PASS',scenario,'exit',wanted,'single cleanup; artifact custody checked')
print('fixtures',root)
