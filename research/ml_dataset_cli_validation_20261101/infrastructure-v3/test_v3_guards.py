import importlib.util,json,tempfile,unittest,types,copy,inspect
from pathlib import Path
from unittest.mock import patch
ROOT=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('v3',ROOT/'driver.v3.py');d=importlib.util.module_from_spec(spec);spec.loader.exec_module(d)
class Guards(unittest.TestCase):
 def test_exact_status_allowance_only(self):
  with tempfile.TemporaryDirectory() as t:
   out=Path(t);(out/'stdout').write_bytes(b'')
   runner=d.Runner.__new__(d.Runner)
   with patch.object(runner,'command',return_value=out)as c:
    for name,args,want in [('source-clean',('status','--porcelain=v1','--untracked-files=all'),120),('other',('status','--porcelain=v1','--untracked-files=all'),45),('source-clean',('status',),45),('source-head',('rev-parse','HEAD'),45)]:
     runner.git(name,'/read-only-placeholder',*args);self.assertEqual(c.call_args.kwargs['timeout'],want)
   self.assertEqual(inspect.signature(d.Runner.command).parameters['timeout'].default,45)
 def test_bound_guard_passes_readonly(self):
  body=json.loads((ROOT/'protocol.v3.draft.json').read_text());b=body['extensions']['software_binding'];args=types.SimpleNamespace(protocol_draft=ROOT/'protocol.v3.draft.json',source_commit=b['source_commit'],binary_sha256=b['binary_sha256'],binary=b['binary_path'],fixture=body['study_dir'],gates=ROOT.parent/'cli-gates-032469-v2/gates.json')
  with patch.object(Path,'mkdir',side_effect=AssertionError('mkdir forbidden')),patch.object(d,'Runner',side_effect=AssertionError('Runner forbidden')):
   actual,prior=d.amendment_binding_guard(args);self.assertEqual(actual,body);self.assertTrue(prior)
 def test_wrong_version_allowance_and_manifest_rejected(self):
  body=json.loads((ROOT/'protocol.v3.draft.json').read_text());b=body['extensions']['software_binding']
  for mode in ['version','allowance','manifest','binary','destination']:
   with self.subTest(mode=mode),tempfile.TemporaryDirectory()as t:
    changed=copy.deepcopy(body)
    if mode=='version':changed['version']=2
    if mode=='allowance':changed['extensions']['infrastructure_amendment']['source_clean_timeout_seconds']=121
    if mode=='manifest':changed['extensions']['infrastructure_amendment']['prior_v2_sha256']='0'*64
    if mode=='binary':changed['extensions']['software_binding']['binary_sha256']='0'*64
    if mode=='destination':changed['study_dir']='/var/tmp/wrong'
    p=Path(t)/'draft.json';p.write_text(json.dumps(changed))
    args=types.SimpleNamespace(protocol_draft=p,source_commit=b['source_commit'],binary_sha256=b['binary_sha256'],binary=b['binary_path'],fixture=body['study_dir'],gates=ROOT.parent/'cli-gates-032469-v2/gates.json')
    with patch.object(Path,'mkdir',side_effect=AssertionError('mkdir forbidden')),patch.object(d,'Runner',side_effect=AssertionError('Runner forbidden')):
     with self.assertRaises(RuntimeError):d.amendment_binding_guard(args)
 def test_generation_and_analysis_functions_unchanged(self):
  import ast
  old=ast.parse((ROOT.parent/'cli46-prime-repair-binding-plan/driver.explicit-toolchain.py').read_text());new=ast.parse((ROOT/'driver.v3.py').read_text())
  af={n.name:ast.dump(n,include_attributes=False)for n in old.body if isinstance(n,(ast.FunctionDef,ast.ClassDef))};bf={n.name:ast.dump(n,include_attributes=False)for n in new.body if isinstance(n,(ast.FunctionDef,ast.ClassDef))}
  self.assertEqual(set(af),set(bf));self.assertEqual({k for k in af if af[k]!=bf[k]},{'Runner','amendment_binding_guard','prepare'})
  oa={n.name:ast.dump(n,include_attributes=False)for n in next(n for n in old.body if isinstance(n,ast.ClassDef)and n.name=='Runner').body if isinstance(n,ast.FunctionDef)};na={n.name:ast.dump(n,include_attributes=False)for n in next(n for n in new.body if isinstance(n,ast.ClassDef)and n.name=='Runner').body if isinstance(n,ast.FunctionDef)}
  self.assertEqual({k for k in oa if oa[k]!=na[k]},{'git'})
if __name__=='__main__':unittest.main()
