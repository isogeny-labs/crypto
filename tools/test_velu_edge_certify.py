"""Tests for tools/velu_edge_certify.py: cross-checks against brute force and
existing volcano-rho / volcano-cost-v2 records, plus negative controls."""
import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import velu_edge_certify as V  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def points(p, a, b):
    sq = {}
    for y in range(p):
        sq.setdefault(y * y % p, []).append(y)
    return [(x, y) for x in range(p) for y in sq.get((x ** 3 + a * x + b) % p, [])]


def small_cases():
    for p in (31, 101, 103):
        for a in range(0, 12):
            for b in range(1, 12):
                if (4 * a ** 3 + 27 * b * b) % p == 0:
                    continue
                for ell in (2, 3):
                    for x0 in V.kernel_roots(ell, a, b, p):
                        if ell == 3 and (x0 ** 3 + a * x0 + b) % p == 0:
                            continue
                        yield p, a, b, ell, x0


@pytest.mark.parametrize("p,a,b,ell,x0", list(small_cases()))
def test_every_rational_kernel_certifies_and_matches_brute_force(p, a, b, ell, x0):
    v = V.certify_edge(p, a, b, ell, x0)
    assert v.ok, v.reason
    A, B = v.codomain
    # Independent check: map every affine point; images lie on E', #E = #E'.
    t, u, _ = V.velu_params(ell, a, b, x0, p)
    pts = points(p, a, b)
    for x, y in pts:
        if x == x0:
            continue
        i = pow(x - x0, -1, p)
        X = (x + t * i + u * i * i) % p
        Y = y * (1 - t * i * i - 2 * u * i ** 3) % p
        assert (Y * Y - X ** 3 - A * X - B) % p == 0
    assert len(pts) == len(points(p, A, B))


def test_cross_check_brute_force_kernel_roots_match_fast_path():
    p, a, b = 10007, 3, 7
    for ell in (2, 3):
        f = V.division_poly(ell, a, b, p)
        brute = [x for x in range(p) if V.peval(f, x, p) == 0]
        assert V.kernel_roots(ell, a, b, p) == brute


# ----------------------------------------------------------- negative controls
def first_case(ell):
    return next(c for c in small_cases() if c[3] == ell)


@pytest.mark.parametrize("ell", [2, 3])
def test_wrong_kernel_rejected(ell):
    p, a, b, _, x0 = first_case(ell)
    roots = set(V.kernel_roots(ell, a, b, p))
    bad = next(x for x in range(p) if x not in roots)
    v = V.certify_edge(p, a, b, ell, bad)
    assert not v.ok and "not a root" in v.reason


def test_wrong_kernel_even_with_forced_velu_codomain_rejected():
    # Feed the Velu codomain computed from a non-kernel x0: the record would be
    # self-consistent, but the map identity / division-polynomial check fails.
    p, a, b, _, _ = first_case(2)
    roots = set(V.kernel_roots(2, a, b, p))
    bad = next(x for x in range(p) if x not in roots and (3 * x * x + a) % p)
    t, u, (A, B) = V.velu_params(2, a, b, bad, p)
    ok, _ = V.check_map_identity(a, b, bad, t, u, A, B, p)
    assert not ok
    assert not V.certify_edge(p, a, b, 2, bad, A, B).ok


def test_degenerate_non_kernel_passes_sampled_checks_but_is_rejected():
    # Gap witness: x0 with 3*x0^2 + a = 0 is not a 2-kernel, yet the l=2 Velu
    # formulas give t = u = 0, i.e. the identity E -> E. That map satisfies all
    # three sampled producer checks (image on codomain, homomorphism,
    # n*phi(P) = O). Only the division-polynomial check rejects it.
    p, a, b = 31, 1, 3
    x0 = 17
    assert (3 * x0 * x0 + a) % p == 0 and x0 not in V.kernel_roots(2, a, b, p)
    t, u, (A, B) = V.velu_params(2, a, b, x0, p)
    assert (t, u, A, B) == (0, 0, a, b)
    assert V.check_map_identity(a, b, x0, t, u, A, B, p)[0]
    v = V.certify_edge(p, a, b, 2, x0, A, B)
    assert not v.ok and "not a root" in v.reason


def test_three_kernel_passed_as_two_rejected():
    p, a, b, _, x0 = first_case(3)
    if x0 in V.kernel_roots(2, a, b, p):
        pytest.skip("coincident root")
    assert not V.certify_edge(p, a, b, 2, x0).ok


def test_twist_and_tampered_codomain_rejected():
    p, a, b, ell, x0 = first_case(2)
    A, B = V.certify_edge(p, a, b, ell, x0).codomain
    w = next(c for c in range(2, p) if not V.is_square(c, p))
    twist = (w * w * A % p, w ** 3 * B % p)
    v = V.certify_edge(p, a, b, ell, x0, *twist)
    assert not v.ok and "twist" in v.reason
    assert not V.certify_edge(p, a, b, ell, x0, A, (B + 1) % p).ok
    s = 5  # a square
    assert V.certify_edge(p, a, b, ell, x0, s * s * A % p, s ** 3 * B % p).ok


# ----------------------------------------------------------- existing records
RUNS = sorted((ROOT / "research/isogeny-volcano-rho/runs").glob("*/results.json"))
RUNS = [r for r in RUNS if "problem" in json.loads(r.read_text())]


@pytest.mark.parametrize("path", RUNS, ids=lambda p: p.parent.name)
def test_volcano_rho_records_certify_exactly(path):
    res = V.certify_results_json(path)
    assert res["failed"] == []
    assert res["checked"] == res["recorded_edges_verified"] > 0


def test_volcano_rho_record_with_altered_kernel_rejected():
    d = json.loads(RUNS[0].read_text())
    p = d["problem"]["p"]
    vs = {v["id"]: v for v in d["vertices"]}
    child = next(v for v in d["vertices"] if v.get("edge"))
    par = vs[child["parent"]]
    e = child["edge"]
    assert V.certify_edge(p, par["a"], par["b"], e["ell"], e["kernel_x"], child["a"], child["b"]).ok
    others = [x for x in V.kernel_roots(e["ell"], par["a"], par["b"], p) if x != e["kernel_x"]]
    for x in others + [(e["kernel_x"] + 1) % p]:
        assert not V.certify_edge(p, par["a"], par["b"], e["ell"], x, child["a"], child["b"]).ok


CLASSES = sorted((ROOT / "research/isogeny-volcano-cost-v2_20261017/classes").glob("l*-s100[1-3]/curves.jsonl"))


@pytest.mark.parametrize("path", CLASSES, ids=lambda p: p.parent.name)
def test_volcano_cost_v2_edges_certify(path):
    ell = int(path.parent.name[1])
    res = V.certify_curves_jsonl(path, ell)
    assert res["failed"] == [] and res["checked"] > 0
