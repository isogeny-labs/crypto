#!/usr/bin/env python3
"""Checks for comparison of saved panel receipts."""
import hashlib, importlib.util, json, pathlib, tempfile, unittest

HERE=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('compare_panels',HERE/'compare_panels.py')
compare=importlib.util.module_from_spec(spec); spec.loader.exec_module(compare)


def save(path,obj): path.write_text(json.dumps(obj,sort_keys=True)+'\n')
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def fixture(directory, *, filtered=False, target=17, incomplete=False, mode_targets=None):
    directory.mkdir()
    plan={'common_args':['--p','101','--curve-a','1','--curve-b','32'] + (['--modulus','2','--residue','0'] if filtered else []), 'seeds':[7], 'timeout_seconds':10}
    save(directory/'plan.json',plan)
    rows=[]
    for mode in compare.MODES:
        name=f'seed-7-{mode}'
        actual_target=(mode_targets or {}).get(mode,target)
        for suffix in ('stdout','stderr'):
            (directory/f'{name}.{suffix}').write_bytes(b'')
        result=directory/f'{name}.result.json'
        save(result,{'schema':'e2e-relation-measurement/v1','config':{'seed':7,'mode':mode},'status':'complete','target':{'a':1,'b':2,'point':[actual_target,1]},'useful_relations':1,'relation_counts':{'nonzero_b':1},'source_sha256_at_start':'m','accounting_sha256_at_start':'a'})
        missing=incomplete and mode=='walk'
        row={'seed':7,'mode':mode,'status':'timeout' if missing else 'exited', 'exit_code':-9 if missing else 0, 'result_status':None if missing else 'complete', 'target':None if missing else {'a':1,'b':2,'point':[actual_target,1]},'whole_process_wall_ns':20,'relation_counts':None if missing else {'nonzero_b':1},'useful_relations':None if missing else 1,'result_sha256':digest(result),'checkpoint_sha256':None,'stdout_sha256':digest(directory/f'{name}.stdout'),'stderr_sha256':digest(directory/f'{name}.stderr')}
        rows.append(row); save(directory/f'{name}.receipt.json',row)
    summary={'plan_sha256':digest(directory/'plan.json'),'measure_source_unchanged':True,'accounting_source_unchanged':True,'measure_sha256_at_start':'m','accounting_sha256_at_start':'a','panel_sha256':'p','platform':'host','resource_metadata_at_start':{'machine':'local','cpu_model':'cpu','logical_cpus':1,'parent_limits':{'RLIMIT_CPU':[-1,-1]},'rss_native_unit':'bytes','thread_environment':{}},'resource_metadata_at_end':{'machine':'local','cpu_model':'cpu','logical_cpus':1,'parent_limits':{'RLIMIT_CPU':[-1,-1]},'rss_native_unit':'bytes','thread_environment':{}},'runs':rows}
    save(directory/'summary.json',summary)


class ComparisonTest(unittest.TestCase):
    def test_matched_and_tampered(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b,filtered=True)
            result=compare.compare(a,b)
            self.assertTrue(result['matching_checks_pass']); self.assertEqual(len(result['pairs']),3)
            self.assertEqual(result['resource_metadata_status'],'metadata_match_only')
            self.assertIsNone(result['per_mode']['walk']['left']['peak_rss_native_max']['observed_value'])
            self.assertEqual(result['per_mode']['walk']['left']['whole_process_wall_ns'],20)
            self.assertEqual(result['per_mode']['walk']['left']['candidates']['unknown_attempts'],1)
            self.assertEqual(result['right_filter']['--modulus'],'2')
            (b/'seed-7-walk.result.json').write_text('{}')
            with self.assertRaisesRegex(ValueError,'artifact hash mismatch'): compare.compare(a,b)
    def test_mismatch_and_incomplete(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b,target=18,incomplete=True)
            result=compare.compare(a,b)
            self.assertFalse(result['matching_checks_pass'])
            self.assertIn('some targets unavailable',result['mismatch_or_unknown'])
            self.assertEqual(result['per_mode']['walk']['right']['nonzero_b_unknown_runs'],1)
            self.assertEqual(result['per_mode']['walk']['right']['whole_process_wall_ns'],20)
            self.assertIn('some attempts incomplete or invalid',result['mismatch_or_unknown'])
            self.assertTrue(any('target differs' in reason for reason in result['mismatch_or_unknown']))
    def test_checkpoint_counts_checked_before_aggregation(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b,incomplete=True)
            summary_path=b/'summary.json'; summary=json.loads(summary_path.read_text())
            row=next(row for row in summary['runs'] if row['mode']=='walk')
            checkpoint=b/'seed-7-walk.checkpoint.json'
            save(checkpoint,{'schema':'e2e-relation-measurement/v1','config':{'seed':7,'mode':'walk'},'counts':{'candidates':9},'source_sha256_at_start':'m','accounting_sha256_at_start':'a'})
            row['observation_source']='checkpoint'; row['checkpoint_sha256']=digest(checkpoint); row['candidate_counts']={'candidates':9}
            save(b/'seed-7-walk.receipt.json',row); save(summary_path,summary)
            result=compare.compare(a,b)
            self.assertEqual(result['per_mode']['walk']['right']['candidates']['observed_sum'],9)
            row['candidate_counts']={'candidates':99}; save(b/'seed-7-walk.receipt.json',row); save(summary_path,summary)
            with self.assertRaisesRegex(ValueError,'checkpoint does not match receipt'): compare.compare(a,b)

    def test_modes_must_share_target_within_each_panel(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'
            fixture(a,mode_targets={'walk':19}); fixture(b,mode_targets={'walk':19})
            result=compare.compare(a,b)
            self.assertFalse(result['matching_checks_pass'])
            self.assertTrue(any('modes use different targets' in reason for reason in result['mismatch_or_unknown']))

    def test_receipt_cannot_relabel_result(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b)
            summary_path=b/'summary.json'; summary=json.loads(summary_path.read_text())
            row=summary['runs'][0]; row['target']={'a':9,'b':9,'point':[9,9]}
            save(b/'seed-7-independent.receipt.json',row); save(summary_path,summary)
            with self.assertRaisesRegex(ValueError,'result does not match receipt'): compare.compare(a,b)

    def test_resource_observation_cannot_be_relabelled(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b)
            summary_path=b/'summary.json'; summary=json.loads(summary_path.read_text())
            row=summary['runs'][0]; row['observed_total']={'cpu_ns':1,'peak_rss_native':1}
            save(b/'seed-7-independent.receipt.json',row); save(summary_path,summary)
            with self.assertRaisesRegex(ValueError,'result does not match receipt'): compare.compare(a,b)

    def test_resource_metadata_disagreement_and_absence(self):
        with tempfile.TemporaryDirectory() as d:
            a,b=pathlib.Path(d)/'a',pathlib.Path(d)/'b'; fixture(a); fixture(b)
            summary_path=b/'summary.json'; summary=json.loads(summary_path.read_text())
            summary['resource_metadata_at_end']['machine']='other'; save(summary_path,summary)
            self.assertEqual(compare.compare(a,b)['resource_metadata_status'],'different')
            del summary['resource_metadata_at_end']; save(summary_path,summary)
            result=compare.compare(a,b)
            self.assertEqual(result['resource_metadata_status'],'unavailable')
            self.assertFalse(result['matching_checks_pass'])
            summary['resource_metadata_at_end']=summary['resource_metadata_at_start'].copy(); summary['resource_metadata_at_end']['cpu_model']=None; save(summary_path,summary)
            self.assertEqual(compare.compare(a,b)['resource_metadata_status'],'incomplete')

    def test_ambiguous_arguments_rejected(self):
        with self.assertRaises(ValueError): compare.flags(['--p','101','--p','103'])
        with self.assertRaises(ValueError): compare.flags(['--seed','2'])

if __name__=='__main__': unittest.main()
