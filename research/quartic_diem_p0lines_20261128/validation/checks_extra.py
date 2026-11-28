#!/usr/bin/env python3
"""Validator checks for research/quartic_diem_p0lines_20261128 beyond replay, certificates
and the section-5 recompute: manifests, freeze order, binary-to-source binding through
cargo's build records, analyze.py / explore.py byte reproduction, and the bench record.

Usage:
  python3 -I checks_extra.py REPO SHA SNAP BIN REPLAY_DIR OUT.json

  REPO       repository root (git objects are read with `git -C REPO show SHA:...`; the
             working tree is read only for REPO/target/release (cargo records) and the
             mtimes of the files rustc read)
  SHA        the validated commit (c271f320e40)
  SNAP       `git archive SHA research/quartic_diem_p0lines_20261128
             research/quartic_diem_nofb_20261126/raw` extracted (analyze.py runs there)
  BIN        the binary copied at dispatch
  REPLAY_DIR the replay working directory (REPLAY_DIR/raw/run*.jsonl), or - to skip
Every record key is read with [] (lesson L-99ee952350b2).
"""
import datetime
import hashlib
import json
import os
import re
import subprocess
import sys

STUDY = "research/quartic_diem_p0lines_20261128"
FREEZE = "7f67f384eceb7080ff4b7898f6fa6036f8b57b8e"
UTC = datetime.timezone.utc


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True, text=True).stdout


def show(repo, sha, path):
    return subprocess.run(["git", "-C", repo, "show", f"{sha}:{path}"], check=True, capture_output=True).stdout


def h(b):
    return hashlib.sha256(b).hexdigest()


def hf(p):
    with open(p, "rb") as fh:
        return h(fh.read())


def iso(t):
    return datetime.datetime.fromtimestamp(t, UTC).isoformat()


def ts(s):
    return datetime.datetime.fromisoformat(s.strip().replace("Z", "+00:00")).timestamp()


def jl(b):
    return [json.loads(l) for l in b.decode().splitlines() if l.strip()]


def main():
    repo, sha, snap, binp, replay, outp = sys.argv[1:7]
    sha = git(repo, "rev-parse", sha).strip()
    st = f"{snap}/{STUDY}"
    out = {"schema": "validator-checks-extra/v1", "sha": sha, "freeze": FREEZE}

    # ---------- A. manifests ----------
    man = {}
    for mf in ("frozen.sha256", "data.sha256"):
        res = {}
        for line in show(repo, sha, f"{STUDY}/{mf}").decode().splitlines():
            hh, p = line.split(None, 1)
            res[p] = h(show(repo, sha, f"{STUDY}/{p}")) == hh
        man[mf] = {"entries": res, "ok": all(res.values())}
    bline = show(repo, sha, f"{STUDY}/binary.sha256").decode().splitlines()
    bh = bline[0].split()[0]
    man["binary"] = {"binary.sha256": bh, "binary.sha256_note": bline[1:], "bound_copy": hf(binp),
                     "scratch_path": hf("/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.p0"),
                     "target_release_genus_ladder": hf(f"{repo}/target/release/genus_ladder")}
    man["binary"]["ok"] = man["binary"]["bound_copy"] == bh == man["binary"]["scratch_path"]
    src = {}
    for line in show(repo, sha, f"{STUDY}/source.sha256").decode().splitlines():
        hh, p = line.split(None, 1)
        src[p] = {"freeze_7f67f384ece": h(show(repo, FREEZE, p)) == hh, "validated_sha": h(show(repo, sha, p)) == hh}
    man["source.sha256"] = {"entries": src, "ok": all(all(v.values()) for v in src.values())}
    out["manifests"] = man

    # ---------- B. freeze order ----------
    def cdate(c):
        return git(repo, "log", "-1", "--format=%cI", c).strip()
    study_commits = git(repo, "log", "--format=%H %cI %s", f"{FREEZE}^..{sha}", "--", STUDY).splitlines()
    stamps = {f: show(repo, sha, f"{STUDY}/raw/{f}").decode().strip()
              for f in ("main_start.txt", "main_end.txt", "r131101_start.txt", "r131101_end.txt")}
    done = {f: [r for r in jl(show(repo, sha, f"{STUDY}/raw/{f}")) if r["kind"] == "done"][0]["wall_s"]
            for f in ("run.jsonl", "run131101.jsonl")}
    first_add = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
        cs = git(repo, "log", "--format=%H", "--diff-filter=A", sha, "--", f"{STUDY}/{f}").split()
        first_add[f] = {"adding_commits": cs, "date": cdate(cs[-1]) if cs else None,
                        "freeze_is_ancestor": subprocess.run(["git", "-C", repo, "merge-base", "--is-ancestor", FREEZE, cs[-1]]).returncode == 0 if cs else None}
    frozen_files = [l.split(None, 1)[1] for l in show(repo, sha, f"{STUDY}/frozen.sha256").decode().splitlines()]
    frozen_unchanged = subprocess.run(["git", "-C", repo, "diff", "--quiet", FREEZE, sha, "--"]
                                      + [f"{STUDY}/{p}" for p in frozen_files + ["frozen.sha256"]]).returncode == 0
    in_freeze = {p: subprocess.run(["git", "-C", repo, "cat-file", "-e", f"{FREEZE}:{STUDY}/{p}"], stderr=subprocess.DEVNULL).returncode == 0
                 for p in frozen_files}
    raw_in_freeze = subprocess.run(["git", "-C", repo, "cat-file", "-e", f"{FREEZE}:{STUDY}/raw"], stderr=subprocess.DEVNULL).returncode == 0
    t_main = ts(stamps["main_end.txt"]) - ts(stamps["main_start.txt"])
    t_131 = ts(stamps["r131101_end.txt"]) - ts(stamps["r131101_start.txt"])
    out["freeze_order"] = {
        "study_commits": study_commits, "freeze_commit_date": cdate(FREEZE), "raw_stamps": stamps,
        "done_wall_s": done, "stamp_intervals_s": {"main": t_main, "131101": t_131},
        "raw_first_added": first_add, "raw_dir_absent_at_freeze": not raw_in_freeze,
        "frozen_files_present_at_freeze": in_freeze,
        "frozen_files_unchanged_freeze_to_sha": frozen_unchanged,
        "freeze_before_main_start_s": ts(stamps["main_start.txt"]) - ts(cdate(FREEZE)),
        "ok": (frozen_unchanged and all(in_freeze.values()) and not raw_in_freeze
               and all(v["freeze_is_ancestor"] and ts(v["date"]) > ts(cdate(FREEZE)) for v in first_add.values())
               and ts(cdate(FREEZE)) < ts(stamps["main_start.txt"])),
    }

    # ---------- C. binary-to-source binding through cargo's records ----------
    tr = f"{repo}/target/release"
    fp, deps = f"{tr}/.fingerprint", f"{tr}/deps"
    bb = {}
    bins = [f for f in os.listdir(deps) if f.startswith("genus_ladder-") and not f.endswith(".d")]
    match = [f for f in bins if hf(f"{deps}/{f}") == bh]
    bb["deps_matching_binary"] = match
    hb = match[0].split("-", 1)[1]
    inv_bin = os.stat(f"{fp}/crypto-{hb}/invoked.timestamp").st_mtime
    bin_fp = json.load(open(f"{fp}/crypto-{hb}/bin-genus_ladder.json"))
    lib_dep = [d for d in bin_fp["deps"] if d[1] == "crypto_lib"][0]
    want = int(lib_dep[3])
    libs = []
    for d in os.listdir(fp):
        f = f"{fp}/{d}/lib-crypto_lib"
        if d.startswith("crypto-") and os.path.exists(f):
            v = open(f).read().strip()
            # cargo writes the u64 fingerprint as little-endian hex
            libs.append((int.from_bytes(bytes.fromhex(v), "little"), d))
    lib = [d for v, d in libs if v == want]
    bb["bin_fingerprint_lists_lib_fingerprint"] = format(want, "016x")
    bb["lib_fingerprint_dirs_matching"] = lib
    lh = lib[0].split("-", 1)[1]
    inv_lib = os.stat(f"{fp}/{lib[0]}/invoked.timestamp").st_mtime
    done_lib = os.stat(f"{fp}/{lib[0]}/lib-crypto_lib").st_mtime
    dfile_lib = f"{deps}/crypto_lib-{lh}.d"
    dinfo_written = os.stat(dfile_lib).st_mtime

    def depfiles(p):
        txt = open(p).read().split("\n\n")[0]
        _, d = txt.split(":", 1)
        return [x.replace("\\ ", " ") for x in re.split(r"(?<!\\) ", d.replace("\\\n", " ").strip()) if x]

    def envdeps(p):
        return [l[len("# env-dep:"):] for l in open(p).read().splitlines() if l.startswith("# env-dep:")]

    reflog = git(repo, "reflog", "--date=unix", "--format=%H %gd %gs").splitlines()
    head_moves = []
    for l in reflog:
        m = re.match(r"(\w+) HEAD@\{(\d+)\} (.*)", l)
        if m and inv_lib - 3600 <= int(m.group(2)) <= inv_lib + 7200:
            head_moves.append({"utc": iso(int(m.group(2))), "to": m.group(1)[:11], "what": m.group(3)})
    changed_after = set(git(repo, "diff", "--name-only", FREEZE, sha).split())
    bench_c = json.loads(show(repo, sha, "benchmarks/records/20261128T0345Z-quartic-diem-p0lines.json"))
    rec_created = ts(bench_c["created_utc"])
    res = {}
    for name, dfile, inv in (("lib", dfile_lib, inv_lib), ("bin", f"{deps}/genus_ladder-{hb}.d", inv_bin)):
        fs = [f[len(repo) + 1:] if f.startswith(repo + "/") else f for f in depfiles(dfile)]
        outside = [f for f in fs if f.startswith("/")]
        rel = [os.path.normpath(f) for f in fs if not f.startswith("/")]
        cls = {"unmodified_since_before_invocation_and_equal_freeze": 0,
               "rewritten_later_by_merge_changed_in_git": 0,
               "rewritten_later_but_unchanged_in_git": [],
               "modified_in_build_window": [], "differs_from_freeze_and_old_mtime": [],
               "not_in_freeze": []}
        for f in rel:
            mt = os.stat(f"{repo}/{f}").st_mtime
            try:
                at_freeze = show(repo, FREEZE, f)
            except subprocess.CalledProcessError:
                cls["not_in_freeze"].append(f)
                continue
            with open(f"{repo}/{f}", "rb") as fh:
                now = fh.read()
            if mt < inv:
                if now == at_freeze:
                    cls["unmodified_since_before_invocation_and_equal_freeze"] += 1
                else:
                    cls["differs_from_freeze_and_old_mtime"].append(f)
            elif mt <= rec_created:
                cls["modified_in_build_window"].append({"file": f, "mtime": iso(mt)})
            elif f in changed_after:
                cls["rewritten_later_by_merge_changed_in_git"] += 1
            else:
                cls["rewritten_later_but_unchanged_in_git"].append({"file": f, "mtime": iso(mt)})
        tracked_fail = subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch", "--"] + rel,
                                      capture_output=True, text=True).returncode != 0
        res[name] = {"fingerprint": f"crypto-{lh if name == 'lib' else hb}", "invoked_utc": iso(inv),
                     "n_source_files": len(fs), "outside_repo": outside, "all_tracked": not tracked_fail,
                     "env_deps": envdeps(dfile), **cls}
    bb["dep_info"] = res
    bb["lib_compile"] = {"invoked_utc": iso(inv_lib), "dep_info_written_utc": iso(dinfo_written),
                         "finished_utc": iso(done_lib)}
    bb["bin_link"] = {"invoked_utc": iso(inv_bin), "finished_utc": iso(os.stat(f"{fp}/crypto-{hb}/bin-genus_ladder").st_mtime),
                      "deps_binary_mtime_utc": iso(os.stat(f"{deps}/{match[0]}").st_mtime)}
    bb["bench_record_at_sha"] = {"created_utc": bench_c["created_utc"], "commit": bench_c["commit"],
                                 "commit_dirty": bench_c["commit_dirty"],
                                 "dirty_rule": "tools/benchrec.py: git status --porcelain --untracked-files=no"}
    bb["head_moves_around_build"] = head_moves
    bb["cargo_lock_toml_equal_freeze_and_sha"] = subprocess.run(
        ["git", "-C", repo, "diff", "--quiet", FREEZE, sha, "--", "Cargo.lock", "Cargo.toml"]).returncode == 0
    bb["src_dirs_changed_freeze_to_sha"] = len([f for f in changed_after if f.startswith(("src/", "crates/"))])
    bb["study_sources_equal_freeze_and_sha"] = subprocess.run(
        ["git", "-C", repo, "diff", "--quiet", FREEZE, sha, "--", "src/cryptanalysis/quartic_ladder.rs",
         "src/bin/genus_ladder.rs"]).returncode == 0
    # HEAD stayed at the freeze commit from the freeze to after the bench record (reflog), the
    # record says clean at 03:45:36, rustc had read every source when it wrote dep-info, and no
    # source file carries an mtime inside the build window.
    hm_ok = all(not (inv_lib - 60 <= ts(x["utc"]) <= rec_created) for x in head_moves if x["to"] != FREEZE[:11])
    bb["head_at_freeze_through_build"] = hm_ok
    bb["ok"] = (bool(match) and len(lib) == 1 and bench_c["commit"] == FREEZE and bench_c["commit_dirty"] is False
                and hm_ok and inv_lib > ts(cdate(FREEZE)) and dinfo_written < rec_created
                and all(not v["outside_repo"] and v["all_tracked"] and not v["modified_in_build_window"]
                        and not v["differs_from_freeze_and_old_mtime"] and not v["not_in_freeze"]
                        and not v["rewritten_later_but_unchanged_in_git"] for v in res.values()))
    bb["strength"] = ("mtime and cargo-record evidence, weaker than a rebuild: rustc's dep-info for the linked "
                      "crypto_lib was written 10 s after invocation, so every source had been read by then; files "
                      "later rewritten by the 03:45:46 and 04:00:32 fast-forwards cannot be re-read as they were "
                      "at build time, and for those the binding rests on HEAD = freeze in the reflog and the bench "
                      "record's commit_dirty = false at 03:45:36")
    out["binary_binding"] = bb

    # ---------- D. byte reproduction of the frozen analysis and the exploratory table ----------
    def run_in(cwd, *args):
        return subprocess.run([sys.executable, "-I", *args], cwd=cwd, check=True, capture_output=True).stdout
    an = run_in(st, "analyze.py", "raw/run.jsonl", "raw/run131101.jsonl")
    ex = run_in(st, "explore.py")
    repro = {"analysis_json_from_committed_raw": h(an) == h(show(repo, sha, f"{STUDY}/analysis.json")),
             "explore_json_from_committed_raw": h(ex) == h(show(repo, sha, f"{STUDY}/explore.json")),
             "python": sys.version.split()[0]}
    try:
        import numpy
        repro["numpy"] = numpy.__version__
    except ImportError:
        pass
    if replay != "-" and os.path.exists(f"{replay}/raw/run131101.jsonl"):
        # analyze.py on the replayed rows: it reads ../quartic_diem_nofb_20261126/raw relative to cwd
        tmp = f"{replay}/analyze_tree/{STUDY}"
        os.makedirs(f"{tmp}/raw", exist_ok=True)
        os.makedirs(f"{replay}/analyze_tree/research/quartic_diem_nofb_20261126/raw", exist_ok=True)
        for f in ("run.jsonl", "run131101.jsonl"):
            with open(f"{replay}/raw/{f}", "rb") as a, open(f"{tmp}/raw/{f}", "wb") as b:
                b.write(a.read())
            with open(f"{snap}/research/quartic_diem_nofb_20261126/raw/{f}", "rb") as a, \
                    open(f"{replay}/analyze_tree/research/quartic_diem_nofb_20261126/raw/{f}", "wb") as b:
                b.write(a.read())
        with open(f"{st}/analyze.py", "rb") as a, open(f"{tmp}/analyze.py", "wb") as b:
            b.write(a.read())
        with open(f"{st}/explore.py", "rb") as a, open(f"{tmp}/explore.py", "wb") as b:
            b.write(a.read())
        an2 = run_in(tmp, "analyze.py", "raw/run.jsonl", "raw/run131101.jsonl")
        ex2 = run_in(tmp, "explore.py")
        repro["analysis_json_from_replayed_raw"] = h(an2) == h(show(repo, sha, f"{STUDY}/analysis.json"))
        repro["explore_json_from_replayed_raw"] = h(ex2) == h(show(repo, sha, f"{STUDY}/explore.json"))
    out["reproduction"] = repro

    # ---------- E. data on disk and in Git ----------
    sizes = {f: len(show(repo, sha, f"{STUDY}/raw/{f}")) for f in ("run.jsonl", "run131101.jsonl")}
    out["data"] = {"raw_sizes_bytes": sizes,
                   "raw_tracked": subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch",
                                                  f"{STUDY}/raw/run.jsonl", f"{STUDY}/raw/run131101.jsonl"],
                                                 capture_output=True).returncode == 0,
                   "artifacts_json_present": subprocess.run(["git", "-C", repo, "cat-file", "-e", f"{sha}:{STUDY}/.artifacts.json"], stderr=subprocess.DEVNULL).returncode == 0,
                   "bench_record_at_sha": {k: bench_c[k] for k in ("status", "commit", "commit_dirty", "raw", "metrics", "validation", "commands")}}
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1)
        fh.write("\n")
    print(json.dumps({k: (v.get("ok") if isinstance(v, dict) and "ok" in v else None) for k, v in out.items()}))
    print(json.dumps(out["reproduction"]))


if __name__ == "__main__":
    main()
