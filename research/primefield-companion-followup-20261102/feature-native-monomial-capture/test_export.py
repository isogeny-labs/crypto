import unittest,json,sys
from pathlib import Path
import export as e
sys.path.insert(0,str(e.R/'feature-native-monomial-matched'))
from test_analyze import Parser
class Export(unittest.TestCase):
 def fixture(self):
  p=json.loads((e.D/'comparison-protocol.draft.json').read_text());p['freeze']['status']='frozen';p['freeze']['protocol_sha256']=e.protocol_tool.protocol_hash(p)
  receipt={'exit_code':0,'timed_out':False,'protocol_sha256':p['freeze']['protocol_sha256'],'binary_sha256':p['arms'][0]['instrument']['binary_sha256']};return p,receipt
 def test_schema_and_charged_metrics(self):
  p,r=self.fixture();out=e.convert(Parser().panel(),p,r);self.assertEqual(e.protocol_tool.schema_errors(out,'experiment.export/v1'),[]);self.assertEqual(out['flow']['completed'],60);self.assertEqual(out['runs'][0]['measurements']['counted']['modular_product_helpers'],17);self.assertFalse(out['runs'][0]['measurements']['wall_valid'])
 def test_wrong_bindings_and_failure(self):
  for key,value in [('exit_code',7),('timed_out',True),('binary_sha256','0'*64),('protocol_sha256','0'*64)]:
   p,r=self.fixture();r[key]=value
   with self.assertRaises(ValueError):e.convert(Parser().panel(),p,r)
if __name__=='__main__':unittest.main()
