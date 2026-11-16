"""Synthetic schema/control mutations only; never experiment rows."""
import copy,json,unittest
import analyze as a
def fixture():
 rows=[]
 for i,(r,m,c)in enumerate(a.CASES):
  for j in range(5):
   arm=a.ARMS[(i+j)%5]
   for phase in a.PHASES:
    stride=m*(1 if phase=='initial' else 2);n=sum(((r-1)//2-e)//stride+1 for e in range(3))
    counts=[10]*10;counts[8]=0
    if arm=='positive':counts[0]+=1
    rows.append(dict(case=i,r=r,m=m,cap=c,arm=arm,phase=phase,values=[0]*n,counts=counts,overflow=False))
 return rows
class Tests(unittest.TestCase):
 def test_valid_shape(self):self.assertEqual(a.analyze(fixture())['rows'],200)
 def test_mutations(self):
  mutations=[lambda x:x.pop(),lambda x:x.append(x[0]),lambda x:x.reverse(),lambda x:x[0].update(case=True),lambda x:x[0].update(overflow=True),lambda x:x[0].update(overflow=0),lambda x:x[0]['counts'].__setitem__(0,True),lambda x:x[0]['counts'].__setitem__(0,-1),lambda x:x[0]['counts'].__setitem__(0,2**64),lambda x:x[0]['counts'].__setitem__(8,1),lambda x:x[0]['values'].pop(),lambda x:x[0]['values'].__setitem__(0,x[0]['r']),lambda x:x[0].update(extra=1),lambda x:x[6]['counts'].__setitem__(0,11),lambda x:x[8]['counts'].__setitem__(0,10),lambda x:x[2]['values'].__setitem__(0,1)]
  for i,mut in enumerate(mutations):
   with self.subTest(i=i):
    rows=fixture();mut(rows)
    with self.assertRaises(ValueError):a.analyze(rows)
 def test_duplicate_json(self):
  with self.assertRaises(ValueError):json.loads('{"a":1,"a":2}',object_pairs_hook=a.unique)
 def test_negative_delta_retained(self):
  rows=fixture();rows[2]['counts'][0]=4
  self.assertEqual(a.analyze(rows)['deltas'][0]['delta']['scalar_products'],-6)
if __name__=='__main__':unittest.main()
