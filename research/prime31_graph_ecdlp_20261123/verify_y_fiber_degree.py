"""Replay two fixed-target certificates for the degree-three y map.

This checks a geometric fiber count, not relation extraction or solver work.
"""

import hashlib
import itertools
import json

from verify_mitm_graph import A, B, G, P, Q, add


FIBERS = {
    3: (2010301524, 1199300838, 1085364932),
    8: (1082032117, 775374956, 290076574),
    27: (2037228416, 1146858950, 1110879928),
}
EXPECTED_DIGESTS = {
    "G": "2d09c768aa26506515324d261352773aa08a26c9dbe4d751517c9d1b62d3ca61",
    "Q": "99e618e33d120299e233e8f663be21aa28b69783b4eeb23d5e7de1137559ad5f",
}


def negate(point):
    return None if point is None else (point[0], -point[1] % P)


def main():
    for y, roots in FIBERS.items():
        assert len(roots) == len(set(roots)) == 3
        for x in roots:
            assert (y * y - x * x * x - A * x - B) % P == 0
    result = {}
    for label, target in (("G", G), ("Q", Q)):
        assert (target[1] ** 2 - target[0] ** 3 - A * target[0] - B) % P == 0
        fourth_values = []
        for xs in itertools.product(*FIBERS.values()):
            total = None
            for (y, _), x in zip(FIBERS.items(), xs):
                total = add(total, (x, y))
            fourth = add(target, negate(total))
            assert fourth is not None
            fourth_values.append(fourth[1])
        assert len(fourth_values) == len(set(fourth_values)) == 27
        digest = hashlib.sha256(
            ",".join(map(str, sorted(fourth_values))).encode()
        ).hexdigest()
        assert digest == EXPECTED_DIGESTS[label]
        result[label] = {"distinct_fourth_y": 27, "sorted_y_sha256": digest}
    print(json.dumps({"input_y": list(FIBERS), "targets": result}, sort_keys=True))


if __name__ == "__main__":
    main()
