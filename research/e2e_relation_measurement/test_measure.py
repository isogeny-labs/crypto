#!/usr/bin/env python3
"""Independent affine arithmetic checks for the bounded driver."""
import hashlib, importlib.util, json, pathlib, subprocess, tempfile, unittest

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
    def test_solver_protocol(self):
        with tempfile.TemporaryDirectory() as d:
            out=pathlib.Path(d)/'out.json'
            cmd=['sage','-python',str(HERE/'measure.py'),'--p','101','--curve-a','1','--curve-b','32','--px','4','--py','10','--qx','82','--qy','27','--mode','independent','--seed','42','--block-size','10','--output',str(out),'--solver-protocol']
            proc=subprocess.run(cmd,check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            record=json.loads(out.read_text())
            self.assertEqual(record['status'],'complete')
            self.assertTrue(record['solver']['protocol_requested'])
            self.assertEqual(record['solver']['backend'],'sage/libsingular:std')
            begin=b'=== BEGIN GROEBNER BASIS PROTOCOL ==='
            end=b'=== END GROEBNER BASIS PROTOCOL ==='
            self.assertEqual(proc.stdout.count(begin),1)
            self.assertEqual(proc.stdout.count(end),1)
            trace=proc.stdout.split(begin,1)[1].split(end,1)[0]
            self.assertIn(b'product criterion:',trace)
            self.assertGreater(record['useful_relations'],0)
            self.assertIsNone(record['metrics']['solving_degree'])

    def test_reject_unsupported_field(self):
        with tempfile.TemporaryDirectory() as d:
            out=pathlib.Path(d)/'out.json'; checkpoint=pathlib.Path(d)/'checkpoint.json'
            cmd=['sage','-python',str(HERE/'measure.py'),'--p','3','--curve-a','1','--curve-b','1','--px','0','--py','1','--qx','0','--qy','1','--mode','independent','--seed','1','--output',str(out),'--checkpoint',str(checkpoint)]
            proc=subprocess.run(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            self.assertNotEqual(proc.returncode,0)
            self.assertIn(b'p must be a prime greater than 3',proc.stderr)
            self.assertFalse(out.exists())
            self.assertEqual(json.loads(checkpoint.read_text())['phase'],'setup')

    def test_panel_partial_cost_coverage(self):
        spec=importlib.util.spec_from_file_location('panel', HERE/'panel.py')
        module=importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
        records=[
            {'mode':'walk','result_status':'complete','useful_relations':1,'relation_counts':{'nonzero_b':1},'whole_process_wall_ns':20,'candidate_counts':{'candidates':5,'identity':0,'filtered':1,'duplicate_x':0},'observed_timings':{'generation':{'wall_ns':3},'groebner_basis':{'wall_ns':7}}},
            {'mode':'walk','result_status':None,'useful_relations':None,'relation_counts':None,'whole_process_wall_ns':50,'candidate_counts':{'candidates':9,'identity':1,'filtered':2,'duplicate_x':1},'observed_timings':{'generation':{'wall_ns':5}}},
            {'mode':'walk','result_status':None,'useful_relations':None,'relation_counts':None,'whole_process_wall_ns':40,'candidate_counts':None,'observed_timings':None},
        ]
        summary=module.mode_summary(records,'walk')
        self.assertEqual(summary['total_process_wall_ns'],110)
        self.assertEqual(summary['candidate_counts']['candidates'],{'observed_attempts':2,'unknown_attempts':1,'observed_sum':14})
        self.assertEqual(summary['phase_wall_ns']['generation'],{'observed_attempts':2,'unknown_attempts':1,'observed_sum':8})
        self.assertEqual(summary['phase_wall_ns']['groebner_basis'],{'observed_attempts':1,'unknown_attempts':2,'observed_sum':7})
        self.assertEqual(summary['phase_wall_ns']['root_extraction'],{'observed_attempts':0,'unknown_attempts':3,'observed_sum':None})
        self.assertEqual(summary['nonzero_b_unknown_runs'],2)

    def test_relation_count_classification(self):
        spec=importlib.util.spec_from_file_location('accounting', HERE/'accounting.py')
        module=importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
        actual=module.count_witnesses([{'B':0,'verified':False},{'B':7,'verified':True},{'B':8,'verified':False}])
        self.assertEqual(actual, {'total':3,'zero_b':1,'nonzero_b':2,'nonzero_b_verified':1,'nonzero_b_verification_failed':1})
        with self.assertRaises(ValueError): module.count_witnesses([{'B':0,'verified':True}])

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
                ops=r['generation_operations']; candidates=r['counts']['candidates']; draws=ops['target_draws']
                self.assertGreaterEqual(draws,1)
                if mode == 'independent':
                    self.assertEqual(ops['scalar_multiplication_calls'],2*(draws+candidates))
                    self.assertEqual(ops['point_addition_calls'],draws+candidates)
                elif mode == 'walk':
                    starts=len(r['generator']['walks']); self.assertGreaterEqual(starts,1)
                    self.assertEqual(ops['scalar_multiplication_calls'],2*draws+4*starts)
                    self.assertEqual(ops['point_addition_calls'],draws+2*starts+candidates)
                else:
                    self.assertEqual(ops['scalar_multiplication_calls'],2*draws+2*r['config']['group_size'])
                    self.assertEqual(ops['point_addition_calls'],draws+candidates)
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
        system=r['system']; self.assertEqual(system['coefficient_modulus'],MOD)
        accepted=[c for c in r['candidates'] if c['status']=='accepted']
        for x,y in ((0,0),(3,8),(67,75)):
            actual=[sum(t['coefficient'] * pow(x,t['exponents'][0],MOD) * pow(y,t['exponents'][1],MOD) for t in terms)%MOD for terms in system['input_terms']]
            z=r['target']['point'][0]
            s3=((x-y)**2*z*z-2*((x+y)*(x*y+1)+64)*z+(x*y-1)**2-128*(x+y))%MOD
            expected=[s3,1,1]
            for i,c in enumerate(accepted): expected[1+i//10]=expected[1+i//10]*(x-c['point'][0] if i<10 else y-c['point'][0])%MOD
            self.assertEqual(actual,expected)
        self.assertEqual(system['input_term_counts'],[len(terms) for terms in system['input_terms']])
        self.assertGreater(r['useful_relations'],0)
        self.assertEqual(r['relation_counts']['nonzero_b'],r['useful_relations'])
        self.assertEqual(r['relation_counts']['nonzero_b_verification_failed'],0)
        accepted=[c for c in r['candidates'] if c['status']=='accepted']
        for w in r['witnesses']:
            u,v=(accepted[i] for i in w['indices']); su,sv=w['signs']
            self.assertEqual(add(mul(su%N,point(u['point'])),mul(sv%N,point(v['point']))),point(r['target']['point']))
            self.assertEqual((su*u['a']+sv*v['a']-r['target']['a'])%N,w['A'])
            self.assertEqual((su*u['b']+sv*v['b']-r['target']['b'])%N,w['B'])
            self.assertEqual(mul(w['scalar'],P),Q)

    def test_interval_filter(self):
        r=self.run_case('independent',['--x-min','0','--x-max','50','--modulus','2','--residue','0'])
        self.assertEqual(r['status'],'complete')
        self.assertGreater(r['counts']['filtered'],0)
        for c in r['candidates']:
            if c['status']=='accepted': self.assertTrue(0 <= c['point'][0] <= 50 and c['point'][0] % 2 == 0)

    def test_panel_pairs_targets(self):
        with tempfile.TemporaryDirectory() as d:
            plan=pathlib.Path(d)/'plan.json'; out=pathlib.Path(d)/'panel'
            plan.write_text(json.dumps({'common_args':['--p','101','--curve-a','1','--curve-b','32','--px','4','--py','10','--qx','82','--qy','27','--block-size','4'],'seeds':[42],'timeout_seconds':30}))
            subprocess.run(['python3',str(HERE/'panel.py'),'--plan',str(plan),'--out',str(out)],check=True,stdout=subprocess.DEVNULL)
            summary=json.loads((out/'summary.json').read_text())
            self.assertTrue(summary['target_pairing_by_seed']['42'])
            spec=importlib.util.spec_from_file_location('compare_panels', HERE/'compare_panels.py')
            compare=importlib.util.module_from_spec(spec); spec.loader.exec_module(compare)
            comparison=compare.compare(out,out)
            self.assertTrue(comparison['matching_checks_pass'])
            self.assertEqual(comparison['resource_metadata_status'],'metadata_match_only')
            self.assertIn('rss_native_unit',summary['resource_metadata_at_start'])
            self.assertTrue(summary['measure_source_unchanged'])
            self.assertTrue(summary['accounting_source_unchanged'])
            for mode in ('independent','walk','two-group'):
                self.assertEqual(summary['per_mode'][mode]['completed'],1)
                self.assertEqual(summary['per_mode'][mode]['attempts'],1)
                self.assertEqual(summary['per_mode'][mode]['driver_cpu_ns']['observed_attempts'],1)
                self.assertGreater(summary['per_mode'][mode]['driver_cpu_ns']['observed_sum'],0)
                self.assertEqual(summary['per_mode'][mode]['peak_rss_native']['observed_attempts'],1)
                self.assertGreater(summary['per_mode'][mode]['peak_rss_native']['observed_max'],0)
                self.assertEqual(comparison['per_mode'][mode]['left']['peak_rss_native_max']['unknown_attempts'],0)
                self.assertEqual(summary['per_mode'][mode]['phase_wall_ns']['generation']['observed_attempts'],1)
                self.assertEqual(summary['per_mode'][mode]['candidate_counts']['candidates']['unknown_attempts'],0)
                self.assertEqual(summary['per_mode'][mode]['generation_operations']['point_addition_calls']['unknown_attempts'],0)
                r=next(r for r in summary['runs'] if r['mode']==mode)
                checkpoint=out/('seed-42-'+mode+'.checkpoint.json')
                self.assertEqual(r['checkpoint_sha256'],hashlib.sha256(checkpoint.read_bytes()).hexdigest())
                self.assertEqual(r['stdout_sha256'],hashlib.sha256((out/('seed-42-'+mode+'.stdout')).read_bytes()).hexdigest())
                snapshot=json.loads(checkpoint.read_text())
                self.assertEqual(snapshot['phase'],'finished')
                self.assertEqual(snapshot['source_sha256_at_start'],summary['measure_sha256_at_start'])
                self.assertEqual(snapshot['accounting_sha256_at_start'],summary['accounting_sha256_at_start'])

    def test_checkpoint_survives_solver_failure(self):
        program = r"""
import argparse, importlib.util, pathlib, sys
sys.path.insert(0, str(pathlib.Path(sys.argv[1]).parent))
spec=importlib.util.spec_from_file_location('measure', sys.argv[1])
m=importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
original=m.PolynomialRing
class StopBeforeSolver:
    def __init__(self, ring): self.ring=ring
    def gens(self): return self.ring.gens()
    def ideal(self, *args): raise RuntimeError('injected solver failure')
def intercepted(*args, **kwargs): return StopBeforeSolver(original(*args, **kwargs))
m.PolynomialRing=intercepted
a=argparse.Namespace(p=101,curve_a=1,curve_b=32,px=4,py=10,qx=82,qy=27,mode='independent',seed=42,block_size=4,group_size=8,max_candidates=1000,modulus=1,residue=0,x_min=0,x_max=100,walk_length=8,output='unused',checkpoint=sys.argv[2],solver_protocol=True)
try: m.run(a)
except RuntimeError as exc:
    assert str(exc)=='injected solver failure'
else: raise AssertionError('failure was not injected')
"""
        with tempfile.TemporaryDirectory() as d:
            checkpoint=pathlib.Path(d)/'checkpoint.json'
            proc=subprocess.run(['sage','-python','-c',program,str(HERE/'measure.py'),str(checkpoint)],check=True,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL)
            self.assertIn(b'=== BEGIN GROEBNER BASIS PROTOCOL ===',proc.stdout)
            self.assertIn(b'=== END GROEBNER BASIS PROTOCOL ===',proc.stdout)
            r=json.loads(checkpoint.read_text())
            self.assertEqual(r['phase'],'groebner_basis')
            self.assertEqual(r['solver']['backend'],'sage/libsingular:std')
            self.assertEqual(r['status'],'running')
            self.assertEqual(len(r['candidates']),r['counts']['candidates'])
            self.assertEqual(len(r['system']['input_polynomials']),3)
            self.assertEqual(len(r['system']['input_terms']),3)
            self.assertIn('generation',r['timings'])
            self.assertIn('polynomial_construction',r['timings'])
            self.assertNotIn('groebner_basis',r['timings'])
            self.assertIsNone(r['metrics']['solving_degree'])

    def test_distinct_filter_panels_pair(self):
        with tempfile.TemporaryDirectory() as d:
            directory=pathlib.Path(d)
            common=['--p','101','--curve-a','1','--curve-b','32','--px','4','--py','10','--qx','82','--qy','27','--block-size','4']
            outputs=[]
            for name,extra in (('all',[]),('interval',['--x-max','70'])):
                plan=directory/(name+'.json'); out=directory/name
                plan.write_text(json.dumps({'common_args':common+extra,'seeds':[42],'timeout_seconds':30}))
                subprocess.run(['python3',str(HERE/'panel.py'),'--plan',str(plan),'--out',str(out)],check=True,stdout=subprocess.DEVNULL)
                outputs.append(out)
            spec=importlib.util.spec_from_file_location('compare_panels', HERE/'compare_panels.py')
            compare=importlib.util.module_from_spec(spec); spec.loader.exec_module(compare)
            result=compare.compare(*outputs)
            self.assertTrue(result['matching_checks_pass'],result['mismatch_or_unknown'])
            self.assertEqual(len(result['pairs']),3)
            self.assertTrue(all(row['target_match'] for row in result['pairs']))
            summary=json.loads((outputs[1]/'summary.json').read_text())
            self.assertGreater(sum(row['candidate_counts']['filtered'] for row in summary['runs']),0)

    def test_panel_preserves_timeout(self):
        with tempfile.TemporaryDirectory() as d:
            plan=pathlib.Path(d)/'plan.json'; out=pathlib.Path(d)/'panel'
            plan.write_text(json.dumps({'common_args':['--p','101','--curve-a','1','--curve-b','32','--px','4','--py','10','--qx','82','--qy','27'],'seeds':[42],'timeout_seconds':0.001}))
            subprocess.run(['python3',str(HERE/'panel.py'),'--plan',str(plan),'--out',str(out)],check=True,stdout=subprocess.DEVNULL)
            summary=json.loads((out/'summary.json').read_text())
            self.assertEqual(len(summary['runs']),3)
            for mode in ('independent','walk','two-group'):
                item=summary['per_mode'][mode]['phase_wall_ns']['groebner_basis']
                self.assertEqual(item, {'observed_attempts':0,'unknown_attempts':1,'observed_sum':None})
                self.assertEqual(summary['per_mode'][mode]['peak_rss_native']['unknown_attempts'],1)
                self.assertIsNone(summary['per_mode'][mode]['peak_rss_native']['observed_max'])
            for r in summary['runs']:
                self.assertEqual(r['status'],'timeout')
                self.assertIsNone(r['useful_relations'])
                self.assertGreater(r['whole_process_wall_ns'],0)
                self.assertTrue((out/('seed-42-'+r['mode']+'.receipt.json')).exists())

    def test_exhaustion_is_recorded(self):
        r=self.run_case('independent',['--max-candidates','1'])
        self.assertEqual(r['status'],'candidate_budget_exhausted'); self.assertEqual(r['counts']['candidates'],1)
        self.assertEqual(r['generation_operations']['point_addition_calls'],r['generation_operations']['target_draws']+1)
        self.assertNotIn('witnesses',r)

if __name__=='__main__': unittest.main()
