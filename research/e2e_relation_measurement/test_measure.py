#!/usr/bin/env python3
"""Independent affine arithmetic checks for the bounded driver."""
import json, pathlib, subprocess, tempfile, unittest

HERE=pathlib.Path(__file__).resolve().parent
P=(4,10); Q=(82,27); MOD=101; N=101

def add(p,q):
    if p is None: return q
    if q is None: return p
    x,y=p; u,v=q
    if x==u and (y+v)%MOD==0: return None
    slope=((3*x*x+1)*pow(2*y,-1,MOD) if p==q else (v-y)*pow(u-x,-1,MOD))%MOD
    z=(slope*slope-x-u)%MOD
    return z,(slope*(x-z)-y)%MOD

def mul(k,p):
    result=None
    while k:
        if k&1: result=add(result,p)
        p=add(p,p); k>>=1
    return result

def point(p): return None if p is None else tuple(p)

class MeasurementTest(unittest.TestCase):
    def run_case(self,mode,extra=()):
        with tempfile.TemporaryDirectory() as d:
            out=pathlib.Path(d)/'out.json'
            subprocess.run(['sage','-python',str(HERE/'measure.py'),'--p','101','--curve-a','1','--curve-b','32','--px','4','--py','10','--qx','82','--qy','27','--mode',mode,'--seed','42','--block-size','4','--output',str(out),*extra],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
            return json.loads(out.read_text())
    def test_modes_and_replay(self):
        for mode in ('independent','walk','two-group'):
            with self.subTest(mode=mode):
                r=self.run_case(mode); self.assertEqual(r['status'],'complete')
                t=r['target']; self.assertEqual(add(mul(t['a'],P),mul(t['b'],Q)),point(t['point']))
                accepted=[]
                for c in r['candidates']:
                    self.assertEqual(add(mul(c['a'],P),mul(c['b'],Q)),point(c['point']))
                    if c['status']=='accepted': accepted.append(c)
                self.assertEqual(len({c['point'][0] for c in accepted}),8)
                for w in r['witnesses']:
                    u,v=(accepted[i] for i in w['indices']); su,sv=w['signs']
                    self.assertEqual(add(mul(su%N,point(u['point'])),mul(sv%N,point(v['point']))),point(t['point']))
                    self.assertEqual((su*u['a']+sv*v['a']-t['a'])%N,w['A'])
                    self.assertEqual((su*u['b']+sv*v['b']-t['b'])%N,w['B'])
                    if w['B']: self.assertEqual(mul(w['scalar'],P),Q)
                again=self.run_case(mode)
                for key in ('target','generator','candidates','roots','witnesses','counts'): self.assertEqual(r[key],again[key])
    def test_positive_signed_relations(self):
        r=self.run_case('independent',['--block-size','10'])
        self.assertEqual(len(r['roots']),3)
        self.assertGreater(r['useful_relations'],0)
        accepted=[c for c in r['candidates'] if c['status']=='accepted']
        for w in r['witnesses']:
            u,v=(accepted[i] for i in w['indices']); su,sv=w['signs']
            self.assertEqual(add(mul(su%N,point(u['point'])),mul(sv%N,point(v['point']))),point(r['target']['point']))
            self.assertEqual((su*u['a']+sv*v['a']-r['target']['a'])%N,w['A'])
            self.assertEqual((su*u['b']+sv*v['b']-r['target']['b'])%N,w['B'])
            self.assertEqual(mul(w['scalar'],P),Q)

    def test_exhaustion_is_recorded(self):
        r=self.run_case('independent',['--max-candidates','1'])
        self.assertEqual(r['status'],'candidate_budget_exhausted'); self.assertEqual(r['counts']['candidates'],1)
        self.assertNotIn('witnesses',r)

if __name__=='__main__': unittest.main()
