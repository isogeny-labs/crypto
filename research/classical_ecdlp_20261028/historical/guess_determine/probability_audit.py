"""Count repeated candidate subsets and calibrate the scalar probability model.

This is a separate diagnostic, not an alternative ECDLP benchmark. Distinct
group exponents model distinct nonidentity points in a prime-order EC group.
The attack in guess_determine.py does not call this diagnostic or receive its
known-exponent oracle. The independence assumption is tested, not adopted.
"""
from collections import Counter
from itertools import combinations
import json
from math import comb,expm1,log1p,sqrt
from pathlib import Path
from random import Random


def candidate_multiplicities(h,d):
    result = Counter()
    for b in combinations(range(h),h-d):
        remaining = [i for i in range(2*h) if i not in b]
        for extension in combinations(remaining,d):
            result[tuple(sorted(b+extension))] += 1
    occurrences = comb(h,d)*comb(h+d,d)
    unique = sum(comb(h,j)**2 for j in range(d+1))
    assert sum(result.values()) == occurrences
    assert len(result) == unique
    return result


def wilson(hits,trials):
    z = 1.959963984540054
    f = hits/trials
    denominator = 1+z*z/trials
    center = (f+z*z/(2*trials))/denominator
    radius = z*sqrt(f*(1-f)/trials+z*z/(4*trials*trials))/denominator
    return center-radius,center+radius


def main():
    r,h,trials = 991,6,3000
    results = []
    for d in [2,3]:
        occurrences = candidate_multiplicities(h,d)
        N = sum(occurrences.values())
        rng = Random(20261027+d)
        hits = 0
        for unused in range(trials):
            # 2h P-side and 2h Q-side points, globally distinct, none identity.
            exponents = rng.sample(range(1,r),4*h)
            # Fix the P-side outer guess. Its complement contains h points.
            target = -sum(exponents[h:2*h]) % r
            Q = exponents[2*h:]
            hits += any(sum(Q[i] for i in I) % r == target
                        for I in occurrences)
        q = (r-2*d+1)/(r*(r-d+1))
        results.append(dict(r=r,h=h,defect=d,trials=trials,hits=hits,
            candidate_occurrences=N,distinct_candidate_subsets=len(occurrences),
            paper_independence_model=-expm1(N*log1p(-q)),
            observed_probability=hits/trials,
            observed_wilson_95=wilson(hits,trials)))
    report = {'source':'https://arxiv.org/html/2607.09814v2#S6',
              'scope':'Separate known-exponent scalar calibration, not EC timings.',
              'distinct_subset_formula':'sum(binomial(h,j)^2,j=0..d)',
              'occurrence_formula':'binomial(h,d)*binomial(h+d,d)',
              'cases':results}
    destination = Path(__file__).with_name('probability-audit.json')
    destination.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(results,indent=2))


if __name__ == '__main__':
    main()
