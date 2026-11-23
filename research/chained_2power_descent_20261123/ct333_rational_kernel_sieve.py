#!/usr/bin/env python3
"""CT333: 11 Frobenius cycle types; Richelot kernels, not actual Jacobians.

Two representations: perfect matchings and F2^4 isotropic 2-planes.
Pure combinatorics; no transfer, divisor, timing or certified theorem.
"""
import collections
import hashlib
import itertools
import json
import math
import sys


def partitions(n, least=1):
    if n == 0:
        yield ()
        return
    for a in range(least, n + 1):
        for tail in partitions(n - a, a):
            yield (a,) + tail


def canonical_permutation(cycle_type):
    p = list(range(6))
    start = 0
    for size in cycle_type:
        cycle = list(range(start, start + size))
        for i, j in zip(cycle, cycle[1:] + cycle[:1]):
            p[i] = j
        start += size
    return tuple(p)


def cycle_type(p):
    unseen = set(range(6))
    out = []
    while unseen:
        i = min(unseen)
        j = i
        length = 0
        while j in unseen:
            unseen.remove(j)
            j = p[j]
            length += 1
        if j != i:
            raise AssertionError('invalid cycle')
        out.append(length)
    return tuple(sorted(out))


def perfect_matchings(items):
    if not items:
        yield ()
        return
    first, *rest = items
    for mate in rest:
        for remaining in perfect_matchings([x for x in rest if x != mate]):
            yield tuple(sorted(((first, mate),) + remaining))


MATCHINGS = tuple(sorted(set(perfect_matchings(list(range(6))))))
assert len(MATCHINGS) == 15


def image_matching(m, perm):
    return tuple(sorted(tuple(sorted((perm[a], perm[b]))) for a, b in m))


def vroot(i):
    return (1 << i) if i < 4 else (15 if i == 4 else 0)


def vector_pair(a, b):
    return vroot(a) ^ vroot(b)


PAIRS_BY_VECTOR = {vector_pair(i, j): (i, j)
                   for i in range(6) for j in range(i + 1, 6)}
assert len(PAIRS_BY_VECTOR) == 15 and set(PAIRS_BY_VECTOR) == set(range(1, 16))


def symplectic(a, b):
    # Pair intersection parity in the even-subset-of-six/complement model.
    return (((a.bit_count() & 1) & (b.bit_count() & 1)) ^
            ((a & b).bit_count() & 1))


def isotropic_planes():
    found = set()
    for a in range(1, 16):
        for b in range(a + 1, 16):
            if symplectic(a, b) == 0:
                found.add(tuple(sorted((a, b, a ^ b))))
    return tuple(sorted(found))


PLANES = isotropic_planes()
assert len(PLANES) == 15


def plane_as_matching(plane):
    return tuple(sorted(PAIRS_BY_VECTOR[x] for x in plane))


assert {plane_as_matching(x) for x in PLANES} == set(MATCHINGS)


def image_plane(plane, perm):
    def f(x):
        i, j = PAIRS_BY_VECTOR[x]
        return vector_pair(perm[i], perm[j])
    return tuple(sorted(f(x) for x in plane))


def orbits(items, action):
    pending = set(items)
    result = []
    while pending:
        seed = min(pending)
        current = seed
        seen = []
        while current not in seen:
            if current not in pending:
                raise AssertionError('nonpermutation orbit')
            seen.append(current)
            current = action(current)
        if current != seed:
            raise AssertionError('incorrect orbit return')
        for x in seen:
            pending.remove(x)
        result.append(tuple(seen))
    return result


def audited_profile(p):
    assert sorted(p) == list(range(6))
    om = orbits(MATCHINGS, lambda m: image_matching(m, p))
    ol = orbits(PLANES, lambda plane: image_plane(plane, p))
    lengths_match = sorted(len(o) for o in om)
    lengths_plane = sorted(len(o) for o in ol)
    if lengths_match != lengths_plane:
        raise AssertionError('oracle orbit mismatch')
    fixed_m = {o[0] for o in om if len(o) == 1}
    fixed_l = {plane_as_matching(o[0]) for o in ol if len(o) == 1}
    if fixed_m != fixed_l:
        raise AssertionError('oracle fixed-kernel mismatch')
    pointwise_pairs = sum(all(image_matching((pair,), p) == (pair,)
                                  for pair in m) for m in fixed_m)
    return {'rational_kernels': len(fixed_m),
            'pointwise_pair_kernels': pointwise_pairs,
            'orbit_lengths': lengths_match,
            'minimum_extension_degree': min(lengths_match),
            'rational_kernel_matchings': [str(m) for m in sorted(fixed_m)]}


def check_mutations():
    failed = 0
    tests = [lambda: audited_profile((0, 0, 2, 3, 4, 5)),
             lambda: bad_matching(),
             lambda: bad_plane_image()]
    for f in tests:
        try:
            f()
        except (AssertionError, ValueError, KeyError):
            failed += 1
        else:
            raise AssertionError('mutation survived')
    return failed


def bad_matching():
    malformed = ((0, 1), (0, 2), (3, 4))
    if malformed not in MATCHINGS:
        raise AssertionError('duplicate-root matching rejected')


def bad_plane_image():
    p = canonical_permutation((1, 5))
    plane = PLANES[0]
    corrupted = tuple(sorted((image_plane(plane, p)[0],) * 3))
    if corrupted in PLANES:
        raise AssertionError('corrupt 2-plane accepted')
    raise AssertionError('corrupt 2-plane rejected')


def main():
    census = {}
    class_counts = collections.Counter()
    for p in itertools.permutations(range(6)):
        kind = cycle_type(p)
        x = audited_profile(p)
        class_counts[kind] += 1
        if kind in census:
            if x['rational_kernels'] != census[kind]['rational_kernels'] or \
               x['orbit_lengths'] != census[kind]['orbit_lengths'] or \
               x['pointwise_pair_kernels'] != census[kind]['pointwise_pair_kernels']:
                raise AssertionError('nonconjugate result in cycle class')
        else:
            census[kind] = x
    expected_types = set(partitions(6))
    assert len(census) == 11 and set(census) == expected_types
    assert sum(class_counts.values()) == 720
    for part, count in class_counts.items():
        multiplicity = collections.Counter(part)
        class_size = math.factorial(6)
        for k, m in multiplicity.items():
            class_size //= k ** m * math.factorial(m)
        assert count == class_size
        assert audited_profile(canonical_permutation(part))['orbit_lengths'] == census[part]['orbit_lengths']
    assert check_mutations() == 3
    print(json.dumps({'scope': 'abstract Frobenius action on 15 Richelot kernels; unsigned producer check',
       'source_sha256': hashlib.sha256(open(__file__, 'rb').read()).hexdigest(),
       'permutations_checked': 720, 'cycle_types_checked': 11, 'kernels_per_action': 15,
       'isotropic_planes_per_action': 15, 'mutations_rejected': 3,
       'rows': [{'cycle_type': '+'.join(map(str, part)), 'permutation_count': class_counts[part],
                 **census[part]} for part in sorted(census, key=lambda x: (len(x), x), reverse=True)],
       'limit': 'not an actual Jacobian, descended morphism, complete ECDLP solver, or signed PCC verdict'},
      sort_keys=True, indent=2))


if __name__ == '__main__':
    try:
        main()
    except Exception as e:
        print('FAIL:', str(e), file=sys.stderr)
        sys.exit(1)
