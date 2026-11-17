#!/usr/bin/env python3
"""Pre-data cost model (PROTOCOL.md section 4): Monte Carlo of the counted group
operations of the three walk arms, from the build rules in ic_boundary.rs
(WalkTable / TargetWalk), in GAE (1 addition = 1 doubling = 1 GAE).

Uniform-walk model with negation folding: entries hit an existing orbit with
probability 2n/r, so the k-th collision arrives at n_k with n_k^2/r ~ Gamma(k, 1).
Fixed costs (factor base 23/27 GAE, final [d]G check 20/24 GAE) are the
earlier study's measured values for the same curves.  Writes data/model.json.
"""
import json, math, os, random

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
CURVES = {20: dict(r=131203, fb=23, ver=20), 24: dict(r=2097349, fb=27, ver=24)}
N = 20000
JUMPS = 16


def pc(x):
    return bin(x).count('1')


def bits(x):
    return x.bit_length()


def plan(r, x, grow=2):
    t = round(x * math.sqrt(r))
    t_max = t * (1 + grow)
    w = max(round(1.2 * math.sqrt(t)), 2)
    return t, t_max, w, 1 + t_max * w


def eval_cost(*ks):
    n = sum(pc(k) for k in ks)
    return max(n - 1, 0)


def current_f1(r, rng, x=0.7):
    """wwalk:x=0.7,g=1,grow=2 on a one-seed base + stride-generator probes."""
    t, t_max, w, bound = plan(r, x)
    setup = (bits(bound) - 1) + (bits(r - 1) - 1)
    al = []
    for _ in range(JUMPS):
        a, ga = rng.randint(1, w), rng.randint(1, r - 1)
        al.append(a)
        setup += eval_cost(a, ga)
    n1 = math.sqrt(rng.gammavariate(1, 1) * r)
    steps = min(max(t, math.ceil(n1)), t_max)
    mean_al = sum(al) / JUMPS
    u = int(steps * mean_al)
    verify = 2 * eval_cost(u, rng.randint(1, r - 1))
    probes = rng.expovariate(2 * steps / r)
    rel = max(math.ceil(probes) - 1, 0) + eval_cost(int(rng.random() * u), rng.randint(1, r - 1))
    return dict(setup=setup, walk=steps, verify=verify, probe=rel, lookups=math.ceil(probes))


def target_walk(r, rng, cheap, x=1.5):
    t, t_max, w, bound = plan(r, x)
    setup = (bits(bound) - 1) + 2 * (bits(r - 1) - 1)
    al = [rng.randint(1, w) for _ in range(JUMPS)]
    if cheap:
        setup += 2 * eval_cost(rng.randint(1, r - 1), rng.randint(1, r - 1)) + (JUMPS - 2)
        setup += sum(eval_cost(a) + 1 for a in al)
    else:
        setup += sum(eval_cost(a, rng.randint(1, r - 1), rng.randint(1, r - 1)) for a in al)
    n2 = math.sqrt(rng.gammavariate(2, 1) * r)
    steps = min(math.ceil(n2), t_max)
    u = int(steps * sum(al) / JUMPS)
    verify = 0
    for _ in range(2):
        for _ in range(2):
            verify += eval_cost(int(rng.random() * u), rng.randint(1, r - 1), rng.randint(1, r - 1))
    return dict(setup=setup, walk=steps, verify=verify, probe=0, lookups=0)


def main():
    rng = random.Random(20261108)
    out = {}
    for k, c in CURVES.items():
        r, sq = c['r'], math.sqrt(c['r'])
        arms = {'f1-x07': lambda: current_f1(r, rng),
                'tw-x15': lambda: target_walk(r, rng, False),
                'twcj-x15': lambda: target_walk(r, rng, True)}
        res = {}
        for name, f in arms.items():
            acc = {}
            for _ in range(N):
                d = f()
                for kk, v in d.items():
                    acc[kk] = acc.get(kk, 0) + v
            m = {kk: v / N / sq for kk, v in acc.items()}
            m['total_S_w0'] = (c['fb'] + c['ver']) / sq + m['setup'] + m['walk'] + m['verify'] + m['probe']
            m['search_S'] = m['walk'] + m['probe']
            res[name] = {kk: round(v, 4) for kk, v in m.items()}
        res['ratios_w0'] = {
            'tw/f1': round(res['tw-x15']['total_S_w0'] / res['f1-x07']['total_S_w0'], 3),
            'twcj/f1': round(res['twcj-x15']['total_S_w0'] / res['f1-x07']['total_S_w0'], 3),
            'twcj/tw': round(res['twcj-x15']['total_S_w0'] / res['tw-x15']['total_S_w0'], 3),
        }
        out[f'k{k}'] = res
    # Search-phase floor, F = 1 with the target in the walk: E[sqrt(Gamma(2,1))].
    out['search_floor_tw'] = round(math.gamma(2.5) / math.gamma(2), 4)
    json.dump(out, open(os.path.join(HERE, 'data/model.json'), 'w'), indent=1)
    print(json.dumps(out, indent=1))


if __name__ == '__main__':
    main()
