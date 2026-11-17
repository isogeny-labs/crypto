import importlib.util,tempfile,unittest,json
from unittest.mock import patch
from pathlib import Path
r=Path(__file__).parent
spec=importlib.util.spec_from_file_location('driver',r/'driver.corrected.py');d=importlib.util.module_from_spec(spec);spec.loader.exec_module(d)
class Matching(unittest.TestCase):
 def test_retained_actual_labels_and_numeric_sequence(self):
  actual=Path('/var/tmp/ml-dataset-cli46-e0a865-final008-20261101')
  self.assertEqual([p.name for p in d.exact_command_directories(actual,'generate')],['017-generate'])
  self.assertEqual([p.name for p in d.exact_command_directories(actual,'preflight')],['016-preflight'])
  self.assertEqual([p.name for p in d.exact_command_directories(actual,'verify')],['018-verify'])
 def test_does_not_hide_real_duplicates_or_match_suffixes(self):
  with tempfile.TemporaryDirectory() as tmp:
   root=Path(tmp);(root/'commands').mkdir()
   for name in ['017-generate','999-generate','027-policy-legacy-generate','abc-generate','017-generate-extra','1017-generatX','001-a.b','002-aXb']:(root/'commands'/name).mkdir()
   self.assertEqual([p.name for p in d.exact_command_directories(root,'generate')],['017-generate','999-generate'])
   self.assertEqual([p.name for p in d.exact_command_directories(root,'a.b')],['001-a.b'])
   self.assertEqual(d.exact_command_directories(root,'absent'),[])
 def test_true_duplicate_still_rejected_by_diagnostic_reader(self):
  fixture=Path('/var/tmp/ml-dataset-cli46-e0a865-final008-20261101');state=d.load(fixture/'prepared.json')
  one=fixture/'commands/017-generate'
  with patch.object(d,'exact_command_directories',return_value=[one,one]):
   with self.assertRaises(Exception):d.diagnostic_command(fixture,state,'generate',d.expected_cli_argv(state,fixture)['generate'])
 def test_original_driver_hash_required_before_reconciliation(self):
  exec(compile((r/'reconciliation-function.py').read_text(),str(r/'reconciliation-function.py'),'exec'),d.__dict__)
  with tempfile.TemporaryDirectory() as tmp:
   wrong=Path(tmp)/'wrong.py';wrong.write_text('wrong')
   with self.assertRaises(Exception):d.reconcile_failure(Path('/var/tmp/ml-dataset-cli46-e0a865-final008-20261101'),wrong)
if __name__=='__main__':unittest.main()
