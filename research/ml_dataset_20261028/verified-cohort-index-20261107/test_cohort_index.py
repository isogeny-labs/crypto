import copy,json,tempfile,unittest
from pathlib import Path
import cohort_index as c
class VerifyTests(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup);self.root=Path(self.tmp.name)
  self.rows={'features.jsonl':[{'schema':'ml-dataset.features/v1','feature_id':'f','split_component':'field-size-n23','split':'train','inputs':{'n':23}}], 'restricted/targets.jsonl':[{'target_id':'t','schema':'ml-dataset.restricted-targets/v1','feature_id':'f','split_component':'field-size-n23','split':'train','labels':{'status':'censored'},'known_answer_scalar':'SECRET'}], 'restricted/index.jsonl':[{'record_id':'t','schema':'ml-dataset.export-index/v1','target_id':'t','feature_id':'f','source_run_sha256':'run','status':'verified_sealed_cell'}]}
 def prepare(self):
  self.plan={'schema':'crypto.ml-dataset.plan/v1','field_sizes':[23],'arities':[3],'families':['binary'],'solvers':['enumerate'],'curve_seeds':1,'generators':1,'keys':1,'seed':42,'factor_base_size':4,'degree_bound':3,'work_limit':100,'twists':True,'protocol_sha256':None,'split_rule':'curve-twist-component-sha256-v1'}
  protocol=self.root/'protocol.json';protocol.write_text('{"schema":"test-protocol"}')
  self.plan['protocol_sha256']=c.digest(protocol.read_bytes());plan=self.root/'plan.json';plan.write_text(json.dumps(self.plan,sort_keys=True,indent=2))
  self.run={'source_commit':'commit','binary_sha256':'binary','plan_sha256':c.typed_plan_binding(self.plan)}
  run=self.root/'run.json';run.write_text(json.dumps(self.run));self.run_hash=c.digest(run.read_bytes())
  self.refs={n:{'path':str(self.root/(n+'.json')),'sha256':c.digest((self.root/(n+'.json')).read_bytes())} for n in ['run','plan','protocol']}
  for r in self.rows['restricted/index.jsonl']:r['source_run_sha256']=self.run_hash
  arts=[]
  for name,rows in self.rows.items():
   p=self.root/name;p.parent.mkdir(parents=True,exist_ok=True);data=(''.join(json.dumps(r)+'\n' for r in rows)).encode();p.write_bytes(data);arts.append({'path':name,'bytes':len(data),'sha256':c.digest(data),'encoding':'jsonl'})
  self.m={'schema':'ml-dataset.training-export/v1','source_run_sha256':self.run_hash,'source_commit':'commit','binary_sha256':'binary','plan_sha256':self.run['plan_sha256'],'split_policy':{'domain':'ml-dataset-whole-field-size-split-v1','row_random_split_permitted':False},'artifacts':arts};return self.manifest()
 def manifest(self):
  p=self.root/'manifest.json';p.write_text(json.dumps(self.m));return {'manifest':str(p),'manifest_sha256':c.digest(p.read_bytes()),'source_bindings':copy.deepcopy(self.refs)}
 def test_valid_no_restricted_values(self):
  entry=self.prepare();result=c.build({'schema':'ml-dataset.cohort-index-input/v1','cohorts':[{'cohort_id':'one','exports':[entry]}]});self.assertNotIn('SECRET',json.dumps(result));self.assertFalse(result['cross_population_pooling_permitted'])
 def test_digest(self):
  e=self.prepare();(self.root/'features.jsonl').write_text('{}');self.assertRaisesRegex(ValueError,'digest',c.verify_export,e,{})
 def test_manifest_digest(self):
  e=self.prepare();e['manifest_sha256']='bad';self.assertRaisesRegex(ValueError,'manifest digest',c.verify_export,e,{})
 def test_path(self):
  self.prepare();self.m['artifacts'][0]['path']='../escape';self.assertRaisesRegex(ValueError,'unsafe artifact',c.verify_export,self.manifest(),{})
 def test_symlink(self):
  e=self.prepare();p=self.root/'features.jsonl';p.rename(self.root/'real');p.symlink_to(self.root/'real');self.assertRaisesRegex(ValueError,'symlink',c.verify_export,e,{})
 def test_duplicate_id(self):
  self.rows['features.jsonl']*=2;self.assertRaisesRegex(ValueError,'duplicate row ID',c.verify_export,self.prepare(),{})
 def test_dangling_join(self):
  self.rows['restricted/targets.jsonl'][0]['feature_id']='missing';self.assertRaisesRegex(ValueError,'dangling',c.verify_export,self.prepare(),{})
 def test_missing_index(self):
  self.rows['restricted/index.jsonl']=[];self.assertRaisesRegex(ValueError,'missing index',c.verify_export,self.prepare(),{})
 def test_split_conflict(self):
  e=self.prepare();self.assertRaisesRegex(ValueError,'cross-cohort split conflict',c.verify_export,e,{'field-size-n23':'test'})
 def test_wrong_deterministic_split(self):
  self.rows['features.jsonl'][0]['split']='test';self.assertRaisesRegex(ValueError,'deterministic',c.verify_export,self.prepare(),{})
 def test_json_duplicate_and_nonfinite(self):
  for raw in ['{"a":1,"a":2}','{"a":NaN}','{"a":Infinity}','{"a":1e999}']:
   with self.subTest(raw=raw),self.assertRaises(ValueError):c.parse(raw)
 def test_rehashed_duplicate_json_rejected(self):
  self.prepare();a=self.m['artifacts'][0];data=b'{"feature_id":"f","feature_id":"g"}\n';(self.root/a['path']).write_bytes(data);a.update(bytes=len(data),sha256=c.digest(data));self.assertRaisesRegex(ValueError,'duplicate JSON',c.verify_export,self.manifest(),{})
 def test_auxiliary_dangling_target(self):
  self.rows['ecdlp-features.jsonl']=[]
  self.rows['restricted/ecdlp-records.jsonl']=[{'schema':'ml-dataset.restricted-ecdlp-record/v1','record_id':'e','source_target_id':'missing','ecdlp_feature_id':None}]
  self.assertRaisesRegex(ValueError,'dangling channel target',c.verify_export,self.prepare(),{})
 def test_manifest_count(self):
  self.prepare();self.m['restricted_target_rows']=2
  self.assertRaisesRegex(ValueError,'row count',c.verify_export,self.manifest(),{})
 def test_protocol_binding_mismatch(self):
  e=self.prepare();p=self.root/'protocol.json';p.write_text('{"schema":"changed-protocol"}')
  e['source_bindings']['protocol']['sha256']=c.digest(p.read_bytes())
  self.assertRaisesRegex(ValueError,'protocol digest binding',c.verify_export,e,{})
 def test_source_bindings_required(self):
  e=self.prepare();del e['source_bindings'];self.assertRaisesRegex(ValueError,'source_bindings required',c.verify_export,e,{})
 def test_updated_plan_hash_does_not_authorize_changed_arity_or_seed(self):
  for field,value in [('arities',[4]),('seed',43)]:
   with self.subTest(field=field):
    e=self.prepare();changed=copy.deepcopy(self.plan);changed[field]=value;p=self.root/'plan.json';p.write_text(json.dumps(changed));e['source_bindings']['plan']['sha256']=c.digest(p.read_bytes())
    self.assertRaisesRegex(ValueError,'typed plan binding mismatch',c.verify_export,e,{})
 def test_typed_order_and_optional_default(self):
  self.prepare();self.assertEqual(c.typed_plan_binding(self.plan),c.typed_plan_binding(dict(reversed(list(self.plan.items())))))
  absent=copy.deepcopy(self.plan);del absent['protocol_sha256'];null=copy.deepcopy(absent);null['protocol_sha256']=None
  self.assertEqual(c.typed_plan_binding(absent),c.typed_plan_binding(null))
  self.assertNotEqual(c.typed_plan_binding(self.plan),c.digest(json.dumps(self.plan,sort_keys=True).encode()))
 def test_unsupported_plan_rejected(self):
  self.prepare();p=copy.deepcopy(self.plan);p['channels']=None
  self.assertRaisesRegex(ValueError,'unsupported typed plan fields',c.typed_plan_binding,p)
  p=copy.deepcopy(self.plan);p['seed']=True
  self.assertRaisesRegex(ValueError,'invalid typed plan integer',c.typed_plan_binding,p)
 def test_rehashed_core_size_mutation(self):
  self.rows['features.jsonl'][0]['inputs']['n']=24
  self.assertRaisesRegex(ValueError,'field size split mismatch',c.verify_export,self.prepare(),{})
 def test_core_public_split_required(self):
  self.rows['features.jsonl'][0].update(split=None,split_component=None)
  self.assertRaisesRegex(ValueError,'public feature split required',c.verify_export,self.prepare(),{})
 def test_core_missing_size(self):
  del self.rows['features.jsonl'][0]['inputs']['n']
  self.assertRaisesRegex(ValueError,'field size required',c.verify_export,self.prepare(),{})
 def test_unavailable_target_can_keep_null_split(self):
  self.rows['restricted/targets.jsonl'][0].update(feature_id=None,split=None,split_component=None)
  self.rows['restricted/index.jsonl'][0]['feature_id']=None
  c.verify_export(self.prepare(),{})
 def test_auxiliary_direct_size_mutation(self):
  self.rows['prefix-features.jsonl']=[{'schema':'ml-dataset.prefix-features/v1','feature_id':'p','inputs':{'n':24},'split_component':'field-size-n23','split':'train'}]
  self.rows['restricted/prefix-records.jsonl']=[]
  self.assertRaisesRegex(ValueError,'field size split mismatch',c.verify_export,self.prepare(),{})
if __name__=='__main__':unittest.main()
