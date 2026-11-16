"""Native exact lattice enumeration, independent Fraction/input audits, bounded reporting."""
from pathlib import Path
import sys,os,time,tempfile,subprocess,json,gzip,hashlib
from .checker import verify_public,audit,norm_verify
p=Path(__file__).resolve().parent;root=p.parents[1];ident=sys.argv[1];start=time.monotonic();raw=[];binary_sha=None;compiler=Path('/usr/bin/g++').resolve()
def call(stage,args,cap):
 with tempfile.TemporaryFile()as out,tempfile.TemporaryFile()as err:
  left=83-(time.monotonic()-start)
  if left<=0:raise RuntimeError('whole-worker child lifetime exhausted')
  t=time.monotonic();child=subprocess.Popen(args,stdout=out,stderr=err);deadline=t+min(cap,left);timed=False
  while True:
   pid,status,usage=os.wait4(child.pid,os.WNOHANG)
   if pid:break
   if time.monotonic()>=deadline:timed=True;child.kill();pid,status,usage=os.wait4(child.pid,0);break
   time.sleep(.005)
  child.returncode=os.waitstatus_to_exitcode(status);out.seek(0);err.seek(0);v=dict(stage=stage,command=args,returncode=child.returncode,timed_out=timed,wall_seconds=time.monotonic()-t,peak_process_rss_kib=usage.ru_maxrss,stdout=out.read().decode(errors='replace'),stderr=err.read().decode(errors='replace'));raw.append(v);assert child.returncode==0 and not timed,stage+' incomplete';return v
try:
 deps=json.load(open(p/'EXTERNAL_GMP.json'));libs=[]
 for dep in deps['entries']:
  q=Path(dep['resolved_path']);assert hashlib.sha256(q.read_bytes()).hexdigest()==dep['sha256'],'GMP dependency mismatch'
  if 'link/runtime'in dep['role']:libs.append(str(q))
 source=root/'research/dual_ladder_20261109_v3/planted_inputs.json';data=json.load(open(source));cases=data['cases'];assert len(cases)==5 and {v['public']['field']for v in cases}=={'P-192','P-224','P-256','P-384','P-521'}
 with tempfile.TemporaryDirectory(prefix='primal-enum-')as td:
  td=Path(td);b=td/'native';call('compile',['/usr/bin/g++','-std=c++17','-O2',str(p/'native.cpp'),'-o',str(b),*libs],35);binary_sha=hashlib.sha256(b.read_bytes()).hexdigest();linked=call('linked-libraries',['/usr/bin/ldd',str(b)],10)
  for dep in deps['entries']:
   if 'link/runtime'not in dep['role']:continue
   soname=Path(dep['path']).name.split('.so')[0]+'.so';hits=[line.split('=>',1)[1].split('(',1)[0].strip()for line in linked['stdout'].splitlines()if soname in line and '=>'in line];assert len(hits)==1 and Path(hits[0]).resolve()==Path(dep['resolved_path']),'GMP loader identity'
  public=td/'public.txt';public.write_text('\n'.join(' '.join(map(str,[v['public']['field'],v['public']['p'],v['public']['M'],*v['public']['T'],*v['public']['U']]))for v in cases)+'\n');run=call('native',[str(b),ident,str(public)],80);native=json.loads(run['stdout']);assert native['count']==5;lookup={v['public']['field']:v for v in cases};assert {v['field']for v in native['cases']}==set(lookup)
  rows=[];complete=True
  for row in native['cases']:
   inp=lookup[row['field']];pub=inp['public'];assert verify_public(pub)
   if not row['enumeration_complete']:complete=False;rows.append(dict(field=row['field'],complete=False,nodes=row['nodes'],conditional_ordered_root_count_exact=None));continue
   sols,nodes=audit(pub,row);tuples=[norm_verify(pub,z)for z in sols];expected=sorted(map(int,inp['expected']['residual_roots']));assert any(sorted(t[:3])==expected and t[3]==int(inp['expected']['k'])for t in tuples),'planted witness not recovered'
   iscomplete=row['complete'];complete&=iscomplete;rows.append(dict(field=row['field'],complete=iscomplete,nodes=row['nodes'],independent_nodes=nodes,lattice_box_points=len(row['points']),conditional_ordered_root_count_exact=len(tuples)if iscomplete else None,conditional_ordered_root_tuples=tuples,unresolved_root_candidates=row['unresolved_root_candidates']))
  result=dict(schema='scientific_result_v1',experiment_id=ident,status='passed'if complete else'inconclusive',summary='Exact selected planted-chart ordered residual roots with independent Fraction/literal-norm checks; no global root or speedup claim.',evidence=dict(scope_complete=complete,conditional_output_contract='all ordered residual triples plus k for selected fixed rational-anchor/h charts; not original S3 auxiliary tuples',rows=rows,input_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),solver_received_expected_roots=False,independent_public_quadratic_binding=True,independent_Fraction_enumeration=True,independent_literal_norm_replay=True,planted_not_benchmark=True,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False),next_actions=['Preregister an unplanted positive/negative ladder and retain complete exceptional/root-auxiliary coverage before any genuine baseline comparison.'])
except (OSError,RuntimeError,AssertionError,ValueError,KeyError)as exc:
 result=dict(schema='scientific_result_v1',experiment_id=ident,status='inconclusive',summary='Primal enumeration incomplete; raw stages retained.',evidence=dict(error=str(exc),scope_complete=False,conditional_ordered_root_count_exact=None,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False),next_actions=['Repair the concrete failed assumption in a new frozen variant, not an identical retry.'])
blob=gzip.compress(json.dumps(raw,separators=(',',':')).encode(),mtime=0)
with(p/'raw.json.gz').open('xb')as f:f.write(blob)
result['evidence'].update(raw_artifact_sha256=hashlib.sha256(blob).hexdigest(),compiled_binary_sha256=binary_sha,compiler_sha256=hashlib.sha256(compiler.read_bytes()).hexdigest(),stage_metrics=[{k:v[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']}for v in raw],memory_metric='wait4 per-process high-water KiB, not aggregate descendants')
text=json.dumps(result,separators=(',',':'))
if time.monotonic()-start>=86:
 result.update(status='inconclusive',summary='Final primal report exceeded acceptance budget.');result['evidence'].update(scope_complete=False,conditional_ordered_root_count_exact=None,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False)
 for row in result['evidence'].get('rows',[]):row.update(complete=False,conditional_ordered_root_count_exact=None)
 text=json.dumps(result,separators=(',',':'))
assert time.monotonic()-start<88,'final serialization deadline';print(text)
