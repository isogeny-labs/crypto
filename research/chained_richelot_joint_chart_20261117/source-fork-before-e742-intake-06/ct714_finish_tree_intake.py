from pathlib import Path
import os,re,subprocess,json
p=Path('research/chained_richelot_joint_chart_20261117/source-fork-before-e742-intake-06')
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
target=subprocess.check_output(['git','rev-parse','origin/main'],env=env).decode().strip();rows=[]
def run(argv):
    x=subprocess.run(argv,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    rows.append({'argv':argv,'exit':x.returncode,'stdout':x.stdout.decode(),'stderr':x.stderr.decode()})
    (p/'NORMAL_CURRENT_MAIN_FINAL_NARROW_TREE_INTAKE.json').write_text(json.dumps(rows,indent=2)+'\n')
    return x
for i in range(60):
    x=run(['git','-c','gc.auto=0','read-tree','--reset','-i',target])
    if x.returncode==0:break
    assert b'Could not read' in x.stderr or b'unable to read tree' in x.stderr,x.stderr
    m=re.search(rb'[0-9a-f]{40}',x.stderr);assert m,x.stderr
    oid=m.group().decode()
    x=run(['git','-c','gc.auto=0','-c','fetch.recurseSubmodules=false','-c','fetch.negotiationAlgorithm=noop','fetch','--no-tags','--no-write-fetch-head','--filter=tree:0','origin',oid]);assert x.returncode==0,x.stderr
else:raise AssertionError('bounded metadata intake limit60 reached')
a=subprocess.check_output(['git','write-tree','--missing-ok'],env=env).strip();b=subprocess.check_output(['git','rev-parse',target+'^{tree}'],env=env).strip();assert a==b
x=run(['git','reset','--soft',target]);assert x.returncode==0,x.stderr
print('full exact tree',a.decode(),'head',target,'new narrow trees',sum('fetch' in x['argv'] for x in rows))
