#!/usr/bin/env python3
"""Sparse signed-column target relation check for the frozen n35 candidate."""
import random


def ic_online(full, base, reps, encoding, G, index, logs, target):
    rng = random.Random(73035)
    clock = full.OnlineClock()
    shifts = []
    for shift_number in range(full.SPEC['target_shift_cap']):
        shift = 0 if shift_number == 0 else rng.randrange(1, full.R)
        shifted = target + shift * G if shift else target
        clock.switch('target_pdp')
        indices, attempts = full.lookup(index, base, shifted, rng)
        shifts.append({'shift_number': shift_number, 'shift_scalar': shift,
                       'third_point_attempts': attempts,
                       'status': 'exhausted' if indices is None else 'verified'})
        if indices is None:
            clock.switch('target_query')
            continue
        a, b, c = indices
        clock.switch('target_relation_check')
        assert base[a] + base[b] + base[c] == shifted
        row = full.row_for(indices, base, encoding)
        signed = {}
        for i in indices:
            col, sign = encoding[base[i]]
            signed[col] = signed.get(col, 0) + sign
        assert len(signed) <= 3
        assert all(row[j] == signed.get(j, 0) % full.R for j in range(len(reps)))
        assert sum((v * reps[j] for j, v in signed.items()), G.curve()(0)) == shifted
        clock.switch('target_descent')
        scalar = (sum(v * logs[j] for j, v in enumerate(row)) - shift) % full.R
        clock.switch('target_recovery_check')
        assert scalar * G == target
        elapsed = clock.finish()
        return {'status': 'verified', 'scalar': scalar, 'wall_ns': elapsed,
                'phase_wall_ns': clock.times, 'target_shifts': shifts,
                'witness_indices': [a, b, c],
                'witness': [full.point_record(base[i]) for i in indices],
                'row_sha256': full.digest(row),
                'nonzero_signed_columns': sum(v != 0 for v in signed.values()),
                'signed_coefficients': [[j, signed[j]] for j in sorted(signed)]}
    elapsed = clock.finish()
    return {'status': 'censored_shift_cap', 'scalar': None, 'wall_ns': elapsed,
            'phase_wall_ns': clock.times, 'target_shifts': shifts,
            'witness_indices': None, 'witness': None, 'row_sha256': None,
            'nonzero_signed_columns': None, 'signed_coefficients': None}
