"""Synthetic software contract tests only; no native solver or dataset execution."""
import copy, importlib.util, json, tempfile, unittest
from pathlib import Path
spec=importlib.util.spec_from_file_location('adapter',Path(__file__).with_name('export_actual.py'));a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
spec=importlib.util.spec_from_file_location('canonical',a.TOOL);tool=importlib.util.module_from_spec(spec);spec.loader.exec_module(tool)
class Contract(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='sat-export-contract-');self.path=Path(self.tmp.name)/'manifest.json'
        self.m={'schema':'crypto.ml-dataset.cell/v1','config':{'twist':False},'outcome':{'status':'partial','initial_checked':True,'solver_completed':False,'scalar_recovered_without_witness':False,'patterns_checked':1,'patterns_total':2,'supervisor':{'exit_code':0}}}
        self.write()
        self.analysis={'schema':'local-sat-cli-custody-analysis/v1','status':'PASS','initial_prefix_checked_cells':2,'solver_completed_cells':0,'sat_training_admitted':False,'cells':[{'cell_index':0,'twist':False,'manifest_sha256':a.sha(self.path),'actual_outcome':copy.deepcopy(self.m['outcome'])},{'cell_index':1,'twist':True,'manifest_sha256':'0'*64,'actual_outcome':{}}]}
    def tearDown(self):self.tmp.cleanup()
    def write(self):self.path.write_text(json.dumps(self.m))
    def check(self):a.check_pass_cell(self.m,self.path,0,self.analysis)
    def test_exact_synthetic_contract(self):self.check()
    def test_digest_binding(self):
        self.path.write_text(json.dumps(self.m,indent=2))
        with self.assertRaisesRegex(RuntimeError,'digest'):self.check()
    def test_exact_outcome_fields_even_rehashed(self):
        original=copy.deepcopy(self.m)
        for field,value in [('status','complete'),('initial_checked',False),('solver_completed',True),('scalar_recovered_without_witness',True),('patterns_checked',2),('patterns_total',3),('patterns_checked',True)]:
            with self.subTest(field=field,value=value):
                self.m=copy.deepcopy(original);self.m['outcome'][field]=value;self.write();self.analysis['cells'][0]['manifest_sha256']=a.sha(self.path);self.analysis['cells'][0]['actual_outcome']=copy.deepcopy(self.m['outcome'])
                with self.assertRaises(RuntimeError):self.check()
    def test_analysis_joins(self):
        original=copy.deepcopy(self.analysis)
        for mutation in ['outcome','twist','duplicate','aggregate']:
            self.analysis=copy.deepcopy(original)
            if mutation=='outcome':self.analysis['cells'][0]['actual_outcome']['patterns_checked']=2
            elif mutation=='twist':self.analysis['cells'][0]['twist']=True
            elif mutation=='duplicate':self.analysis['cells'][1]['cell_index']=0
            else:self.analysis['initial_prefix_checked_cells']=1
            with self.subTest(mutation=mutation),self.assertRaises(RuntimeError):self.check()
    def test_censoring_and_real_canonical_row_schema(self):
        schema=json.loads((a.TOOL.parent.parent/'schemas/experiment-export.v1.json').read_text())
        row_schema={'$schema':schema['$schema'],'$defs':schema['$defs'],**schema['properties']['runs']['items']}
        import jsonschema
        validator=jsonschema.Draft202012Validator(row_schema)
        for status,exit_code,want in [('partial',0,'succeeded'),('timeout',0,'timeout'),('trace_budget_exhausted',0,'cancelled'),('worker_failed',1,'failed'),('worker_failed',None,'infra_error')]:
            with self.subTest(status=status):
                self.m['outcome']['status']=status;self.m['outcome']['supervisor']['exit_code']=exit_code
                actual=a.execution_status(self.m['outcome']);self.assertEqual(actual,want)
                row=tool.run_row(run_id='synthetic-contract',arm='sat-plain',instance_id='0',seed=42,status=actual,attempts=1)
                self.assertEqual(list(validator.iter_errors(row)),[])
                self.assertEqual(row['outcome_class'],'completed' if want in ['succeeded','failed'] else 'not_completed')
                if want!='succeeded':
                    with self.assertRaises(RuntimeError):self.check()
if __name__=='__main__':unittest.main(verbosity=2)
