"""Replay fixed-target certificates for degree-three and degree-four maps.

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
LINEAR_FIBERS = {
    3: (1099723460, 733362665, 314397531),
    5: (1658027060, 304879890, 184576706),
    28: (2015881824, 1931235943, 347849536),
}
LINEAR_DIGESTS = {
    "G": "69309ec7634034e1f325a472bd1f169f6d8bc1e3215521723098f48e86d3ed65",
    "Q": "0f80b9e796b899441367e151cc093932ab7d8744dd6b2cdd057d602f5bf8c6e3",
}
QUADRATIC_FIBERS = {
    20: (148674779, 1112905506, 1747803852, 1762801742),
    33: (71066899, 1050877732, 1744862841, 1905378407),
    85: (289599482, 295758643, 791465495, 1247878612),
}
QUADRATIC_DIGESTS = {
    "G": "91598f246e2e9cc34227bcb0da8dac0f475ac6bd822a87915b1c7e608ae9dde3",
    "Q": "4da5674aa11b5c9029a1f7ad761c232f1d5120d3effb66ee3add6afd76c6e75e",
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
    for t, roots in LINEAR_FIBERS.items():
        assert len(roots) == len(set(roots)) == 3
        for x in roots:
            y = (3 * x + 5 + t) % P
            assert (y * y - x * x * x - A * x - B) % P == 0
    linear_result = {}
    for label, target in (("G", G), ("Q", Q)):
        fourth_values = []
        for xs in itertools.product(*LINEAR_FIBERS.values()):
            total = None
            for (t, _), x in zip(LINEAR_FIBERS.items(), xs):
                total = add(total, (x, (3 * x + 5 + t) % P))
            fourth = add(target, negate(total))
            assert fourth is not None
            fourth_values.append((fourth[1] - 3 * fourth[0] - 5) % P)
        assert len(fourth_values) == len(set(fourth_values)) == 27
        digest = hashlib.sha256(
            ",".join(map(str, sorted(fourth_values))).encode()
        ).hexdigest()
        assert digest == LINEAR_DIGESTS[label]
        linear_result[label] = {"distinct_fourth_linear": 27, "sorted_sha256": digest}
    for t, roots in QUADRATIC_FIBERS.items():
        assert len(roots) == len(set(roots)) == 4
        for x in roots:
            y = (3 * x * x + 5 * x + t) % P
            assert (y * y - x * x * x - A * x - B) % P == 0
    quadratic_result = {}
    for label, target in (("G", G), ("Q", Q)):
        fourth_values = []
        for xs in itertools.product(*QUADRATIC_FIBERS.values()):
            total = None
            for (t, _), x in zip(QUADRATIC_FIBERS.items(), xs):
                total = add(total, (x, (3 * x * x + 5 * x + t) % P))
            fourth = add(target, negate(total))
            assert fourth is not None
            fourth_values.append(
                (fourth[1] - 3 * fourth[0] * fourth[0] - 5 * fourth[0]) % P
            )
        assert len(fourth_values) == len(set(fourth_values)) == 64
        digest = hashlib.sha256(
            ",".join(map(str, sorted(fourth_values))).encode()
        ).hexdigest()
        assert digest == QUADRATIC_DIGESTS[label]
        quadratic_result[label] = {
            "distinct_fourth_quadratic": 64,
            "sorted_sha256": digest,
        }
    print(json.dumps({
        "input_y": list(FIBERS), "targets": result,
        "linear_graph": {"a": 3, "b": 5,
                         "input_offsets": list(LINEAR_FIBERS),
                         "targets": linear_result},
        "quadratic_graph": {"a": 3, "b": 5,
                            "input_offsets": list(QUADRATIC_FIBERS),
                            "targets": quadratic_result},
    }, sort_keys=True))


if __name__ == "__main__":
    main()
