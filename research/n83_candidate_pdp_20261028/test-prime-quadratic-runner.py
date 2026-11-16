"""Failure-injection controls; preserve all fixtures under the supplied root."""
from pathlib import Path
import importlib.util,json,os,sys,unittest
spec=importlib.util.spec_from_file_location('runner',Path(__file__).with_name('run-prime-quadratic.py'));runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
ROOT=Path(os.environ['N83_QUADRATIC_RUNNER_TEST_ARTIFACTS']);ROOT.mkdir(parents=True,exist_ok=False)
class Controls(unittest.TestCase):
    def test_failure_and_timeout_do_not_skip_later_templates(self):
        fixture=ROOT/'fake-instrument';fixture.write_text('#!'+sys.executable+'''\nimport sys,time,json
from pathlib import Path
mode,m,limits,out=sys.argv[1:];m=int(m);out=Path(out)
if mode=='template':
 out.mkdir();(out/'journal.jsonl').write_text('{"prefix":true}\\n')
 if m==4:raise SystemExit(9)
 if m==6:time.sleep(10)
 (out/'result.json').write_text('{}')
''');fixture.chmod(0o755);limits=ROOT/'limits.json';limits.write_text('{}')
        rows=runner.templates(fixture,ROOT/'panel',limits,0.5,512*1024*1024)
        self.assertEqual([x['m'] for x in rows],list(range(3,10)))
        self.assertEqual(rows[1]['producer_exit_code'],9);self.assertEqual(rows[1]['replay_exit_code'],125)
        self.assertTrue(rows[3]['producer']['timed_out']);self.assertEqual(rows[3]['replay_exit_code'],125)
        self.assertEqual(rows[-1]['replay_exit_code'],0)
        self.assertTrue((ROOT/'panel/templates/m4/native/journal.jsonl').exists())
        self.assertTrue((ROOT/'panel/templates/m6/native/journal.jsonl').exists())
    def test_missing_executable_records_failure(self):
        result=runner.child([str(ROOT/'absent')],ROOT/'missing.log',1,512*1024*1024)
        self.assertEqual(result['exit_code'],126);self.assertFalse(result['started'])
if __name__=='__main__':unittest.main()
