"""Synthetic software fixtures only: no mathematical cohorts or approvals."""
import hashlib
import copy
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
import package as pkg

STUDY = Path(__file__).resolve().parents[1]
TOOL = Path('/home/ec2-user/crypto-autoresearcher-fall-degree-submission-20261029/tools/experiment_protocol.py')


class PackagingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='pencil-export-software-fixture-')
        self.addCleanup(self.temp.cleanup)
        self.d = Path(self.temp.name)
        self.tool = pkg.canonical(self.binding(TOOL))
        p = pkg.load(STUDY/'protocol_draft/accounting_amendment_20261029/protocol.json')
        p['protocol_id'] = 'software-fixture-only'
        p['extensions']['canonical_packaging']={'adapter_sha256':pkg.digest(pkg.__file__),'canonical_tool_sha256':pkg.digest(TOOL)}
        p['sample_size']['instances'] = 1
        p['sample_size']['preflight_runtime']['estimated_total_seconds'] = 1
        p['analysis_plan']['frozen_before_data'] = True
        p['freeze'].update(status='frozen', frozen_at='2026-10-29T00:00:00Z')
        p['freeze']['protocol_sha256'] = self.tool.protocol_hash(p)
        self.p = p
        self.write('protocol.json', p)
        arms = [{'name':a['id'],'method':{'id':a['parameters']['method']}} for a in p['arms']]
        self.plan = {'schema':'ecbench.plan/v1','arms':arms,'workloads':[{'workload_id':'software-W','curve_spec':{'p':739}}], 'executions':[{'seq':i,'arm':i%3,'workload':0,'round':i//3,'warmup':i<3,'algorithm_seed':100+i//3} for i in range(18)]}
        self.write('plan.json',self.plan)
        self.rows=[]
        for ex in self.plan['executions']:
            arm=arms[ex['arm']]
            self.rows.append({'schema':'ecbench.record/v1','record_id':'software-'+str(ex['seq']),'session_id':'software-session','binary_sha256':p['arms'][0]['instrument']['binary_sha256'],'seq':ex['seq'],'round':ex['round'],'warmup':ex['warmup'],'arm':arm['name'],'method':arm['method'],'algorithm_seed':str(ex['algorithm_seed']),'workload':self.plan['workloads'][0],'outcome':{'status':'verified','matches_target':True,'matches_planted':True,'error':None},'isolation':{'level':'L2'},'time':{'method_call_wall_ns':120,'solve_wall_ns':2,'max_rss_kib':None,'parent_audit_verification_ns':3,'online_clock_diagnostics':{'error':None}},'online':{'wall_ns':5,'phases_ns':dict.fromkeys(pkg.PHASES,1)},'counters':{'actual':4}})
        self.write('receipt.json',{'schema':'pencil-prepared-plan-validation/v1','passed':True,'files':{'plan.json':pkg.digest(self.d/'plan.json')}})
        self.write('resources.json',{'schema':'software-resource-fixture/v1','attempts':[{'attempt_id':'interrupted-fixture','status':'interrupted','restored':True}],'not_cohort':True})
        self.refresh()

    def write(self,name,v):
        (self.d/name).write_text(json.dumps(v))

    def binding(self,path):
        return {'path':str(path),'sha256':pkg.digest(path)}

    def refresh(self):
        (self.d/'records.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in self.rows))
        m={'schema':'pencil-price-cohort-input/v1','protocol_sha256':self.p['freeze']['protocol_sha256'],'binary_sha256':self.p['arms'][0]['instrument']['binary_sha256'],'prepared_plan_validation':self.binding(self.d/'receipt.json'),'plans':[dict(self.binding(self.d/'plan.json'),name='plan.json',session_id='software-session',records=self.binding(self.d/'records.jsonl'))],'separate_resources':self.binding(self.d/'resources.json')}
        pins=self.p['extensions']['analysis_support_sha256']
        native={'schema':'pencil-price-cohort-output/v1','input_manifest':m,'expected_executions':18,'observed_executions':len(self.rows),'missing_executions':[{}]*(18-len(self.rows)),'separate_resource_observations':pkg.load(self.d/'resources.json'),'aggregate_source_sha256':pins['native_price/src/aggregate.rs'],'paired_export_source_sha256':pins['paired_export/src/lib.rs'],'paired_export_plan_source_sha256':pins['paired_export/src/plan.rs'],'mapping':{'mapping_sha256':self.p['extensions']['finite_cost_model']['mapping_sha256']},'qualified_ratios':None,'aggregate_cells':[]}
        native['input_manifest_integrity_sha256']=hashlib.sha256(json.dumps(m,sort_keys=True,separators=(',', ':'),ensure_ascii=False).encode()).hexdigest()
        self.write('native.json',native)
        self.write('history.json',{'schema':'pencil-execution-history/v1','protocol_sha256':self.p['freeze']['protocol_sha256'],'binary_sha256':m['binary_sha256'],'records':[{'record_id':r['record_id'],'session_id':r['session_id'],'seq':r['seq'],'attempts':1,'resumed':0,'checkpoint':None,'evidence':[self.binding(self.d/'resources.json')]} for r in self.rows]})
        script=STUDY/'native_price/src/bin/analyze_cohort.rs'
        self.request={'schema':'pencil-canonical-package-input/v1','canonical_tool':self.binding(TOOL),'protocol':self.binding(self.d/'protocol.json'),'native_analysis':self.binding(self.d/'native.json'),'analysis_script':self.binding(script),'export_id':'software-fixture-export','execution_history':self.binding(self.d/'history.json')}

    def run_package(self):
        return pkg.package(self.request,self.d)

    def test_complete_canonical_schema_and_check_export(self):
        e,findings=self.run_package()
        self.assertEqual(findings,[])
        self.assertEqual(len(e['runs']),18)
        self.assertEqual(e['runs'][0]['measurements']['wall_seconds'],120/1e9)
        self.assertIsNone(e['runs'][0]['measurements']['max_rss_bytes'])
        self.assertTrue(all(not r['measurements']['wall_valid'] for r in e['runs']))
        self.assertEqual(sum(r['warmup'] for r in e['runs']),3)
        self.write('export.json',e)
        result=subprocess.run([sys.executable,str(TOOL),'check-export',str(self.d/'export.json'),'--protocol',str(self.d/'protocol.json'),'--verify-files'],capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stdout+result.stderr)

    def test_missing_keeps_shortfall_and_no_fake_attempt(self):
        self.rows.pop();self.refresh()
        e,f=self.run_package()
        self.assertEqual(len(e['runs']),17)
        self.assertEqual(len(e['backend_record']['missing_slots']),1)
        self.assertTrue(any('measured rows' in x for x in f))

    def test_wrong_phase_names_never_qualify(self):
        self.rows[4]['online']['phases_ns']={'arbitrary':5};self.refresh()
        e,_=self.run_package()
        self.assertIn('phase_names_not_exact_five',e['backend_record']['diagnostics'][4]['online_structure_reasons'])
        self.assertFalse(e['runs'][4]['measurements']['wall_valid'])

    def test_phase_sum_and_absent_diagnostics(self):
        self.rows[4]['online']['wall_ns']=6
        del self.rows[4]['time']['online_clock_diagnostics'];self.refresh()
        e,_=self.run_package();reasons=e['backend_record']['diagnostics'][4]['online_structure_reasons']
        self.assertIn('invalid_phase_sum',reasons);self.assertIn('online_diagnostics_absent_or_error',reasons)

    def test_failures_and_resources_retained(self):
        self.rows[4]['outcome']={'status':'timeout'};self.refresh()
        e,_=self.run_package()
        self.assertEqual(e['runs'][4]['status'],'timeout')
        self.assertTrue(e['backend_record']['resource_observations_present'])
        self.assertTrue(any(x['sha256']==pkg.digest(self.d/'resources.json') for x in e['files']))

    def test_record_seed_identity_rejected(self):
        self.rows[4]['algorithm_seed']='999';self.refresh()
        with self.assertRaisesRegex(ValueError,'exact plan identity'):self.run_package()

    def test_record_binary_identity_rejected(self):
        self.rows[4]['binary_sha256']='0'*64;self.refresh()
        with self.assertRaisesRegex(ValueError,'record identity'):self.run_package()

    def test_raw_hash_mismatch_rejected(self):
        (self.d/'records.jsonl').write_text('changed')
        with self.assertRaisesRegex(ValueError,'hash mismatch'):self.run_package()

    def test_changed_attached_file_detected_by_canonical_checker(self):
        e,_=self.run_package();(self.d/'resources.json').write_text('{}')
        self.assertTrue(any('file changed' in x for x in self.tool.lint_export(e,self.p,self.d,True)))

    def test_duplicate_record_rejected(self):
        self.rows[-1]=copy.deepcopy(self.rows[0]);self.refresh()
        with self.assertRaisesRegex(ValueError,'duplicate'):self.run_package()

    def test_draft_not_exportable(self):
        self.p['freeze']['status']='draft';self.write('protocol.json',self.p);self.refresh()
        with self.assertRaisesRegex(ValueError,'frozen protocol'):self.run_package()

    def test_native_manifest_hash_mismatch_rejected(self):
        n=pkg.load(self.d/'native.json');n['input_manifest_integrity_sha256']='0'*64;self.write('native.json',n);self.request['native_analysis']=self.binding(self.d/'native.json')
        with self.assertRaisesRegex(ValueError,'manifest integrity'):self.run_package()

    def test_entire_missing_workload_is_not_fabricated(self):
        self.rows=[];self.refresh();e,f=self.run_package()
        self.assertEqual(e['runs'],[])
        self.assertEqual(len(e['backend_record']['missing_slots']),18)
        self.assertTrue(f)

    def test_unknown_history_refused_without_export(self):
        h=pkg.load(self.d/'history.json');h['records'][0]['resumed']=None;self.write('history.json',h);self.request['execution_history']=self.binding(self.d/'history.json')
        with self.assertRaisesRegex(ValueError,'explicit known'):self.run_package()

    def test_missing_history_record_refused(self):
        h=pkg.load(self.d/'history.json');h['records'].pop();self.write('history.json',h);self.request['execution_history']=self.binding(self.d/'history.json')
        with self.assertRaisesRegex(ValueError,'missing or mismatched'):self.run_package()

    def test_known_retry_restore_counts_preserved(self):
        h=pkg.load(self.d/'history.json');h['records'][4].update(attempts=3,resumed=1,checkpoint={'policy':'criu','generations_written':2,'restored_from_generation':1});self.write('history.json',h);self.request['execution_history']=self.binding(self.d/'history.json')
        e,f=self.run_package();self.assertEqual(f,[]);self.assertEqual(e['runs'][4]['attempts'],3);self.assertEqual(e['runs'][4]['resumed'],1);self.assertFalse(e['runs'][4]['measurements']['wall_valid'])

    def test_native_mapping_mismatch_rejected(self):
        n=pkg.load(self.d/'native.json');n['mapping']['mapping_sha256']='0'*64;self.write('native.json',n);self.request['native_analysis']=self.binding(self.d/'native.json')
        with self.assertRaisesRegex(ValueError,'mapping identity'):self.run_package()


if __name__=='__main__':unittest.main()
