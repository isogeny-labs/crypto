"""Producer self-tests; independent proof-carrying verification remains open."""
import unittest
from .core import s3,specialized_summation,eval_poly
from .field import Field
from .projective import projective_chain_roots,affine_fp_chain_roots,roots_quadratic_projective,coeffs
from .solver import chained_gb,direct_gb,symmetric_s4_gb,linear_gb
from .harness import gate

class SemaevSplitTests(unittest.TestCase):
    def test_s3_symmetric(self):
        for x,y,z in [(1,2,3),(0,0,1),(3,3,5)]:
            self.assertEqual((s3(x,y,z,1,1)-s3(y,z,x,1,1))%11,0)
    def test_infinity_counterexample(self):
        p,a,b,xs=5,0,1,[0,0,1]
        roots={X for X in range(p) if eval_poly(specialized_summation(p,a,b,xs),X,p)==0}
        self.assertEqual(roots,{1})
        self.assertEqual(affine_fp_chain_roots(p,a,b,xs),set())
        self.assertEqual(projective_chain_roots(p,a,b,xs),roots)
        self.assertIn(None,roots_quadratic_projective(Field(p),coeffs(Field(p),(0,0),(0,0),a,b)))
    def test_extension_counterexample(self):
        p,a,b,xs=5,0,1,[0,1,0]
        self.assertEqual(projective_chain_roots(p,a,b,xs),{1})
        self.assertEqual(affine_fp_chain_roots(p,a,b,xs),set())
        states=roots_quadratic_projective(Field(p),coeffs(Field(p),(0,0),(1,0),a,b))
        self.assertTrue(all(t is not None and t[1]!=0 for t in states))
    def test_50_case_gates(self):
        for p,a,b,m in ((5,0,1,3),(7,1,1,3),(11,1,1,3),(11,1,1,4),(19,2,3,4),(19,2,3,5)):
            cases,n=gate(p,a,b,m,50,12042026)
            self.assertEqual(len(cases),50)
    def test_gb_projected_roots(self):
        for p,a,b,xs in [(5,0,1,(0,0,1)),(5,0,1,(0,1,0)),(7,1,1,(0,0,1)),(11,1,1,(1,1,1,2))]:
            truth=direct_gb(p,a,b,xs)[0]
            self.assertEqual(chained_gb(p,a,b,xs)[0],truth)
            self.assertEqual(linear_gb(p,a,b,xs)[0],truth)
            if len(xs)==3:self.assertEqual(symmetric_s4_gb(p,a,b,xs)[0],truth)

if __name__=='__main__':unittest.main()
