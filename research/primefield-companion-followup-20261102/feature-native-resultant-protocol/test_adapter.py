"""Synthetic protocol/export/capture tests only. Never invokes the native panel."""
import copy,json,tempfile,unittest,subprocess,time,sys
from pathlib import Path
from unittest.mock import patch
import common,capture
export=common.load('export_adapter',common.D/'export.py')
D=common.D
class AdapterTests(unittest.TestCase):
 def draft(self):return json.loads((D/'protocol.draft.json').read_text())
 def frozen(self):
  p=self.draft();p['sample_size']['preflight_runtime']['estimated_total_seconds']=1;p['analysis_plan']['frozen_before_data']=True
  for arm in p['arms']:arm['instrument']['binary_sha256']='c'*64
  p['freeze'].update(status='frozen',protocol_sha256=common.canonical.protocol_hash(p),commit='1b552f1205221afb1399a5087c8b8300515049bb',frozen_at='2026-11-07T00:00:00Z',filed_before_affected_data=True)
  return p
 def test_draft_valid_and_not_launchable(self):
  p=self.draft();self.assertEqual(common.canonical.schema_errors(p,'experiment.protocol/v1'),[]);self.assertEqual(common.canonical.lint_protocol(p,D),[])
  with self.assertRaises(ValueError):common.protocol(D/'protocol.draft.json',common.canonical.protocol_hash(p))
 def test_frozen_without_preflight_rejected(self):
  p=self.frozen();p['sample_size']['preflight_runtime']['estimated_total_seconds']=None;p['freeze']['protocol_sha256']=common.canonical.protocol_hash(p)
  self.assertTrue(any('preflight'in x for x in common.canonical.lint_protocol(p,D)))
 def admission(self,p,out):
  return dict(admitted=True,protocol_sha256=p['freeze']['protocol_sha256'],source_commit=p['arms'][0]['instrument']['commit'],expires_unix=time.time()+600,charged_and_reserved_remainder=1,reserved_usd=1,max_attempts=1,cpus=1,memory_mib=1024,concurrency=1,role='job-submitter',namespace='isolab',quota_readback_verified=True,publication_verified=True,job_uid='synthetic-job',pod_uid='synthetic-pod',image_digest='synthetic-image',budget_evidence_sha256='d'*64,capacity_evidence_sha256='d'*64,publication_evidence_sha256='d'*64,attempt_directory=str(out))
 def test_admission_refusals(self):
  p=self.frozen();a=self.admission(p,Path('/tmp/synthetic-only'));capture.admission(p,a,p['freeze']['protocol_sha256'],time.time())
  for k,v in [('admitted',False),('protocol_sha256','e'*64),('source_commit','e'*40),('expires_unix',0),('charged_and_reserved_remainder',None),('reserved_usd',0),('reserved_usd',True),('expires_unix',float('inf')),('charged_and_reserved_remainder',float('nan')),('cpus',True),('budget_evidence_sha256','bad'),('max_attempts',2),('cpus',2),('memory_mib',2048),('role','admin'),('quota_readback_verified',False),('publication_verified',False),('job_uid','')]:
   with self.subTest(k=k),self.assertRaises(ValueError):capture.admission(p,dict(a,**{k:v}),p['freeze']['protocol_sha256'],time.time())
 def test_synthetic_export_canonical_and_bad_rows(self):
  p=self.frozen();b=common.bindings(p,D);inputs=common.analyzer.fixtures(b['inputs'].read_text());names=b['metrics'].read_text().splitlines();rows=[]
  for c in inputs:
   for off in range(5):
    arm=common.analyzer.ARMS[(off+c['case_id'])%5];m=dict.fromkeys(names,0);m.update(Product=10+(arm=='positive'),Projection=1)
    rows.append(dict({k:c[k]for k in ['case_id','p','kind','n','m','expected']},arm=arm,ordinal=len(rows),value=c['expected'],protocol_sha256=p['freeze']['protocol_sha256'],input_sha256=common.sha(b['inputs']),metrics=m))
  # Rewrite binding paths for the temporary protocol; canonical hash changes with those paths.
  for item in p['extensions']['bindings'].values():item['path']=str((D/item['path']).resolve())
  p['analysis_plan']['script']['path']=str(b['analysis']);p['freeze']['protocol_sha256']=common.canonical.protocol_hash(p)
  for r in rows:r['protocol_sha256']=p['freeze']['protocol_sha256']
  with tempfile.TemporaryDirectory()as tmp:
   base=Path(tmp);pp=base/'protocol.json';pp.write_text(json.dumps(p));raw=base/'rows.jsonl';raw.write_text('\n'.join(json.dumps(r)for r in rows)+'\n')
   receipt=dict(exit_code=0,raw_sha256=common.sha(raw),protocol_sha256=p['freeze']['protocol_sha256'],binary_sha256='c'*64,input_sha256=common.sha(b['inputs']),metrics_sha256=common.sha(b['metrics']),job_uid='synthetic')
   rp=base/'receipt.json';rp.write_text(json.dumps(receipt));e,analysis=export.build(p,pp,raw,rp,base)
   self.assertEqual(len(e['runs']),240);self.assertTrue(analysis['controls_passed']);self.assertTrue(all(not r['measurements']['wall_valid']and r['verification']['status']=='no_claim'for r in e['runs']))
   self.assertEqual(common.canonical.schema_errors(e,'experiment.export/v1'),[]);self.assertEqual(common.canonical.lint_export(e,p,base,True),[])
   cmd=[sys.executable,str(D/'export.py'),'--protocol',str(pp),'--expected-protocol',p['freeze']['protocol_sha256'],'--rows',str(raw),'--receipt',str(rp),'--out',str(base/'exported')]
   cli=subprocess.run(cmd,capture_output=True);self.assertEqual(cli.returncode,0,cli.stderr)
   check=subprocess.run([sys.executable,str(D/'authority/tools/experiment_protocol.py'),'check-export',str(base/'exported/export.json'),'--protocol',str(pp),'--verify-files'],capture_output=True);self.assertEqual(check.returncode,0,check.stdout+check.stderr)
   saved=(base/'exported/export.json').read_bytes();self.assertNotEqual(subprocess.run(cmd,capture_output=True).returncode,0);self.assertEqual(saved,(base/'exported/export.json').read_bytes())
   for k,v in [('exit_code',1),('binary_sha256','e'*64),('raw_sha256','e'*64)]:
    rp.write_text(json.dumps(dict(receipt,**{k:v})))
    with self.assertRaises(ValueError):export.build(p,pp,raw,rp,base)
   rows.pop();raw.write_text('\n'.join(json.dumps(r)for r in rows)+'\n');receipt['raw_sha256']=common.sha(raw);rp.write_text(json.dumps(receipt))
   with self.assertRaises(ValueError):export.build(p,pp,raw,rp,base)
 def test_capture_check_failure_timeout_and_single_directory(self):
  p=self.frozen()
  with tempfile.TemporaryDirectory()as tmp:
   t=Path(tmp);binary=t/'not-executed';binary.write_bytes(b'synthetic fixture never executed')
   for arm in p['arms']:arm['instrument']['binary_sha256']=common.sha(binary)
   p['freeze']['protocol_sha256']=common.canonical.protocol_hash(p)
   for status,returncode,effect in [('failed',7,None),('timeout',124,subprocess.TimeoutExpired('synthetic',120)),('infra_error',None,OSError('synthetic missing executable'))]:
    out=t/status;a=self.admission(p,out)
    with patch.object(capture.subprocess,'run',side_effect=effect,return_value=subprocess.CompletedProcess([],7))as child:
     check=capture.capture(p,D/'protocol.draft.json',binary,a,out,False);self.assertFalse(check['executed']);child.assert_not_called()
     r=capture.capture(p,D/'protocol.draft.json',binary,a,out,True);self.assertEqual(r['status'],status);self.assertEqual(r['exit_code'],returncode);self.assertTrue((out/'receipt.json').exists())
     with self.assertRaises(FileExistsError):capture.capture(p,D/'protocol.draft.json',binary,a,out,True)
     with self.assertRaises(ValueError):capture.capture(p,D/'protocol.draft.json',binary,a,t/'renamed',True)
 def test_deadline_expired_during_checks_does_not_start_child(self):
  p=self.frozen()
  with tempfile.TemporaryDirectory()as tmp:
   t=Path(tmp);binary=t/'fixture';binary.write_bytes(b'never execute')
   for arm in p['arms']:arm['instrument']['binary_sha256']=common.sha(binary)
   p['freeze']['protocol_sha256']=common.canonical.protocol_hash(p);out=t/'attempt';a=self.admission(p,out);now=time.time()
   with patch.object(capture.time,'time',side_effect=[now,now+1000]),patch.object(capture.subprocess,'run')as child:
    r=capture.capture(p,D/'protocol.draft.json',binary,a,out,True);child.assert_not_called();self.assertEqual(r['status'],'timeout');self.assertTrue((out/'receipt.json').exists())
if __name__=='__main__':unittest.main()
