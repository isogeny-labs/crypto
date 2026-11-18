"""Scoring arithmetic of the optimization-level metric (optlevel.record/v1).

Pure functions, standard library only.  Definitions and the reasons for them:
docs/OPTIMIZATION_LEVEL.md.  Nothing here measures anything; `optlevel.py`
produces the calibration and measurement documents these functions combine.
"""
from __future__ import annotations

import math

TOOL_VERSION = "1.0.0"
RECORD_SCHEMA = "optlevel.record/v1"
CALIBRATION_SCHEMA = "optlevel.calibration/v1"
MEASUREMENT_SCHEMA = "optlevel.measurement/v1"
OPS_SCHEMA = "optlevel.ops/v1"

TIERS = ("T-pmu", "T-timing", "T-gpu-ncu", "T-gpu-timing")
# Published grade rule (docs/OPTIMIZATION_LEVEL.md section 6).  Lower edges.
GRADE_EDGES = (("A", 0.70), ("B", 0.40), ("C", 0.20), ("D", 0.08), ("E", 0.0))
NEAR_ROOF = 0.70          # an attainment at or above this is "near the roof"
CEILING_SLACK = 0.02      # attainment above 1 + slack + noise raises CEILING_EXCEEDED
MIN_KERNEL_SECONDS = 0.5  # shorter timed regions raise TOO_SHORT
CONTENDED_CORES = 0.25    # CPU-seconds per second used by everything else on the host
PMU_ONLY = ("ipc", "cycles_per_op", "instructions_per_op", "branch_miss_rate",
            "llc_misses_per_op", "l1d_misses_per_op", "dtlb_misses_per_op",
            "topdown", "vector_share_sampled")
NOT_A_SPEED_CLAIM = (
    "An optimization level describes how close one implementation runs to what its own "
    "machine can do for the operations it counts. It is not a speed result, not a "
    "comparison between algorithms, not a benchmark or leaderboard row, and not evidence "
    "under the validation gate. A perfectly tuned bad algorithm scores high.")


def stats(xs):
    xs = sorted(float(x) for x in xs if x is not None)
    if not xs:
        return None
    n = len(xs)
    med = xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2
    return {"median": med, "min": xs[0], "max": xs[-1], "n": n}


def interp_scale(points, k):
    """Value at k threads from measured points {threads: value}.

    Exact when k was measured; linear between the two neighbours otherwise
    (returned with interpolated=True); clamped to the largest measured point
    beyond it (extrapolated=True), because nothing was measured there.
    """
    pts = sorted((int(a), float(b)) for a, b in points.items())
    if not pts:
        raise ValueError("no scaling points")
    for a, b in pts:
        if a == k:
            return b, {"interpolated": False, "extrapolated": False}
    if k < pts[0][0]:
        return pts[0][1] * k / pts[0][0], {"interpolated": True, "extrapolated": False}
    if k > pts[-1][0]:
        return pts[-1][1], {"interpolated": False, "extrapolated": True}
    for (a0, b0), (a1, b1) in zip(pts, pts[1:]):
        if a0 < k < a1:
            return b0 + (b1 - b0) * (k - a0) / (a1 - a0), {"interpolated": True, "extrapolated": False}
    raise AssertionError("unreachable")


def roofline(peak_ops_per_s, bandwidth_bytes_per_s, bytes_per_op):
    """Classic two-roof roofline in operations per second.

    compute roof = peak rate of the operation class;
    memory roof  = bandwidth / bytes per operation  (= intensity * bandwidth);
    bound        = the lower one.  With unknown traffic (bytes_per_op None)
    the bound is the compute roof and `binding` says so: the true bound can
    only be lower, so an attainment against it is a lower bound.
    """
    if peak_ops_per_s is None or peak_ops_per_s <= 0:
        raise ValueError("peak rate must be positive")
    out = {"compute_roof_ops_per_s": peak_ops_per_s, "memory_roof_ops_per_s": None,
           "bound_ops_per_s": peak_ops_per_s, "binding": "compute (traffic unknown)",
           "next_roof_ratio": None}
    if bytes_per_op is None:
        return out
    if bytes_per_op <= 0:
        out["binding"] = "compute"
        return out
    mem = bandwidth_bytes_per_s / bytes_per_op
    out["memory_roof_ops_per_s"] = mem
    out["bound_ops_per_s"] = min(peak_ops_per_s, mem)
    out["binding"] = "memory" if mem < peak_ops_per_s else "compute"
    out["next_roof_ratio"] = max(peak_ops_per_s, mem) / min(peak_ops_per_s, mem)
    return out


def attainment(rate_stats, bound, bound_low=None):
    """achieved / bound, with an interval from the run spread and, when the
    calibration's own median ceiling is given as bound_low, from the ceiling
    spread too (ceilings are maxima over repeats; the median is the low end)."""
    if rate_stats is None or not bound:
        return None
    lo_bound = bound_low if bound_low else bound
    return {"value": rate_stats["median"] / bound,
            "low": rate_stats["min"] / bound,
            "high": rate_stats["max"] / lo_bound}


def noise_floor(a_rates, b_rates):
    """A/A noise floor of one kernel: the same command in interleaved slots
    A and B.  aa_ratio_deviation = |median(A)/median(B) - 1|; rel_range =
    (max - min)/median over all runs.  The larger one is `value`."""
    sa, sb = stats(a_rates), stats(b_rates)
    if not sa or not sb or sa["n"] < 1 or sb["n"] < 1:
        return None
    allr = stats(list(a_rates) + list(b_rates))
    dev = abs(sa["median"] / sb["median"] - 1.0)
    rng = (allr["max"] - allr["min"]) / allr["median"]
    return {"aa_ratio_deviation": dev, "rel_range": rng, "value": max(dev, rng),
            "pairs": min(sa["n"], sb["n"]), "sufficient": min(sa["n"], sb["n"]) >= 5}


def letter(score):
    if score is None or math.isnan(score):
        return None
    for name, edge in GRADE_EDGES:
        if score >= edge:
            return name
    return "E"


def letter_range(score, rel_noise):
    """Letters the score could carry inside its noise floor, e.g. 'B' or 'B/C'."""
    if score is None:
        return None
    hi, lo = letter(score * (1 + rel_noise)), letter(score * (1 - rel_noise))
    return hi if hi == lo else f"{hi}/{lo}"


def parallel_block(single_rate, full_rate, threads, bound_single, bound_full, curve=None):
    """Raw and roofline-normalised parallel efficiency.

    raw        = rate(k) / (k * rate(1))
    normalised = [rate(k)/rate(1)] / [bound(k)/bound(1)]  = A_full / A_single
    The second does not charge the kernel for the machine's own saturation
    (memory bandwidth and all-core frequency do not scale with k).
    """
    if not single_rate or not full_rate or not threads:
        return None
    raw = full_rate / (threads * single_rate)
    norm = (full_rate / single_rate) / (bound_full / bound_single)
    return {"threads": threads, "raw_efficiency": raw,
            "roofline_normalised_efficiency": norm,
            "machine_scaling": bound_full / (threads * bound_single),
            "curve": curve or []}


def summary_grade(a_single, a_full, rel_noise, invalid_reasons):
    """Published rule: score = sqrt(A_single * A_full) when both operating
    points were measured (scope 'single+parallel'), else the one measured
    (scope names it).  A score is never compared across scopes."""
    if invalid_reasons:
        return {"score": None, "letter": None, "letter_range": None, "scope": "invalid",
                "invalid": list(invalid_reasons), "rule": "sqrt(A_single * A_full); letters A>=0.70 B>=0.40 C>=0.20 D>=0.08 E"}
    if a_single is not None and a_full is not None:
        s, scope = math.sqrt(a_single * a_full), "single+parallel"
    elif a_single is not None:
        s, scope = a_single, "single-thread only"
    elif a_full is not None:
        s, scope = a_full, "parallel only"
    else:
        s, scope = None, "invalid"
    return {"score": s, "letter": letter(s), "letter_range": letter_range(s, rel_noise or 0.0),
            "scope": scope, "invalid": [],
            "rule": "sqrt(A_single * A_full); letters A>=0.70 B>=0.40 C>=0.20 D>=0.08 E"}


def topdown_limit(td):
    """Name the largest non-retiring top-down share, refined by level 2."""
    l1 = {k: td.get(k) for k in ("frontend_bound", "bad_speculation", "backend_bound", "retiring")}
    if any(v is None for v in l1.values()):
        return None
    worst = max(("frontend_bound", "bad_speculation", "backend_bound"), key=lambda k: l1[k])
    if l1["retiring"] >= max(l1[worst], 0.5):
        return "retiring-dominated: the core is busy with useful slots, so the loss is instructions per operation (not vectorized, or overhead around the arithmetic)"
    if worst == "backend_bound":
        mem, core = td.get("memory_bound"), td.get("core_bound")
        if mem is not None and core is not None:
            return ("backend: memory bound (cache or TLB misses inside a compute-bound kernel)" if mem >= core
                    else "backend: core bound (dependency chains, divider, port contention)")
        return "backend bound (level 2 unavailable)"
    return {"frontend_bound": "frontend bound (decode, instruction-cache or BTB misses)",
            "bad_speculation": "bad speculation (branch mispredicts, machine clears)"}[worst]


def limiting_factor(tier, roof, a_single, par, diag):
    """The published decision rule, in order.  Returns (text, basis)."""
    if a_single is None and par is None:
        return "nothing measured", "none"
    norm = par["roofline_normalised_efficiency"] if par else None
    if a_single is not None and a_single >= NEAR_ROOF and (norm is None or norm >= NEAR_ROOF):
        if roof["binding"] == "memory":
            return ("at the memory-bandwidth roof: only higher arithmetic intensity (blocking, packing, "
                    "fewer bytes per operation) or a different algorithm helps", "roofline")
        return "none above threshold: near the roofline; what remains is the algorithmic axis", "roofline"
    if norm is not None and (a_single is None or norm < a_single):
        return (f"parallel scaling: roofline-normalised efficiency {norm:.2f} at {par['threads']} threads "
                "(synchronisation, serial sections, false sharing, or a host-side feeder)"), "scaling curve"
    binding = roof["binding"]
    if binding == "memory":
        lat = diag.get("latency_attainment")
        if lat is not None and 0.5 <= lat <= 2.0:
            return ("memory latency bound: the rate matches one outstanding dependent miss at a time "
                    f"(latency attainment {lat:.2f}); far below the bandwidth roof"), "roofline + latency calibration"
        if tier == "T-pmu" and diag.get("dtlb_misses_per_op") is not None and diag["dtlb_misses_per_op"] > 0.05:
            return "memory bound below the bandwidth roof with TLB misses: page size or access order", "pmu"
        return ("memory bound, below the bandwidth roof: random or dependent access, TLB, or NUMA placement"), "roofline"
    if tier == "T-pmu" and diag.get("topdown"):
        t = topdown_limit(diag["topdown"])
        if t:
            return t, "pmu top-down"
    sr = diag.get("scalar_ceiling_ratio")
    head = diag.get("simd_headroom")
    vs = diag.get("vector_share_static")
    suffix = "" if binding == "compute" else " (memory traffic unknown: the bound may be lower than assumed)"
    if vs is not None and vs < 0.2 and head and head > 1.5:
        return (f"core bound / not vectorized (static vector share {vs:.2f} in the named symbols; "
                f"the class's SIMD ceiling is {head:.1f}x its scalar ceiling)" + suffix), "timing + static disassembly"
    if sr is not None and head and head > 1.5 and sr <= 1.15:
        return (f"core bound / not vectorized or a serial dependency: the rate is {sr:.2f}x the class's scalar "
                f"ceiling and the SIMD ceiling is {head:.1f}x higher (inferred from timing; "
                "a PMU top-down and a vector share are needed to split the two)" + suffix), "timing inference"
    return ("core bound below the compute roof: instruction overhead, partial vectorization or dependency "
            "chains (timing tier cannot split these)" + suffix), "timing inference"


def enforce_tier(record):
    """Raise if a record claims a component its tier cannot have measured."""
    tier = record.get("tier")
    if tier not in TIERS:
        raise ValueError(f"unknown tier {tier!r}")
    diag = record.get("diagnosis") or {}
    if tier != "T-pmu":
        bad = [k for k in PMU_ONLY if diag.get(k) is not None]
        if bad:
            raise ValueError(f"tier {tier} must not report PMU components: {', '.join(bad)}")
        src = (record.get("intensity") or {}).get("source")
        if src in ("llc-miss", "imc"):
            raise ValueError(f"tier {tier} cannot have measured memory traffic ({src})")
    if tier != "T-gpu-ncu" and (diag.get("gpu") or {}).get("ncu") is not None:
        raise ValueError(f"tier {tier} must not report ncu components")
    if not tier.startswith("T-gpu") and diag.get("gpu") is not None:
        raise ValueError("a CPU-tier record must not carry GPU components")
    if record.get("noise_floor") is None and (record.get("grade") or {}).get("score") is not None:
        raise ValueError("a graded record must carry its A/A noise floor")
    return True


# ---- a small JSON-schema subset validator (type, required, properties, enum,
# const, items, minimum, maximum, additionalProperties: false) --------------
_TYPES = {"object": dict, "array": list, "string": str, "boolean": bool, "null": type(None)}


def _type_ok(v, t):
    if t == "number":
        return isinstance(v, (int, float)) and not isinstance(v, bool)
    if t == "integer":
        return isinstance(v, int) and not isinstance(v, bool)
    return isinstance(v, _TYPES[t])


def validate(doc, schema, path="$"):
    errs = []
    t = schema.get("type")
    if t is not None:
        ts = t if isinstance(t, list) else [t]
        if not any(_type_ok(doc, x) for x in ts):
            return [f"{path}: expected {t}, got {type(doc).__name__}"]
    if "enum" in schema and doc not in schema["enum"]:
        errs.append(f"{path}: {doc!r} not in {schema['enum']}")
    if "const" in schema and doc != schema["const"]:
        errs.append(f"{path}: expected {schema['const']!r}")
    if isinstance(doc, (int, float)) and not isinstance(doc, bool):
        if "minimum" in schema and doc < schema["minimum"]:
            errs.append(f"{path}: {doc} < {schema['minimum']}")
        if "maximum" in schema and doc > schema["maximum"]:
            errs.append(f"{path}: {doc} > {schema['maximum']}")
    if isinstance(doc, dict):
        for k in schema.get("required", []):
            if k not in doc:
                errs.append(f"{path}: missing {k}")
        props = schema.get("properties", {})
        for k, v in doc.items():
            if k in props:
                errs += validate(v, props[k], f"{path}.{k}")
            elif schema.get("additionalProperties") is False:
                errs.append(f"{path}: unexpected {k}")
    if isinstance(doc, list) and "items" in schema:
        for i, v in enumerate(doc):
            errs += validate(v, schema["items"], f"{path}[{i}]")
    return errs


# ---- record assembly -------------------------------------------------------
def class_ceiling(calib, op_class, threads):
    """(peak, peak_median, meta) of an operation class at `threads` cores."""
    c = (calib.get("compute") or {}).get(op_class)
    if not c:
        raise KeyError(f"calibration has no ceiling for operation class {op_class!r}; "
                       f"classes: {sorted((calib.get('compute') or {}))}")
    peak, meta = interp_scale({k: v["max"] for k, v in c["total_ops_per_s"].items()}, threads)
    med, _ = interp_scale({k: v["median"] for k, v in c["total_ops_per_s"].items()}, threads)
    return peak, med, meta


def bandwidth(calib, threads):
    b = calib["memory"]["bandwidth_bytes_per_s"]
    peak, meta = interp_scale({k: v["max"] for k, v in b.items()}, threads)
    med, _ = interp_scale({k: v["median"] for k, v in b.items()}, threads)
    return peak, med, meta


def build_record(calib, meas, *, algorithmic=None, extra_diag=None, kernel=None):
    """Combine one calibration and one measurement into optlevel.record/v1."""
    flags = list(meas.get("flags", []))
    tier = meas["tier"]
    op_class = meas["op_class"]
    if calib["host"]["fingerprint"] != meas["host"]["fingerprint"]:
        flags.append("CALIBRATION_HOST_MISMATCH")
    for f in calib.get("flags", []):
        flags.append("CALIBRATION_" + f)

    # memory traffic: measured sources outrank the declared one
    intensity = {"source": "none", "bytes_per_op": None, "ops_per_byte": None,
                 "declared_bytes_per_op": meas.get("declared_bytes_per_op"),
                 "measured_bytes_per_op": None, "declared_over_measured": None}
    mt = meas.get("memory_traffic") or {}
    if tier == "T-pmu" and mt.get("bytes_per_op") is not None:
        intensity.update(source=mt["source"], bytes_per_op=mt["bytes_per_op"], measured_bytes_per_op=mt["bytes_per_op"])
        if intensity["declared_bytes_per_op"] and mt["bytes_per_op"] > 0:
            intensity["declared_over_measured"] = intensity["declared_bytes_per_op"] / mt["bytes_per_op"]
    elif intensity["declared_bytes_per_op"] is not None:
        intensity.update(source="declared", bytes_per_op=intensity["declared_bytes_per_op"])
        ws, llc = meas.get("working_set_bytes"), (calib.get("memory") or {}).get("llc_bytes")
        intensity["working_set_bytes"] = ws
        if ws is not None and llc and ws <= llc:
            # the bytes stay in cache: nothing is charged to DRAM.  Cache-level
            # bandwidths are not calibrated, so between L2 and the LLC the
            # compute roof is an upper bound on the true roof.
            intensity.update(bytes_per_op=0.0, in_cache=True)
            l2 = max((v for k, v in (calib["host"].get("caches_bytes") or {}).items() if k.startswith("L2")), default=None)
            if l2 and ws > l2:
                flags.append("WORKING_SET_IN_LLC")
    if intensity["bytes_per_op"]:
        intensity["ops_per_byte"] = 1.0 / intensity["bytes_per_op"]
    if intensity["source"] == "none":
        flags.append("INTENSITY_UNKNOWN")

    points = meas["points"]  # {threads(str): {"ops_per_s": stats, ...}}
    single = points.get("1")
    full_k = max((int(k) for k in points if int(k) > 1), default=None)
    full = points.get(str(full_k)) if full_k else None

    def roof_at(k):
        peak, peak_med, m1 = class_ceiling(calib, op_class, k)
        bw, bw_med, m2 = bandwidth(calib, k)
        r = roofline(peak, bw, intensity["bytes_per_op"])
        lo = roofline(peak_med, bw_med, intensity["bytes_per_op"])["bound_ops_per_s"]
        r.update(threads=k, bandwidth_bytes_per_s=bw, bound_low_ops_per_s=lo,
                 ceiling_interpolated=m1["interpolated"] or m2["interpolated"],
                 ceiling_extrapolated=m1["extrapolated"] or m2["extrapolated"])
        return r

    roofs, att = {}, {}
    if single:
        roofs["single"] = roof_at(1)
        att["single"] = attainment(single["ops_per_s"], roofs["single"]["bound_ops_per_s"], roofs["single"]["bound_low_ops_per_s"])
    if full:
        roofs["full"] = roof_at(full_k)
        att["full"] = attainment(full["ops_per_s"], roofs["full"]["bound_ops_per_s"], roofs["full"]["bound_low_ops_per_s"])
        if roofs["full"]["ceiling_extrapolated"]:
            flags.append("CEILING_EXTRAPOLATED")
    a1 = att["single"]["value"] if "single" in att else None
    an = att["full"]["value"] if "full" in att else None
    # Validity of the full point: k copies need k free cores.  When everything
    # else on the host used more than CONTENDED_CORES during those runs, the
    # full point is reported but neither graded nor used for diagnosis.
    full_other = ((full or {}).get("other_cpu_cores") or {}).get("median")
    full_usable = full is not None and not (full_other is not None and full_other > CONTENDED_CORES)
    if full is not None and not full_usable:
        flags.append("FULL_POINT_CONTENDED")

    par = None
    if single and full:
        curve = [{"threads": int(k), "ops_per_s": v["ops_per_s"]["median"],
                  "raw_efficiency": v["ops_per_s"]["median"] / (int(k) * single["ops_per_s"]["median"])}
                 for k, v in sorted(points.items(), key=lambda kv: int(kv[0]))]
        par = parallel_block(single["ops_per_s"]["median"], full["ops_per_s"]["median"], full_k,
                             roofs["single"]["bound_ops_per_s"], roofs["full"]["bound_ops_per_s"], curve)

    nf = meas.get("noise_floor")
    rel_noise = nf["value"] if nf else 0.0
    if nf is None:
        flags.append("NO_NOISE_FLOOR")
    elif not nf.get("sufficient"):
        flags.append("NOISE_FLOOR_UNDER_5_PAIRS")

    diag = {k: None for k in PMU_ONLY}
    diag.update({"scalar_ceiling_ratio": None, "simd_headroom": None, "vector_share_static": None,
                 "latency_attainment": None, "startup_share": meas.get("startup_share"), "gpu": None,
                 "unavailable": {}})
    cc = (calib.get("compute") or {}).get(op_class, {})
    if single and cc.get("scalar_ops_per_s") and cc.get("simd_ops_per_s"):
        diag["scalar_ceiling_ratio"] = single["ops_per_s"]["median"] / cc["scalar_ops_per_s"]
        diag["simd_headroom"] = cc["simd_ops_per_s"] / cc["scalar_ops_per_s"]
    lat = (calib.get("memory") or {}).get("latency_ns")
    if single and lat and roofs["single"]["binding"] == "memory":
        # dependent misses per second one thread can serve, one at a time
        diag["latency_attainment"] = single["ops_per_s"]["median"] * lat["median"] * 1e-9
    if tier == "T-pmu":
        for k, v in (meas.get("pmu") or {}).items():
            if k in diag:
                diag[k] = v
    else:
        why = meas.get("pmu_unavailable_reason") or "no hardware PMU on the measuring host"
        for k in PMU_ONLY:
            diag["unavailable"][k] = why
    for k, v in (extra_diag or {}).items():
        diag[k] = v
    if diag.get("gpu") is None and tier.startswith("T-gpu"):
        diag["gpu"] = meas.get("gpu")

    invalid = []
    for name in ("single", "full"):
        if name == "full" and not full_usable:
            continue
        spread = 0.0
        if name == "full":
            fs = full["ops_per_s"]
            spread = (fs["max"] - fs["min"]) / fs["median"]
            if nf is not None:
                nf = dict(nf, full_rel_range=spread)
        slack = CEILING_SLACK + rel_noise      # the single-thread A/A floor, for both points
        if name in att and att[name]["value"] > 1.0 + slack:
            flags.append("CEILING_EXCEEDED")
            invalid.append(f"attainment {att[name]['value']:.2f} at the {name} point exceeds 1: the calibrated "
                           "ceiling is not a ceiling for this kernel, or its operation counter overcounts")
            break
    if "TOO_SHORT" in flags:
        invalid.append(f"timed region shorter than {MIN_KERNEL_SECONDS} s")
    if "OPS_MISSING" in flags:
        invalid.append("the command reported no operation count")

    grade = summary_grade(a1, an if full_usable else None, rel_noise, invalid)
    if grade["score"] is not None and roofs:
        text, basis = limiting_factor(tier, roofs.get("single") or roofs["full"], a1, par if full_usable else None, diag)
    else:
        text, basis = "invalid: " + "; ".join(invalid) if invalid else "nothing measured", "validity checks"
    grade.update(limiting_factor=text, limiting_basis=basis)

    record = {
        "schema": RECORD_SCHEMA, "kind": "record", "tool_version": TOOL_VERSION,
        "created_utc": meas.get("created_utc"), "label": meas.get("label", ""),
        "tier": tier, "device": "gpu" if tier.startswith("T-gpu") else "cpu",
        "kernel": dict({"op_class": op_class, "command": meas.get("command"),
                        "ops_source": meas.get("ops_source")}, **(kernel or {})),
        "host": meas["host"],
        "calibration": {"fingerprint": calib["host"]["fingerprint"], "created_utc": calib.get("created_utc"),
                        "sha256": calib.get("_sha256"), "conditions": calib.get("conditions")},
        "achieved": {("single" if k == "1" else f"threads_{k}"): v for k, v in points.items()},
        "intensity": intensity, "roofline": roofs, "attainment": att, "parallel": par,
        "diagnosis": diag,
        "algorithmic": algorithmic or {"counted_ops": None, "best_known_ops": None, "ratio": None,
                                       "note": "not supplied; never folded into the score"},
        "noise_floor": nf, "conditions": meas.get("conditions"),
        "grade": grade, "flags": sorted(set(flags)),
        "not_a_speed_claim": NOT_A_SPEED_CLAIM,
    }
    enforce_tier(record)
    return record


def benchrec_metrics(record):
    """The record's values in benchrec's unit vocabulary (ops_per_s, ratio, ipc).
    These describe an implementation on a host; they carry no reference kind."""
    out = []
    lab = record["kernel"]["op_class"]
    for name, pt in record["achieved"].items():
        s = pt["ops_per_s"]
        out.append({"name": f"optlevel:{lab}:{name}:ops_per_s", "value": s["median"], "unit": "ops_per_s",
                    "better": "higher", "n": s["n"], "spread_low": s["min"], "spread_high": s["max"]})
    for name, a in record["attainment"].items():
        out.append({"name": f"optlevel:{lab}:{name}:attainment", "value": a["value"], "unit": "ratio",
                    "better": "neutral", "spread_low": a["low"], "spread_high": a["high"]})
    if record["diagnosis"].get("ipc") is not None:
        out.append({"name": f"optlevel:{lab}:ipc", "value": record["diagnosis"]["ipc"], "unit": "ipc", "better": "neutral"})
    return out
