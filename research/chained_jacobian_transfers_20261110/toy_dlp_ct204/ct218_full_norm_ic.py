#!/usr/bin/env python3
"""CT218: exact finite norm-projected relation collection for public F101 ECDLP.

Unlike CT217's norm-log recovery, this source solves factor-base logarithms by
linear algebra over F109. No elliptic DLP oracle/table is consulted by the
algorithm. A full scalar-recovery verdict needs rank, public-point verification,
and complete cost evidence; this file supplies finite proof-of-concept counts,
not a physical cold-cost comparison or production Rust implementation.
"""
import argparse
import itertools
import json

import ct216_genus4_coset_screen as c
import ct217_norm_relation_audit as n

P, R, G = c.P, c.R, n.G


def build_base():
    xs = {}
    for x in range(P):
        ys = [y for y in range(P) if (y*y-c.evalp(c.F,x))%P == 0]
        if ys:
            assert len(ys) == 2 and sum(ys) % P == 0
            xs[x] = min(ys)
    assert len(xs) == 54 and 2*len(xs) == R-1
    return xs


class Solver:
    def __init__(self, xs):
        self.index = {x:i for i,x in enumerate(sorted(xs))}
        self.positive = xs
        self.cols = len(xs)
        self.basis = {}
        self.la_modular_muladds = 0
        self.rows_seen = 0
        self.consistent_dependent_rows = 0

    def vector(self, support, offset):
        row = [0]*self.cols
        for p,coefficient in itertools.chain(
            ((pt,1) for pt in support), ((pt,-1) for pt in offset)
        ):
            x,y=p
            assert x in self.index and (y==self.positive[x] or y==(-self.positive[x])%P)
            sign=1 if y==self.positive[x] else -1
            row[self.index[x]] = (row[self.index[x]]+coefficient*sign)%R
        return row

    def add(self, row, rhs):
        v = list(row)+[rhs%R]
        self.rows_seen += 1
        for pivot in sorted(self.basis):
            value = v[pivot]
            if value:
                old = self.basis[pivot]
                v = [(a-value*b)%R for a,b in zip(v,old)]
                self.la_modular_muladds += len(v)
        pivot = next((i for i in range(self.cols) if v[i]), None)
        if pivot is None:
            assert v[-1]==0, ("inconsistent equation",self.rows_seen)
            self.consistent_dependent_rows += 1
            return False
        inv=pow(v[pivot],-1,R)
        self.basis[pivot] = [(z*inv)%R for z in v]
        self.la_modular_muladds += len(v)
        return True

    def solve(self):
        if len(self.basis)!=self.cols:
            return None
        mat=[self.basis[k][:] for k in sorted(self.basis)]
        for col in range(self.cols):
            pivot=next((i for i in range(col,self.cols) if mat[i][col]),None)
            assert pivot is not None
            mat[pivot],mat[col]=mat[col],mat[pivot]
            inv=pow(mat[col][col],-1,R)
            mat[col]=[(x*inv)%R for x in mat[col]]
            self.la_modular_muladds += len(mat[col])
            for i in range(self.cols):
                if i!=col and mat[i][col]:
                    factor=mat[i][col]
                    mat[i]=[(a-factor*b)%R for a,b in zip(mat[i],mat[col])]
                    self.la_modular_muladds+=len(mat[i])
        assert all(mat[i][j]==int(i==j) for i in range(self.cols) for j in range(self.cols))
        return [row[-1] for row in mat]

    def known_sum(self, pts, logs):
        return sum(
            (1 if y==self.positive[x] else -1)*logs[self.index[x]]
            for x,y in pts
        ) % R


def perform():
    base=build_base()
    solver=Solver(base)
    # The only seed known log: the rational lift of public source G.
    generators=[(x,G[1]) for x in base if pow(x,3,P)==G[0]]
    assert len(generators)==1
    source_preimage=generators[0]
    solver.add(solver.vector([source_preimage],[]),1)

    j=[c.IDENTITY]
    for _ in range(1,R):
        j.append(c.cantor(j[-1],c.image(G)))
    assert c.cantor(j[-1],c.image(G))==c.IDENTITY
    first=[entry[1][0] for entry in n.roster()[:8]]
    signed=first+[(x,(-y)%P) for x,y in first]
    assert len({p for p in signed})==16
    seen_offsets=set()
    attempts=0
    relation_count=0
    independent_relations=0
    offset_count=0
    offset_composition_adds=0
    norm_control_checked=0
    stages=[]
    registered_offsets=[]

    for label, seeds, sizes in [
        ('A',first,(2,3,4)),
        ('B',signed,(2,3))
    ]:
        stage_start_rank=len(solver.basis)
        stage_start_attempts=attempts
        for size in sizes:
            for group in itertools.combinations(seeds,size):
                d0=c.sums(group)
                if d0 in seen_offsets:
                    continue
                seen_offsets.add(d0)
                registered_offsets.append((label,group,d0))
                offset_count+=1
                offset_composition_adds+=len(group)
                norm_offset=n.ecsum(map(n.norm_point,group))
                for k, sk in enumerate(j):
                    trial=c.cantor(d0,sk)
                    attempts+=1
                    support=n.split_support(trial)
                    if support is None:
                        continue
                    # Check every accepted relation against the distinct
                    # elliptic norm arithmetic, including its fixed offset.
                    assert n.ecsum(map(n.norm_point,support))==c.ecadd(norm_offset,n.ecmul(3*k,G))
                    norm_control_checked+=1
                    relation_count+=1
                    row=solver.vector(support,group)
                    independent_relations+=int(solver.add(row,3*k))
                    if len(solver.basis)==solver.cols:
                        break
                if len(solver.basis)==solver.cols:
                    break
            if len(solver.basis)==solver.cols:
                break
        stages.append({'stage':label,'rank_at_end':len(solver.basis),
                       'new_attempts':attempts-stage_start_attempts,
                       'rank_gain':len(solver.basis)-stage_start_rank})
        if len(solver.basis)==solver.cols:
            break

    logs=solver.solve()
    target_output={}
    if logs is not None:
        # This independent verification oracle is called only AFTER solving.
        for x,y in base.items():
            image=n.norm_point((x,y))
            assert n.ecmul(logs[solver.index[x]],G)==image

        # Public challenge; the scalar below is used only for fixture setup
        # and final verification, never to build matrix/target relation.
        fixture_scalar=37
        target=n.ecmul(fixture_scalar,G)
        jt=c.image(target)
        candidate_count=0
        target_adds=0
        found=False
        for label,points,d0 in registered_offsets:
            start=c.cantor(d0,jt)
            target_adds+=1
            for a,sk in enumerate(j):
                v=c.cantor(start,sk)
                target_adds+=1
                candidate_count+=1
                support=n.split_support(v)
                if support is None:
                    continue
                recovered=((solver.known_sum(support,logs)-solver.known_sum(points,logs)-3*a)*pow(3,-1,R))%R
                assert n.ecmul(recovered,G)==target and recovered==fixture_scalar
                target_output={'target':list(target),
                    'recovered_scalar':recovered,
                    'target_offset_stage':label,
                    'target_candidate_trials':candidate_count,
                    'target_jacobian_additions':target_adds,
                    'target_walk_a':a}
                found=True
                break
            if found:
                break
        assert found, "target has no smooth relation among registered offsets"

    return {
      'schema':'ct218-f101-norm-projected-full-ic-v1',
      'pcc':'unverified',
      'full_factor_base_columns':solver.cols,
      'factor_base_signed_negation_fold':True,
      'generator_anchor_count':1,
      'relations_tested':attempts,
      'smooth_relations':relation_count,
      'independent_relation_rows_after_anchor':independent_relations,
      'matrix_rank':len(solver.basis),
      'matrix_nullity':solver.cols-len(solver.basis),
      'dependent_rows_consistency_checked':solver.consistent_dependent_rows,
      'offset_cosets_generated':offset_count,
      'offset_composition_jacobian_adds':offset_composition_adds,
      'relation_walk_jacobian_adds':attempts,
      'generator_walk_jacobian_adds':R-1,
      'norm_identities_checked':norm_control_checked,
      'la_modular_multiply_adds':solver.la_modular_muladds,
      'stages':stages,
      'factor_base_all_logs_verified_on_public_E':logs is not None,
      'target_result':target_output,
      'standing':'finite norm-projected IC toy only; no Rust/host cold cost advantage',
    }


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--self-test',action='store_true')
    args=parser.parse_args()
    result=perform()
    if args.self_test:
        assert result['matrix_rank']>30
        print('CT218_SELFTEST',result['matrix_rank'],'/',result['full_factor_base_columns'],
              'smooth',result['smooth_relations'],'target',bool(result['target_result']))
    else:
        print(json.dumps(result,sort_keys=True,indent=2))
