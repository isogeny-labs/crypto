"""Pure analyzer controls. Synthetic receipts are not native SAT evidence."""
import copy,importlib.util,json,unittest
from pathlib import Path
ROOT=Path(__file__).parent
spec=importlib.util.spec_from_file_location('acceptance_driver',ROOT/'driver.py');d=importlib.util.module_from_spec(spec);spec.loader.exec_module(d)
class AnalyzerControls(unittest.TestCase):
    def baseline(self,name='verify'):
        p={'run_sha256':'a'*64,'work_per_cell':10**12,'bytes_per_cell':8*10**9,'aggregate_work':2000001000000,'aggregate_bytes':16001000000}
        reads=2+(name!='custody-restore')
        r={'schema':'crypto.ml-dataset.sat-replay-invocation/v1','run_sha256':'a'*64,'verification_returned_ok':True,'solver_completed':False,'training_export_admitted':False,'slots':[{'cell_index':i,'consumed':True,'failed':False,'verdict':'checked_initial','visits':1} for i in range(2)],'reserved_work':2*p['work_per_cell']+65536+8226*18,'reserved_bytes':2*p['bytes_per_cell']+65536+8192*18+2*8192,'caller_charged_work':65536+8226*reads,'caller_charged_bytes':8192*reads}
        return p,r
    def check(self,p,r,name='verify'):return d.check_replay_receipt(name,r,p,'a'*64)
    def test_synthetic_positive_arithmetic_for_generate_verify_restore(self):
        for name in ['generate','verify','custody-restore']:
            p,r=self.baseline(name);self.assertEqual(self.check(p,r,name),8192)
    def test_duplicate_or_boolean_slot_index_rejected(self):
        for bad in [0,True]:
            p,r=self.baseline();r['slots'][1]['cell_index']=bad
            with self.assertRaises(RuntimeError):self.check(p,r)
    def test_zero_or_wrong_charges_and_reservations_rejected(self):
        for field in ['caller_charged_work','caller_charged_bytes','reserved_work','reserved_bytes']:
            p,r=self.baseline();r[field]=0
            with self.assertRaises(RuntimeError):self.check(p,r)
        p,r=self.baseline();r['caller_charged_work']+=8226
        with self.assertRaises(RuntimeError):self.check(p,r)
    def test_schema_visit_and_overclaim_mutations_rejected(self):
        for field,value in [('schema','wrong'),('solver_completed',True),('training_export_admitted',True),('verification_returned_ok',1)]:
            p,r=self.baseline();r[field]=value
            with self.assertRaises(RuntimeError):self.check(p,r)
        for visits in [0,9,True]:
            p,r=self.baseline();r['slots'][0]['visits']=visits
            with self.assertRaises(RuntimeError):self.check(p,r)
    def cell(self):
        schedule=d.load(ROOT/'schedule.json');channels=d.load(ROOT/'channels.json');cfg={k:v for k,v in schedule[0].items() if k!='index'};cfg['channels']=channels
        return schedule,channels,{'cell_index':0,'config':cfg}
    def test_actual_full_synthetic_schedule_and_swapped_twist_index(self):
        s,c,m=self.cell();d.check_cell_schedule(m,s[0],c)
        m['config']['twist']=True
        with self.assertRaises(RuntimeError):d.check_cell_schedule(m,s[0],c)
        m['cell_index']=1
        with self.assertRaises(RuntimeError):d.check_cell_schedule(m,s[0],c)
    def test_modified_workload_fields_rejected(self):
        for key,value in [('seed',43),('curve_seed',1),('generator_index',1),('key_index',1),('degree_bound',4),('work_limit',65536),('factor_base_policy',{'kind':'sampled_control'}),('factor_base_size',8)]:
            s,c,m=self.cell();m['config'][key]=value
            with self.assertRaises(RuntimeError):d.check_cell_schedule(m,s[0],c)
if __name__=='__main__':unittest.main()
