"""Constructed SOFTWARE UNIT fixtures only; official checker runs, native code never does."""
import copy
import json
from pathlib import Path
import subprocess
import types
import workflow
import tempfile
import unittest
from unittest.mock import patch
import adapter
from protocol_population import amend

HERE=Path(__file__).resolve().parent
TOOLING=Path('/home/ec2-user/ml-dataset-work-20261028/protocol-tooling-f755085ff951')
PYTHON=Path('/home/ec2-user/ml-dataset-work-20261028/protocol-checker-venv/bin/python')
# Exercise the integrated draft builder; it applies the workflow amendment once.
import prepare

class AdapterChecks(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(dir=HERE); self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name); self.attempt=self.root/'attempt'; self.attempt.mkdir()
        (self.attempt/'frozen-crypto').write_bytes(b'SOFTWARE UNIT: inert binary bytes, never executed\n')
        self.api=adapter.official(TOOLING)
        with patch.object(prepare,'file_bindings',return_value={}):
            self.protocol=prepare.prepare('1'*40,adapter.sha(self.attempt/'frozen-crypto'))
        p=self.protocol; p['protocol_id']='SOFTWARE-UNIT-SAT-EXPORT'; p['question']='SOFTWARE UNIT fixture; constructed records only, no native observations or research evidence.'
        p['analysis_plan']['script']=None
        p['sample_size']['preflight_runtime']={'fixed_seconds':0,'per_sample_seconds':0,'threads':1,'estimated_total_seconds':0,'measured_on':'SOFTWARE UNIT fabricated shape values; not a runtime observation'}
        p['freeze']={'status':'frozen','protocol_sha256':self.api.protocol_hash(p),'commit':'1'*40,'frozen_at':'2026-10-30T00:00:00Z','filed_before_affected_data':True}
        adapter.write(self.attempt/'protocol.json',p)
        adapter.write(self.attempt/'host.json',{'software_unit_fixture':True})
        self.receipt={'software_unit_fixture':True,'protocol':{'file_sha256':adapter.sha(self.attempt/'protocol.json'),'body_sha256':p['freeze']['protocol_sha256']},
          'retained_protocol':{'frozen_files':{'protocol.json':adapter.sha(self.attempt/'protocol.json'),'frozen-crypto':adapter.sha(self.attempt/'frozen-crypto')}},
          'native_preflight':{'source_commit':'1'*40,'binary_sha256':adapter.sha(self.attempt/'frozen-crypto'),'source_dirty':False},
          'source_inventories_unchanged':{'0':True,'1':True},'status':'native_fixture_passed_unified_export_pending','selected_pair':[0,1],'transition_witness':{'software_unit_fixture':True},
          'candidates':[{'cell_index':0,'status':'checked','verified_decisions':2**53+1},{'cell_index':1,'status':'checked','verified_decisions':3}],
          'unrun_cells':list(range(2,16)),'unrun_reason':'not_run_after_first_qualified_pair',
          'controls':[{'name':n,'passed':True} for n in ('map-substitution','forged-reason','unpaired-decision','dynamic-clause')],
          'commands':[{'name':'generate-00','status':'completed','exit_code':0,'child_user_cpu_seconds':0.1,'child_system_cpu_seconds':0.01},
                      {'name':'control-forged-reason','status':'nonzero_exit','exit_code':1,'expected_failure':True}],
          'workflows':[{'arm':a,'invoked':True,'started_utc':'2026-10-30T00:00:01Z','finished_utc':'2026-10-30T00:00:02Z','status':'succeeded','reason':'SOFTWARE UNIT synthetic success','command_names':[n]}
                       for a,n in zip(adapter.ARMS,('generate-00','control-forged-reason'))]}
    def save(self):
        adapter.write(self.attempt/'receipt.json',self.receipt)
    def run_gate(self,name='export'):
        self.save(); out=self.root/name
        return adapter.export_attempt(self.attempt,out,TOOLING,PYTHON),out
    def test_real_official_schema_files_flow_pass_constructed_fixture(self):
        gate,out=self.run_gate()
        self.assertEqual(gate['status'],'passed',gate)
        e=adapter.load(out/'export.json')
        self.assertEqual(e['flow'],{'planned':2,'attempted':2,'completed':2,'not_completed':0,'interrupted':0,'resumed':0,'excluded':[]})
        self.assertEqual(e['runs'][0]['measurements']['counted']['verified-decisions'],str(2**53+4))
        self.assertEqual(e['backend_record']['receipt'],self.receipt)
        self.assertEqual(e['backend_record']['cell_flow']['unrun'],list(range(2,16)))
        self.assertTrue(all(r['measurements']['wall_valid'] is False for r in e['runs']))
        self.assertIn('SOFTWARE UNIT FIXTURE',e['notes'][0]['text'])
    def test_missing_fixture_controls_invoked_but_zero_native_calls_is_honest(self):
        self.receipt.update(status='missing_fixture',selected_pair=None,transition_witness=None,controls=[])
        self.receipt['commands']=self.receipt['commands'][:1]
        for w in self.receipt['workflows']: w.update(status='failed',reason='missing_fixture' if w['arm']=='native-sat' else 'missing_native_precondition')
        self.receipt['workflows'][1]['command_names']=[]
        gate,out=self.run_gate()
        self.assertEqual(gate['status'],'passed',gate); self.assertFalse(gate['fixture_acceptance'])
        e=adapter.load(out/'export.json'); self.assertEqual(e['runs'][1]['measurements']['counted']['cli-command-receipts'],0)
        self.assertEqual(e['runs'][1]['verification']['status'],'not_run')
        self.assertEqual(e['flow']['completed'],2) # official vocabulary: terminal failed != succeeded
    def test_admission_failure_does_not_invent_workflows_or_export(self):
        self.receipt['workflows']=[]
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertFalse((out/'export.json').exists())
        self.assertEqual(adapter.load(self.attempt/'receipt.json'),self.receipt)
    def test_partition_failure_rejected(self):
        self.receipt['workflows'][1]['command_names']+=['generate-00']
        gate,_=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertIn('partition',gate['error'])
    def test_omitted_unrun_cell_rejected(self):
        self.receipt['unrun_cells'].pop()
        gate,_=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertIn('sixteen',gate['error'])
    def test_false_control_success_rejected(self):
        self.receipt['controls'].pop()
        gate,_=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertIn('four',gate['error'])
    def test_interrupted_workflow_retained_with_invalid_clock(self):
        self.receipt['workflows'][0].update(status='interrupted',reason='unit interrupt')
        self.receipt['workflows'][1].update(status='failed',reason='missing_native_precondition',command_names=[])
        self.receipt['commands']=self.receipt['commands'][:1]; self.receipt['commands'][0].update(status='interrupted',exit_code=None)
        self.receipt.update(selected_pair=None,transition_witness=None,controls=[])
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'passed',gate); self.assertFalse(gate['fixture_acceptance'])
        e=adapter.load(out/'export.json'); self.assertEqual(e['flow']['interrupted'],1); self.assertEqual(e['flow']['not_completed'],1)
    def test_official_verify_files_rejects_postexport_artifact_change(self):
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'passed',gate)
        (out/'snapshot/host.json').write_bytes(b'changed')
        result=subprocess.run([str(PYTHON),str(out/'tooling/tools/experiment_protocol.py'),'check-export',str(out/'export.json'),'--protocol',str(out/'snapshot/protocol.json'),'--verify-files'],capture_output=True,check=False,timeout=10)
        self.assertNotEqual(result.returncode,0)
        self.assertIn(b'file changed since export',result.stdout+result.stderr)
    def test_deadline_failure_retains_raw_and_reports_gate_failure(self):
        self.save(); out=self.root/'expired'
        gate=adapter.export_attempt(self.attempt,out,TOOLING,PYTHON,timeout=-1)
        self.assertEqual(gate['status'],'failed'); self.assertIn('deadline',gate['error'])
        self.assertTrue((self.attempt/'receipt.json').exists())
    def test_workflow_no_selection_invokes_bookkeeping_without_controls(self):
        driver=types.SimpleNamespace(receipt={'commands':[]},checkpoint=lambda:None)
        calls=[]
        workflow.execute(driver,lambda:None,lambda selected:calls.append(selected))
        self.assertEqual(calls,[])
        self.assertEqual(len(driver.receipt['workflows']),2)
        self.assertTrue(all(w['invoked'] for w in driver.receipt['workflows']))
        self.assertEqual(driver.receipt['workflows'][1]['reason'],'missing_native_precondition')
        self.assertEqual(driver.receipt['status'],'missing_fixture')
    def test_workflow_native_exception_records_control_precondition_then_reraises(self):
        driver=types.SimpleNamespace(receipt={'commands':[]},checkpoint=lambda:None)
        def failed():
            driver.receipt['commands'].append({'name':'unit','status':'timeout'})
            raise ValueError('unit failure')
        with self.assertRaisesRegex(ValueError,'unit failure'):
            workflow.execute(driver,failed,lambda selected:self.fail('native control invoked'))
        self.assertEqual(driver.receipt['workflows'][0]['status'],'timeout')
        self.assertEqual(driver.receipt['workflows'][0]['command_names'],['unit'])
        self.assertEqual(driver.receipt['workflows'][1]['command_names'],[])

    def test_later_final_receipt_inventory_updates_do_not_break_sealed_export(self):
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'passed',gate)
        (self.attempt/'receipt.json').write_bytes(b'final receipt after gate')
        (self.attempt/'inventory.json').write_bytes(b'final inventory after gate')
        result=subprocess.run([str(PYTHON),str(out/'tooling/tools/experiment_protocol.py'),'check-export',str(out/'export.json'),'--protocol',str(out/'snapshot/protocol.json'),'--verify-files'],capture_output=True,check=False,timeout=10)
        self.assertEqual(result.returncode,0,result.stdout+result.stderr)
    def test_incomplete_control_command_is_started_not_unrun(self):
        self.receipt['controls']=[]
        self.receipt['commands'][1].update(status='timeout',exit_code=None)
        self.receipt['workflows'][1].update(status='timeout',reason='unit timeout')
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'passed',gate)
        flow=adapter.load(out/'export.json')['backend_record']['control_flow']
        self.assertEqual(flow['incomplete_names'],['forged-reason'])
        self.assertNotIn('forged-reason',flow['unrun_names'])
    def test_missing_checked_candidate_count_is_rejected_not_imputed(self):
        del self.receipt['candidates'][0]['verified_decisions']
        gate,_=self.run_gate(); self.assertEqual(gate['status'],'failed')
    def test_signal_and_resource_workflow_statuses_preserved(self):
        for message,expected in [('signal 14','timeout'),('signal 15','interrupted'),('signal 2','interrupted'),('attempt storage admission ceiling reached','cancelled')]:
            driver=types.SimpleNamespace(receipt={'commands':[]},checkpoint=lambda:None)
            def failed(): raise ValueError(message)
            with self.assertRaises(ValueError): workflow.execute(driver,failed,lambda x:self.fail('unexpected native control'))
            self.assertEqual(driver.receipt['workflows'][0]['status'],expected)

    def test_runtime_dependency_mismatch_rejected_before_native_work(self):
        settings=copy.deepcopy(self.protocol['extensions']['ml_dataset_sat_fixture']['unified_export_tooling'])
        settings['dependencies']['jsonschema']='3.2.0'
        with self.assertRaisesRegex(ValueError,'dependencies differ'): adapter.assert_checker_runtime(settings)
    def test_current_pinned_runtime_has_exact_declared_dependencies(self):
        runtime=adapter.assert_checker_runtime(self.protocol['extensions']['ml_dataset_sat_fixture']['unified_export_tooling'])
        self.assertTrue(runtime['draft202012_validator_available'])
        self.assertEqual(runtime['dependencies'],adapter.CHECKER_DEPENDENCIES)
    def test_configurable_launcher_defaults_to_frozen_checker_python(self):
        source=(HERE/'run.sh').read_text()
        self.assertIn('ML_DATASET_FIXTURE_PYTHON:-'+str(PYTHON),source)
        self.assertIn('exec "$fixture_python"',source)

    def test_source_failure_prevents_standalone_fixture_acceptance(self):
        self.receipt['status']='failed_source_changed'
        self.receipt['source_inventories_unchanged']['0']=False
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'passed',gate)
        self.assertFalse(gate['fixture_acceptance'])
        self.assertEqual(adapter.load(out/'export.json')['backend_record']['receipt']['status'],'failed_source_changed')

    def test_wrong_frozen_population_fails_instead_of_padding(self):
        self.protocol['sample_size']['instances']=16
        (self.attempt/'protocol.json').unlink(); adapter.write(self.attempt/'protocol.json',self.protocol)
        gate,out=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertFalse((out/'export.json').exists())
    def test_frozen_protocol_amendment_forbidden(self):
        with self.assertRaisesRegex(ValueError,'never amend'): amend(self.protocol)
    def test_binary_substitution_rejected(self):
        (self.attempt/'frozen-crypto').write_bytes(b'changed')
        gate,_=self.run_gate(); self.assertEqual(gate['status'],'failed'); self.assertIn('binary mismatch',gate['error'])

if __name__=='__main__': unittest.main(verbosity=2)
