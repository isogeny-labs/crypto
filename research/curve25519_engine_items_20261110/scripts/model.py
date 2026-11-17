#!/usr/bin/env python3
"""Pre-data model (PROTOCOL.md section 4) -> data/model.json.

Uses only code and the committed tw_jumps study (research/curve25519_tw_jumps_20261108,
data/pooled_measured.json, its rho-neg and twcj-j16 arms on the same curves), no data of
this study.

(A) cv=0 runs the same walk from the same seed as the current arm and drops exactly the
    collision-verification ledger (tw_verify_adds): S(cv0) = S(twcj) - verify.
(B) rho.negation_chained replaces jumps 2..m-1 (m = rho_jumps_for(r) = 8 at both sizes) of
    the independent table, each a joint double-and-add with coefficients uniform in [1, 2^c)
    (c = jump_coefficient_bits(8) = 9), by one addition each. The expected joint_mul cost is
    enumerated exactly; the walk is assumed unchanged (the fruitless-cycle risk of the chain's
    small relations is not modelled; see PROTOCOL.md section 4).
"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
PRIOR = os.path.join(HERE, '../../curve25519_tw_jumps_20261108/data/pooled_measured.json')
OUT = os.path.join(HERE, '../data/model.json')


def joint_mul_cost(bits):
    tot, n = 0, 0
    for a in range(1, 1 << bits):
        for b in range(1, 1 << bits):
            o = a | b
            top = o.bit_length()
            dbl = top - 1
            add = bin(o & ((1 << (top - 1)) - 1)).count('1')
            tot += dbl + add; n += 1
    return tot / n


def main():
    prior = json.load(open(PRIOR))
    jm = joint_mul_cost(9)
    m = 8
    saving = (m - 2) * jm - (m - 2)
    out = dict(joint_mul_9bit_gae=jm, rho_jumps=m, chained_saving_gae=saving, sizes={})
    for k in ('k20', 'k24'):
        d = prior[k]
        sq = d['sqrt_r']
        tw_S = {w: d['S']['twcj-j16'][w]['mean'] for w in ('0', '0.1', '1')}
        tw_St = {w: d['S']['twcj-j16'][w]['mean_table_work'] for w in ('0', '0.1', '1')}
        ver = d['phases']['twcj-j16']['collision verify']
        rho_S = {w: d['S']['rho-neg'][w]['mean'] for w in ('0', '0.1', '1')}
        rho_setup = d['phases']['rho-neg']['setup']
        sv = saving / sq
        A = {w: (tw_S[w] - ver) / tw_S[w] for w in tw_S}
        A_priced = {w: (tw_St[w] - ver) / tw_St[w] for w in tw_St}
        B = {w: (rho_S[w] - sv) / rho_S[w] for w in rho_S}
        out['sizes'][k] = dict(
            sqrt_r=sq,
            A=dict(twcj_S=tw_S, twcj_S_priced=tw_St, verify=ver, cv0_S=tw_S['0'] - ver,
                   ratio=A, ratio_priced=A_priced),
            B=dict(rho_S=rho_S, rho_setup=rho_setup, rho_setup_gae=rho_setup * sq,
                   chained_setup_gae=rho_setup * sq - saving, saving=sv, chained_S=rho_S['0'] - sv,
                   ratio=B),
            distance_w0=dict(twcj_over_rho=tw_S['0'] / rho_S['0'],
                             twcj_over_chained=tw_S['0'] / (rho_S['0'] - sv),
                             cv0_over_rho=(tw_S['0'] - ver) / rho_S['0'],
                             cv0_over_chained=(tw_S['0'] - ver) / (rho_S['0'] - sv)))
    json.dump(out, open(OUT, 'w'), indent=1, sort_keys=True)
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == '__main__':
    main()
