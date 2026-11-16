#!/usr/bin/env python3
"""Recheck the frozen v7 claims without calling the IC or rho producer.

Reads each claim-selected run (including clean retries), independently checks
both discrete-log statements in pure Python, recomputes the six point-estimate
comparisons, and checks the 48 matched strong-rho reference observations.
The generated files are evidence for the existing validation gate; the raw
run and claim files are never modified.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import pathlib
import random
import statistics
import sys

BASE = pathlib.Path(__file__).resolve().parent
ROOT = BASE.parents[3]


def sha(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_hash(value: dict) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


class F2m:
    """Binary-field curve y² + xy = x³ + ax² + 1, independent of Rust."""

    def __init__(self, n: int, low: list[int], a: int):
        self.n = n
        self.mod = (1 << n) | sum(1 << t for t in low)
        self.a = a

    def mul(self, x: int, y: int) -> int:
        z = 0
        while y:
            if y & 1:
                z ^= x
            y >>= 1
            x <<= 1
            if x & (1 << self.n):
                x ^= self.mod
        return z

    def inv(self, x: int) -> int:
        if not x:
            raise ValueError("zero has no inverse")
        u, v, g, h = x, self.mod, 1, 0
        while u != 1:
            j = u.bit_length() - v.bit_length()
            if j < 0:
                u, v, g, h, j = v, u, h, g, -j
            u ^= v << j
            g ^= h << j
        return g

    def on(self, p: tuple[int, int]) -> bool:
        x, y = p
        m = self.mul
        return m(y, y) ^ m(x, y) == m(m(x, x), x) ^ m(self.a, m(x, x)) ^ 1

    def add(self, p, q):
        if p is None:
            return q
        if q is None:
            return p
        m = self.mul
        if p[0] == q[0]:
            if p[1] ^ q[1] == p[0]:
                return None
            lam = p[0] ^ m(p[1], self.inv(p[0]))
            x = m(lam, lam) ^ lam ^ self.a
        else:
            lam = m(p[1] ^ q[1], self.inv(p[0] ^ q[0]))
            x = m(lam, lam) ^ lam ^ p[0] ^ q[0] ^ self.a
        return x, m(p[0] ^ x, lam) ^ x ^ p[1]


def mul(c: F2m, k: int, p):
    out = None
    while k:
        if k & 1:
            out = c.add(out, p)
        p = c.add(p, p)
        k >>= 1
    return out


def equal(a: float, b: float) -> bool:
    return math.isclose(a, b, rel_tol=1e-10, abs_tol=1e-12)


def prime_below_341e12(n: int) -> bool:
    """Deterministic Miller-Rabin bases for n < 341,550,071,728,321."""
    if n < 2 or n >= 341550071728321:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        if not any((x := x * x % n) == n - 1 for _ in range(s - 1)):
            return False
    return True


def poly_remainder(x: int, divisor: int) -> int:
    while x.bit_length() >= divisor.bit_length():
        x ^= divisor << (x.bit_length() - divisor.bit_length())
    return x


def poly_gcd(a: int, b: int) -> int:
    while b:
        a, b = b, poly_remainder(a, b)
    return a


def irreducible_binary_polynomial(n: int, modulus: int) -> bool:
    """Rabin's irreducibility criterion over GF(2)."""
    factors = {p for p in range(2, n + 1) if n % p == 0 and all(p % q for q in range(2, math.isqrt(p) + 1))}
    witness_exponents = {n // p for p in factors}
    x = 2
    for i in range(1, n + 1):
        squared = 0
        for bit in range(x.bit_length()):
            if x & (1 << bit):
                squared ^= 1 << (2 * bit)
        x = poly_remainder(squared, modulus)
        if i in witness_exponents and poly_gcd(x ^ 2, modulus) != 1:
            return False
    return x == 2


def rel(path: pathlib.Path) -> str:
    return path.relative_to(BASE).as_posix()


def run():
    analysis_path = BASE / "analysis.json"
    analysis = json.loads(analysis_path.read_text())
    errors: list[str] = []
    run_records = []
    raw_reports = []
    headline = []
    ref_records = []
    ref_raw = []
    size_results = []
    interval_checks = []
    cert_count = neg_count = repetition_results = 0
    validated_fields = set()

    for size in analysis["sizes"]:
        n, a = size["n"], size["a"]
        group = f"k{a}n{n}"
        rows = []
        curve_keys = set()
        for cp in sorted((BASE / "claims" / group).glob("*.claim.json")):
            label = cp.name.removesuffix(".claim.json")
            claim = json.loads(cp.read_text())["claim"]
            rp = cp.with_name(label + ".replay.json")
            replay = json.loads(rp.read_text())
            report_path = (ROOT / claim["report"]).resolve()
            if not report_path.is_relative_to(BASE) or not report_path.is_file():
                raise ValueError(f"report path invalid: {claim['report']}")
            report = json.loads(report_path.read_text())
            if (claim["n"], claim["a"]) != (n, a) or (report["n"], report["a"]) != (n, a):
                errors.append(f"{group}/{label}: size or curve mismatch")
            if not report["all_verified"] or not report["ic_and_rho_agree"]:
                errors.append(f"{group}/{label}: producer reports invalid result")
            if claim["ic_target_hash"] != claim["rho_target_hash"]:
                errors.append(f"{group}/{label}: different targets")
            selected_attempts = [x for x in claim["attempts"] if x["file"] == report_path.name]
            if len(selected_attempts) != 1 or selected_attempts[0]["status"] != "complete" or selected_attempts[0]["contended"] or selected_attempts[0]["exit_status"] != 0 or claim["resource_caps"]["contended"] or claim["same_resource_envelope"] is not True:
                errors.append(f"{group}/{label}: selected run is contended or does not share resources")
            if report["single_thread"] is not True or report["threads"] != 1 or "one process" not in report["resource_envelope"]["arms"]:
                errors.append(f"{group}/{label}: arms are not recorded in one process")
            if claim["report"] != report_path.relative_to(ROOT).as_posix():
                errors.append(f"{group}/{label}: noncanonical report path")
            report_hash = sha(report_path)
            raw_reports.append({"path": rel(report_path), "sha256": report_hash})
            arms = {}
            for arm in ("ic", "rho"):
                z = report["certificates"][arm]
                meta = z["curve"]
                field = F2m(meta["degree"], meta["irreducible_low_terms"], meta["curve_a"])
                g = tuple(map(int, meta["generator"]))
                q = tuple(map(int, z["target"]))
                r = int(meta["subgroup_order"])
                d = int(z["scalar"])
                curve_keys.add((r, g, field.mod, field.a))
                field_key = (field.n, field.mod, r)
                if field_key not in validated_fields:
                    if not irreducible_binary_polynomial(field.n, field.mod) or not prime_below_341e12(r):
                        errors.append(f"{group}: reducible field polynomial or nonprime subgroup order")
                    validated_fields.add(field_key)
                digest = canonical_hash(z)
                if digest != report["certificates"][arm + "_sha256"] or digest != claim[arm + "_replay_certificate_sha256"] or digest != replay[arm]["digest_recomputed"]:
                    errors.append(f"{group}/{label}/{arm}: certificate digest mismatch")
                if not (field.on(g) and field.on(q) and mul(field, r, g) is None):
                    errors.append(f"{group}/{label}/{arm}: curve or order invalid")
                good = mul(field, d % r, g) == q
                negative = mul(field, (d + 1) % r, g) != q
                if not good or not negative or not replay[arm]["digest_matches"] or not replay[arm]["statement_holds"]:
                    errors.append(f"{group}/{label}/{arm}: scalar replay failed")
                arms[arm] = {"scalar": d, "verified": good, "negative_control_rejected": negative}
                cert_count += 1
                neg_count += int(negative)
            if arms["ic"]["scalar"] != arms["rho"]["scalar"]:
                errors.append(f"{group}/{label}: arm scalar mismatch")
            median = report["median"]
            repetitions = report["repetitions"]
            if not repetitions:
                errors.append(f"{group}/{label}: no measured repetitions")
            for rep in repetitions:
                for arm, key in (("ic", "ic_online"), ("rho", "rho_online")):
                    result = rep[key]
                    if result["verified"] is not True or int(result["recovered"]) != arms[arm]["scalar"]:
                        errors.append(f"{group}/{label}: {arm} repetition result differs from certificate")
                    repetition_results += 1
            medians = (("setup_units", [x["setup_units"] for x in repetitions]),
                       ("rho_setup_units", [x["rho_setup_units"] for x in repetitions]),
                       ("ic_online_units", [x["ic_online"]["units"] for x in repetitions]),
                       ("rho_online_units", [x["rho_online"]["units"] for x in repetitions]),
                       ("ic_online_wall_ns", [x["ic_online"]["wall_ns"] for x in repetitions]),
                       ("rho_online_wall_ns", [x["rho_online"]["wall_ns"] for x in repetitions]))
            for key, values in medians:
                if not equal(statistics.median(values), median[key]):
                    errors.append(f"{group}/{label}: {key} median differs from repetitions")
            for arm, key in (("ic", "ic_online"), ("rho", "rho_online")):
                index = median[f"{arm}_online_repetition"]
                if not 0 <= index < len(repetitions) or not equal(repetitions[index][key]["wall_ns"], median[f"{arm}_online_wall_ns"]):
                    errors.append(f"{group}/{label}: {arm} median repetition pointer wrong")
            if not equal(claim["ic_online_wall_ms"], median["ic_online_wall_ms"]) or not equal(claim["rho_online_wall_ms"], median["rho_online_wall_ms"]):
                errors.append(f"{group}/{label}: claimed timing differs from report")
            if not equal(claim["online_speedup"], median["rho_online_wall_ns"] / median["ic_online_wall_ns"]):
                errors.append(f"{group}/{label}: claimed online wall ratio wrong")
            scale = math.sqrt(int(report["r"]))
            for key, unit in (("s_ic_online", "ic_online_units"), ("s_rho_online", "rho_online_units"), ("s_setup", "setup_units"), ("s_rho_setup", "rho_setup_units")):
                if not equal(median[key], median[unit] / scale) or not equal(claim["units"][key], median[key]):
                    errors.append(f"{group}/{label}: {key} unit conversion or claim mismatch")
            run_records.append({"claim": rel(cp), "claim_sha256": sha(cp), "replay": rel(rp), "replay_sha256": sha(rp), "report": rel(report_path), "report_sha256": report_hash, "arms": arms})
            rows.append((label, median))

        r1 = [m for name, m in rows if name.endswith("-R1")]
        r2 = [m for name, m in rows if name.endswith("-R2")]
        if len(r1) != size["rows"]["R1"] or len(r2) != size["rows"]["R2"] or len(curve_keys) != 1:
            errors.append(f"{group}: row inventory or curve mismatch")
        si = statistics.median(m["s_setup"] for m in r1)
        sr = statistics.median(m["s_rho_setup"] for m in r1)
        ic = statistics.mean(m["s_ic_online"] for m in r1)
        rho = statistics.mean(m["s_rho_online"] for m in r1)
        cold = (si + ic) / (sr + rho)
        online = rho / ic
        for metric, claimed, computed in (("cold IC/rho", size["cold_ratio_ic_over_rho"]["value"], cold), ("online rho/IC", size["online_speedup"]["mean_ratio"], online), ("IC setup", size["s_setup"], si), ("rho setup", size["s_rho_setup"], sr), ("IC online", size["s_ic_online_mean"], ic), ("rho online", size["s_rho_online_mean"], rho)):
            headline.append({"name": f"{group} {metric}", "claimed": claimed, "recomputed": computed, "rel_tol": 1e-10})
            if not equal(claimed, computed):
                errors.append(f"{group}: headline {metric} mismatch")
        size_results.append({"group": group, "curve": size["curve"], "R1": len(r1), "R2": len(r2), "IC_cold_S": si + ic, "rho_cold_S": sr + rho, "cold_ratio": cold, "online_speedup": online})
        # A second bootstrap with an independent seed checks the published
        # uncertainty scale. Monte Carlo endpoints need not match bytewise.
        rng = random.Random(20261024 + n)
        cold_samples, online_samples = [], []
        for _ in range(4000):
            draw = [r1[rng.randrange(len(r1))] for __ in r1]
            i = statistics.mean(m["s_ic_online"] for m in draw)
            r = statistics.mean(m["s_rho_online"] for m in draw)
            cold_samples.append((statistics.median(m["s_setup"] for m in draw) + i) /
                                (statistics.median(m["s_rho_setup"] for m in draw) + r))
            online_samples.append(r / i)
        cold_samples.sort()
        online_samples.sort()
        for metric, samples, published in (("cold", cold_samples, size["cold_ratio_ic_over_rho"]["ci95"]),
                                           ("online", online_samples, size["online_speedup"]["ci95"])):
            computed = [samples[100], samples[3900]]
            deviations = [abs(x - y) / y for x, y in zip(computed, published)]
            interval_checks.append({"group": group, "metric": metric, "published": published,
                                    "independent_4000_draw_interval": computed,
                                    "max_relative_endpoint_deviation": max(deviations)})
            if max(deviations) > 0.03:
                errors.append(f"{group}: {metric} interval differs by over 3%")

        for target in size["reference_check"]["targets"]:
            t = target["target"]
            label = f"T{t:02d}"
            cp = BASE / "claims" / group / f"{label}-R1.claim.json"
            claim = json.loads(cp.read_text())["claim"]
            price_path = ROOT / claim["report"]
            price = json.loads(price_path.read_text())
            candidates = sorted((BASE / "runs/reference" / group).glob(f"{label}*.strong.json"))
            matching = [p for p in candidates if equal((lambda d: d["walk_ms"] * 1e6 / d["walk_steps"])(json.loads(p.read_text())), target["strong_ns_per_step"])]
            if len(matching) != 1:
                errors.append(f"{group}/{label}: reference selection ambiguous")
                continue
            strong_path = matching[0]
            strong = json.loads(strong_path.read_text())
            cert = price["certificates"]["rho"]
            strong_ns = strong["walk_ms"] * 1e6 / strong["walk_steps"]
            rho_ns = price["median"]["rho_online_wall_ns"] / price["rho_counts"]["steps"]
            same_target = list(map(int, cert["target"])) == strong["published_q"]
            same_group = list(map(int, cert["curve"]["generator"])) == strong["generator"] and int(cert["curve"]["subgroup_order"]) == strong["subgroup_order"]
            if not (same_target and same_group and strong["verified"] is True and strong["reference_grade"] == "strong" and strong["quotient_mode"] == "signed_frobenius" and int(cert["scalar"]) == strong["recovered_fixture_scalar"] and rho_ns <= strong_ns and equal(strong_ns, target["strong_ns_per_step"]) and equal(rho_ns, target["rho_ns_per_step"])):
                errors.append(f"{group}/{label}: strongest-rho cross-check failed")
            ref_records.append({"group": group, "target": t, "strong": rel(strong_path), "strong_sha256": sha(strong_path), "price": rel(price_path), "price_sha256": sha(price_path), "same_target": same_target, "strong_ns_per_step": strong_ns, "measured_rho_ns_per_step": rho_ns, "measured_rho_over_strong": rho_ns / strong_ns})
            ref_raw.append({"path": rel(strong_path), "sha256": sha(strong_path)})

    if len(run_records) != 408 or cert_count != 816 or len(ref_records) != 48 or len(size_results) != 6:
        errors.append("expected complete 408/816/48/6 evidence inventory")
    status = "pass" if not errors else "fail"
    replay = {"schema": "rule-v7-independent-replay/v1", "status": "verified" if not errors else "fail", "independent": True, "scalars_verified": not errors, "scalar_certificates_checked": cert_count, "repetition_arm_outcomes_checked": repetition_results, "negative_controls_rejected": neg_count, "runs": run_records, "errors": errors, "method": "Pure Python GF(2^n) curve arithmetic checks [d]G=Q and rejects [d+1]G=Q for each selected IC and rho certificate; every repetition's recovered scalar and verification flag are checked against that certificate."}
    validator = {"schema": "rule-v7-independent-validator/v1", "status": status, "independent": True, "analysis": {"path": "analysis.json", "sha256": sha(analysis_path)}, "raw_data": raw_reports, "headline": headline, "sizes": size_results, "independent_interval_checks": interval_checks, "errors": errors, "limits": ["The recorded bootstrap intervals are checked by independent seeded resampling to within 3% of their endpoints, not byte-reproduced.", "The six point estimates and source-bound scalars are independently recomputed; this does not prove a general ECDLP speedup."]}
    reference = {"schema": "rule-v7-independent-rho-reference/v1", "status": status, "strong_comparisons": ref_records, "raw_data": ref_raw, "all_measured_rho_faster_per_step": all(r["measured_rho_over_strong"] <= 1 for r in ref_records), "cold_by_size": size_results, "gae_band": {str(w): [{"group": x["group"], "ic": x["IC_cold_S"], "rho": x["rho_cold_S"]} for x in size_results] for w in (0, 0.1, 1)}, "gae_band_basis": "Both full arms were timed in one process and all native operations are inside those intervals, as frozen PROTOCOL.md states; the unpriced count is zero, so adding 0, 0.1 or 1 GAE per unpriced operation leaves each measured S unchanged. Count-only model readings remain lower bounds.", "errors": errors, "limits": ["The strong reference is a separate binary on the same public points; the measured comparison rho is the same-process arm and is faster per step on all 48 checked points.", "No claim is made about unexamined rho implementations or deployed curves."]}
    worst = min(size_results, key=lambda x: x["cold_ratio"])
    manifest = {"schema": "validation-gate/v1", "rows": [x["curve"] for x in size_results], "replay_certificate": {"path": "replay-certificate.json", "sha256": None}, "validator_report": {"path": "validator-report.json", "sha256": None}, "rho_reference": {"kind": "negation_frobenius", "protocol": "Frozen v7 PROTOCOL.md: same-process signed-Frobenius rho on each public target, cross-checked against the strong signed-Frobenius walk on 48 matching targets.", "unit": "recorded S per sqrt(subgroup order), both arms from the same process", "binary_or_process": "Each selected price report records one process running both IC and rho arms; binary_blake3 is shared by those arms. The strong walk is a separate same-target cross-check, not the substituted rho arm.", "matches_protocol": status == "pass", "strongest_available": status == "pass" and reference["all_measured_rho_faster_per_step"], "same_unit": True, "same_binary_or_process": True, "gae_band": {str(w): {"ic": worst["IC_cold_S"], "rho": worst["rho_cold_S"]} for w in (0, 0.1, 1)}, "gae_band_basis": reference["gae_band_basis"], "gae_band_rows": "rho-reference.json", "order_holds_across_band": all(x["cold_ratio"] > 1 for x in size_results)}}
    if errors:
        raise ValueError("validation failed: " + "; ".join(errors[:10]))
    outputs = {"replay-certificate.json": replay, "validator-report.json": validator, "rho-reference.json": reference}
    for name, content in outputs.items():
        content_bytes = (json.dumps(content, indent=2, sort_keys=True) + "\n").encode()
        outputs[name] = content_bytes
    manifest["replay_certificate"]["sha256"] = hashlib.sha256(outputs["replay-certificate.json"]).hexdigest()
    manifest["validator_report"]["sha256"] = hashlib.sha256(outputs["validator-report.json"]).hexdigest()
    outputs["VALIDATION.json"] = (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()

    # The historical benchmark row points at this same frozen run, but the
    # benchmark gate resolves paths from benchmarks/<row>/, not this directory.
    # Keep a separate small wrapper with paths to the identical SHA-bound raw
    # reports; never duplicate or rewrite the original experiment.
    benchmark_name = "20261016-ic-rule-v7"
    bench_prefix = f"../../research/ic_tool_program/rule/v7/"
    benchmark_report = {
        "schema": "rule-v7-independent-benchmark-validator/v1",
        "status": status,
        "independent": True,
        "source_validator": {"path": bench_prefix + "validator-report.json",
                             "sha256": manifest["validator_report"]["sha256"]},
        "raw_data": [{"path": bench_prefix + item["path"], "sha256": item["sha256"]}
                     for item in raw_reports],
        "headline": headline,
        "sizes": size_results,
        "limits": validator["limits"],
    }
    benchmark_report_bytes = (json.dumps(benchmark_report, indent=2, sort_keys=True) + "\n").encode()
    benchmark_manifest = {
        "schema": "validation-gate/v1",
        "rows": [benchmark_name],
        "replay_certificate": {"path": bench_prefix + "replay-certificate.json",
                               "sha256": manifest["replay_certificate"]["sha256"]},
        "validator_report": {"path": "validator-report.json",
                             "sha256": hashlib.sha256(benchmark_report_bytes).hexdigest()},
        "rho_reference": {**manifest["rho_reference"],
                          "gae_band_rows": bench_prefix + "rho-reference.json"},
    }
    bench_rel = f"../../../../benchmarks/{benchmark_name}/"
    outputs[bench_rel + "validator-report.json"] = benchmark_report_bytes
    outputs[bench_rel + "VALIDATION.json"] = (
        json.dumps(benchmark_manifest, indent=2, sort_keys=True) + "\n"
    ).encode()
    return outputs


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    outputs = run()
    for name, content in outputs.items():
        path = BASE / name
        if args.check:
            if not path.is_file() or path.read_bytes() != content:
                raise SystemExit(f"stale: {name}")
        else:
            path.write_bytes(content)
    print("validated 408 selected reports, 816 scalar certificates, 48 matched strong-rho references; 6 size ratios")


if __name__ == "__main__":
    main()
