#!/usr/bin/env python3
"""Regenerate the bands and weighted comparisons of the studies that name a
byte counter, under the band rule of option B (byte counters listed, not
priced; `band.rs` NON_OPERATION_COUNTERS).

Nothing under `sessions/` is written.  For each study this script
  * runs `ecbench band` on every session with the weights of the committed band,
    into <study>/analysis/exbytes/band-<session>.{json,txt};
  * re-runs every committed *weighted* comparison (`pricing_weight` > 0, the
    only ones whose price can change) with the same arms, weight and bootstrap
    seed, into <study>/analysis/exbytes/comparisons/<session>__<a>__<b>__w<w>.json;
  * writes <study>/analysis/exbytes/CHANGES.md: the old and new verdict and S
    ratio of every re-run comparison.
w = 0 comparisons are not re-run: the price at w = 0 has no operation count.

Usage: regen_exbytes.py ECBENCH_BINARY STUDY_DIR [STUDY_DIR ...]
"""
import json, os, re, subprocess, sys


def run(cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout


def band_part(c):
    """The ops ratio and the band clause of a comparison's verdict string."""
    v = c.get("verdict") or ""
    ops = re.search(r"ops \([^)]*\): (\S+ / \S+ = [\d.]+ \[[^\]]*\])", v)
    band = re.search(r"band \([^)]*\): ([^;]*)", v)
    return ((ops.group(1) if ops else "?") + "; " + (band.group(1).strip() if band else "no band")).replace("|", "/")


def main():
    ecbench, studies = sys.argv[1], sys.argv[2:]
    for study in studies:
        out = os.path.join(study, "analysis", "exbytes")
        os.makedirs(os.path.join(out, "comparisons"), exist_ok=True)
        rows = []
        for sess in sorted(os.listdir(os.path.join(study, "sessions"))):
            sdir = os.path.join(study, "sessions", sess)
            bj = os.path.join(study, "analysis", f"band-{sess}.json")
            if not (os.path.isdir(sdir) and os.path.isfile(bj)):
                continue
            weights = ",".join(repr(w) if w != int(w) else str(int(w))
                               for w in json.load(open(bj))["weights"])
            txt = run([ecbench, "band", "--dir", sdir, "--weights", weights,
                       "--json", os.path.join(out, f"band-{sess}.json")])
            open(os.path.join(out, f"band-{sess}.txt"), "w").write(txt)
            cdir = os.path.join(sdir, "comparisons")
            if not os.path.isdir(cdir):
                continue
            seen = set()
            for f in sorted(os.listdir(cdir)):
                old = json.load(open(os.path.join(cdir, f)))
                w = old.get("pricing_weight")
                if not w:
                    continue
                key = (old["a"]["arm"], old["b"]["arm"], w)
                if key in seen:
                    continue
                seen.add(key)
                a, b = old["a"]["arm"], old["b"]["arm"]
                cmd = [ecbench, "compare", "--dir", sdir, "--a", a, "--b", b,
                       "--weight", repr(w), "--seed", str(old["resample_seed"]),
                       "--json"]
                if old.get("allow_interrupted"):
                    cmd.append("--allow-interrupted")
                new = json.loads(run(cmd))
                json.dump(new, open(os.path.join(
                    out, "comparisons", sess + f"__{a}__{b}__w{w}.json"), "w"), indent=1)
                rows.append((sess, a, b, w, old, new))
        with open(os.path.join(out, "CHANGES.md"), "w") as fh:
            fh.write("| session | a | b | w | verdict as committed | verdict, byte counters not priced |\n"
                     "|:--|:--|:--|--:|:--|:--|\n")
            for sess, a, b, w, old, new in rows:
                fh.write(f"| {sess} | {a} | {b} | {w} | {band_part(old)} | {band_part(new)} |\n")
        print(study, len(rows), "weighted comparisons re-run")


if __name__ == "__main__":
    main()
