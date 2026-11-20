"""All launches mocked. Fabricated panels are temporary, never workload data."""
import json,copy,tempfile,unittest,subprocess,time,sys
from pathlib import Path
from unittest.mock import patch
import common,capture
export=common.load('quotient_export_test',common.D/'export.py')
class Tests(unittest.TestCase):
 def frozen(self,binary):
  p=json.loads((common.D/'protocol.draft.json').read_text());p['sample_size']['preflight_runtime']['estimated_total_seconds']=1;p['analysis_plan']['frozen_before_data']=True
  for arm in p['arms']:arm['instrument'].update(commit='c'*40,binary_sha256=common.sha(binary))
  for item in p['extensions']['bindings'].values():item['path']=str((common.D/item['path']).resolve())
  p['analysis_plan']['script']['path']=str(common.D/'pair.py');p['freeze'].update(status='frozen',protocol_sha256=common.canonical.protocol_hash(p),commit='c'*40,frozen_at='2026-11-07T00:00:00Z',filed_before_affected_data=True);return p
 def admitted(self,p,out):
  return dict(admitted=True,protocol_sha256=p['freeze']['protocol_sha256'],source_commit='c'*40,expires_unix=time.time()+600,charged_and_reserved_remainder=1,reserved_usd=1,max_attempts=1,cpus=1,memory_mib=1024,concurrency=1,role='job-submitter',namespace='isolab',quota_readback_verified=True,publication_verified=True,job_uid='synthetic-job',pod_uid='synthetic-pod',image_digest='synthetic-image',budget_evidence_sha256='d'*64,capacity_evidence_sha256='d'*64,publication_evidence_sha256='d'*64,attempt_directory=str(out))
 def panel(self):
  rows=[]
  for p,r in common.trace.FIXTURES:
   for k in range(r):rows.extend([f'target,{p},{r},{k},1,1,0',f'quotient,{p},{r},{k},0,4,4,2,3,3,4,0,true,{p}'])
  return ('\n'.join(rows)+'\n').encode()
 def setup_tmp(self,t):
  binary=t/'dummy';binary.write_bytes(b'never execute');p=self.frozen(binary);pp=t/'protocol.json';pp.write_text(json.dumps(p));out=t/'attempt';return binary,p,pp,out,self.admitted(p,out)
 def success(self,*args,**kwargs):
  kwargs['stdout'].write(self.panel());self.assertEqual(kwargs['env']['PF_QUOTIENT_TRACE_ADMITTED'],'1');self.assertIs(kwargs['preexec_fn'],capture.limit_files);return subprocess.CompletedProcess([],0)
 def test_draft_refuses_and_profile_mutations(self):
  p=json.loads((common.D/'protocol.draft.json').read_text());self.assertEqual(common.canonical.schema_errors(p,'experiment.protocol/v1'),[]);self.assertEqual(common.canonical.lint_protocol(p,common.D),[])
  with self.assertRaises(ValueError):common.protocol(common.D/'protocol.draft.json',common.canonical.protocol_hash(p))
  for field,value in [('instances',618),('replicates',2),('warmups',1)]:
   q=copy.deepcopy(p);q['sample_size'][field]=value
   with self.assertRaises(ValueError):common.profile(q)
 def test_success_export_and_canonical_cli(self):
  with tempfile.TemporaryDirectory()as tmp:
   t=Path(tmp);binary,p,pp,out,a=self.setup_tmp(t)
   with patch.object(capture.subprocess,'run',side_effect=self.success)as child:
    self.assertFalse(capture.capture(p,pp,binary,a,out)['executed']);child.assert_not_called();r=capture.capture(p,pp,binary,a,out,True);self.assertEqual(child.call_count,2)
   self.assertEqual(r['status'],'succeeded');dest=t/'export';dest.mkdir();e,result=export.build(p,pp,out,dest);self.assertEqual(len(e['runs']),1234);self.assertEqual(len(result['targets']),617);self.assertEqual(common.canonical.lint_export(e,p,dest,True),[])
   self.assertTrue(all(not x['measurements']['wall_valid']and x['verification']['status']=='no_claim'for x in e['runs']))
   common.canonical.write_once(dest/'export.json',e)
   q=subprocess.run([sys.executable,str(common.PREVIOUS/'authority/tools/experiment_protocol.py'),'check-export',str(dest/'export.json'),'--protocol',str(pp),'--verify-files'],capture_output=True);self.assertEqual(q.returncode,0,q.stdout+q.stderr)
   with self.assertRaises(FileExistsError):capture.capture(p,pp,binary,a,out,True)
   with self.assertRaises(ValueError):capture.capture(p,pp,binary,a,t/'renamed',True)
   (out/'aa.csv').write_bytes(self.panel()+b'extra\n')
   with self.assertRaises(ValueError):export.build(p,pp,out,dest)
 def test_shared_deadline_no_second_launch(self):
  with tempfile.TemporaryDirectory()as tmp:
   binary,p,pp,out,a=self.setup_tmp(Path(tmp))
   with patch.object(capture.time,'monotonic',side_effect=[0,1,121]),patch.object(capture.subprocess,'run',side_effect=self.success)as child:r=capture.capture(p,pp,binary,a,out,True)
   self.assertEqual(child.call_count,1);self.assertEqual(r['status'],'timeout');self.assertFalse(r['processes'][1]['executed']);self.assertTrue((out/'receipt.json').exists())
 def test_failures_preserved_no_extra_launch(self):
  for effect,status in [(subprocess.TimeoutExpired('mock',120),'timeout'),(OSError('mock failure'),'infra_error'),(subprocess.CompletedProcess([],9),'failed')]:
   with tempfile.TemporaryDirectory()as tmp:
    binary,p,pp,out,a=self.setup_tmp(Path(tmp));kw={'side_effect':effect}if isinstance(effect,Exception)else{'return_value':effect}
    with patch.object(capture.subprocess,'run',**kw)as child:r=capture.capture(p,pp,binary,a,out,True)
    self.assertEqual(child.call_count,1);self.assertEqual(r['status'],status);self.assertTrue((out/'receipt.json').exists());self.assertEqual(r['processes'][0]['executed'],status!='infra_error')
    with self.assertRaises(ValueError):export.build(p,pp,out,Path(tmp))
 def test_invalid_pair_and_valid_but_unequal_pair(self):
  for bad in [b'broken\n',self.panel().replace(b',1,1,0\n',b',2,1,0\n',1)]:
   with tempfile.TemporaryDirectory()as tmp:
    binary,p,pp,out,a=self.setup_tmp(Path(tmp));buffers=iter([self.panel(),bad])
    def fake(*args,**kw):kw['stdout'].write(next(buffers));return subprocess.CompletedProcess([],0)
    with patch.object(capture.subprocess,'run',side_effect=fake):r=capture.capture(p,pp,binary,a,out,True)
    self.assertEqual(r['status'],'invalid_data');self.assertEqual(len(r['processes']),2)
    with self.assertRaises(ValueError):export.build(p,pp,out,Path(tmp))
 def test_admission_and_binding_and_binary_refusal(self):
  with tempfile.TemporaryDirectory()as tmp:
   binary,p,pp,out,a=self.setup_tmp(Path(tmp))
   for field,val in [('admitted',False),('role','admin'),('charged_and_reserved_remainder',None),('cpus',True),('expires_unix',0),('reserved_usd',0)]:
    with self.assertRaises(ValueError):capture.capture(p,pp,binary,dict(a,**{field:val}),out)
   binary.write_bytes(b'changed')
   with self.assertRaises(ValueError):capture.capture(p,pp,binary,a,out)
   q=copy.deepcopy(p);q['extensions']['bindings']['analysis']['sha256']='e'*64
   with self.assertRaises(ValueError):common.bindings(q,pp.parent)
 def test_file_limit_and_prelaunch_deadline(self):
  for hard,expected in [(capture.resource.RLIM_INFINITY,common.FILE_LIMIT),(1024,1024)]:
   with patch.object(capture.resource,'getrlimit',return_value=(hard,hard)),patch.object(capture.resource,'setrlimit')as setlimit:
    capture.limit_files();setlimit.assert_called_once_with(capture.resource.RLIMIT_FSIZE,(expected,expected))
  with tempfile.TemporaryDirectory()as tmp:
   binary,p,pp,out,a=self.setup_tmp(Path(tmp))
   with patch.object(capture.time,'monotonic',side_effect=[0,121]),patch.object(capture.subprocess,'run')as child:r=capture.capture(p,pp,binary,a,out,True)
   child.assert_not_called();self.assertEqual(r['status'],'timeout');self.assertFalse(r['processes'][0]['executed'])
if __name__=='__main__':unittest.main()
