from pathlib import Path
import hashlib,json,unittest,sys
p=Path(__file__).resolve().parent;files=[p/'supervise.py',p/'test_supervise.py',p/'PLAN.md',p/'run_tests.py'];hashes={f.name:hashlib.sha256(f.read_bytes()).hexdigest() for f in files}
suite=unittest.defaultTestLoader.discover(str(p),pattern='test_supervise.py');r=unittest.TextTestRunner(verbosity=2).run(suite)
record={'schema':'software.mock-controls/v1','tests':r.testsRun,'failures':len(r.failures),'errors':len(r.errors),'skipped':len(r.skipped),'passed':r.wasSuccessful(),'external_commands_executed':0,'source_hashes':hashes,'sources_unchanged':all(hashlib.sha256(f.read_bytes()).hexdigest()==hashes[f.name] for f in files),'runtime_qualification':False,'python':sys.version};(p/'test-result.json').write_text(json.dumps(record,indent=2)+'\n');raise SystemExit(0 if r.wasSuccessful() and record['sources_unchanged'] else 1)
