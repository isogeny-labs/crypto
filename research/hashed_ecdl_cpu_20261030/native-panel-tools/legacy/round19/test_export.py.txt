#!/usr/bin/env python3
"""Synthetic export packaging tests; never executes a solver or saves run evidence."""
import csv
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent

class ExportChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.study = self.root / 'round19'
        self.out = self.study / 'results'
        self.out.mkdir(parents=True)
        template = self.root / 'panel2/results'
        template.mkdir(parents=True)
        shutil.copyfile(HERE.parent/'panel2/results/export.json', template/'export.json')
        for name in ['export.py','analyze.rs','protocol.json','build.json',
                     'candidate-materialization.json','previous-materialization.json',
                     'inverse-materialization.json']:
            shutil.copyfile(HERE/name,self.study/name)
            if name.endswith('.json'): shutil.copyfile(HERE/name,self.out/name)
        header = ('arm,instance_id,seed,replicate,warmup,verification,scalar,attempts,'
                  'relations,curve_ms,bases_ms,oracle_ms,reduction_other_ms,replay_ms,'
                  'complete_ms,oracle_calls,preparations,built_rows,probed_rows,'
                  'query_inversions,table_bytes,digest,wall_seconds,tier')
        with (self.out/'rows.csv').open('w') as f:
            f.write(header+'\n')
            writer=csv.writer(f,lineterminator='\n')
            for arm in 'uvwxyz':
                for case in range(3):
                    for rep in range(10):
                        writer.writerow([arm,case,111*(case+1),rep,str(rep==0).lower(),
                            'verified',[42,45678,54321][case],10,10]+[1.0]*6+
                            [0]*7+[0.001,'L0'])
                        (self.out/f'cert-{arm}-{case}-{rep}.txt').write_text(
                            f'SYNTHETIC EXPORT TEST ONLY case={case} rep={rep}\n')
        with (self.out/'analysis.log').open('w') as log:
            subprocess.run([os.environ['ANALYZER'],str(self.out/'rows.csv'),str(self.out)],
                           stdout=log,check=True)
        (self.out/'exit-code.txt').write_text('0\n')
        (self.out/'cpuinfo.txt').write_text('model name : SYNTHETIC TEST ONLY\n')
        (self.out/'kernel.txt').write_text('SYNTHETIC TEST ONLY\n')
    def tearDown(self):
        self.temp.cleanup()
    def invoke(self):
        return subprocess.run([sys.executable,'-W','error',str(self.study/'export.py')],
                              text=True,capture_output=True)
    def test_complete_six_arm_export(self):
        result=self.invoke()
        self.assertEqual(result.returncode,0,result.stderr)
        d=json.loads((self.out/'export.json').read_text())
        self.assertEqual(len(d['runs']),180)
        self.assertEqual(sum(r['warmup'] for r in d['runs']),18)
        self.assertEqual({a['id'] for a in d['arms']},set('uvwxyz'))
        self.assertEqual(d['flow']['planned'],180)
        self.assertNotEqual(self.invoke().returncode,0, 'must not overwrite evidence')
    def test_wrong_returned_manifest_rejected(self):
        (self.out/'inverse-materialization.json').write_text('{}')
        self.assertNotEqual(self.invoke().returncode,0)
        self.assertFalse((self.out/'export.json').exists())
    def test_incomplete_rows_rejected(self):
        p=self.out/'rows.csv';lines=p.read_text().splitlines();p.write_text('\n'.join(lines[:-1])+'\n')
        self.assertNotEqual(self.invoke().returncode,0)
        self.assertFalse((self.out/'export.json').exists())

if __name__=='__main__': unittest.main()
