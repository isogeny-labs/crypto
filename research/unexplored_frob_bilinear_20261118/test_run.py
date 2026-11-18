#!/usr/bin/env python3
"""Independent fast algebra/correctness regression for the toy runner."""
import unittest
from run import BinaryField, CELLS,run_cell
class TestArithmetic(unittest.TestCase):
 def test_fields_and_curves(self):
  for n,a,poly,N,r in CELLS:
   F=BinaryField(n,poly);F.prepare_trace()
   self.assertEqual(F.trace(0),0)
   for x in range(1,min(F.q,65)):
    self.assertEqual(F.mul(x,F.inv(x)),1)
    self.assertEqual(F.trace(F.sq(x)),F.trace(x))
   pts=F.points(a)
   self.assertEqual(len(pts)+1,N)
   for P in pts[::max(1,len(pts)//80)]:
    self.assertTrue(F.on_curve(P,a))
    self.assertEqual(F.add(P,F.neg(P),a),None)
 def test_frozen_primary(self):
  row=run_cell(*CELLS[0])
  self.assertEqual(row['samples']['train'],8000)
  self.assertEqual(row['samples']['test'],8000)
  self.assertTrue(row['subgroup_verified'])
  self.assertEqual(sum(row['holdout']['confusion'].values()),8000)
if __name__=='__main__':unittest.main()
