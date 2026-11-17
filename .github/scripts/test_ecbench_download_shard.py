import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

spec=importlib.util.spec_from_file_location('planner',Path(__file__).with_name('ecbench-download-shard.py'))
planner=importlib.util.module_from_spec(spec)
spec.loader.exec_module(planner)

class DownloadPlanTests(unittest.TestCase):
    def test_exact_coverage_and_balancing_with_repeated_exceptions(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);names=['a','b','c','incomplete','defective']
            for name in names:
                p=root/name;p.mkdir();(p/'session.json').write_text('{"schema":"ecbench.session/v1"}')
            (root/'a'/'records.jsonl').write_bytes(b'x'*120)
            (root/'b'/'records.jsonl.gz').write_bytes(b'x'*30)
            (root/'c'/'records.jsonl').write_bytes(b'x'*100)
            exceptions=root/'pins.tsv';exceptions.write_text('# pins\nincomplete\thash\ndefective\thash\n')
            shards,loads=planner.plan(root,names,[exceptions],2)
            self.assertEqual(shards,[['b','incomplete','defective'],['a','c','incomplete','defective']])
            self.assertEqual(sum(p in shard for shard in shards for p in ['a','b','c']),3)
            self.assertEqual(loads,[271,282])
    def test_more_shards_than_sessions_and_tie_order(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            for name in ['b','a']:
                (root/name).mkdir();(root/name/'session.json').write_text('{"schema":"ecbench.session/v1"}')
            shards,_=planner.plan(root,['b','a'],[],16)
            self.assertEqual(shards[:3],[['b'],['a'],[]])
    def test_missing_exception_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);pins=root/'pins';pins.write_text('missing\thash\n')
            with self.assertRaises(ValueError):planner.plan(root,['a'],[pins],16)
    def test_missing_session_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(FileNotFoundError):planner.plan(Path(tmp),['a'],[],16)
    def test_bad_paths_and_duplicates_fail(self):
        for rows in [[],['a','a'],['../outside'],['/outside'],['a\tb']]:
            with self.assertRaises(ValueError):planner.checked_paths(rows)
    def test_workflow_shards_before_download_and_does_not_shard_twice(self):
        text=Path(__file__).parents[1].joinpath('workflows/ecbench.yml').read_text()
        job=text.split('  evidence_audit_pr:',1)[1].split('  replay_sentinels_pr:',1)[0]
        self.assertIn('ecbench-audit-downloads',job)
        self.assertNotIn('git ls-tree',job)
        self.assertIn('AUDIT_SHARD: 0/1',job)
        self.assertIn('audit-shards/${SHARD}.txt',job)
        self.assertLess(job.index('audit-shards/${SHARD}.txt'),job.index('artifacts.py pull'))

if __name__=='__main__':unittest.main()
