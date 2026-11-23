#!/usr/bin/env python3
"""Fixed-precision formal-scalar construction for the excluded H116 control."""

import json
import sys
from pathlib import Path

FIXTURE_P = 31
FIXTURE_A = 1
FIXTURE_B = 19
FIXTURE_T = -5
FIXTURE_R = 37
FIXTURE_SCALAR = (FIXTURE_T - 1) % FIXTURE_P
JET_PRECISION = 11
RECURRENCE_PRECISION = 9


class Counter:
    """Exact counts for the dense modular-series implementation."""

    def __init__(self):
        self.additions = 0
        self.multiplications = 0
        self.inversions = 0

    def as_dict(self):
        return {
            "additions": self.additions,
            "multiplications": self.multiplications,
            "inversions": self.inversions,
        }


def zeros(precision):
    return [0] * precision


def prefix(value, precision):
    return (value[:precision] + [0] * precision)[:precision]


def inverse(value, p, counter):
    counter.inversions += 1
    return pow(value, -1, p)


def add(left, right, p, precision, counter):
    counter.additions += precision
    return [(left[i] + right[i]) % p for i in range(precision)]


def sub(left, right, p, precision, counter):
    counter.additions += precision
    return [(left[i] - right[i]) % p for i in range(precision)]


def scale(value, scalar, p, precision, counter):
    counter.multiplications += precision
    return [scalar * value[i] % p for i in range(precision)]


def mul(left, right, p, precision, counter):
    out = zeros(precision)
    for i, a in enumerate(left[:precision]):
        for j, b in enumerate(right[: precision - i]):
            counter.multiplications += 1
            counter.additions += 1
            out[i + j] = (out[i + j] + a * b) % p
    return out


def inverse_unit(value, p, precision, counter):
    out = zeros(precision)
    out[0] = inverse(value[0], p, counter)
    for degree in range(1, precision):
        convolution = 0
        for i in range(1, degree + 1):
            counter.multiplications += 1
            counter.additions += 1
            convolution += value[i] * out[degree - i]
        counter.multiplications += 1
        out[degree] = -out[0] * convolution % p
    return out


def compose(outer, inner, p, precision, counter):
    if inner[0] != 0:
        raise ValueError("series composition requires zero inner constant term")
    out = zeros(precision)
    for coefficient in reversed(prefix(outer, precision)):
        out = mul(out, inner, p, precision, counter)
        counter.additions += 1
        out[0] = (out[0] + coefficient) % p
    return out


def curve_q(curve_a, curve_b, p, precision, counter):
    """Solve q=1+A*z^4*q^2+B*z^6*q^3 coefficient by coefficient."""
    q = zeros(precision)
    q[0] = 1
    for degree in range(1, precision):
        value = 0
        quadratic_degree = degree - 4
        if quadratic_degree >= 0:
            quadratic = 0
            for i in range(quadratic_degree + 1):
                counter.multiplications += 1
                counter.additions += 1
                quadratic += q[i] * q[quadratic_degree - i]
            counter.multiplications += 1
            counter.additions += 1
            value += curve_a * quadratic
        cubic_degree = degree - 6
        if cubic_degree >= 0:
            cubic = 0
            for i in range(cubic_degree + 1):
                for j in range(cubic_degree - i + 1):
                    counter.multiplications += 2
                    counter.additions += 1
                    cubic += q[i] * q[j] * q[cubic_degree - i - j]
            counter.multiplications += 1
            counter.additions += 1
            value += curve_b * cubic
        q[degree] = value % p
    return q


def formal_log(curve_a, curve_b, p, precision, counter):
    """Compute L(z)=integral dx/(2y) in z=-x/y."""
    q = curve_q(curve_a, curve_b, p, precision, counter)
    q_inverse = inverse_unit(q, p, precision, counter)
    derivative = zeros(precision)
    for i in range(1, precision):
        counter.multiplications += 1
        derivative[i - 1] = i * q_inverse[i] % p
    q_derivative = mul(q, derivative, p, precision, counter)
    inverse_two = inverse(2, p, counter)
    correction = zeros(precision)
    for i in range(1, precision):
        counter.multiplications += 1
        correction[i] = q_derivative[i - 1] * inverse_two % p
    omega = zeros(precision)
    omega[0] = 1
    omega = sub(omega, correction, p, precision, counter)
    logarithm = zeros(precision)
    for i in range(precision - 1):
        if omega[i] == 0:
            continue
        denominator_inverse = inverse(i + 1, p, counter)
        counter.multiplications += 1
        logarithm[i + 1] = omega[i] * denominator_inverse % p
    return logarithm


def scalar_parameter(
    curve_a, curve_b, scalar, p, precision, counter, verification_counter
):
    """Return [scalar](z) from L([scalar](z))=scalar*L(z)."""
    logarithm = formal_log(curve_a, curve_b, p, precision, counter)
    target = scale(logarithm, scalar, p, precision, counter)
    result = zeros(precision)
    result[1] = scalar % p
    for degree in range(2, precision):
        current = compose(
            logarithm[: degree + 1],
            result[: degree + 1],
            p,
            degree + 1,
            counter,
        )
        counter.additions += 1
        result[degree] = (target[degree] - current[degree]) % p
    if compose(logarithm, result, p, precision, verification_counter) != target:
        raise ValueError("formal scalar equation failed")
    return result


def revert_unit_linear(value, p, precision, counter, verification_counter):
    """Revert value(s)=s+O(s^2) coefficient by coefficient."""
    if value[0] != 0 or value[1] != 1:
        raise ValueError("series reversion requires s+O(s^2)")
    inverse_series = zeros(precision)
    inverse_series[1] = 1
    identity = zeros(precision)
    identity[1] = 1
    for degree in range(2, precision):
        current = compose(
            value[: degree + 1],
            inverse_series[: degree + 1],
            p,
            degree + 1,
            counter,
        )
        inverse_series[degree] = -current[degree] % p
    if compose(value, inverse_series, p, precision, verification_counter) != identity:
        raise ValueError("series reversion failed")
    return inverse_series


def normalized_x_ratio_jet(
    p,
    source_a,
    source_b,
    target_a,
    target_b,
    normalized_parameter,
    precision,
    counter,
    verification_counter,
):
    """Re-expand x_target(z_target)/x_source(z) in w=1/x_source."""
    z_precision = 2 * precision + 2
    normalized_parameter = prefix(normalized_parameter, z_precision)
    q_source = curve_q(source_a, source_b, p, z_precision, counter)
    q_target = curve_q(target_a, target_b, p, z_precision, counter)
    parameter_ratio = zeros(z_precision)
    for i in range(z_precision - 1):
        parameter_ratio[i] = normalized_parameter[i + 1]
    parameter_ratio_inverse = inverse_unit(
        parameter_ratio, p, z_precision, counter
    )
    target_q_at_parameter = compose(
        q_target, normalized_parameter, p, z_precision, counter
    )
    ratio = mul(
        mul(
            mul(
                parameter_ratio_inverse,
                parameter_ratio_inverse,
                p,
                z_precision,
                counter,
            ),
            q_source,
            p,
            z_precision,
            counter,
        ),
        inverse_unit(target_q_at_parameter, p, z_precision, counter),
        p,
        z_precision,
        counter,
    )
    if any(ratio[i] != 0 for i in range(1, 2 * precision, 2)):
        raise ValueError("normalized x-ratio is not even in z")
    ratio_in_s = [ratio[2 * i] for i in range(precision)]
    source_q_in_s = [q_source[2 * i] for i in range(precision)]
    w_from_s = zeros(precision)
    for i in range(1, precision):
        w_from_s[i] = source_q_in_s[i - 1]
    s_from_w = revert_unit_linear(
        w_from_s, p, precision, counter, verification_counter
    )
    return compose(ratio_in_s, s_from_w, p, precision, counter)


def scaled_target(curve_a, curve_b, scalar, p, counter):
    counter.multiplications += 5
    scalar_squared = scalar * scalar % p
    scalar_fourth = scalar_squared * scalar_squared % p
    target_a = scalar_fourth * curve_a % p
    scalar_sixth = scalar_fourth * scalar_squared % p
    target_b = scalar_sixth * curve_b % p
    return target_a, target_b


def formal_scalar_jet(
    p, curve_a, curve_b, scalar, precision, counter, verification_counter
):
    z_precision = 2 * precision + 2
    scalar_map = scalar_parameter(
        curve_a,
        curve_b,
        scalar,
        p,
        z_precision,
        counter,
        verification_counter,
    )
    scalar_inverse = inverse(scalar, p, counter)
    normalized_parameter = scale(
        scalar_map, scalar_inverse, p, z_precision, counter
    )
    target_a, target_b = scaled_target(curve_a, curve_b, scalar, p, counter)
    jet = normalized_x_ratio_jet(
        p,
        curve_a,
        curve_b,
        target_a,
        target_b,
        normalized_parameter,
        precision,
        counter,
        verification_counter,
    )
    return jet, normalized_parameter, (target_a, target_b)


def quotient_prefix(numerator, denominator, p, precision, counter):
    if denominator[0] != 1:
        raise ValueError("oracle denominator is not normalized")
    out = zeros(precision)
    for degree in range(precision):
        value = numerator[degree]
        for i in range(1, degree + 1):
            counter.multiplications += 1
            counter.additions += 1
            value -= denominator[i] * out[degree - i]
        out[degree] = value % p
    return out


def recurrence_prefix(p, source_a, source_b, target_a, target_b, counter):
    out = zeros(RECURRENCE_PRECISION)
    out[0] = 1
    inverse_five = inverse(5, p, counter)
    inverse_seven = inverse(7, p, counter)
    counter.additions += 2
    counter.multiplications += 2
    out[2] = (source_a - target_a) * inverse_five % p
    out[3] = (source_b - target_b) * inverse_seven % p
    for k in range(3, 8):
        convolution = 0
        for i in range(1, k - 1):
            counter.multiplications += 1
            counter.additions += 1
            convolution += out[i + 1] * out[k - i]
        step = 2 * k + 3
        inverse_step = inverse(step % p, p, counter)
        inverse_product = inverse(((k - 2) * step) % p, p, counter)
        counter.multiplications += 8
        counter.additions += 2
        first = 3 * convolution * inverse_product
        second = (2 * k - 3) * source_a * out[k - 1] * inverse_step
        third = 2 * (k - 3) * source_b * out[k - 2] * inverse_step
        out[k + 1] = (first - second - third) % p
    return out


def synthetic_frobenius_boundary(prime, precision, counter):
    """Identity scalar plus the leading normalized -z^p Frobenius term."""
    if (-16 * (4 + 27)) % prime == 0:
        raise ValueError("synthetic boundary curve is singular")
    z_precision = 2 * precision + 2
    identity_parameter = zeros(z_precision)
    identity_parameter[1] = 1
    frobenius_parameter = identity_parameter.copy()
    frobenius_parameter[prime] = -1 % prime
    base = normalized_x_ratio_jet(
        prime,
        1,
        1,
        1,
        1,
        identity_parameter,
        precision,
        counter,
        counter,
    )
    perturbed = normalized_x_ratio_jet(
        prime,
        1,
        1,
        1,
        1,
        frobenius_parameter,
        precision,
        counter,
        counter,
    )
    differences = [i for i in range(precision) if base[i] != perturbed[i]]
    return base, perturbed, differences


def evaluate(source_path):
    candidate_counter = Counter()
    candidate_verification_counter = Counter()
    candidate, normalized_parameter, target = formal_scalar_jet(
        FIXTURE_P,
        FIXTURE_A,
        FIXTURE_B,
        FIXTURE_SCALAR,
        JET_PRECISION,
        candidate_counter,
        candidate_verification_counter,
    )

    # The full-map record is intentionally loaded only after candidate output.
    record = json.loads(source_path.read_text(encoding="utf-8"))
    if record.get("schema") != "h116.map-certificate-control/v1":
        raise ValueError("wrong full-map source schema")
    if (
        record.get("fixture"),
        record.get("p"),
        record.get("r"),
    ) != ("excluded-h115-fp31", FIXTURE_P, FIXTURE_R):
        raise ValueError("wrong excluded fixture")
    if FIXTURE_R != FIXTURE_P + 1 - FIXTURE_T:
        raise ValueError("fixture trace/order mismatch")
    if tuple((record["target_a"], record["target_b"])) != target:
        raise ValueError("full-map target scaling mismatch")

    oracle_counter = Counter()
    oracle = quotient_prefix(
        record["numerator"],
        record["denominator"],
        FIXTURE_P,
        JET_PRECISION,
        oracle_counter,
    )
    recurrence_counter = Counter()
    recurrence = recurrence_prefix(
        FIXTURE_P,
        FIXTURE_A,
        FIXTURE_B,
        target[0],
        target[1],
        recurrence_counter,
    )

    boundary_23_counter = Counter()
    _, boundary_23, differences_23 = synthetic_frobenius_boundary(
        23, 12, boundary_23_counter
    )
    boundary_19_counter = Counter()
    _, boundary_19, differences_19 = synthetic_frobenius_boundary(
        19, 10, boundary_19_counter
    )

    wrong_scalar_counter = Counter()
    wrong_scalar, _, _ = formal_scalar_jet(
        FIXTURE_P,
        FIXTURE_A,
        FIXTURE_B,
        (FIXTURE_SCALAR + 1) % FIXTURE_P,
        JET_PRECISION,
        wrong_scalar_counter,
        wrong_scalar_counter,
    )
    unscaled_counter = Counter()
    unscaled_target = normalized_x_ratio_jet(
        FIXTURE_P,
        FIXTURE_A,
        FIXTURE_B,
        FIXTURE_A,
        FIXTURE_B,
        normalized_parameter,
        JET_PRECISION,
        unscaled_counter,
        unscaled_counter,
    )
    coordinate_counter = Counter()
    coordinate_lambda = 2
    coordinate_a = pow(coordinate_lambda, 4, FIXTURE_P) * FIXTURE_A % FIXTURE_P
    coordinate_b = pow(coordinate_lambda, 6, FIXTURE_P) * FIXTURE_B % FIXTURE_P
    coordinate_jet, _, _ = formal_scalar_jet(
        FIXTURE_P,
        coordinate_a,
        coordinate_b,
        FIXTURE_SCALAR,
        JET_PRECISION,
        coordinate_counter,
        coordinate_counter,
    )
    coordinate_expected = [
        coefficient * pow(coordinate_lambda, 2 * i, FIXTURE_P) % FIXTURE_P
        for i, coefficient in enumerate(candidate)
    ]

    candidate_matches_oracle = candidate == oracle
    recurrence_matches = candidate[:RECURRENCE_PRECISION] == recurrence
    wrong_scalar_rejected = wrong_scalar != oracle
    unscaled_target_rejected = unscaled_target != oracle
    coordinate_isomorphism_ok = coordinate_jet == coordinate_expected
    boundary_23_ok = differences_23 == [11]
    boundary_19_ok = differences_19 == [9]
    if not all(
        (
            candidate_matches_oracle,
            recurrence_matches,
            wrong_scalar_rejected,
            unscaled_target_rejected,
            boundary_23_ok,
            boundary_19_ok,
            coordinate_isomorphism_ok,
        )
    ):
        raise ValueError("formal-scalar jet or a frozen control failed")

    return {
        "schema": "h116.formal-scalar-jet-control/v1",
        "fixture": record["fixture"],
        "candidate_input": {
            "p": FIXTURE_P,
            "source_a": FIXTURE_A,
            "source_b": FIXTURE_B,
            "trace": FIXTURE_T,
            "scalar_t_minus_one": FIXTURE_SCALAR,
            "precision": JET_PRECISION,
        },
        "target": {"a": target[0], "b": target[1]},
        "candidate_loaded_before_full_map": True,
        "candidate_prefix": candidate,
        "oracle_prefix": oracle,
        "recurrence_prefix": recurrence,
        "candidate_matches_full_map": candidate_matches_oracle,
        "candidate_matches_recurrence": recurrence_matches,
        "wrong_scalar_rejected": wrong_scalar_rejected,
        "unscaled_target_rejected": unscaled_target_rejected,
        "coordinate_isomorphism_control": {
            "lambda": coordinate_lambda,
            "transformed_source_a": coordinate_a,
            "transformed_source_b": coordinate_b,
            "prefix": coordinate_jet,
            "expected_scaled_prefix": coordinate_expected,
            "passed": coordinate_isomorphism_ok,
        },
        "synthetic_frobenius_boundaries": {
            "p23": {
                "first_difference": differences_23[0],
                "differences": differences_23,
                "perturbed_prefix": boundary_23,
            },
            "p19": {
                "first_difference": differences_19[0],
                "differences": differences_19,
                "perturbed_prefix": boundary_19,
            },
        },
        "operation_counts": {
            "candidate": candidate_counter.as_dict(),
            "candidate_internal_verification": candidate_verification_counter.as_dict(),
            "recurrence_control": recurrence_counter.as_dict(),
            "full_map_quotient_oracle": oracle_counter.as_dict(),
            "wrong_scalar_control": wrong_scalar_counter.as_dict(),
            "unscaled_target_control": unscaled_counter.as_dict(),
            "coordinate_isomorphism_control": coordinate_counter.as_dict(),
            "synthetic_p23_boundary_control": boundary_23_counter.as_dict(),
            "synthetic_p19_boundary_control": boundary_19_counter.as_dict(),
        },
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: h116_formal_scalar_jet.py MAP_CERTIFICATE_CONTROL.json")
    result = evaluate(Path(sys.argv[1]))
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
