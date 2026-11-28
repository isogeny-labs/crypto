import importlib.util,json,tempfile,unittest
from pathlib import Path
from unittest.mock import patch,Mock
HERE=Path(__file__).parent
spec=importlib.util.spec_from_file_location('draft',HERE/'driver.proposed.py');d=importlib.util.module_from_spec(spec);spec.loader.exec_module(d)
class Guards(unittest.TestCase):
 def test_instrument_binding_rejects_conflicts(self):
  p={'arms':[{'id':'a','role':'candidate','instrument':{'commit':'c','binary_sha256':'b'}}]};s={'source_commit':'c','binary_sha256':'b'}
  for mutation in ['ok','role','commit','prepared','duplicate']:
   v={'arms':[{'id':'a','role':'candidate','commit':None,'binary_sha256':None}]};state=dict(s)
   if mutation=='role':v['arms'][0]['role']='baseline'
   if mutation=='commit':v['arms'][0]['commit']='wrong'
   if mutation=='prepared':state['binary_sha256']='wrong'
   if mutation=='duplicate':v['arms'].append(dict(v['arms'][0]))
   if mutation=='ok':d.bind_export_instruments(v,p,state);self.assertEqual(v['arms'][0]['commit'],'c')
   else:
    with self.assertRaises(RuntimeError):d.bind_export_instruments(v,p,state)
 def test_inventory_uses_actual_argv_not_rows_or_labels(self):
  with tempfile.TemporaryDirectory() as t:
   f=Path(t);(f/'commands').mkdir()
   def add(name,args):
    x=f/'commands'/name;x.mkdir();d.save(x/'argv.json',args)
   add('001-dataset-label',['git','status']);self.assertTrue(d.dataset_command_inventory(f,{})['no_dataset_commands'])
   add('002-negative',['/binary','dataset','verify']);r=d.dataset_command_inventory(f,{});self.assertFalse(r['no_dataset_commands']);self.assertEqual(r['dataset_command_count'],1)
   (f/'commands'/'003-incomplete').mkdir();r=d.dataset_command_inventory(f,{});self.assertIsNone(r['no_dataset_commands']);self.assertFalse(r['command_inventory_complete'])
 def test_package_orders_binding_before_validation_and_never_rewrites(self):
  for rejected in [False,True]:
   with tempfile.TemporaryDirectory() as t:
    f=Path(t);protocol={'arms':[{'id':'a','role':'candidate','instrument':{'commit':'c','binary_sha256':'b'}}],'analysis_plan':{'script':{'sha256':'script'}}}
    d.save(f/'protocol.json',protocol);state={'source_commit':'c','binary_sha256':'b','protocol_file_sha256':d.sha(f/'protocol.json'),'driver_sha256':d.sha(d.__file__)};d.save(f/'prepared.json',state)
    tool=Mock();tool.from_rows.return_value={'arms':[{'id':'a','role':'candidate','commit':None,'binary_sha256':None}],'runs':[]};tool.schema_errors.return_value=[]
    def bind(_f,value):value['analysis']={'script':{'sha256':'script'}};return value
    def lint(value,*args):
     self.assertEqual(value['arms'][0]['commit'],'c');self.assertEqual(value['analysis']['script']['sha256'],'script');self.assertFalse((f/'experiment.export.json').exists());return ['injected finding'] if rejected else []
    tool.lint_export.side_effect=lint
    with patch.object(d,'pinned_export_checker',return_value=tool),patch.object(d,'bind_success_case_metadata',side_effect=bind),patch.object(d.subprocess,'Popen',side_effect=AssertionError('no child')):
     if rejected:
      with self.assertRaises(RuntimeError):d.package_success_export(f)
      self.assertFalse((f/'experiment.export.json').exists())
     else:
      self.assertTrue(d.package_success_export(f)['accepted']);before=d.sha(f/'experiment.export.json')
      with self.assertRaises(RuntimeError):d.package_success_export(f)
      self.assertEqual(before,d.sha(f/'experiment.export.json'))
    tool.from_rows.assert_called_once();tool.lint_export.assert_called_once()
 def test_failed_attempt_refused_before_converter(self):
  with tempfile.TemporaryDirectory() as t:
   f=Path(t);d.save(f/'execution-failed.json',{'no_retry':True})
   with patch.object(d,'pinned_export_checker',side_effect=AssertionError('must not run')):
    with self.assertRaises(RuntimeError):d.package_success_export(f)
 def test_official_converter_metadata_contract_on_synthetic_rows(self):
  # Protocol metadata only, no recorded dataset rows or results consumed.
  original=Path('/var/tmp/ml-dataset-cli46-032469-v3-20261101')
  state=d.load(original/'prepared.json');tool=d.pinned_export_checker(state);protocol=d.load(original/'protocol.json')
  with tempfile.TemporaryDirectory() as t:
   f=Path(t);d.save(f/'protocol.json',protocol)
   rows=[{'arm':arm['id'],'instance_id':str(i),'run_id':arm['id']+'-'+str(i),'seed':42,'replicate':0,'warmup':False,'status':'succeeded','verification':'verified','cell_contract_pass':1,'pipeline_contract_pass':1} for arm in protocol['arms'] for i in range(46)]
   (f/'rows.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in rows))
   value=tool.from_rows(f/'rows.jsonl',protocol,'protocol.json','synthetic-packaging-unit',{})
   self.assertEqual(len(tool.lint_export(value,protocol,f,False)),5)
   before=json.dumps(value['runs'],sort_keys=True);d.bind_export_instruments(value,protocol,state)
   (f/'analysis.py').write_bytes((original/'retained-analysis.py').read_bytes())
   value['analysis']['script']=d.diagnostic_file(f,f/'analysis.py','analysis')
   self.assertEqual(before,json.dumps(value['runs'],sort_keys=True));self.assertEqual(tool.schema_errors(value,'experiment.export/v1'),[]);self.assertEqual(tool.lint_export(value,protocol,f,True),[])
if __name__=='__main__':unittest.main(verbosity=2)
