#!/usr/bin/env python3
"""Generate finite Lean certificates from the independently replayed R126 terms."""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('verify126', HERE / 'verify126.py')
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)

lines = [
    'import Std.Tactic', '', 'namespace N51Valuations126',
    'def mul4 (a b : Nat) : Nat :=',
    '  match a, b with',
    '  | 0, _ => 0 | _, 0 => 0',
    '  | 1, x => x | x, 1 => x',
    '  | 2, 2 => 3 | 2, 3 => 1 | 3, 2 => 1 | 3, 3 => 2',
    '  | _, _ => 0',
    'def plus (m : Nat) (a b : List Nat) : List Nat :=',
    '  (List.range m).map fun i => Nat.xor (a.getD i 0) (b.getD i 0)',
    'def times (m : Nat) (a b : List Nat) : List Nat :=',
    '  (List.range m).map fun k =>',
    '    (List.range (k+1)).foldl (fun z i => Nat.xor z (mul4 (a.getD i 0) (b.getD (k-i) 0))) 0',
    'def one (m : Nat) : List Nat := 1 :: List.replicate (m-1) 0',
    'def zero (m : Nat) : List Nat := List.replicate m 0',
    'def pow (m : Nat) (w : List Nat) (j : Nat) : List Nat :=',
    '  (List.range j).foldl (fun acc _ => times m acc w) (one m)',
    'def eval (m : Nat) (rows : List (Nat × List Nat)) (w : List Nat) : List Nat :=',
    '  rows.foldl (fun acc p => plus m acc (times m p.2 (pow m w p.1))) (zero m)',
    ''
]

for key, stage_number in v.P['terminal_stage'].items():
    for recorded in v.OUT['branches'][key]:
        alpha = recorded['alpha']
        previous = next(e for e in v.data[125]['branches'][key] if e['alpha'] == alpha)
        stage = next(e for e in v.data[stage_number]['branches'][key] if e['alpha'] == alpha)
        degree = recorded['first_nonzero_offset']
        m = degree + 1
        q = recorded['q']
        terminal = {j: {ex: a for ex, a in row['nonzero_support_below_bound']}
                    for j, row in zip((0, 1, 4, 8, 16), stage['translated_rows'])}
        g = v.normalize(terminal, q, recorded['g_edge'], degree)
        u = [0] * m
        for k in range(1, m):
            u[k] = v.value(g, [1] + u[1:], k)[k]
        w = [1] + u[1:]
        n_coeff, _ = v.translated_n(key, alpha, v.data, [tuple(x) for x in previous['substitution_path']])
        n = v.normalize(dict(enumerate(n_coeff)), q, recorded['n_candidate_order'], degree)
        suffix = key + '_a' + str(alpha)
        rows = lambda table: '[' + ', '.join('(' + str(j) + ', ' + str(coeff) + ')' for j, coeff in table.items() if any(coeff)) + ']'
        lines += [
            f'def w_{suffix} : List Nat := {w}',
            f'def g_{suffix} : List (Nat × List Nat) := {rows(g)}',
            f'def n_{suffix} : List (Nat × List Nat) := {rows(n)}',
            f'theorem g_{suffix}_zero : eval {m} g_{suffix} w_{suffix} = zero {m} := by native_decide',
            f'theorem n_{suffix}_first : (eval {m} n_{suffix} w_{suffix}).take {degree} = zero {degree} ∧',
            f'    (eval {m} n_{suffix} w_{suffix}).getD {degree} 0 = {recorded["first_nonzero_F4"]} := by native_decide',
            ''
        ]
lines += [
    'theorem x0_degree_weighted_sum :',
    '    (2 : Int) * ((-30) + (-21) + 0 + 0 + (-141) + (-78) + 42 + 49) = -358 := by native_decide',
    '', 'end N51Valuations126', ''
]
dest = HERE / 'lean' / 'N51Valuations126.lean'
dest.parent.mkdir(exist_ok=True)
dest.write_text('\n'.join(lines))
print(dest, dest.stat().st_size, 'bytes')
