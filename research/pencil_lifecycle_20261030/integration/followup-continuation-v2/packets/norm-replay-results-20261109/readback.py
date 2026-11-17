"""Read archived native records only; no compiler subprocess or payload."""
from pathlib import Path
import hashlib,json,re
P=Path(__file__).resolve().parent
N=Path('/home/ec2-user/pencil-native-nonbranch-successor-20261109');O=Path(str(N)+'.outer')
C=Path('/home/ec2-user/pencil-validation-receipts/norm-20261109-control');F=P.parent/'nonbranch-successor-20261109'
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
freeze=json.loads((F/'freeze.json').read_text());release=json.loads((O/'coordinator-release.json').read_text())
assert H(F/'freeze.json')==release['freeze_sha256']
for n,h in freeze['files'].items():assert H(F/n)==h
modules=['Nonbranch','IndependentControls','Readback','ExpectedFailure']
for n in modules:assert H(N/(n+'.lean'))==freeze['files'][n+'.lean.txt']
names=json.loads((F/'names.json').read_text());expected=names['producer']+names['reviewer'];assert len(expected)==30 and len(set(expected))==30
rows=[]
for line in (N/'Readback.stdout').read_text().splitlines():
 match=re.fullmatch(r"'([^']+)' depends on axioms: \[([^\]]*)\]",line);assert match,line
 axioms=[a.strip() for a in match[2].split(',') if a.strip()];assert set(axioms)<={'propext','Classical.choice','Quot.sound'}
 rows.append({'name':match[1],'axioms':axioms})
assert len(rows)==30 and sorted(x['name'] for x in rows)==sorted(expected)
commands=json.loads((N/'commands.json').read_text());assert [c['module'] for c in commands]==modules
for c,e in zip(commands,[0,0,0,1]):assert c['exit']==c['expected_exit']==e and c['reaped'] and c['remaining_group']==[]
neg=(N/'ExpectedFailure.stdout').read_text();assert len([l for l in neg.splitlines() if 'error:' in l])==1
assert 'error: unsolved goals' in neg and re.search(r'⊢\s*False\b',neg)
assert not (N/'ExpectedFailure.stderr').read_text()
for n in modules[:3]:assert 'sorry' not in ((N/(n+'.stdout')).read_text()+(N/(n+'.stderr')).read_text()).lower()
pre=json.loads((N/'pre-hashes.json').read_text());post=json.loads((N/'post-hashes.json').read_text());assert pre==post and len(pre)==16736 and sum(x['bytes'] for x in pre.values())==3111892318
completion=json.loads((O/'completion.json').read_text());assert completion['status']=='passed';phases=[]
for phase,limit in [('pre',900),('compiler',600),('post',900)]:
 r=json.loads((O/(phase+'-outer-exit.json')).read_text());native=json.loads((N/(phase+'-receipt.json')).read_text());assert r['passed'] and r['returncode']==0 and r['terminal_owner_confirmed'] and not r['errors'] and r['elapsed_seconds']<limit
 assert native['status']=='passed' and not native['errors'] and not native['pending_signals']
 if phase!='compiler':assert native['checked_dependency_files']==16736 and native['checked_dependency_bytes']==3111892318
 unit='pencil-nonbranch-successor-20261109-'+phase+'.service';owner=json.loads((O/(unit+'-manager-terminal.json')).read_text());assert owner['invocation_id']==r['invocation_id']
 valid=[x for x in owner['records'] if x.get('_PID')=='1' and x.get('_UID')=='0' and x.get('_SYSTEMD_UNIT')=='init.scope' and x.get('_BOOT_ID')=='31b37cfb156242ff8b0ffadc5cec5a12' and x.get('UNIT')==unit and x.get('INVOCATION_ID')==r['invocation_id']]
 starts=[int(x['__MONOTONIC_TIMESTAMP']) for x in valid if x.get('MESSAGE_ID')=='39f53479d3a045ac8e11786248231fbf' and x.get('JOB_TYPE')=='start' and x.get('JOB_RESULT')=='done'];ends=[int(x['__MONOTONIC_TIMESTAMP']) for x in valid if x.get('MESSAGE_ID')=='7ad2d189f7e94e70a38c781354912448'];assert starts and ends and min(ends)>=max(starts)
 cpu=[int(x['CPU_USAGE_NSEC']) for x in valid if 'CPU_USAGE_NSEC' in x]
 phases.append({'phase':phase,'elapsed_seconds':r['elapsed_seconds'],'limit_seconds':limit,'native_elapsed_seconds':native['elapsed_seconds'],'manager_cpu_nsec':cpu,'invocation':r['invocation_id'],'manager_start_success_join':True})
caller=json.loads((C/'outer-exit.json').read_text());assert caller['exit']==0 and caller['charged_seconds']==2400
result={'schema':'norm.native-archival-review/v1','recorded_machine_status':'passed','source_fidelity':'faithful_scoped_conditional_nonbranch_norm_and_explicit_Q_controls','independent_kernel_replay':False,'runtime_rerun':False,'freeze_sha256':H(F/'freeze.json'),'source_hashes':{n:H(N/(n+'.lean')) for n in modules},'readback_names':len(rows),'readback_axioms':rows,'commands':commands,'phases':phases,'outer_elapsed_seconds':completion['elapsed_seconds'],'caller_elapsed_seconds':caller['elapsed_seconds'],'charged_worker_seconds':2400,'dependency_files':len(pre),'dependency_bytes':sum(x['bytes'] for x in pre.values()),'pre_post_dictionary_equal':True,'pre_hash_manifest_sha256':H(N/'pre-hashes.json'),'post_hash_manifest_sha256':H(N/'post-hashes.json'),'expected_failure':'one unsolved False goal, exit1, empty stderr','warning':'Nonbranch has two letI style warnings; retained stdout','trust':'Pinned shared caches; pre/post equality does not exclude transient concurrent change. No independent binary/kernel rechecker. Reviewer did not rerun Lean; no validated_scoped formal promotion.'}
(P/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(H(P/'result.json'))
