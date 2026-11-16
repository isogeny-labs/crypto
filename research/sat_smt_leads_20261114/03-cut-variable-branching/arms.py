import time, threading
from pysat.solvers import Cadical195
from pysat.engines import Propagator

class CutProp(Propagator):
    def __init__(s, cut, decide_cut):
        s.cut = list(cut); s.cs = set(cut); s.decide_cut = decide_cut
        s.val = {}; s.fixed = set(); s.levels = [[]]; s.ndec = 0
    def on_assignment(s, lit, fixed=False):
        v = abs(lit)
        if v in s.cs:
            s.val[v] = lit
            if fixed: s.fixed.add(v)
            else: s.levels[-1].append(v)
    def on_new_level(s): s.levels.append([])
    def on_backtrack(s, to):
        while len(s.levels) > to + 1:
            for v in s.levels.pop():
                if v not in s.fixed: s.val.pop(v, None)
    def check_model(s, model): return True
    def decide(s):
        if not s.decide_cut: return 0
        for v in s.cut:
            if v not in s.val: s.ndec += 1; return -v
        return 0
    def propagate(s): return []
    def provide_reason(s, lit): return []
    def add_clause(s): return []

ARMS = {
 'D':      dict(prop=False, cut=False, opts={}),
 'Dp':     dict(prop=True,  cut=False, opts={}),
 'C':      dict(prop=True,  cut=True,  opts={}),
 'C-lean': dict(prop=True,  cut=True,  opts=dict(reduceint=10, reducetarget=100, reducetier1glue=1, reducetier2glue=1)),
 'C-keep': dict(prop=True,  cut=True,  opts=dict(reduce=0)),
}

def run_arm(arm, clauses, cut, cap_conf):
    a = ARMS[arm]
    s = Cadical195()
    if a['opts']: s.configure(a['opts'])
    s.append_formula(clauses)
    p = None
    if a['prop']:
        p = CutProp(cut, a['cut']); s.connect_propagator(p)
        for v in cut: s.observe(v)
    s.conf_budget(cap_conf)
    t = time.time(); r = s.solve_limited(); dt = time.time() - t
    st = dict(s.accum_stats())
    model = s.get_model() if r else None
    s.delete()
    return dict(arm=arm, result={True: 'SAT', False: 'UNSAT', None: 'CAP'}[r], wall_s=round(dt, 3),
                cut_decisions=(p.ndec if p else 0), **st), model
