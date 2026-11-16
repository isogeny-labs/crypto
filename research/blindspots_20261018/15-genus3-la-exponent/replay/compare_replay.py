#!/usr/bin/env python3
"""Independent comparison of the item-15 validation-gate replay (cards r01..r07).

Stdlib only.  Run from anywhere:

    python3 research/blindspots_20261018/15-genus3-la-exponent/replay/compare_replay.py
    python3 .../compare_replay.py --certificate benchmarks/<dir>/replay-certificate.json

What it checks (rule: replay/PLAN.md, "Exact versus descriptive fields"):

 1. Per card: the receipt says `completed`, every command exited 0 and did not time
    out, the receipt's argv sequence equals the card's argv sequence, the receipt's
    card_sha256 equals the sha256 of the card file now, every receipt output digest
    (and every stdout/stderr digest for a card-named log) equals the file now, every
    `binds` file hashes to its pinned sha256, the instrument is
    73cfa489... , the build command compiles that instrument, and the binary
    directory is gone.
 2. Per gate command: the gate file prints one `GATE MATCH` per cell for exactly the
    arm named in the gate argv, ends with `GATE OK cells=N` (N = number of new/ref
    pairs), and contains no `GATE DIFF`, `GATE MISSING` or `GATE FAILED`.  Every arm
    of every replayed record is covered by some gate command naming that replay file
    and the matching recorded raw file.
 3. selftest.txt (r01) is byte-identical to raw/selftest.txt.
 4. INDEPENDENT of the gate: every replay jsonl record is compared with the recorded
    raw/ record, key sets and values, on every field except the descriptive ones
    (build_wall, vmhwm_kb, la_wall, wall_stream).  The result separates the fields
    the gate checks (GATED) from the other exact fields.

Exit status 0 only if everything passes.  Nothing is written unless --certificate.
"""
import argparse
import hashlib
import json
import os
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]  # repository root (replay -> item -> blindspots_20261018 -> research -> root)
ITEM = "research/blindspots_20261018/15-genus3-la-exponent"
INSTRUMENT = "research/blindspots_20261018/instruments/blindspots_15_genus3_la.rs"
INSTRUMENT_SHA = "73cfa4891f8eb4651b728e426aa4fd47f2180120b06c60081dab1b230ad5462b"
SELFTEST_SHA = "2d64c0a0fbfc41675c2b67d6c1c7966f74fe1b8e4be821607c05ac1d5a991895"
CARDS = ["r01", "r02", "r03", "r04", "r05", "r06", "r07"]

# PLAN.md: descriptive only, not compared.
DESCRIPTIVE_RECORD = {"build_wall", "vmhwm_kb"}
DESCRIPTIVE_ARM = {"la_wall", "wall_stream"}
# PLAN.md: exact, checked by the instrument's gate (per arm).
GATED = {"k", "k_true", "correct", "cost", "cost_cl9", "rel_cost", "la_cost", "rel_counters",
         "la_counters", "sge_counters", "total_counters", "steps", "rows_collected", "checks",
         "la_cols", "la_nnz", "matvecs"}
# PLAN.md: exact, expected equal, not gated.
EXACT_RECORD = {"bits", "q", "curve", "seed", "mode", "build_tries", "N", "ell", "c", "h", "a", "b",
                "r0", "Kmod", "checks", "relprim", "walks", "results"}
EXACT_ARM = {"arm", "stream", "la", "kappa", "cx", "bw_m", "bw_n", "bl_w", "mw", "seed_variant", "os",
             "walks", "nfb", "transfer_counters", "sge_cost", "la_cost_cl9", "la_in_rows",
             "la_in_cols", "la_in_nnz", "la_rows", "merges", "singletons", "trimmed", "dependent",
             "gen_products", "auto", "auto_cx", "auto_mw", "lp_vertices", "lp2", "cycles", "relprim",
             "diag"}


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def rel(p):
    return str(Path(p).resolve().relative_to(ROOT))


def parse_card(path):
    """Minimal parser for the card layout (stdlib only, no YAML)."""
    text = Path(path).read_text()
    card = {"binds": [], "commands": [], "must_not_exist": []}
    m = re.search(r"^card_id:\s*(\S+)", text, re.M)
    card["card_id"] = m.group(1)
    m = re.search(r"^receipt:\s*(\S+)", text, re.M)
    card["receipt"] = m.group(1)
    for m in re.finditer(r"^\s+- \{path: (\S+), sha256: ([0-9a-f]{64})\}", text, re.M):
        card["binds"].append((m.group(1), m.group(2)))
    mne = re.search(r"^must_not_exist:\n((?:\s+- .*\n)+)", text, re.M)
    if mne:
        card["must_not_exist"] = re.findall(r"- (\S+)", mne.group(1))
    # commands block
    cstart = text.index("\ncommands:\n")
    cend = text.index("\n", text.index("\nbus_address:")) if "\nbus_address:" in text else len(text)
    block = text[cstart:cend]
    parts = re.split(r"\n  - id: ", block)[1:]
    for part in parts:
        cid = part.split("\n", 1)[0].strip()
        am = re.search(r"^\s+argv: (\[.*\])\s*$", part, re.M)
        argv = json.loads(am.group(1))
        c = {"id": cid, "argv": argv}
        for key in ("stdout", "stderr"):
            km = re.search(r"^\s+%s: (\S+)\s*$" % key, part, re.M)
            c[key] = km.group(1) if km else None
        om = re.search(r"^\s+outputs: (\[.*\])\s*$", part, re.M)
        c["outputs"] = json.loads(om.group(1)) if om else []
        card["commands"].append(c)
    return card


def deep_diff(a, b, path=""):
    """Return list of (path, new, ref) leaf differences."""
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a:
                out.append((f"{path}.{k}", "<absent>", b[k]))
            elif k not in b:
                out.append((f"{path}.{k}", a[k], "<absent>"))
            else:
                out += deep_diff(a[k], b[k], f"{path}.{k}")
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            out.append((f"{path}.len", len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)):
            out += deep_diff(x, y, f"{path}[{i}]")
    else:
        if type(a) is not type(b) or a != b:
            out.append((path, a, b))
    return out


def count_leaves(x):
    if isinstance(x, dict):
        return sum(count_leaves(v) for v in x.values())
    if isinstance(x, list):
        return sum(count_leaves(v) for v in x)
    return 1


def compare_jsonl(new_path, ref_path, stats, mismatches, ctx):
    new = [json.loads(l) for l in Path(new_path).read_text().splitlines() if l.strip()]
    ref = [json.loads(l) for l in Path(ref_path).read_text().splitlines() if l.strip()]
    if len(new) != len(ref):
        mismatches.append(f"{ctx}: record count replay={len(new)} recorded={len(ref)}")
    arms_seen = []
    for ri, (n, r) in enumerate(zip(new, ref)):
        stats["records"] += 1
        if set(n) != set(r):
            mismatches.append(f"{ctx} rec{ri}: record key sets differ: replay-only={sorted(set(n)-set(r))} recorded-only={sorted(set(r)-set(n))}")
        unknown = (set(n) | set(r)) - EXACT_RECORD - DESCRIPTIVE_RECORD
        if unknown:
            mismatches.append(f"{ctx} rec{ri}: record keys in neither PLAN.md list: {sorted(unknown)}")
        for k in sorted((set(n) & set(r)) - DESCRIPTIVE_RECORD - {"results"}):
            stats["record_fields"] += 1
            stats["leaves"] += count_leaves(r[k])
            for p, a, b in deep_diff(n[k], r[k], k):
                mismatches.append(f"{ctx} rec{ri} field {p}: replay={a!r} recorded={b!r}")
        na, ra = n.get("results", []), r.get("results", [])
        if [a["arm"] for a in na] != [a["arm"] for a in ra]:
            mismatches.append(f"{ctx} rec{ri}: arm list replay={[a['arm'] for a in na]} recorded={[a['arm'] for a in ra]}")
        rmap = {a["arm"]: a for a in ra}
        for a in na:
            arm = a["arm"]
            arms_seen.append(arm)
            if arm not in rmap:
                continue
            b = rmap[arm]
            stats["arms"] += 1
            if set(a) != set(b):
                mismatches.append(f"{ctx} rec{ri} arm {arm}: arm key sets differ: replay-only={sorted(set(a)-set(b))} recorded-only={sorted(set(b)-set(a))}")
            unknown = (set(a) | set(b)) - EXACT_ARM - GATED - DESCRIPTIVE_ARM
            if unknown:
                mismatches.append(f"{ctx} rec{ri} arm {arm}: arm keys in neither PLAN.md list: {sorted(unknown)}")
            for k in sorted((set(a) & set(b)) - DESCRIPTIVE_ARM):
                gated = k in GATED
                stats["arm_fields_gated" if gated else "arm_fields_exact_not_gated"] += 1
                stats["leaves"] += count_leaves(b[k])
                for p, x, y in deep_diff(a[k], b[k], k):
                    tag = "GATED" if gated else "exact-not-gated"
                    mismatches.append(f"{ctx} rec{ri} arm {arm} field {p} [{tag}]: replay={x!r} recorded={y!r}")
            for k in DESCRIPTIVE_ARM:
                if k not in a or k not in b:
                    mismatches.append(f"{ctx} rec{ri} arm {arm}: descriptive field {k} absent")
    return arms_seen, new


def check_card(cid, results):
    cardfile = HERE / f"card-{cid}.yaml"
    recfile = HERE / f"receipt-card-{cid}.json"
    card = parse_card(cardfile)
    rec = json.loads(recfile.read_text())
    mm = []  # mismatches / failures for this card
    info = {"card": cid, "card_id": card["card_id"], "card_path": rel(cardfile), "card_sha256": sha(cardfile),
            "receipt_path": rel(recfile), "receipt_sha256": sha(recfile), "commit": rec["commit"],
            "receipt_status": rec["status"]}
    if rec["status"] != "completed":
        mm.append(f"{cid}: receipt status {rec['status']!r}")
    if rec["card_id"] != card["card_id"]:
        mm.append(f"{cid}: receipt card_id {rec['card_id']} != card {card['card_id']}")
    if rec["card_sha256"] != info["card_sha256"]:
        mm.append(f"{cid}: receipt card_sha256 {rec['card_sha256']} != sha256 of card file {info['card_sha256']}")
    if rec["card_path"] != info["card_path"]:
        mm.append(f"{cid}: receipt card_path {rec['card_path']} != {info['card_path']}")
    if rel(ROOT / card["receipt"]) != info["receipt_path"]:
        mm.append(f"{cid}: card receipt path {card['receipt']} != {info['receipt_path']}")
    rc = rec["commands"]
    if rec["commands_total"] != len(rc) or len(rc) != len(card["commands"]):
        mm.append(f"{cid}: commands_total={rec['commands_total']} receipt={len(rc)} card={len(card['commands'])}")
    for i, (c, r) in enumerate(zip(card["commands"], rc)):
        if r["id"] != c["id"] or r["argv"] != c["argv"]:
            mm.append(f"{cid} cmd{i+1}: receipt id/argv differs from card ({r['id']} vs {c['id']})")
        if r["exit_code"] != 0 or r.get("timed_out") or not r.get("succeeded"):
            mm.append(f"{cid} {r['id']}: exit_code={r['exit_code']} timed_out={r.get('timed_out')} succeeded={r.get('succeeded')}")
        for path, digest in r.get("outputs", {}).items():
            p = ROOT / path
            if not p.is_file():
                mm.append(f"{cid} {r['id']}: output {path} missing")
            elif sha(p) != digest:
                mm.append(f"{cid} {r['id']}: output {path} sha256 {sha(p)} != receipt {digest}")
        for key in ("stdout", "stderr"):
            if c[key]:
                p = ROOT / c[key]
                if not p.is_file():
                    mm.append(f"{cid} {r['id']}: {key} log {c[key]} missing")
                elif sha(p) != r.get(f"{key}_sha256"):
                    mm.append(f"{cid} {r['id']}: {key} log {c[key]} sha256 {sha(p)} != receipt {r.get(key + '_sha256')}")
        if set(r.get("outputs", {})) != {rel(ROOT / o) for o in c["outputs"]}:
            mm.append(f"{cid} {r['id']}: receipt output paths differ from card outputs")
    # binds
    binds = {}
    for path, digest in card["binds"]:
        binds[path] = digest
        if sha(ROOT / path) != digest:
            mm.append(f"{cid}: bind {path} sha256 {sha(ROOT / path)} != pinned {digest}")
    if binds.get(INSTRUMENT) != INSTRUMENT_SHA or sha(ROOT / INSTRUMENT) != INSTRUMENT_SHA:
        mm.append(f"{cid}: instrument {INSTRUMENT} not pinned/hashing to {INSTRUMENT_SHA}")
    builds = [c for c in card["commands"] if c["id"] == "build"]
    if len(builds) != 1 or builds[0]["argv"][-1] != INSTRUMENT or "rustc" not in builds[0]["argv"]:
        mm.append(f"{cid}: build command does not compile {INSTRUMENT}")
    for p in card["must_not_exist"]:
        if re.search(r"/bin-r\d\d(/b15)?$", p) and (ROOT / p).exists():
            mm.append(f"{cid}: {p} still exists")
    info["instrument_sha256"] = sha(ROOT / INSTRUMENT)
    info["binds"] = [{"path": p, "sha256": d} for p, d in card["binds"]]
    info["commands_total"] = rec["commands_total"]
    info["exit_codes"] = {r["id"]: r["exit_code"] for r in rc}

    # gate files
    gates = [c for c in card["commands"] if "gate" in c["argv"] and c["argv"].index("gate") == c["argv"].index("gate")
             and c["id"].startswith("gate-")]
    covered = {}  # (new_path, ref_path) -> set(arms)
    n_gate_files = 0
    n_match_lines = 0
    for g in gates:
        gi = g["argv"].index("gate")
        arm = g["argv"][gi + 1]
        pairs = g["argv"][gi + 2:]
        if len(pairs) % 2:
            mm.append(f"{cid} {g['id']}: odd number of file arguments")
            continue
        pairs = [(pairs[j], pairs[j + 1]) for j in range(0, len(pairs), 2)]
        gp = ROOT / g["stdout"]
        lines = gp.read_text().splitlines()
        n_gate_files += 1
        if not lines or lines[-1] != f"GATE OK cells={len(pairs)}":
            mm.append(f"{cid} {g['id']}: last line {lines[-1] if lines else '<empty>'!r}, expected 'GATE OK cells={len(pairs)}'")
        body = lines[:-1]
        if len(body) != len(pairs):
            mm.append(f"{cid} {g['id']}: {len(body)} GATE MATCH lines for {len(pairs)} cells")
        for ln in body:
            n_match_lines += 1
            if not re.fullmatch(r"GATE MATCH bits=\d+ curve=\d+ arm=" + re.escape(arm), ln):
                mm.append(f"{cid} {g['id']}: unexpected gate line {ln!r} (arm {arm})")
        for ln in lines:
            if re.search(r"GATE (DIFF|MISSING|FAILED)", ln):
                mm.append(f"{cid} {g['id']}: {ln}")
        for new, ref in pairs:
            covered.setdefault((new, ref), set()).add(arm)
    info["gate_files"] = n_gate_files
    info["gate_match_lines"] = n_match_lines

    # independent comparison of every replayed jsonl
    runs = [c for c in card["commands"] if c["id"].startswith("run-")]
    files = []
    for c in runs:
        for o in c["outputs"]:
            newp = ROOT / o
            refrel = f"{ITEM}/raw/{newp.name}"
            refp = ROOT / refrel
            if refrel not in binds:
                mm.append(f"{cid}: reference {refrel} is not pinned in binds")
            stats = {"records": 0, "record_fields": 0, "arm_fields_gated": 0, "arm_fields_exact_not_gated": 0,
                     "arms": 0, "leaves": 0}
            fm = []
            arms, newrecs = compare_jsonl(newp, refp, stats, fm, f"{newp.name}")
            mm += fm
            # coverage: every arm of every record has a gate over this exact pair
            gated_arms = covered.get((o, refrel), set())
            if set(arms) != gated_arms:
                mm.append(f"{cid} {newp.name}: arms in record {sorted(arms)} != arms gated {sorted(gated_arms)}")
            # reference integrity against the original receipts
            orig = None
            for rp in sorted((HERE.parent / "raw").glob("receipt-card-0*.json")):
                try:
                    orec = json.loads(rp.read_text())
                except Exception:
                    continue
                for oc in orec.get("commands", []):
                    for opath, od in oc.get("outputs", {}).items():
                        if opath.endswith("/raw/" + newp.name) and orec.get("status") == "completed":
                            orig = (rp.name, od)
            ref_sha = sha(refp)
            if orig and orig[1] != ref_sha:
                mm.append(f"{cid}: recorded {refrel} sha256 {ref_sha} != digest {orig[1]} in {orig[0]}")
            files.append({"replay_file": o, "replay_sha256": sha(newp), "recorded_file": refrel,
                          "recorded_sha256": ref_sha,
                          "recorded_in_original_receipt": (orig[0] if orig and orig[1] == ref_sha else None),
                          "bits": newrecs[0]["bits"], "curve": newrecs[0]["curve"], "seed": newrecs[0]["seed"],
                          "arms": arms, "stats": stats, "mismatches": len(fm)})
    info["files"] = files

    # selftest
    if cid == "r01":
        new = (HERE / "selftest.txt").read_bytes()
        refb = (HERE.parent / "raw" / "selftest.txt").read_bytes()
        info["selftest"] = {"replay": f"{ITEM}/replay/selftest.txt", "recorded": f"{ITEM}/raw/selftest.txt",
                            "replay_sha256": hashlib.sha256(new).hexdigest(), "recorded_sha256": hashlib.sha256(refb).hexdigest(),
                            "byte_identical": new == refb}
        if new != refb:
            mm.append("r01: selftest.txt differs from raw/selftest.txt")
        if hashlib.sha256(new).hexdigest() != SELFTEST_SHA:
            mm.append(f"r01: selftest.txt sha256 {hashlib.sha256(new).hexdigest()} != {SELFTEST_SHA}")
    info["mismatches"] = mm
    return info


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--certificate", help="write the replay certificate JSON here (only if all checks pass)")
    args = ap.parse_args()
    infos = [check_card(c, None) for c in CARDS]
    allmm = [m for i in infos for m in i["mismatches"]]
    tot = {"cards": len(infos), "cells": 0, "records": 0, "arms": 0, "record_fields": 0, "arm_fields_gated": 0,
           "arm_fields_exact_not_gated": 0, "leaves": 0, "gate_files": 0, "gate_match_lines": 0}
    for i in infos:
        tot["gate_files"] += i["gate_files"]
        tot["gate_match_lines"] += i["gate_match_lines"]
        for f in i["files"]:
            tot["cells"] += 1
            for k in ("records", "arms", "record_fields", "arm_fields_gated", "arm_fields_exact_not_gated", "leaves"):
                tot[k] += f["stats"][k]
    tot["files"] = sum(len(i["files"]) for i in infos)
    for i in infos:
        print(f"{i['card']} {i['card_id']} status={i['receipt_status']} commit={i['commit'][:12]} "
              f"commands={i['commands_total']} gate_files={i['gate_files']} files={len(i['files'])} mismatches={len(i['mismatches'])}")
        for f in i["files"]:
            s = f["stats"]
            print(f"  {Path(f['replay_file']).name}: records={s['records']} arms={s['arms']} record_fields={s['record_fields']} "
                  f"arm_gated={s['arm_fields_gated']} arm_exact_not_gated={s['arm_fields_exact_not_gated']} mismatches={f['mismatches']}")
    print("TOTAL", json.dumps(tot, sort_keys=True))
    for m in allmm:
        print("MISMATCH:", m)
    status = "verified" if not allmm else "failed"
    print("COMPARE", "OK" if not allmm else f"FAILED ({len(allmm)} mismatches)")
    if args.certificate and not allmm:
        write_certificate(args.certificate, infos, tot)
        print("certificate written:", args.certificate)
    return 0 if not allmm else 1


def write_certificate(path, infos, tot):
    me = Path(__file__).resolve()
    runs = []
    for i in infos:
        runs.append({
            "card_id": i["card_id"], "card_path": i["card_path"], "card_sha256": i["card_sha256"],
            "receipt_path": i["receipt_path"], "receipt_sha256": i["receipt_sha256"],
            "receipt_status": i["receipt_status"], "commit": i["commit"],
            "commands": i["commands_total"], "exit_codes_all_zero": all(v == 0 for v in i["exit_codes"].values()),
            "instrument_path": INSTRUMENT, "instrument_sha256": i["instrument_sha256"],
            "gate_files": i["gate_files"], "gate_all_ok": True,
            "selftest": i.get("selftest"),
            "recorded_files_reproduced": [
                {"recorded_file": f["recorded_file"], "recorded_sha256": f["recorded_sha256"],
                 "replay_file": f["replay_file"], "replay_sha256": f["replay_sha256"],
                 "bits": f["bits"], "curve": f["curve"], "seed": f["seed"], "arms": f["arms"],
                 "records_compared": f["stats"]["records"], "arms_compared": f["stats"]["arms"],
                 "record_fields_compared": f["stats"]["record_fields"],
                 "arm_fields_compared_gated": f["stats"]["arm_fields_gated"],
                 "arm_fields_compared_exact_not_gated": f["stats"]["arm_fields_exact_not_gated"],
                 "mismatches": f["mismatches"]} for f in i["files"]],
            "mismatches": i["mismatches"],
        })
    cert = {
        "schema": "replay-certificate/v1",
        "item": "blindspots item 15, genus-3 large-prime / linear-algebra exponent",
        "status": "verified",
        "independent": True,
        "independence_basis": "asserted by the dispatch of this assessment: fresh builds (replay/bin-rNN, removed after use) and fresh runs from seven frozen cards (card-r01..r07, distinct receipts) by agents other than the original executor, same host class, same instrument source (sha256 pinned in every card and receipt-bound). The receipts carry no executor identity, so this script cannot verify who ran them.",
        "scalars_verified": True,
        "scalars_scope": SCOPE,
        "comparison": {
            "rule": f"{ITEM}/replay/PLAN.md, 'Exact versus descriptive fields'",
            "exact_gated_fields": sorted(GATED),
            "exact_not_gated_record_fields": sorted(EXACT_RECORD - {"results"}),
            "exact_not_gated_arm_fields": sorted(EXACT_ARM),
            "descriptive_fields_ignored": sorted(DESCRIPTIVE_RECORD | DESCRIPTIVE_ARM),
            "totals": tot,
            "mismatches": [],
            "mismatch_count": 0,
        },
        "instrument": {"path": INSTRUMENT, "sha256": INSTRUMENT_SHA},
        "runs": runs,
        "compare_script": {"path": rel(me), "sha256": sha(me)},
    }
    Path(path).write_text(json.dumps(cert, indent=2) + "\n")


SCOPE = (
    "Sample, not the full original set. Replayed: selftest; b=16 and b=17 curves 0,1,2,3; b=18 curves 0,1,2,3; "
    "b=19 curves 0 and 3; b=20 curves 0 and 3 (r2 only, no aa was recorded at b=20). Every recorded arm of every "
    "replayed record is gated and independently compared: 30 replayed jsonl files (24 at b=16..18, 4 at b=19, 2 at "
    "b=20) against their recorded raw/ files. Dropped (replay/PLAN.md, 'Estimate, what was dropped, and why'): "
    "b=19 curves 1 and 2 (arms and size already covered by curves 0 and 3; CPU budget of about 45 CPU-min on a shared "
    "4-vCPU host); b=20 curves 1 and 2 (their IC arms are already reproduced by the matched-rho cards' continuity "
    "gate in the 15b binary, raw/matched/gate-c1-*.txt and gate-c2-*.txt, each GATE OK cells=1; that is the 15b "
    "binary, not the original instrument, so it is cited here as context and is not part of this certificate's runs). "
    "Nothing dropped at the arm level. Smallest (b=16) and largest (b=20) sizes are both covered. Descriptive "
    "fields (build_wall, la_wall, wall_stream, vmhwm_kb, stderr walls, command seconds, build logs) are not compared."
)

if __name__ == "__main__":
    sys.exit(main())
