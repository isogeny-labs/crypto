"""Bounded local exact/software checks; see CHECK_SCOPE.md."""
from pathlib import Path
import copy, importlib.util, json, sys, unittest
D=Path(__file__).resolve().parent

def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec);sys.modules[name]=module
    spec.loader.exec_module(module);return module

repair=load('fiber_repair',D/'repair.py')
checker=load('fiber_check',D/'independent_check.py')
coef=load('coefficient_unit',D.parent/'feature-coefficient-schur/coefficient.py')
sparse=load('sparse_check_unit',D.parent/'feature-coefficient-schur/independent_check.py')
sys.path.insert(0,str(D.parent/'feature-signature-selection'))
selection=load('selection_unit',D.parent/'feature-signature-selection/selection.py')
bound=load('bound_unit',D.parent/'feature-signature-selection/check_bound.py')

class Identities(unittest.TestCase):
    def test_saved_duals_and_independent_repair(self):
        z=json.loads((D/'inputs/saved-dual.json').read_text())
        self.assertEqual((z['r'],z['q'],z['max_degree']),(479,15,3))
        w0,w1=z['coordinate_duals'];self.assertEqual((len(w0),len(w1)),(479,479))
        for c,pair in [(338,(25,173)),(126,(292,440))]:
            actual=[(a+c*b)%479 for a,b in zip(w0,w1)]
            expected=[0]*479;expected[pair[0]]=178;expected[pair[1]]=301
            self.assertEqual(actual,expected)
            self.assertEqual(sum(i*a for i,a in enumerate(actual))%479,1)
        packet=repair.construct(479,15,((25,173),(292,440)),15)
        checked=checker.verify(packet)
        self.assertEqual(checked['residual_pairings'],[0,0])
        for x,y in packet['pairs']:
            self.assertEqual(repair.residual(packet,x),repair.residual(packet,y))

    def test_repair_corruption_and_bad_proposals(self):
        packet=repair.construct(479,15,((25,173),(292,440)),15)
        bad=[]
        z=copy.deepcopy(packet);z['coefficients']=((z['coefficients'][0]+1)%479,z['coefficients'][1]);bad.append(z)
        z=copy.deepcopy(packet);z['projection']=((0,0),z['projection'][1]);bad.append(z)
        z=copy.deepcopy(packet);z['distinguishing_offset']=15;bad.append(z)
        z=copy.deepcopy(packet);z['pairs']=((25,25),(292,440));bad.append(z)
        for z in bad:
            with self.subTest(packet=z),self.assertRaises(ValueError):checker.verify(z)
        with self.assertRaises(ValueError):repair.construct(479,15,((25,173),(292,440)),14)
        with self.assertRaises(ValueError):repair.construct(479,15,((25,174),(292,440)),15)

    def test_scalar_pair_schur_and_counters(self):
        for points in [(0,),(0,1)]:
            with self.subTest(points=points):
                m=coef.Meter(7);root=coef.build(7,1,2,(1,1),m)
                before=(m.products,m.additions,m.inversions)
                child,frame=coef.pivot(root,points,m)
                k=len(points);n=2
                expected_products=2*k*n*n+2*k*k*n+2*k*n+4*k+(6 if k==2 else 0)
                expected_adds=(2*k+1)*n*n+2*k*k*n+2*k*n+n+k+(3 if k==2 else 0)
                self.assertEqual((m.products-before[0],m.additions-before[1],m.inversions-before[2]),
                                 (expected_products,expected_adds,1))
                if k==1:
                    self.assertEqual(child.C,((0,0),(0,1)))
                    self.assertEqual(child.u,(0,0))
                    child,frame2=coef.pivot(child,(1,),m);frames=[frame,frame2]
                else:frames=[frame]
                self.assertEqual(child.C,((0,0),(0,0)));self.assertEqual(child.u,(0,1))
                witness=coef.terminal(child,2,m)
                for f in reversed(frames):witness=f.lift(witness,m)
                self.assertEqual(witness.moment,(0,0))
                self.assertEqual(sparse.verify(7,1,witness.support)['normalized_pairing'],1)
                # Independent direct residual evaluation on every remaining F7 point.
                squares={1,2,4}
                for x in range(7):
                    if x not in child.used:
                        char=0 if x==0 else 1 if x in squares else 6
                        self.assertEqual((x-char)%7,(x-sum(a*b for a,b in zip((1,char),child.u)))%7)
                with self.assertRaises(ValueError):frames[0].lift(coef.terminal(child,2,m),m) if k==1 else frames[0].lift(witness,m)

    def test_invalid_pivots_and_terminal(self):
        m=coef.Meter(7);root=coef.build(7,1,2,(1,1),m)
        with self.assertRaises(ValueError):coef.pivot(root,(1,2),m)
        with self.assertRaises(ValueError):coef.pivot(root,(1,1),m)
        with self.assertRaises(ValueError):coef.terminal(root,2,m)
        child,frame=coef.pivot(root,(0,),m)
        with self.assertRaises(ValueError):coef.pivot(child,(0,),m)

    def test_stream_and_theorem_record(self):
        m=coef.Meter(7);s=selection.Stream(7,1,m)
        self.assertEqual(s.offer(0)['status'],'independent')
        self.assertEqual(s.offer(0)['status'],'consistent_dependent')
        self.assertEqual(s.offer(1)['status'],'independent')
        result=s.offer(2);self.assertEqual(result['status'],'witness')
        sparse.verify(7,1,result['support'])
        bad=list(result['support']);x,w=bad[0];bad[0]=(x,(w+1)%7)
        with self.assertRaises(ValueError):sparse.verify(7,1,bad)
        with self.assertRaises(ValueError):s.offer(3)
        self.assertEqual(selection.collect(7,1,[0,1],2,coef.Meter(7))['status'],'inconclusive')
        record=selection.signature_bound(7,1);self.assertEqual(bound.verify(record)['status'],'analytic_excluded')
        record['agreement_cap']+=1
        with self.assertRaises(ValueError):bound.verify(record)
        self.assertEqual(bound.verify(selection.signature_bound(7,3))['status'],'inconclusive')

if __name__=='__main__':unittest.main(verbosity=2)
