"""Synthetic wrapper controls only: no backend execution or crypto computation."""
import copy,unittest,json,subprocess,sys,tempfile
from pathlib import Path
import analyze as a
NAMES=['Product','Projection','live_bytes','allocation_failures','accounting_invalid','counter_overflow','Add']
def synthetic():
 c=dict(case_id=0,p=13,kind='synthetic',n=1,m=1,expected=7)
 rows=[]
 for ordinal,arm in enumerate(a.ARMS):
  m=dict.fromkeys(NAMES,0);m.update(Product=10+(arm=='positive'),Projection=1,Add=3)
  rows.append(dict(c,arm=arm,ordinal=ordinal,value=7,protocol_sha256='a'*64,input_sha256='b'*64,metrics=m))
 return [c],rows
def check(rows,inputs):return a.validate_rows(rows,inputs,NAMES,'a'*64,'b'*64)
class Controls(unittest.TestCase):
 def test_valid(self):
  c,r=synthetic();self.assertTrue(check(r,c)['controls_passed'])
 def test_rejections(self):
  mutations=[lambda r:r.pop(),lambda r:r.append(copy.deepcopy(r[0])),lambda r:r.__setitem__(1,copy.deepcopy(r[0])),lambda r:r[0].update(value=6),lambda r:r[0].update(expected=6),lambda r:r[0].update(protocol_sha256='c'*64),lambda r:r[0].update(input_sha256='c'*64),lambda r:r[0].update(n=2),lambda r:r[0].update(extra=0),lambda r:r[0]['metrics'].update(Product=True),lambda r:r[0]['metrics'].update(Product=-1),lambda r:r[0]['metrics'].update(Product=2**64),lambda r:r[0]['metrics'].pop('Add'),lambda r:r[0]['metrics'].update(extra=0),lambda r:r[0]['metrics'].update(live_bytes=1),lambda r:r[0]['metrics'].update(allocation_failures=1),lambda r:r[0]['metrics'].update(accounting_invalid=1),lambda r:r[0]['metrics'].update(counter_overflow=1),lambda r:r[0]['metrics'].update(Projection=2),lambda r:r[3]['metrics'].update(Add=4),lambda r:r[4]['metrics'].update(Product=10),lambda r:r[4]['metrics'].update(Add=4)]
  for i,change in enumerate(mutations):
   with self.subTest(mutation=i):
    c,r=synthetic();change(r)
    with self.assertRaises(ValueError):check(r,c)
 def test_json_duplicates_and_nonfinite(self):
  for s in ['{"x":1,"x":2}','{"x":NaN}','{"x":Infinity}']:
   with self.assertRaises(ValueError):a.read_json(s)
 def test_wrong_input_panel(self):
  with self.assertRaises(ValueError):a.fixtures('case_id\tp\tkind\tn\tm\troots\tpoints\texpected\n')
 def test_complete_cli_custody(self):
  here=Path(__file__).resolve().parent;inp=(here/'inputs.tsv').read_bytes();inputs=a.fixtures(inp.decode());names=(here/'metrics.txt').read_text().splitlines();rows=[]
  for c in inputs:
   for offset in range(5):
    arm=a.ARMS[(offset+c['case_id'])%5];m=dict.fromkeys(names,0);m.update(Product=10+(arm=='positive'),Projection=1)
    rows.append(dict({k:c[k]for k in ['case_id','p','kind','n','m','expected']},arm=arm,ordinal=len(rows),value=c['expected'],protocol_sha256='a'*64,input_sha256=a.digest(inp),metrics=m))
  with tempfile.TemporaryDirectory()as t:
   t=Path(t);raw=('\n'.join(json.dumps(r)for r in rows)+'\n').encode();(t/'rows').write_bytes(raw)
   receipt=dict(exit_code=0,protocol_sha256='a'*64,input_sha256=a.digest(inp),binary_sha256='c'*64,raw_sha256=a.digest(raw),metrics_sha256=a.digest((here/'metrics.txt').read_bytes()))
   base=[sys.executable,str(here/'analyze.py'),'--rows',str(t/'rows'),'--inputs',str(here/'inputs.tsv'),'--metrics',str(here/'metrics.txt'),'--receipt',str(t/'receipt'),'--expected-protocol','a'*64,'--expected-input',a.digest(inp),'--expected-binary','c'*64,'--output',str(t/'output')]
   def run(r):
    (t/'receipt').write_text(json.dumps(r));return subprocess.run(base,capture_output=True)
   for key,value in [('exit_code',1),('raw_sha256','d'*64),('protocol_sha256','d'*64),('input_sha256','d'*64),('binary_sha256','d'*64),('metrics_sha256','d'*64)]:
    with self.subTest(pin=key):
     self.assertNotEqual(run(dict(receipt,**{key:value})).returncode,0);self.assertFalse((t/'output').exists())
   ok=run(receipt);self.assertEqual(ok.returncode,0,ok.stderr);saved=(t/'output').read_bytes();self.assertEqual(json.loads(saved)['rows'],240)
   self.assertNotEqual(run(receipt).returncode,0);self.assertEqual((t/'output').read_bytes(),saved)
if __name__=='__main__':unittest.main()
