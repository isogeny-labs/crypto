#!/usr/bin/env python3
"""A finite-parameter baseline cost audit, not a best-attack estimate.

Equations (7),(8) in Menezes--Teske, CORR 2004-25, use degree-one
factor bases. Their different operation units are retained explicitly.
"""
import json
import math
from pathlib import Path
from analyze import IPSEC_ORDER


def main():
    r = IPSEC_ORDER//12
    rows = []
    for g in [15, 16]:
        l = 31
        log_rg = math.log2(g*g*l/4) + l + math.lgamma(g+1)/math.log(2)
        log_la = math.log2(g/4) + 2*l
        w = 1 << (l-1)
        rows.append({'genus': g, 'subfield_degree': l,
                     'factor_base_size_approx': w,
                     'log2_relation_generation_F2_31_multiplications': log_rg,
                     'log2_linear_algebra_mod_r_multiplications': log_la,
                     'illustrative_32bit_column_indices_GiB': g*w*4/(1<<30)})
    results = {'curve_order': str(IPSEC_ORDER), 'cofactor': 12,
               'large_factor': str(r), 'large_factor_bits': r.bit_length(),
               'log2_large_factor': math.log2(r),
               'log2_rho_EC_additions_without_negation_optimization':
                   .5*(math.log2(math.pi/2)+math.log2(r)),
               'baseline_degree_one_factor_base': rows,
               'limitations': [
                   'Operation units differ; these exponents are not directly comparable.',
                   'Does not benchmark this curve or its Jacobian.',
                   'Does not model modern sieve/large-prime variants or all solvers.',
                   'A genus-992 lower bound is not a constructed curve or a DLP lower bound.',
                   'The 2021 GLS demonstration uses a different curve and a 115-bit subgroup.'
               ],
               'sources': [
                   'https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf',
                   'https://arxiv.org/abs/2106.09967',
                   'https://www.rfc-editor.org/rfc/rfc2409.html#section-6.3']}
    Path(__file__).with_name('cost_results.json').write_text(
        json.dumps(results,indent=2)+'\n')
    print(json.dumps(results,indent=2))


if __name__ == '__main__':
    main()
