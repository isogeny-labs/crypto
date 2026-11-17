"""Assemble the replay certificates and validator reports (validator-side).

Usage: python3 -I assemble.py <repo-root> <scratch-validator-dir>

Reads only validator outputs (selection, comparison, verifier output, recompute
and headline JSON) and writes:
  benchmarks/20261018-weil-descent-n2-routes/{replay-certificate,validator-report}.json
  benchmarks/20261018-weil-descent-round4-genus2-ic/{replay-certificate,validator-report}.json
"""
import hashlib
import json
import os
import re
import sys

PINNED = "d01c574a271dbf8d04b19636d4612cf9926915eb"
BRIEFED = "3a1aaa916c8017d58b53f08e15f9a8726640323d"
LATER = "dc46011b90c4ab81a9127616539fbd69bf5a2153"
R = "research/weil_descent_angles_20261018"
VALD = f"{R}/validation"
B3 = "benchmarks/20261018-weil-descent-n2-routes"
B4 = "benchmarks/20261018-weil-descent-round4-genus2-ic"
TOOL_SHA = "33f1411d384b85c2fe72979a215181c9ba9336e2aa4e778f369c4b9a0bb0f230"

RUN_META = {
    "itemOJ-run-20261018T2132Z": dict(
        study="r3", recorded_commit="e8568184d74fa862a5ea28e3a52285b249811958",
        recorded_cmd="n2 <dir> run (N2_THREADS=4, sizes 20..40, 10 targets per instance)",
        job_identity="mode run; job = (bits, instance), bits in {20,24,28,32,36,40}, instance in {0,1,2}; seed = splitmix(0x6e32_5eed ^ (bits << 32) ^ instance); 10 targets; arms R, J1, D1, J2, D2, J4, D4 (70 rows per job)",
        csv="routes.csv"),
    "itemF-control-run-20261018T2128Z": dict(
        study="r3", recorded_commit="c51acec659201b5aed5a412ca673ea3ccabfe761",
        recorded_cmd="n2 <dir> control (N2_THREADS=4, sizes 20..40)",
        job_identity="mode control; job = (bits, instance), same seeds/instances as the run mode; 3 targets; arms Jwalk, Jind at c = 2 (6 rows per job)",
        csv="control.csv"),
    "H1-hsweep-run-20261018T2240Z": dict(
        study="r4", recorded_commit="0bd2a0f5c0b0f0005d57850c12466fa82b414c59",
        recorded_cmd="rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 ./n2 \"$OUT\" hsweep",
        job_identity="mode hsweep; job = (bits, h, instance), bits in {32,36,40}, h in {4,8,12,16,24,32,48,64,96,128}, instance 0..3; seed = splitmix(0x4831_0000 ^ (bits << 40) ^ (h << 8) ^ instance); 10 targets; arms R, J (+ Jind at 32 bits)",
        csv="hsweep.csv"),
    "H2-glue-run-20261018T2240Z": dict(
        study="r4", recorded_commit="0bd2a0f5c0b0f0005d57850c12466fa82b414c59",
        recorded_cmd="rustc --edition 2021 -O examples/n2_routes.rs -o n2 && ./n2 \"$OUT\" gluecheck && N2_THREADS=16 ./n2 \"$OUT\" glue",
        job_identity="mode glue; job = (bits, h, instance), bits in {28,32,36,40}, h in {1,4}, instance 0..3; seed = splitmix(0x6c75_0000 ^ (bits << 40) ^ (h << 8) ^ instance); 10 targets; arms R, Jglue (20 rows per job)",
        csv="glue.csv"),
    "H4-memsweep-run-20261018T2209Z": dict(
        study="r4", recorded_commit="6083c58aa45e04e4d3194689b3b3e6ef5c7be115",
        recorded_cmd="n2_routes <dir> memsweep (N2_THREADS=4, sizes 32,36)",
        job_identity="mode memsweep; job = (bits, theta, instance), bits in {32,36}, theta in {1/2,1/4,1/8}, instance 0..3; seed = splitmix(0x4831_0000 ^ (bits << 40) ^ (4 << 8) ^ instance) (the h = 4 instances of H1); 10 targets; arm Jtheta (10 rows per job)",
        csv="memsweep.csv"),
    "H6-large-run-20261018T2210Z": dict(
        study="r4", recorded_commit="6083c58aa45e04e4d3194689b3b3e6ef5c7be115",
        recorded_cmd="rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 N2_SIZES=44,48 N2_CS=2 ./n2 \"$OUT\" run",
        job_identity="mode run with N2_CS=2; job = (bits, instance), bits in {44,48}, instance 0..2; seed = splitmix(0x6e32_5eed ^ (bits << 32) ^ instance); 10 targets; arms R, J2, D2 (30 rows per job)",
        csv="routes.csv"),
}


def sha_file(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def to_yaml(o, ind=0):
    pad = "  " * ind
    if isinstance(o, dict):
        if not o:
            return " {}"
        out = ""
        for k, v in o.items():
            if isinstance(v, (dict, list)) and v:
                out += f"\n{pad}{k}:" + to_yaml(v, ind + 1)
            else:
                out += f"\n{pad}{k}:" + to_yaml(v, ind + 1)
        return out
    if isinstance(o, list):
        if not o:
            return " []"
        out = ""
        for v in o:
            if isinstance(v, dict) and v:
                body = to_yaml(v, ind + 1).lstrip("\n")
                out += f"\n{pad}- " + body.lstrip()
            else:
                out += f"\n{pad}-" + to_yaml(v, ind + 1)
        return out
    if isinstance(o, bool):
        return " true" if o else " false"
    if o is None:
        return " null"
    if isinstance(o, (int, float)):
        return f" {o!r}"
    return " " + json.dumps(str(o))


def parse_verify(path):
    out, cur = {}, None
    for line in open(path):
        line = line.rstrip("\n")
        if line.startswith("FILE "):
            cur = os.path.basename(os.path.dirname(line[5:]))
            out[cur] = dict(checks={}, arms={})
        elif cur and line.startswith("  lines "):
            m = re.match(r"  lines (\d+) distinct_curves (\d+) j_in_Fp (\d+) j_0_or_1728 (\d+)", line)
            out[cur].update(lines=int(m[1]), distinct_curves=int(m[2]), j_in_Fp=int(m[3]), j_0_or_1728=int(m[4]))
        elif cur and line.startswith("  check "):
            m = re.match(r"  check (\S+): (\d+)/(\d+)", line)
            out[cur]["checks"][m[1]] = [int(m[2]), int(m[3])]
        elif cur and line.startswith("  arm "):
            m = re.match(r"  arm (\S+): (\d+)/(\d+)", line)
            out[cur]["arms"][m[1]] = [int(m[2]), int(m[3])]
        elif cur and line.startswith("  FILE_STATUS "):
            out[cur]["status"] = line.split()[-1]
    return out


def main():
    repo, sv = sys.argv[1:3]
    nonce = open(f"{sv}/nonce.txt").read().strip()
    nonce_time = open(f"{sv}/nonce_time.txt").read().strip()
    sel = json.load(open(f"{sv}/selection.json"))
    cmp_ = json.load(open(f"{sv}/compare_out.json"))
    ver = parse_verify(f"{sv}/verify_all.txt")
    rec = json.load(open(f"{sv}/recompute_out.json"))
    heads = json.load(open(f"{sv}/headlines_out.json"))
    assert not heads["failures"], heads["failures"]
    files = {name: dict(path=f"{VALD}/{name}", sha256=sha_file(f"{repo}/{VALD}/{name}")) for name in (
        "verify_dump.rs", "select_jobs.py", "compare_replay.py", "recompute.py", "headlines.py",
        "replay_pass1.sh", "replay_pass2_supplementary.sh", "assemble.py")}
    art = {name: dict(path=f"{VALD}/replay/{name}", sha256=sha_file(f"{repo}/{VALD}/replay/{name}")) for name in (
        "nonce.txt", "selection.json", "replay_log.txt", "replay-comparison.json", "verify_dump.out.txt",
        "recompute.out.json", "headlines.out.json")}

    sampling_rule = (
        "select_jobs.py (fixed before the nonce was drawn): for each source run, enumerate its job list as n2_routes.rs main builds it "
        "for the recorded command; score each job by sha256('<nonce>|<run-dir>|<job-id>') ascending; select jobs of distinct sizes in "
        "score order up to the run's quota (itemOJ 3, itemF 2, H1 3, H2 3, H4 2, H6 1), a second pass admitting new cells if a run has "
        "fewer sizes than its quota. The replay unit is the smallest cell the tool can be restricted to by env (N2_SIZES, N2_HS, N2_CS), "
        "so every row of the selected job's cell is replayed and compared, and the selected job's own rows are reported separately. "
        "Supplementary cells (validator-chosen, NOT nonce-drawn, flagged nonce_drawn=false) extend coverage to every remaining recorded "
        "cell; they do not replace the nonce sample.")

    rlog = {}
    for line in open(f"{sv}/replay/replay_log.txt"):
        m = re.match(r"(\S+) mode=(\S+) env=(.*) rc=(\d+) wall=(\d+)s", line.strip())
        if m:
            rlog[m[1]] = dict(command=f"env {m[3]} N2_THREADS=4 N2_DUMP=<cell>/dump.jsonl n2 <cell> {m[2]}".replace("env  ", "env "),
                              exit_code=int(m[4]), wall_seconds=int(m[5]))

    def run_entry(run):
        meta = RUN_META[run]
        cells = [c for c in cmp_ if c["run"].endswith(run)]
        ent = dict(run=f"{R}/results/{run}", recorded_with_commit=meta["recorded_commit"], recorded_cmd=meta["recorded_cmd"],
                   job_identity=meta["job_identity"], nonce_selected_jobs=sel[run], replay_cells=[])
        tot = dict(rows_recorded_in_replayed_cells=0, rows_compared=0, rows_matched=0, nonce_rows_compared=0, nonce_rows_matched=0,
                   scalars_checked=0, scalars_ok=0, dump_lines=0)
        checks = {}
        for c in cells:
            v = ver[c["cell"]]
            line_ok = min(v["checks"]["k_found_P_eq_Q"][0], v["checks"]["k_found_eq_k"][0], v["checks"]["Q_on_E"][0])
            cell_checks_ok = all(ok == t for ok, t in v["checks"].values()) and v["status"] == "ok"
            for k, (ok, t) in v["checks"].items():
                a = checks.setdefault(k, [0, 0])
                a[0] += ok
                a[1] += t
            csvp = f"{VALD}/replay/{c['cell']}/{c['csv']}"
            dumpp = f"{VALD}/replay/{c['cell']}/dump.jsonl.gz"
            ent["replay_cells"].append(dict(
                cell=c["cell"], nonce_drawn=c["nonce_drawn"], cell_filter=c["cell_filter"], replay=rlog[c["cell"]],
                rows_recorded_in_cell=c["rows_recorded_in_cell"], rows_replayed=c["rows_replayed"], rows_matched=c["rows_matched"],
                exact_match=c["exact_match"], compared_columns=c["compared_columns"],
                selected_job_rows=c["selected_jobs"],
                replay_csv=dict(path=csvp, sha256=sha_file(f"{repo}/{csvp}")),
                dump=dict(path=dumpp, sha256_gz=sha_file(f"{repo}/{dumpp}"), sha256_uncompressed=sha_file(f"{sv}/replay/{c['cell']}/dump.jsonl"), lines=c["dump_lines"]),
                dump_tied_to_rows=c["dump_tie_ok"],
                scalars_checked=v["lines"], scalars_ok=line_ok, scalars_ok_definition="min over dump lines passing Q_on_E, k_found*P = Q and k_found == k", all_scalar_checks_ok=cell_checks_ok,
                distinct_curves=v["distinct_curves"], curves_with_j_in_Fp=v["j_in_Fp"], curves_with_j_0_or_1728=v["j_0_or_1728"],
                answers_verified_per_arm=v["arms"]))
            tot["rows_recorded_in_replayed_cells"] += c["rows_recorded_in_cell"]
            tot["rows_compared"] += c["rows_replayed"]
            tot["rows_matched"] += c["rows_matched"]
            if c["nonce_drawn"]:
                tot["nonce_rows_compared"] += c["rows_replayed"]
                tot["nonce_rows_matched"] += c["rows_matched"]
            tot["scalars_checked"] += v["lines"]
            tot["scalars_ok"] += line_ok
            tot["dump_lines"] += c["dump_lines"]
        recorded_rows = sum(1 for _ in open(f"{repo}/{R}/results/{run}/{meta['csv']}")) - 1
        ent.update(tot)
        ent["rows_recorded_in_run"] = recorded_rows
        ent["coverage"] = f"{tot['rows_compared']}/{recorded_rows} recorded rows replayed"
        ent["scalar_checks"] = checks
        ent["all_rows_match"] = all(c["exact_match"] and c["dump_tie_ok"] for c in cells) and tot["rows_matched"] == tot["rows_compared"]
        ent["all_scalars_ok"] = all(ok == t for ok, t in checks.values()) and tot["scalars_ok"] == tot["scalars_checked"]
        return ent

    common_cert = dict(
        schema="replay-certificate (validation-gate/v1 fields + validator detail)",
        independent=True,
        validator="Validator subagent (claude-opus-5-5, crypto-autoresearcher agents/validator.md), launched by the producing session but with its own nonce, sampling rule, replay builds, verifier and comparison code; the producer's numbers were not used as inputs",
        date="2026-10-24",
        commit=PINNED,
        commit_notes=[
            f"Pinned snapshot {PINNED} (coordinator update). The brief named {BRIEFED}; the only difference in examples/n2_routes.rs is inside gae_band (Arm D priced over 6 cells), not on any replay path (validator-checked git diff).",
            f"The replay binary was built from the {PINNED} source (sha256 {TOOL_SHA}); the {BRIEFED} source has sha256 0937a69b77be0e0ecf3cbb864446c49b2d29922d2bd05d985786838b7255c8f9.",
            f"HEAD later moved to {LATER} (adds Erratum V text to PROTOCOL.md, REPORT.md, REPORT_ROUND4.md only); every file used here is byte-identical between {PINNED} and {LATER} except those three documents, and claimed values were read from the {PINNED} tree (git archive).",
        ],
        tool="examples/n2_routes.rs",
        tool_sha256=TOOL_SHA,
        tool_build="rustc 1.97.0 (2d8144b78 2026-07-07) --edition 2021 -O; host: this session's cloud container, 4 vCPU (the recorded H1/H2/H6 runs used rustc 1.99.1 on AMD EPYC 4564P RunPod pods; rows replay exactly across compiler and host)",
        nonce=nonce,
        nonce_source="python3 -I -c 'import os; print(os.urandom(32).hex())'",
        nonce_drawn_at=nonce_time,
        sampling_rule=sampling_rule,
        selection_script=files["select_jobs.py"],
        comparison=dict(rule="every replayed CSV row equals a recorded row of the same cell in every column except a trailing wall-clock `seconds` column (control.csv has none and is compared on all columns); multiset equality per cell; each dump line is tied 1:1 to its replayed row by (bits, [h], instance, target, arm[, theta]) with equal p, ell, N and verified == (k_found == k)",
                        script=files["compare_replay.py"],
                        negative_control="a copy of routes.csv with walk_M + 1 in one 28-bit row gave 209/210 matched (exact false); a seconds-only change still matched 210/210"),
        verifier=dict(path=files["verify_dump.rs"]["path"], sha256=files["verify_dump.rs"]["sha256"],
                      build="rustc --edition 2021 -O verify_dump.rs (unit test: rustc --test)",
                      written_in="/tmp/claude-0/-home-user/829a5db7-e7b3-5b24-827a-595408da4b3a/scratchpad/validator/ first, then copied byte-identical",
                      independence="std-only; its own F_{p^2} = F_p[i]/(i^2 - kappa), affine Weierstrass arithmetic, deterministic Miller-Rabin and Tonelli-Shanks over F_{p^2}; shares no code with n2_routes.rs",
                      checks_per_line=["p prime", "kappa a non-residue mod p", "4a^3 + 27b^2 != 0", "j(E) not in F_p (and not 0 or 1728)", "P on E, P != O", "Q on E", "ell prime (deterministic Miller-Rabin)", "ell*P = O", "ell | N", "|N - (p^2 + 1)| <= 2p", "N*R = O for 4 random R on E", "1 <= k < ell", "k_found*P = Q", "k_found == k"],
                      negative_controls="one real 44-bit dump line mutated 7 ways (k_found+1, k changed, Q negated, P off-curve, ell+2, N+2, N outside Hasse): each fails the expected checks, exit 1",
                      output=art["verify_dump.out.txt"]),
        artifacts=art,
    )

    for study, bdir, runs in (("r3", B3, ["itemOJ-run-20261018T2132Z", "itemF-control-run-20261018T2128Z"]),
                              ("r4", B4, ["H1-hsweep-run-20261018T2240Z", "H2-glue-run-20261018T2240Z",
                                          "H4-memsweep-run-20261018T2209Z", "H6-large-run-20261018T2210Z"])):
        entries = [run_entry(r) for r in runs]
        ok = all(e["all_rows_match"] and e["all_scalars_ok"] for e in entries)
        cert = dict(status="verified" if ok else "failed", scalars_verified=all(e["all_scalars_ok"] for e in entries))
        cert.update(common_cert)
        cert["runs"] = entries
        cert["totals"] = {k: sum(e[k] for e in entries) for k in ("rows_compared", "rows_matched", "nonce_rows_compared", "nonce_rows_matched", "scalars_checked", "scalars_ok")}
        cert["totals"]["rows_recorded"] = sum(e["rows_recorded_in_run"] for e in entries)
        lim = []
        if study == "r3":
            lim.append("Replay is of the recorded command at the pinned tool; the item-OJ and item-F runs were made at e8568184d / c51acec65 in this session's container, whose dirty-tree state was not recorded; the exact replay at d01c574a binds the recorded rows to committed code regardless.")
        else:
            lim.append("H4 ran in the producer's container at 6083c58aa without a recorded dirty-tree state or rusage; H1/H2/H6 pods fetched the tool at the cited commit with a sha256 check that the validator re-derived from git (fd8faeb6... for 0bd2a0f5c, fb19b56a... for 6083c58aa).")
        lim.append("Wall-clock `seconds` is excluded from the match by rule; no timing claim is certified.")
        lim.append("Operation counts (walk_M, jac_M, la_native, ...) are certified as reproducible (exact replay), not as independently re-derived: the verifier checks answers and group data, not the counting.")
        cert["limitations"] = lim
        os.makedirs(f"{repo}/{bdir}", exist_ok=True)
        with open(f"{repo}/{bdir}/replay-certificate.json", "w") as f:
            json.dump(cert, f, indent=1)
            f.write("\n")
        print(bdir, "certificate", cert["status"], cert["totals"])

    # ------------------------------------------------------------ validator reports
    V = rec["values"]
    tc = {c["file"]: c for c in rec["text_comparisons"]}

    def raw(bdir, rel):
        return dict(path=os.path.relpath(f"{repo}/{rel}", f"{repo}/{bdir}"), repo_path=rel, sha256=sha_file(f"{repo}/{rel}"))

    def gate(vals, key_prefix):
        out = {}
        for w in ("0", "0.1", "1"):
            g = vals[f"{key_prefix}/w{w}"]
            out[w] = dict(ic=g["ic"], rho=g["rho"], ratio=g["ratio"], ic_rows=g["n_ic"], rho_rows=g["n_rho"])
        out["order_holds_across_band"] = all(out[w]["ic"] > out[w]["rho"] for w in ("0", "0.1", "1")) or all(out[w]["ic"] < out[w]["rho"] for w in ("0", "0.1", "1"))
        out["cheaper_at_all_three_points"] = "rho" if all(out[w]["ic"] > out[w]["rho"] for w in ("0", "0.1", "1")) else ("IC" if all(out[w]["ic"] < out[w]["rho"] for w in ("0", "0.1", "1")) else "neither")
        return out

    tools = [files[k] for k in ("recompute.py", "headlines.py")]
    tools[0] = dict(tools[0], invocation=f"python3 -I {VALD}/recompute.py <tree of {PINNED}> recompute.out.json")
    tools[1] = dict(tools[1], invocation=f"python3 -I {VALD}/headlines.py <tree of {PINNED}> recompute.out.json headlines.out.json")

    sym_note = ("Symmetry: arm R is rho on E(F_{p^2}) with the negation map (r = 64 adding walk, distinguished points, Bos-Kleinjung-Lenstra fruitless-cycle handling). "
                "The verifier computed j(E) for every replayed instance: j is never in F_p and never 0 or 1728 (char p >= 1091). So Aut(E) over the algebraic closure is {+-1}: "
                "no small-order automorphism beyond negation exists; and since j^p != j, E is not isomorphic to its conjugate E^sigma, so no GLS-style twisted Frobenius of order 2 or 4 acts on E(F_{p^2}). "
                "A map psi = phi o Frobenius built from an isogeny phi: E^sigma -> E has degree p*deg(phi) > 1, so it is not an automorphism and does not give rho equivalence classes "
                "(reasoned, not measured). Negation is therefore the strongest symmetry available to rho on these curves. Same binary and unit: arms R and IC run in the same n2_routes process and thread, "
                "on the same instance, base point and target, with the same thread-local counters (F_p mult M; F_{p^2} mul 3M, sqr 2M, inv 1I + 4M; hashing / table lookups / LA mod ell as native ops), "
                "and are priced by the same cost function.")

    yaml_reports = []
    def curve_census(runs):
        curves, insts = set(), set()
        for c in cmp_:
            if any(c["run"].endswith(r) for r in runs):
                for line in open(f"{sv}/replay/{c['cell']}/dump.jsonl"):
                    d = json.loads(line)
                    curves.add((d["p"], d["kappa"], tuple(d["a"]), tuple(d["b"])))
        import csv as _csv
        for r in runs:
            with open(f"{repo}/{R}/results/{r}/{RUN_META[r]['csv']}") as fh:
                for row in _csv.DictReader(fh):
                    insts.add((row["p"], row["ell"]))
        return len(curves), len(insts)

    for study, bdir in (("r3", B3), ("r4", B4)):
        if study == "r3":
            raws = [raw(bdir, f"{R}/results/itemOJ-run-20261018T2132Z/routes.csv"), raw(bdir, f"{R}/results/itemF-control-run-20261018T2128Z/control.csv")]
            text_files = [f"{R}/results/itemOJ-run-20261018T2132Z/summary.txt", f"{R}/results/itemOJ-run-20261018T2132Z/routes_summary.csv",
                          f"{R}/results/itemOJ-run-20261018T2132Z/ranking.csv", f"{R}/results/itemF-control-run-20261018T2128Z/control_summary.txt",
                          f"{VALD}/gae-band/itemOJ-run-20261018T2132Z.txt"]
            gates = {f"J c={c}, 40 bits (30 IC rows, 30 rho rows)": gate(V["gae_oj"], f"gate/40/0/J{c}") for c in ("1", "2", "4")}
        else:
            raws = [raw(bdir, f"{R}/results/H1-hsweep-run-20261018T2240Z/hsweep.csv"), raw(bdir, f"{R}/results/H2-glue-run-20261018T2240Z/glue.csv"),
                    raw(bdir, f"{R}/results/H4-memsweep-run-20261018T2209Z/memsweep.csv"), raw(bdir, f"{R}/results/H6-large-run-20261018T2210Z/routes.csv")]
            text_files = [f"{R}/results/H1-hsweep-run-20261018T2240Z/hsweep_summary.txt", f"{R}/results/H2-glue-run-20261018T2240Z/glue_summary.txt",
                          f"{R}/results/H4-memsweep-run-20261018T2209Z/memsweep_summary.txt", f"{R}/results/H6-large-run-20261018T2210Z/summary.txt",
                          f"{R}/results/H6-large-run-20261018T2210Z/routes_summary.csv", f"{R}/results/H6-large-run-20261018T2210Z/ranking.csv",
                          f"{VALD}/gae-band/H2-glue-run-20261018T2240Z.txt", f"{VALD}/gae-band/H6-large-run-20261018T2210Z.txt"]
            gates = {"H2 Jglue, 40 bits, h = 1 (40 IC rows, 40 rho rows)": gate(V["gae_h2"], "gate/40/1/Jglue")}
        hl = heads["headlines"][study]
        texts = [tc[f] for f in text_files]
        all_text = all(t["identical"] for t in texts)
        all_head = not [h for h in hl if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
        notes = []
        notes.append(f"Every line of the committed summary texts and gae-band outputs was re-derived from the CSVs by the validator's own code and compared byte for byte: "
                     + "; ".join(f"{t['file'].split('/')[-2] + '/' + t['file'].split('/')[-1]}: {t['lines_committed']} lines, identical {t['identical']}" for t in texts)
                     + ". No line of the CSVs contradicts a summary.")
        runs_of = (["itemOJ-run-20261018T2132Z", "itemF-control-run-20261018T2128Z"] if study == "r3" else
                   ["H1-hsweep-run-20261018T2240Z", "H2-glue-run-20261018T2240Z", "H4-memsweep-run-20261018T2209Z", "H6-large-run-20261018T2210Z"])
        ncurv, ninst = curve_census(runs_of)
        notes.append(sym_note + f" Coverage of this check: {ncurv} distinct curves in this study's replay dumps, against {ninst} distinct recorded instances (distinct (p, ell) in the CSVs).")
        if study == "r3":
            k6 = V["oj"]["K6/40"]
            notes.append(f"Reference strength (K6): rho steps/p at 40 bits {k6['mean']:.4f} +- {k6['se']:.4f} against 0.886*sqrt(ell)/p = {k6['theory']:.4f}; the reference is not weak at the gate row.")
            notes.append("Gate three points (I = 3M, measured Jacobian op, native 0 / 0.1 / 1 GAE; 1 GAE = the row's eadd_M + 3*eadd_I = 15 M in every row): rho is cheaper at all three points for J at c = 1, 2 and 4 "
                         "(IC/rho 2.76-2.98x). Distance to the matched rho at measured cost: about 2.8-3.0x at 40 bits. The full Erratum V band (12 cells incl. the IC-favourable 'one Jacobian op = one E(F_{p^2}) addition' price and I = 2 log2 p M) "
                         "is unresolved for J (min 0.932-0.945, IC cheaper in 1 of 12 cells) and rho-cheaper for D (6 cells).")
            notes.append("The GAE re-pricing (Erratum V) was written after every run; it widens the band (upper ends rise, lower ends unchanged for J) and changes no verdict. At the pinned commit the GAE definition is in n2_routes.rs gae_band and validation/gae-band/*.txt; "
                         f"the PROTOCOL/REPORT erratum text was committed later in {LATER}. Its aggregate claims (J 20-40 bits 0.51-2.98, J 40 bits 0.93-2.98, D 40 bits 11.2-70.1) also match the recomputation.")
            notes.append("control.csv: Jind (independent trials) vs Jwalk steps/p by cofactor reproduce exactly (h=4: 0.3688 vs 0.3443; h=8: 0.2718 vs 0.2917); the Amendment F control is present and consistent with the report.")
            notes.append("The best c per size is a post-hoc minimum (flagged by the protocol); all three c are reported here, none changes a verdict.")
        else:
            a = V["h1"]["alpha/J"]
            ar = V["h1"]["alpha/R"]
            notes.append(f"H1: alpha_J = {a['alpha']:.4f} with the bootstrap replicated exactly (xorshift seed 0xa1fa, 2000 resamples): [{a['lo']:.4f}, {a['hi']:.4f}], entirely below 1/2 as frozen in the decision rule. "
                         f"Rho's own fit is alpha_R = {ar['alpha']:.4f} [{ar['lo']:.4f}, {ar['hi']:.4f}], which excludes the theoretical 1/2: in H1 the rho arm runs on average ~8% above 0.886/sqrt(h) at large h "
                         "(cell ratios up to 1.25; z ~ 4 in two cells, s.e. from 4 instances). That makes the reference slightly weaker than ideal at large h (it favours the IC) and does not change the H1 verdict.")
            notes.append("H2 gate three points (I = 3M, measured Jacobian op 41.3M + 1.00I, Legendre split test at one I per Jglue step (H3), native 0 / 0.1 / 1 GAE): rho is cheaper at all three points at 40 bits, h = 1 "
                         "(IC/rho 1.78 / 1.80 / 1.88). Distance to the matched rho at measured cost: about 1.8x. The full Erratum V band (12 cells, incl. the IC-favourable price) is unresolved at h = 1 (0.759-1.876) and rho-cheaper at h = 4 for 36/40 bits.")
            notes.append("Report-text inconsistency (IC-favourable, small): REPORT_ROUND4.md (pinned tree) gives the h = 1 distance to rho as 'about 1.7x at 40 bits under measured costs', which is glue_summary.txt's "
                         "'Jglue/rho cost ratio (measured, I=3M, w=0) 1.677'. That line prices walk_M + 3*walk_I only and omits the H3 Legendre split test (one I per Jglue step) that Addendum H3 and the band use; "
                         "with it the same point is 1.7775 (the gate point at w = 0, and the 1.78x in the later erratum text). All glue_summary lines reproduce exactly; only the report's choice of line understates the gap by ~6%.")
            notes.append(f"H2 rho reference at h = 1: steps/p 0.956 / 0.844 / 0.778 / 0.860 at 28 / 32 / 36 / 40 bits vs 0.886 theory (all within ~1.1 s.e.); not weak at the gate row (40 bits).")
            notes.append("H4: the pre-registered prediction (trials proportional to 1/theta) is contradicted by the CSV (trials/p 0.56 / 0.62 / 0.70 at theta 1/2, 1/4, 1/8, 32 bits), and REPORT_ROUND4 says so ('The prediction (time proportional to 1/theta) is falsified').")
            notes.append("H6: K6 at 44/48 bits 0.3284 / 0.4478 vs theory 0.3132 / 0.4430 (within 1 s.e.), as the report states.")
            notes.append(f"The GAE re-pricing (Erratum V) was written after every run and changes no verdict; its round-4 aggregate claims (committed later in {LATER}: H2 h=1 0.59-2.04, h=4 36-40 bits 1.14-2.88, H6 J 0.99-3.04 / 0.77-2.38, gate 1.78 / 1.80 / 1.88) match the recomputation.")
        notes.append("rel_tol for printed decimals = half a unit in the last printed digit / |claimed|; 0 for integer counts. 'claimed_at' cites the file and line in the pinned tree.")
        rep = dict(
            schema="validator-report (validation-gate/v1 fields + validator detail)",
            status="pass" if (all_text and all_head) else "fail",
            independent=True,
            validator=common_cert["validator"],
            date="2026-10-24",
            commit=PINNED,
            commit_notes=common_cert["commit_notes"],
            raw_data=raws,
            recompute_tools=tools,
            recompute_outputs=[art["recompute.out.json"], art["headlines.out.json"]],
            text_reproduction=texts,
            headline=hl,
            gate_three_point_band=dict(
                definition="mean over rows of (row cost / row GAE), 1 GAE = eadd_M + 3*eadd_I, I = 3M, measured Jacobian pricing, native weight w GAE per native op, IC and rho priced by the same function (H2 Jglue adds the Legendre test at one I per step)",
                values=gates),
            notes=notes,
            validation_report=dict(
                id=f"VAL-20261024-{'wd3' if study == 'r3' else 'wd4'}",
                task_id="(subagent task from the producing session; no TASK card)",
                run_ids=[e for e in (["itemOJ-run-20261018T2132Z", "itemF-control-run-20261018T2128Z"] if study == "r3" else ["H1-hsweep-run-20261018T2240Z", "H2-glue-run-20261018T2240Z", "H4-memsweep-run-20261018T2209Z", "H6-large-run-20261018T2210Z"])],
                artifact_checks=["all source-run artifacts tracked and clean at the pinned commit", "pod-run tool sha256 re-derived from git and equal to the pinned hash in rusage.txt", "replay-certificate.json in this directory: exact row replay and scalar verification"],
                metric_recomputations=f"{len(hl)} headline numbers within rel_tol; {sum(t['lines_committed'] for t in texts)} committed summary/band lines reproduced byte for byte",
                control_checks=("K6 rho positive control at every size; Amendment F control (Jind vs Jwalk) reproduced" if study == "r3" else "K6 at 44/48 bits; H2 gluecheck G1-G5 output (glue_checks.txt) replayed byte-identical; rho in H1 slightly above theory at large h (noted)"),
                heuristic_validation_checks=("P8/P8b/P8c and the Amendment C ranking rule were frozen before the recorded run (Amendments C, E, F, G); the GAE re-pricing is post-hoc and only widens the band" if study == "r3" else "H1 decision rule and H2 break-even frozen before the runs (Addendum H, H-a, H-b); H4 prediction falsified and reported as such; GAE re-pricing post-hoc, band-widening only"),
                cost_model_checks="unit declared (F_p multiplications M, inversions I priced at 3M or 2 log2 p M, native ops at 0/0.1/1 GAE); IC-favourable price flagged; memory not part of these cost rows (H4 reports stored rows separately); total cost per solve is the mean over verified solves (success probability 1 for every recorded row)",
                proof_architecture_checks="not applicable",
                verdict="passed" if (all_text and all_head) else "failed",
                limitations=[
                    "Recomputation follows the producer's definitions (PROTOCOL Amendment C / Addendum H / Erratum V and the n2_routes summarize functions, which the validator read); it is an independent implementation of the same definitions, not a blind re-derivation.",
                    "Operation counts are reproducible (exact replay) but were not re-derived by a separate implementation of the arms.",
                    "A passed report makes the receipts admissible evidence; it does not support an ECDLP claim, demonstrate a speedup over rho, or authorize promotion. At the gate's three points rho is cheaper in both studies.",
                ],
                artifact_paths=[f"{bdir}/replay-certificate.json", f"{bdir}/validator-report.json", f"{VALD}/validation_report.yaml"]),
            review_attestation=dict(
                joints_owned=["replay", "scalar verification", "headline recomputation", "rho-reference symmetry/unit check"],
                sources_read=["docs/VALIDATION_GATE.md", "scripts/validation_gate.py (check_* functions)", "scripts/fixtures/validation_gate/positive/*",
                              f"{R}/PROTOCOL.md (Parts O/J, Amendments C, E, F, G, Addendum H, H-a, H-b, Erratum V)", f"{R}/REPORT.md and REPORT_ROUND4.md (claimed-value sections)",
                              "examples/n2_routes.rs (main, dump_run, run_job, control_job, hsweep_job, glue_job, row_cost, summarize*, gae_band, arm_rho, counters, xorshift/splitmix)",
                              "the six source run directories (env.txt, CSVs, summaries, rusage.txt)", f"{VALD}/gae-band/*.txt",
                              "crypto-autoresearcher docs/agent-runtime-core.md and agents/validator.md"],
                read_sibling_reports=False,
                verdict="holds" if (all_text and all_head) else "breaks"),
        )
        with open(f"{repo}/{bdir}/validator-report.json", "w") as f:
            json.dump(rep, f, indent=1)
            f.write("\n")
        print(bdir, "validator report", rep["status"], len(hl), "headlines")
        yaml_reports.append(dict(validation_report=dict(rep["validation_report"], evidence_directory=bdir,
                                                        replay_certificate=f"{bdir}/replay-certificate.json",
                                                        validator_report=f"{bdir}/validator-report.json",
                                                        commit=PINNED, nonce=nonce),
                                 review_attestation=rep["review_attestation"]))
    with open(f"{repo}/{VALD}/validation_report.yaml", "w") as f:
        f.write("# Validator output (agents/validator.md 'Required output'), one document per evidence directory.\n")
        f.write("# Generated by research/weil_descent_angles_20261018/validation/assemble.py from the validator's own outputs.\n")
        for d in yaml_reports:
            f.write("---" + to_yaml(d, 0) + "\n")


if __name__ == "__main__":
    main()
