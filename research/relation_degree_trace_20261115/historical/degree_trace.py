#!/usr/bin/env python3
"""Exact degree-driven Macaulay closure and redundant-relation ablations."""
import argparse
import csv
import json
from dataclasses import dataclass
from functools import lru_cache
from itertools import combinations_with_replacement
from pathlib import Path
from math import comb
from time import perf_counter

import sympy as sp


def degree(f):
    return max(map(sum, f), default=-1)


def key(m):
    return (sum(m), tuple(-x for x in reversed(m)))  # graded reverse lexicographic


def axpy(a, b, scale, p):
    for m, c in b.items():
        v = (a.get(m, 0) + scale*c) % p
        if v:
            a[m] = v
        else:
            a.pop(m, None)


def shifted(f, m, p, scale=1):
    return {tuple(x+y for x, y in zip(e, m)): c*scale % p for e, c in f.items()}


@lru_cache(None)
def monomials(n, d, exact=False):
    result = []
    for k in ([d] if exact else range(d+1)):
        for indices in combinations_with_replacement(range(n), k):
            m = [0]*n
            for i in indices:
                m[i] += 1
            result.append(tuple(m))
    return tuple(sorted(result, key=key, reverse=True))


def product(a, b, p):
    result = {}
    for m, c in a.items():
        axpy(result, shifted(b, m, p), c, p)
    return result


def normalized(f, p):
    if not f:
        return {}
    c = pow(f[max(f, key=key)], -1, p)
    return {m: v*c % p for m, v in f.items()}


def fingerprint(f, p):
    return tuple(sorted(normalized(f, p).items()))


def encode(f):
    return [{'exponents': list(m), 'coefficient': c} for m, c in sorted(f.items(), key=lambda t: key(t[0]), reverse=True)]


@dataclass
class Row:
    poly: dict
    proof: dict  # (input index, multiplier monomial) -> coefficient


def proof_shift(proof, monomial):
    return {(i, tuple(x+y for x, y in zip(m, monomial))): c for (i, m), c in proof.items()}


class Space:
    def __init__(self, p):
        self.p = p
        self.pivots = {}
        self.reductions = 0

    def reduce(self, poly, proof=None):
        poly = dict(poly)
        proof = {} if proof is None else dict(proof)
        while poly:
            lead = max(poly, key=key)
            basis = self.pivots.get(lead)
            if basis is None:
                break
            scale = -poly[lead]
            axpy(poly, basis.poly, scale, self.p)
            axpy(proof, basis.proof, scale, self.p)
            self.reductions += 1
        return Row(poly, proof)

    def add(self, poly, proof=None):
        row = self.reduce(poly, proof)
        if not row.poly:
            return None
        lead = max(row.poly, key=key)
        inv = pow(row.poly[lead], -1, self.p)
        row.poly = {m: c*inv % self.p for m, c in row.poly.items()}
        row.proof = {m: c*inv % self.p for m, c in row.proof.items()}
        self.pivots[lead] = row
        return row

    def contains(self, f):
        return not self.reduce(f).poly


def verify_proof(row, inputs, p):
    result = {}
    for (i, m), c in row.proof.items():
        axpy(result, shifted(inputs[i], m, p), c, p)
    assert result == row.poly, 'Invalid ideal-membership certificate'


def groebner_reference(inputs, variables, p):
    symbols = sp.symbols(' '.join(variables), seq=True)
    exprs = [sum(c*sp.prod(x**e for x, e in zip(symbols, m)) for m, c in f.items()) for f in inputs]
    gb = sp.groebner(exprs, *symbols, modulus=p, order='grevlex')
    return [{m: int(c) % p for m, c in f.terms()} for f in gb.polys]


def trace_solve(inputs, p, reference, max_degree, max_columns=5000):
    """Degree-D row spaces, closed under monomial multiplication after degree falls.
    Stop when the space contains the independently computed reference GB.
    This is an explicit algorithm-dependent solving-degree measurement.
    """
    n = len(next(iter(inputs[0])))
    zero = (0,)*n
    space = Space(p)
    # Sources include original equations, not only their reduced residuals.
    sources = [Row(dict(f), {(i, zero): 1}) for i, f in enumerate(inputs)]
    source_fingerprints = {fingerprint(f, p) for f in inputs}
    processed = set()
    events, archive = [], []
    started = perf_counter()
    oracle_seconds = 0.0
    min_input = min(map(degree, inputs))
    for d in range(min_input, max_degree+1):
        cols = comb(n+d, n)
        if cols > max_columns:
            return {'status': 'column_budget', 'solving_degree': None, 'trace': events,
                    'elapsed_seconds': perf_counter()-started-oracle_seconds,
                    'membership_oracle_seconds_excluded': oracle_seconds}, archive
        rank_before = len(space.pivots)
        reductions_before = space.reductions
        rows = nnz = falls = rounds = 0
        certificate_degree = 0
        new_degrees = {}
        while True:
            rounds += 1
            pending = []
            for sid, source in enumerate(sources):
                if degree(source.poly) > d:
                    continue
                for m in monomials(n, d-degree(source.poly)):
                    tag = sid, m
                    if tag in processed:
                        continue
                    processed.add(tag)
                    pending.append((sid, source, m))
            if not pending:
                break
            for sid, source, m in pending:
                f = shifted(source.poly, m, p)
                rows += 1
                nnz += len(f)
                row = space.add(f, proof_shift(source.proof, m))
                if row is None:
                    continue
                verify_proof(row, inputs, p)
                certificate_degree = max(certificate_degree, max((degree(inputs[i])+sum(m) for i, m in row.proof), default=0))
                rd = degree(row.poly)
                new_degrees[str(rd)] = new_degrees.get(str(rd), 0)+1
                if rd < degree(f):
                    falls += 1
                fp = fingerprint(row.poly, p)
                if fp not in source_fingerprints:
                    source_fingerprints.add(fp)
                    sources.append(row)
                    archive.append((d, row))
        # This is a membership oracle used to measure completion, not a source of rows.
        oracle_start = perf_counter()
        solver_reductions = space.reductions
        complete = all(space.contains(f) for f in reference)
        oracle_reductions = space.reductions-solver_reductions
        space.reductions = solver_reductions
        oracle_seconds += perf_counter()-oracle_start
        events.append({'degree': d, 'columns': cols, 'new_rows_processed': rows,
                       'new_row_nonzeros': nnz, 'rank': len(space.pivots),
                       'max_new_certificate_product_degree': certificate_degree,
                       'new_pivots': len(space.pivots)-rank_before,
                       'row_reductions': space.reductions-reductions_before,
                       'degree_falls': falls, 'closure_rounds': rounds-1,
                       'new_pivot_degree_histogram': new_degrees,
                       'reference_gb_contained': complete,
                       'membership_oracle_reductions_excluded': oracle_reductions,
                       'elapsed_seconds': round(perf_counter()-started-oracle_seconds, 6)})
        if complete:
            return {'status': 'complete', 'solving_degree': d, 'trace': events,
                    'elapsed_seconds': perf_counter()-started-oracle_seconds,
                    'membership_oracle_seconds_excluded': oracle_seconds}, archive
    return {'status': 'degree_cap', 'solving_degree': None, 'trace': events,
            'elapsed_seconds': perf_counter()-started-oracle_seconds,
                    'membership_oracle_seconds_excluded': oracle_seconds}, archive


def hilbert_trace(inputs, p, max_degree, max_columns=5000):
    """Hilbert function of J generated by the highest homogeneous parts.
    For Artinian J, first H(D)=0 is d_reg(J); otherwise only a truncated trace.
    """
    n = len(next(iter(inputs[0])))
    tops = [{m: c for m, c in f.items() if sum(m) == degree(f)} for f in inputs]
    result = []
    for d in range(max_degree+1):
        cols = comb(n+d-1, n-1)
        if cols > max_columns:
            break
        space = Space(p)
        for f in tops:
            if degree(f) <= d:
                for m in monomials(n, d-degree(f), True):
                    space.add(shifted(f, m, p))
        h = cols - len(space.pivots)
        result.append({'degree': d, 'monomials': cols, 'rank': len(space.pivots), 'hilbert_function': h})
        if h == 0:
            return {'top_form_d_reg': d, 'quotient_regularity_note': 'For this Artinian homogeneous quotient, reg(S/J)=d_reg-1.',
                    'status': 'vanished', 'trace': result}
    return {'top_form_d_reg': None, 'status': 'not_vanished_within_cap', 'trace': result}


def ec_system(p):
    """Two Semaev f3 equations with a shared x-coordinate pair and known witness."""
    a, b = 2, 3
    def add(left, right):
        if left is None:
            return right
        if right is None:
            return left
        x, y = left
        u, v = right
        if x == u and (y+v) % p == 0:
            return None
        slope = ((3*x*x+a)*pow(2*y, -1, p) if left == right else (v-y)*pow((u-x) % p, -1, p)) % p
        z = (slope*slope-x-u) % p
        return z, (slope*(x-z)-y) % p
    g = (3, 6)
    r, s = add(g, g), add(add(g, g), g)
    constants = [add(r, s)[0], g[0]]
    x, y = sp.symbols('x y')
    inputs = []
    for z in constants:
        expr = (x-y)**2*z*z-2*((x+y)*(x*y+a)+2*b)*z+(x*y-a)**2-4*b*(x+y)
        f = {m: int(c) % p for m, c in sp.Poly(expr, x, y, modulus=p).terms()}
        assert sum(c*pow(r[0], m[0], p)*pow(s[0], m[1], p) for m, c in f.items()) % p == 0
        inputs.append(f)
    return inputs, ['x1', 'x2'], {'curve_a': a, 'curve_b': b,
        'witness': [r[0], s[0]], 'f3_constants': constants,
        'meaning': 'Intersection of f3(x1,x2,c_plus) and f3(x1,x2,c_minus); no factor-base or discrete-log constraints.'}


def score(run):
    return (run['solving_degree'] if run['solving_degree'] is not None else float('inf'),
            run.get('top_forms', {}).get('top_form_d_reg') if run.get('top_forms', {}).get('top_form_d_reg') is not None else float('inf'),
            sum(t['new_rows_processed'] for t in run['trace']),
            sum(t['new_row_nonzeros'] for t in run['trace']))


def analyze(inputs, variables, p, max_degree=8, candidate_limit=8):
    if not sp.isprime(p):
        raise ValueError('Modulus must be prime; 2**53 is not a prime-field modulus')
    t = perf_counter()
    reference = groebner_reference(inputs, variables, p)
    oracle_seconds = perf_counter()-t
    def evaluate(equations):
        run, rows = trace_solve(equations, p, reference, max_degree)
        run['top_forms'] = hilbert_trace(equations, p, max_degree)
        return run, rows
    baseline, archive = evaluate(inputs)
    # Choose sparse, low-degree certified consequences, not arbitrary new constraints.
    initial = {fingerprint(f, p) for f in inputs}
    pool = []
    for discovered, row in archive:
        fp = fingerprint(row.poly, p)
        if fp not in initial:
            initial.add(fp)
            pool.append((discovered, row))
    pool.sort(key=lambda item: (degree(item[1].poly), len(item[1].poly), item[0], fingerprint(item[1].poly, p)))
    pool = pool[:candidate_limit]
    candidates = []
    for i, (discovered, row) in enumerate(pool):
        verify_proof(row, inputs, p)
        run, _ = evaluate(inputs+[row.poly])
        proof = [{'input': j, 'multiplier_exponents': list(m), 'coefficient': c}
                 for (j, m), c in sorted(row.proof.items())]
        candidates.append({'id': f'c{i}', 'polynomial': encode(row.poly),
                           'degree': degree(row.poly), 'terms': len(row.poly),
                           'discovered_at_degree': discovered,
                           'certificate_max_product_degree': max((degree(inputs[j])+sum(m) for j, m in row.proof), default=0),
                           'membership_certificate': proof,
                           'single_addition': run,
                           'top_forms': hilbert_trace(inputs+[row.poly], p, max_degree)})
    all_inputs = inputs+[row.poly for _, row in pool]
    all_run, _ = evaluate(all_inputs)
    loo = []
    for i in range(len(pool)):
        run, _ = evaluate(inputs+[row.poly for j, (_, row) in enumerate(pool) if i != j])
        loo.append({'removed': f'c{i}', 'solve': run})
    selected, steps = [], []
    current = baseline
    while len(selected) < len(pool):
        choices = []
        for i, (_, row) in enumerate(pool):
            if i in selected:
                continue
            run, _ = evaluate(inputs+[pool[j][1].poly for j in selected]+[row.poly])
            choices.append((score(run), i, run))
        if not choices:
            break
        best_score, i, best = min(choices, key=lambda c: (c[0], c[1]))
        if best_score >= score(current):
            break
        selected.append(i)
        steps.append({'added': f'c{i}', 'solve': best})
        current = best
    return {'prime': p, 'variables': variables, 'input': [encode(f) for f in inputs],
            'reference_groebner_basis': [encode(f) for f in reference],
            'reference_gb_max_degree': max(map(degree, reference)),
            'reference_oracle_seconds_excluded': oracle_seconds,
            'baseline': baseline, 'baseline_top_forms': hilbert_trace(inputs, p, max_degree),
            'candidate_discovery': {'method': 'baseline Macaulay closure, certified row consequences',
                 'max_degree_paid': max((t['degree'] for t in baseline['trace']), default=None),
                 'rows_paid': sum(t['new_rows_processed'] for t in baseline['trace']),
                 'elapsed_seconds_paid': baseline['elapsed_seconds'],
                 'free_online_preprocessing': False},
            'candidates': candidates, 'all_candidates': all_run,
            'all_candidates_top_forms': hilbert_trace(all_inputs, p, max_degree),
            'leave_one_out': loo, 'greedy_selected': [f'c{i}' for i in selected],
            'greedy_trace': steps, 'greedy_final': current,
            'greedy_objective': ['observed solving degree', 'witnessed top-form d_reg (unwitnessed sorts last)', 'rows processed', 'row nonzeros'],
            'end_to_end_complete': baseline['status'] == current['status'] == 'complete',
            'end_to_end_peak_degree': max((t['degree'] for run in [baseline, current] for t in run['trace']), default=None)}


def load_system(path):
    obj = json.loads(Path(path).read_text())
    p, names = int(obj['prime']), obj['variables']
    if not names or len(set(names)) != len(names):
        raise ValueError('Variables must be distinct and nonempty')
    polys = []
    for terms in obj['polynomials']:
        f = {}
        for term in terms:
            m = tuple(term['exponents'])
            if len(m) != len(names) or any(type(e) is not int or e < 0 for e in m):
                raise ValueError('Invalid monomial exponents')
            axpy(f, {m: int(term['coefficient']) % p}, 1, p)
        if f:
            polys.append(f)
    if not polys:
        raise ValueError('Need nonzero equations')
    return polys, names, p


def write_csv(reports, path):
    with path.open('w', newline='') as handle:
        writer = csv.DictWriter(handle, fieldnames=['prime', 'scenario', 'degree', 'columns', 'new_rows_processed',
              'new_row_nonzeros', 'rank', 'new_pivots', 'degree_falls', 'row_reductions', 'reference_gb_contained'])
        writer.writeheader()
        for report in reports:
            runs = [('baseline', report['baseline']), ('all_candidates', report['all_candidates']), ('greedy_final', report['greedy_final'])]
            runs += [(c['id'], c['single_addition']) for c in report['candidates']]
            for label, run in runs:
                for t in run['trace']:
                    writer.writerow({**{k: t[k] for k in writer.fieldnames if k in t}, 'prime': report['prime'], 'scenario': label})


def plot(reports, path):
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(len(reports), 2, figsize=(11, 3.5*len(reports)), squeeze=False)
    for row, report in zip(axes, reports):
        for label, run in [('Original equations', report['baseline']), ('With selected consequences', report['greedy_final'])]:
            trace = run['trace']
            row[0].plot([t['degree'] for t in trace], [t['rank'] for t in trace], 'o-', label=label)
            row[1].plot([t['degree'] for t in trace], [t['degree_falls'] for t in trace], 'o-', label=label)
        for ax, ylabel in zip(row, ['Accumulated Macaulay rank', 'Degree falls at this degree']):
            ax.set(xlabel='Matrix degree', ylabel=ylabel, title=f"Prime {report['prime']}")
            ax.legend(fontsize=8)
            ax.grid(alpha=.2)
    fig.tight_layout()
    fig.savefig(path)
    plt.close(fig)


def effect_plot(reports, path):
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(len(reports), 2, figsize=(12, 3.7*len(reports)), squeeze=False)
    for row, report in zip(axes, reports):
        labels = ['base']+[c['id'] for c in report['candidates']]+['selected']
        runs = [report['baseline']]+[c['single_addition'] for c in report['candidates']]+[report['greedy_final']]
        degrees = [run['solving_degree'] for run in runs]
        row[0].bar(labels, [v if v is not None else 0 for v in degrees], color=['#b6beca']+['#627bb5']*len(report['candidates'])+['#26836b'])
        for i, value in enumerate(degrees):
            if value is None:
                row[0].text(i, 0.2, 'capped', rotation=90, ha='center')
        row[0].set(xlabel='Added consequence (each c tested alone)', ylabel='Observed solving degree',
                   title=f"Prime {report['prime']}", ylim=(0, max((d for d in degrees if d is not None), default=0)+1))
        traces = [run['top_forms']['trace'] for run in runs]
        length = max(map(len, traces))
        data = [[next((v['hilbert_function'] for v in trace if v['degree'] == d), 0) for d in range(length)] for trace in traces]
        image = row[1].imshow(data, aspect='auto', cmap='Blues', vmin=0)
        row[1].set(xticks=range(length), yticks=range(len(labels)), yticklabels=labels,
                   xlabel='Homogeneous degree', title='Hilbert function of top-form ideal')
        for i, values in enumerate(data):
            for j, value in enumerate(values):
                row[1].text(j, i, str(value), ha='center', va='center', fontsize=8, color='white' if value >= 3 else 'black')
        fig.colorbar(image, ax=row[1], fraction=.04)
    fig.tight_layout()
    fig.savefig(path)
    plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', help='JSON with prime, variables, polynomials as term arrays')
    parser.add_argument('--primes', nargs='+', type=int, default=[2**31-1, 9007199254740881])
    parser.add_argument('--max-degree', type=int, default=8)
    parser.add_argument('--candidate-limit', type=int, default=8)
    parser.add_argument('--output-dir', default='degree-results')
    parser.add_argument('--no-plot', action='store_true')
    args = parser.parse_args()
    if args.max_degree < 0 or args.candidate_limit < 0:
        parser.error('Degree and candidate limit must be nonnegative')
    output = Path(args.output_dir)
    output.mkdir(parents=True, exist_ok=True)
    if args.input:
        f, names, p = load_system(args.input)
        systems = [(f, names, p, {'source': args.input})]
    else:
        systems = []
        for p in args.primes:
            if not sp.isprime(p):
                parser.error(f'{p} is not prime')
            f, names, metadata = ec_system(p)
            systems.append((f, names, p, metadata))
    reports = []
    for f, names, p, metadata in systems:
        report = analyze(f, names, p, args.max_degree, args.candidate_limit)
        report['system'] = metadata
        reports.append(report)
        (output/f'system-{p}.json').write_text(json.dumps({'prime': p, 'variables': names, 'polynomials': [encode(g) for g in f]}, indent=2)+'\n')
        print(f"p={p}: baseline degree {report['baseline']['solving_degree']}; selected {report['greedy_selected']}; "
              f"degree after selection {report['greedy_final']['solving_degree']}; "
              f"including discovery peak {report['end_to_end_peak_degree']}", flush=True)
    (output/'trace.json').write_text(json.dumps({'definition': 'Observed degree-driven affine Macaulay closure to grevlex GB membership; not an implementation-independent invariant.',
        'regularity_definition': 'First vanishing Hilbert degree of the ideal generated by top homogeneous input parts, when witnessed. Not Castelnuovo-Mumford regularity of the affine ideal.',
        'reports': reports}, indent=2)+'\n')
    write_csv(reports, output/'trace.csv')
    with (output/'relation-effects.csv').open('w', newline='') as handle:
        writer = csv.writer(handle)
        writer.writerow(['prime', 'scenario', 'observed_solving_degree', 'top_form_d_reg_if_witnessed', 'rows_processed', 'discovered_at_matrix_degree', 'certificate_product_degree'])
        for report in reports:
            entries = [('baseline', report['baseline'], '', ''), ('selected', report['greedy_final'], '', '')]
            entries += [(c['id'], c['single_addition'], c['discovered_at_degree'], c['certificate_max_product_degree']) for c in report['candidates']]
            entries += [('remove_'+item['removed'], item['solve'], '', '') for item in report['leave_one_out']]
            for label, run, discovered, certificate_degree in entries:
                writer.writerow([report['prime'], label, run['solving_degree'], run['top_forms']['top_form_d_reg'],
                    sum(t['new_rows_processed'] for t in run['trace']), discovered, certificate_degree])
    if not args.no_plot:
        plot(reports, output/'degree-profile.svg')
        effect_plot(reports, output/'relation-effects.svg')


if __name__ == '__main__':
    main()
