import json, math, subprocess, sys
from pathlib import Path
import numpy as np
import pytest
from analysis_helpers import mk_rows, write_results, ANALYSIS
import build_results as br


def lin(f, cens_at=(), proxy_f=None, w_lo=10 ** 9):
    """arm whose solve work is f * 1000*(idx+1) (chain-tower: f = 1); proxy = f_proxy * 500*(idx+1).
    cens_at: set of (curve, label, idx) that are censored (timeout)."""
    pf = proxy_f if proxy_f is not None else f

    def g(idx, label, curve):
        if (curve, label, idx) in cens_at:
            return {"cens": "timeout", "w_lo": w_lo, "proxy_lo": w_lo}
        ff = f(curve) if callable(f) else f
        pp = pf(curve) if callable(pf) else pf
        return {"solve": ff * 1000 * (idx + 1), "extract": 0, "construction": 5, "proxy": pp * 500 * (idx + 1)}
    return g


def run_main(tmp, rows, **kw):
    res = tmp / "res"
    write_results(res, rows)
    out = res / "analysis"
    rc = br.main(["--results", str(res), "--out", str(out)] + sum(([f"--{k}"] for k in kw), []))
    return rc, out, json.loads((out / "analysis.json").read_text())


def rec(doc, cell, arm, st):
    return next(x for x in doc["c2"] if x["cell"] == cell and x["arm"] == arm and x["stratum"] == st)


# ------------------------------------------------------------------ pure statistics (hand-computed)
def test_median_inf_hand_cases():
    assert br.median_inf([3, 1, 2]) == 2
    assert br.median_inf([1, 3]) == 2
    assert br.median_inf([-math.inf, 1, 3, math.inf]) == 2
    assert br.median_inf([-math.inf, 1, 5, math.inf]) == 3
    assert br.median_inf([1, 2, math.inf, math.inf]) == math.inf
    assert math.isnan(br.median_inf([-math.inf, -math.inf, math.inf, math.inf]))


def test_signflip_hand_cases():
    # n = 10 all one sign: exactly the two extreme sign vectors -> p = 2 / 1024
    p, method, n = br.signflip_p([-2.0] * 10)
    assert p == pytest.approx(2 / 1024) and n == 10 and "2^10" in method
    # d = (3, 1, 1) with S_obs = 3 (signs +,+,+ gives 5; observed d = [3, 1, -1] -> |S| = 3):
    # |3a + b + c| >= 3 for 6 of 8 sign vectors -> 0.75 (hand enumeration in the comment of the test)
    assert br.signflip_p([3.0, 1.0, -1.0])[0] == pytest.approx(0.75)
    assert br.signflip_p([1.0, -1.0])[0] == 1.0          # S_obs = 0: every flip is as extreme
    assert br.signflip_p([0.0, 0.0])[0] == 1.0           # zeros carry no information
    # an infinite pair dominates: 1 pair +inf and 9 pairs +1 -> still only the two all-same-sign vectors
    assert br.signflip_p([math.inf] + [1.0] * 9)[0] == pytest.approx(2 / 1024)
    # n > 20 -> Monte Carlo with a fixed seed, reproducible
    p1, m1, _ = br.signflip_p([1.0] * 21)
    p2, _, _ = br.signflip_p([1.0] * 21)
    assert p1 == p2 and "Monte Carlo" in m1


def test_bootstrap_degenerate_and_reproducible():
    ci = br.boot_ci([-2.0] * 10, 1 - 0.05 / 3, 1)
    assert ci["lo"] == ci["hi"] == -2.0 and ci["degenerate"] is True
    a = br.boot_ci([0.5, 1.0, 2.0, 3.0, -1.0, 4.0], 0.95, 7)
    b = br.boot_ci([0.5, 1.0, 2.0, 3.0, -1.0, 4.0], 0.95, 7)
    assert a == b and a["lo"] <= 1.75 <= a["hi"]


def test_pair_value_censoring_rules():
    ok = lambda x: ("ok", x)
    ce = lambda x: ("cens", x)
    assert br.pair_value(ok(250), ok(1000)) == (-2.0, "ok")
    assert br.pair_value(ce(5), ok(1000))[0] == math.inf
    assert br.pair_value(ok(10), ce(5))[0] == -math.inf
    assert br.pair_value(ce(1), ce(2)) == (None, "dropped_both_censored")
    assert br.pair_value(ce(5), ok(1000))[1] == "lb_not_exceeding"          # lower bound below the completed side
    assert br.pair_value(ce(5000), ok(1000))[1] == "cens_pos"
    assert br.pair_value(ce(5), ok(1000), strict_lb=True) == (None, "dropped_lb")
    assert br.pair_value(ok(0), ok(10)) == (None, "dropped_zero")
    assert br.pair_value(ok(0), ok(0)) == (0.0, "ok")


# ------------------------------------------------------------------ whole pipeline on a synthetic rows.jsonl
def spec_basic():
    return {"chain-tower": lin(1), "chain-direct": lin(0.25, proxy_f=0.5), "mono-direct": lin(8, proxy_f=2),
            "sym-direct": lin(lambda c: 0.5 if c == "c1" else 2.0)}


def test_known_medians_and_verdicts(tmp_path):
    rows = mk_rows([("m3t3", 3, 3, 5, spec_basic())])
    rc, out, doc = run_main(tmp_path, rows)
    assert rc == 0
    for st in ("sat", "unsat"):
        cd = rec(doc, "m3t3", "chain-direct", st)
        assert cd["median"] == -2.0 and cd["n_valid"] == 10 and cd["k"] == 3
        assert cd["p"] == pytest.approx(2 / 1024) and cd["ci"]["lo"] == cd["ci"]["hi"] == -2.0
        assert cd["median_w_solve"] == -2.0 and cd["median_w_plus"] < 0 and cd["median_msolve_proxy"] == -1.0
        assert cd["D_AA"] == 0.0 and cd["verdict"] == "supported_within_scope" and cd["direction"] == "lower"
        assert cd["positive_control"] == "detected"
        mo = rec(doc, "m3t3", "mono-direct", st)
        assert mo["median"] == 3.0 and mo["verdict"] == "supported_within_scope"
        sy = rec(doc, "m3t3", "sym-direct", st)          # five -1 and five +1 pairs: median 0, S = 0, p = 1
        assert sy["median"] == 0.0 and sy["p"] == 1.0 and sy["verdict"] == "inconclusive"
        assert sy["per_curve"]["c1"]["median"] == -1.0 and sy["per_curve"]["c2"]["median"] == 1.0
    assert doc["number_of_verdict_records"] == 6
    assert doc["d_aa"]["m3t3/c1"]["D_AA"] == 0.0
    # outputs
    for f in ("RESULTS.md", "analysis.json", "REPORT.typ", "figures/presentation_work.svg", "figures/presentation_work.png"):
        assert (out / f).is_file(), f
    md = (out / "RESULTS.md").read_text()
    assert "development data" in md and "not comparable" in md and "[pcc:unverified]" in md
    assert "not on PATH" in md and "p^3" in md


def test_censoring_enters_as_inf_and_dropped_counted(tmp_path):
    chain = lin(1, cens_at={("c1", "sat", 4), ("c2", "sat", 3)})                 # baseline censored on 2 SAT instances
    cdir = lin(0.25, cens_at={("c1", "sat", 4), ("c1", "sat", 0), ("c1", "sat", 1), ("c1", "sat", 2),
                              ("c2", "sat", 0), ("c2", "sat", 1)}, w_lo=10 ** 9)
    mono = lin(8, cens_at={("c1", "sat", 0), ("c1", "sat", 1), ("c2", "sat", 0), ("c2", "sat", 1)})
    rows = mk_rows([("m3t2", 3, 2, 5, {"chain-tower": chain, "chain-direct": cdir, "mono-direct": mono})])
    rc, out, doc = run_main(tmp_path, rows)
    md = rec(doc, "m3t2", "mono-direct", "sat")
    # 4 candidate-censored pairs (+inf), 2 baseline-censored pairs (-inf: c1 idx 4, c2 idx 3), 4 pairs at +3
    # sorted: -inf,-inf,3,3,3,3,inf,inf,inf,inf -> n = 10, median = (3 + 3) / 2 = 3
    assert md["n_pos_inf"] == 4 and md["n_neg_inf"] == 2 and md["median"] == 3.0 and md["n_valid"] == 10
    assert md["n_censored_pairs"] == 6 and md["point_estimate"] is False
    cd = rec(doc, "m3t2", "chain-direct", "sat")
    # c1: idx 4 both censored -> dropped; idx 0,1,2 candidate censored (+inf); idx 3 = -2
    # c2: idx 3 baseline censored, candidate ok -> -inf; idx 0,1 candidate censored (+inf); idx 2, 4 = -2
    assert cd["n_dropped_both_censored"] == 1 and cd["n_pos_inf"] == 5 and cd["n_neg_inf"] == 1 and cd["n_valid"] == 9
    # sorted: -inf, -2, -2, -2, inf x5 -> n = 9, median = 5th = +inf (a bound)
    assert cd["median"] == "+inf" and cd["median_is_bound"] is True
    assert doc["d_aa"]["m3t2/c1"]["n_not_evaluable"] >= 1                         # aa and chain both censored there


def test_fewer_than_8_pairs_and_unsat_label_strata(tmp_path):
    rows = mk_rows([("m3t3", 3, 3, 5, spec_basic())], n_per=3)                      # 2 curves x 3 = 6 pairs per stratum
    rc, out, doc = run_main(tmp_path, rows)
    cd = rec(doc, "m3t3", "chain-direct", "sat")
    assert cd["n_valid"] == 6 and cd["verdict"] == "inconclusive" and "fewer than 8" in cd["reasons"][0]


def test_c1_failure_invalidates_the_cell(tmp_path):
    def dec(arm, label, name):
        if label == "unsat":
            return []
        if arm == "mono-direct" and name == "d000000":
            return [[[1, 1], [2, 1], [4, -1]]]                                          # differs from the baseline's set
        return [[[1, 1], [2, 1], [3, -1]]]
    rows = mk_rows([("m3t3", 3, 3, 5, spec_basic())], dec_fn=dec)
    rc, out, doc = run_main(tmp_path, rows)
    assert doc["controls"]["m3t3"]["cell_invalid"] is True and len(doc["controls"]["m3t3"]["c1_failures"]) == 2   # d000000 on both curves
    assert all(x["verdict"] == "invalid_measurement" for x in doc["c2"])


def test_false_relation_on_unsat_and_unfound_planted(tmp_path):
    def dec(arm, label, name):
        if arm == "chain-direct" and label == "unsat" and name == "d000005":
            return [[[1, 1], [2, 1], [3, -1]]]
        if arm == "chain-direct" and label == "planted":
            return []
        return [] if label == "unsat" else [[[1, 1], [2, 1], [3, -1]]]
    rows = mk_rows([("m3t3", 3, 3, 5, spec_basic())], dec_fn=dec)
    rc, out, doc = run_main(tmp_path, rows)
    ctl = doc["controls"]["m3t3"]
    assert ctl["negative_failures"] and ctl["planted_failures"] and ctl["cell_invalid"]


def test_symmetric_arms_compare_with_distinct_abscissa_subset(tmp_path):
    two = [[[1, 1], [2, 1], [3, -1]], [[5, 1], [5, -1], [7, 1]]]                        # second has a repeated abscissa
    spec = {"chain-tower": lin(1), "sym-direct": lin(0.5)}

    def dec(arm, label, name):
        if label == "unsat":
            return []
        return two[:1] if arm == "sym-direct" else two
    rows = mk_rows([("m3t3", 3, 3, 5, spec)], dec_fn=dec, rc=(2, 1))
    rc, out, doc = run_main(tmp_path, rows)
    ctl = doc["controls"]["m3t3"]
    assert not ctl["cell_invalid"] and ctl["c1_failures"] == [] and ctl["classifier_failures"] == []
    # a sym arm that returns the repeated-abscissa decomposition as well violates its semantics -> C1 failure
    rows = mk_rows([("m3t3", 3, 3, 5, spec)], dec_fn=lambda a, l, n: [] if l == "unsat" else two, rc=(2, 1))
    rc, out, doc = run_main(tmp_path, rows)
    assert doc["controls"]["m3t3"]["c1_failures"]


def test_not_applicable_arms_are_not_pairs_and_not_in_k(tmp_path):
    na = lambda idx, label, curve: {"cens": "not_applicable"}
    spec = {"chain-tower": lin(1), "chain-direct": lin(0.25), "sym-tower": na}
    rows = mk_rows([("m3t3", 3, 3, 2, spec)])
    rc, out, doc = run_main(tmp_path, rows)
    assert all(x["arm"] != "sym-tower" for x in doc["c2"])
    assert rec(doc, "m3t3", "chain-direct", "sat")["k"] == 1


def test_positive_control_not_detected_makes_cell_inconclusive(tmp_path):
    spec = {"chain-tower": lin(1), "chain-direct": lin(0.25), "mono-direct": lin(1)}   # mono equals baseline: control blind
    rows = mk_rows([("m3t3", 3, 3, 5, spec)])
    rc, out, doc = run_main(tmp_path, rows)
    cd = rec(doc, "m3t3", "chain-direct", "sat")
    assert cd["positive_control"] == "NOT detected" and cd["verdict"] == "inconclusive"
    assert any("positive control" in r for r in cd["reasons"])
    assert rec(doc, "m3t3", "mono-direct", "sat")["verdict"] == "within_control_distribution"


def test_m4_cells_are_not_gated_by_the_m3_control(tmp_path):
    spec = {"chain-tower": lin(1), "tree-tower": lin(0.5), "chain-direct": lin(0.25)}
    rows = mk_rows([("m4t3", 4, 3, 5, spec)])
    rc, out, doc = run_main(tmp_path, rows)
    x = rec(doc, "m4t3", "chain-direct", "sat")
    assert x["positive_control"].startswith("not available") and x["verdict"] == "supported_within_scope"
    assert x["k"] == 2


def test_c3_secant_exact_value(tmp_path):
    t2 = {"chain-tower": lin(1), "chain-direct": lin(1)}
    t3 = {"chain-tower": lin(4), "chain-direct": lin(2)}                                # work x4 and x2 per doubling of B
    rows = mk_rows([("m3t2", 3, 2, 5, t2), ("m3t3", 3, 3, 5, t3)])
    rc, out, doc = run_main(tmp_path, rows)
    g = next(x for x in doc["c3"] if x["arm"] == "chain-tower" and x["reading"] == "w" and x["t_from"] == 2)
    assert g["n_exact"] == 22 and g["gamma_median_exact"] == 2.0 and g["iqr_exact"] == [2.0, 2.0]
    g = next(x for x in doc["c3"] if x["arm"] == "chain-direct" and x["reading"] == "w")
    assert g["gamma_median_exact"] == 1.0
    p = next(x for x in doc["c3"] if x["arm"] == "chain-direct" and x["reading"] == "proxy")
    assert p["gamma_median_exact"] == 1.0


def test_c3_censored_numerator_is_a_lower_bound(tmp_path):
    t2 = {"chain-tower": lin(1)}
    t3 = {"chain-tower": lin(4, cens_at={("c1", "sat", 0)}, w_lo=8000)}
    rows = mk_rows([("m3t2", 3, 2, 5, t2), ("m3t3", 3, 3, 5, t3)])
    rc, out, doc = run_main(tmp_path, rows)
    g = next(x for x in doc["c3"] if x["arm"] == "chain-tower" and x["reading"] == "w")
    assert g["n_lower"] == 1 and g["n_exact"] == 21


def test_missing_field_on_completed_row_fails_loudly(tmp_path):
    rows = mk_rows([("m3t3", 3, 3, 5, spec_basic())])
    for r in rows:
        if r["engine"] == "native" and r["censored"] == "no":
            del r["extract_field_multiplications"]
            break
    write_results(tmp_path / "res", rows)
    with pytest.raises(br.AnalysisError):
        br.main(["--results", str(tmp_path / "res"), "--out", str(tmp_path / "res/analysis")])


def test_cli_exit_code_on_contract_failure(tmp_path):
    (tmp_path / "res").mkdir()
    (tmp_path / "res/rows.jsonl").write_text("{}\n")
    r = subprocess.run([sys.executable, str(ANALYSIS / "build_results.py"), "--results", str(tmp_path / "res"),
                        "--out", str(tmp_path / "o")], capture_output=True, text=True)
    assert r.returncode == 3 and "lacks" in r.stderr
