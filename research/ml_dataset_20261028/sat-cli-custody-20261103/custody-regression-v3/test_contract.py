"""Pure driver contract checks; no native acceptance or fixture writes."""
import ast, importlib.util, json, unittest
from pathlib import Path
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('custody_accept',HERE/'accept.py')
driver=importlib.util.module_from_spec(spec);spec.loader.exec_module(driver)
class ContractTests(unittest.TestCase):
 def test_three_forms_cover_ten_commands(self):
  forms={'create':['--out','/run','--bundle','/bundle','--max-bytes','16777216','--max-files','1000'], 'verify':['--bundle','/bundle'], 'restore':['--bundle','/bundle','--destination','/dest']}
  for sub, tail in forms.items():
   with self.subTest(sub=sub):
    argv=driver.custody_argv(Path('/crypto'),[sub,*tail]);self.assertEqual(argv[:4],['/crypto','dataset','custody',sub])
    help=(HERE/'help'/f'{sub}.stdout').read_text()
    self.assertIn(f'crypto dataset custody {sub}',help)
    for flag in tail[::2]:self.assertIn(flag,help)
 def test_disallow_other_commands(self):
  for args in [[],['generate'],['publish'],['restore-remote']]:
   with self.assertRaises(ValueError):driver.custody_argv('/crypto',args)
 def test_fixed_path_and_finite_caps(self):
  self.assertTrue(str(driver.OUT).endswith('custody-four-cell-acceptance-run-20261103-v3'))
  source=(HERE/'accept.py').read_text();ast.parse(source)
  self.assertIn('timeout=min(240,remaining)',source);self.assertIn('remaining=1800-',source)
  self.assertIn('OUT.mkdir(exist_ok=False)',source)
  self.assertNotIn("'ml-dataset'",source)
 def test_fixture_unchanged(self):
  parent=HERE.parent/'custody-four-cell-acceptance-proposal-20261103-v2'
  self.assertEqual((HERE/'fixture.json').read_bytes(),(parent/'fixture.json').read_bytes())
if __name__=='__main__':unittest.main()
