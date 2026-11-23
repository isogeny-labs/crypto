import unittest,copy,json
import analyze as a
class Parser(unittest.TestCase):
 def panel(self):
  zero=lambda:[[[0]*17 for _ in range(12)]for _ in range(14)]
  rows=[{'schema':'monomial-matched/v1','targets':20,'shape':[14,12,17]}]
  for arm in a.ARMS:
   c=zero();c[1][6][0]=10 if arm!='candidate'else 20
   rows.append(dict(kind='fixed',arm=arm,label=-1,counts=c))
  for f in a.expected():
   for arm in a.ARMS:
    c=zero();c[6][11][0]=5 if arm=='candidate'else 7
    rows.append(dict(kind='query',arm=arm,label=f[0],point=f[1:3],result=f[3:],counts=c))
  rows.append(dict(complete=True,queries=60,aa_equal=True,forward_checked=True));return rows
 def test_setup_charged(self):
  result=a.analyze(self.panel());self.assertEqual(result['pairs'][0]['candidate_minus_baseline'][0],8)
 def test_corruptions(self):
  mutations=[lambda x:x.pop(),lambda x:x[4].update(label=99),lambda x:x[4]['result'].__setitem__(0,99),lambda x:x[4]['counts'][4][0].__setitem__(0,True),lambda x:x[4]['counts'][2][0].__setitem__(0,1),lambda x:x[4]['counts'][4][0].__setitem__(13,1),lambda x:x[6]['counts'][6][11].__setitem__(0,8),lambda x:x[4]['counts'][4][0].__setitem__(7,1),lambda x:x[4]['counts'][4][0].__setitem__(0,2**64),lambda x:x[1]['counts'][4][0].__setitem__(0,1)]
  for change in mutations:
   rows=self.panel();change(rows)
   with self.assertRaises(AssertionError):a.analyze(rows)
 def test_duplicate_keys(self):
  with self.assertRaises(ValueError):json.loads('{"x":1,"x":2}',object_pairs_hook=a.unique)
if __name__=='__main__':unittest.main()
