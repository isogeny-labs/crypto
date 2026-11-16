"""Mocked workflow/admission tests. No Docker subprocess or payload is invoked."""
import copy,json,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
import assemble as a
IMAGE='sha256:'+'1'*64
class Workflow(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.out=Path(self.tmp.name)
  self.x={'daemon_id':'test-daemon','base_layers':['base'],'owner':'test-owner','tag':'test-tag','context':'/bound/context','source_revision':'revision'}
  self.calls=[];self.context_checks=0
  self.parent=dict(Id=a.BASE,OS='linux',Architecture='amd64',OnBuild=None,Layers=['base'])
  self.built=dict(Id=IMAGE,OS='linux',Architecture='amd64',OnBuild=None,Layers=['base','new'],User='0:0',Entrypoint=None,Labels={'isolab.local-assembly.owner':'test-owner','isolab.local-assembly.phase':'runtime','org.opencontainers.image.revision':'revision'})
 def tearDown(self):self.tmp.cleanup()
 def verify(self):self.context_checks+=1
 def backend(self,label,args,cap):
  self.calls.append((label,args,cap))
  if label=='daemon':return 0,b'test-daemon\n',b''
  if label=='base':return 0,json.dumps(self.parent).encode(),b''
  if label=='tag-absence':return 1,b'',b'No such image: test-tag'
  if label=='build-runtime':
   (self.out/'runtime.iid').write_text(IMAGE+'\n');(self.out/'runtime.iid').chmod(0o600);return 0,b'',b''
  if label=='read-iid':return 0,(IMAGE+'\n').encode(),b''
  if label=='built-tag':return 0,json.dumps(self.built).encode(),b''
  raise AssertionError(label)
 def run_flow(self,backend=None):
  # The fixture IID is user-owned; emulate root-owned metadata only. No privilege change.
  original=Path.lstat
  def metadata(p,*args,**kwargs):
   st=original(p,*args,**kwargs)
   if p==self.out/'runtime.iid':
    class Proxy:
     st_uid=0
     def __getattr__(self,k):return getattr(st,k)
    return Proxy()
   return st
  with patch.object(Path,'lstat',metadata):return a.workflow(self.x,self.out,backend or self.backend,self.verify)
 def test_success_one_copy_build(self):
  self.assertEqual(self.run_flow()['status'],'built_pending_qualification');self.assertEqual(self.context_checks,2)
  builds=[args for label,args,cap in self.calls if label=='build-runtime'];self.assertEqual(len(builds),1)
  for option in ['--pull=false','--network=none','--platform=linux/amd64','BASE_IMAGE='+a.BASE]:self.assertIn(option,builds[0])
 def test_onbuild_stops_before_build(self):
  self.parent['OnBuild']=['RUN false']
  with self.assertRaisesRegex(ValueError,'OnBuild'):self.run_flow()
  self.assertFalse(any(x[0]=='build-runtime' for x in self.calls))
 def test_existing_tag_stops_before_build(self):
  def backend(label,args,cap):return (0,b'{}',b'') if label=='tag-absence' else self.backend(label,args,cap)
  with self.assertRaisesRegex(ValueError,'absent'):self.run_flow(backend)
  self.assertFalse(any(x[0]=='build-runtime' for x in self.calls))
 def test_failed_build_not_retried(self):
  def backend(label,args,cap):
   if label=='build-runtime':self.calls.append((label,args,cap));return 1,b'',b'failure'
   return self.backend(label,args,cap)
  with self.assertRaisesRegex(ValueError,'build failed'):self.run_flow(backend)
  self.assertEqual(sum(x[0]=='build-runtime' for x in self.calls),1)
 def test_timeout_not_retried(self):
  def backend(label,args,cap):
   if label=='build-runtime':self.calls.append((label,args,cap));raise TimeoutError('mock client timeout')
   return self.backend(label,args,cap)
  with self.assertRaises(TimeoutError):self.run_flow(backend)
  self.assertEqual(sum(x[0]=='build-runtime' for x in self.calls),1)
 def test_wrong_owner_rejected(self):
  self.built['Labels']['isolab.local-assembly.owner']='other'
  with self.assertRaisesRegex(ValueError,'labels'):self.run_flow()
 def test_changed_context_rejected(self):
  def changed():raise ValueError('context changed')
  with self.assertRaisesRegex(ValueError,'context changed'):a.workflow(self.x,self.out,self.backend,changed)
  self.assertFalse(any(x[0]=='build-runtime' for x in self.calls))
class Admission(unittest.TestCase):
 def test_unadmitted_fails_without_effects(self):
  with patch.object(a.subprocess,'Popen',side_effect=AssertionError('external call')):
   with self.assertRaisesRegex(ValueError,'not admitted'):a.admission({'admitted':False})
 def test_duplicate_json_rejected(self):
  with self.assertRaisesRegex(ValueError,'duplicate'):a.strict('{"admitted":false,"admitted":true}')
class AdmissionBindings(unittest.TestCase):
 def binding(self):
  owner='12345678-1234-1234-1234-123456789abc'
  return {'admitted':True,'review_path':'/unused/review','review_sha256':'0'*64,'inputs':{'assembler_sha256':a.sha(Path(a.__file__).read_bytes()),'owner':owner,'tag':'pencil-runtime-replacement:'+owner,'base_image':a.BASE,'not_after_utc':'2099-01-01T00:00:00Z','author':'producer','limits':{'total_seconds':600,'reconciliation_reserve_seconds':60,'build_client_seconds':240,'attempts':1}}}
 def review(self,b):return {'decision':'approve','scope':'one-local-runtime-copy-build','inputs_sha256':a.core_hash(b),'assembler_sha256':b['inputs']['assembler_sha256'],'independent':True,'reviewer':'other','execution_allowance_approved':True}
 def test_structural_review_joins(self):
  b=self.binding()
  with patch.object(a,'read_pin',return_value=json.dumps(self.review(b)).encode()):self.assertEqual(a.admission(b),b['inputs'])
 def test_same_author_rejected(self):
  b=self.binding();r=self.review(b);r['reviewer']='producer'
  with patch.object(a,'read_pin',return_value=json.dumps(r).encode()):
   with self.assertRaisesRegex(ValueError,'independent'):a.admission(b)
 def test_wrong_review_binding_rejected(self):
  b=self.binding();r=self.review(b);r['inputs_sha256']='1'*64
  with patch.object(a,'read_pin',return_value=json.dumps(r).encode()):
   with self.assertRaisesRegex(ValueError,'review joins'):a.admission(b)
 def test_expired_admission_rejected(self):
  b=self.binding();b['inputs']['not_after_utc']='2000-01-01T00:00:00Z'
  with self.assertRaisesRegex(ValueError,'deadline'):a.admission(b)
 def test_one_use_marker(self):
  with tempfile.TemporaryDirectory() as d:
   m=Path(d)/'attempt';a.claim_once(m,{'first':True})
   with self.assertRaises(FileExistsError):a.claim_once(m,{'second':True})
   self.assertEqual(json.loads(m.read_text()),{'first':True})
if __name__=='__main__':unittest.main()
