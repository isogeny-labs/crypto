#!/usr/bin/env python3
"""Compare two node-dump files and find the first diverging inherited-F4 node.

PLUMBING ONLY (stdlib, no dependencies): it reads the two files that
`mac_dump.sh` writes and prints a diff. It computes no counted quantity; every
number it reads was counted by the Rust engine. AGENTS.md "Implementation
language" applies to the engine, not to this diff.

Usage:  compare.py A.jsonl.gz B.jsonl.gz [--json OUT.json] [--all]

File format (one JSON object per line, written by mac_dump.sh):
  {"type":"meta", ...}                          once
  {"type":"run","seq":S,"arm":..,"oracle":..,"target_index":T,"warmup":bool,
   "solver_ops":N,"nodes":K}                    one per measured child
  {"type":"node","seq":S,"i":I,"h":HASH,"n_vars":..,"polys":..,"terms":..,
   "w_total":W,"w_thread":W}                    one per solver reduction

`w_total` is the process-wide word-op counter at node entry. The words charged
by node i are therefore w[i+1]-w[i] (the last node: run.solver_ops - w[last],
the run's final solver_ops). The first node whose entry counter differs between hosts, with
all earlier entry counters and all node hashes equal, means node i-1's own
reduction charged a different number of words: that node is the culprit and
is reported with its system size.
"""
import argparse
import gzip
import json
import sys


def load(path):
    opener = gzip.open if path.endswith(".gz") else open
    meta, runs, nodes = None, {}, {}
    with opener(path, "rt") as f:
        for n, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                r = json.loads(line)
            except ValueError as e:
                sys.exit(f"{path}:{n}: bad JSON: {e}")
            t = r.get("type")
            if t == "meta":
                meta = r
            elif t == "run":
                runs[r["seq"]] = r
            elif t == "node":
                nodes.setdefault(r["seq"], []).append(r)
    return meta, runs, nodes


SHAPE = ("h", "n_vars", "polys", "terms")


def compare_seq(seq, ra, rb, na, nb):
    out = {"seq": seq, "arm": ra.get("oracle"), "target_index": ra.get("target_index"),
           "warmup": None,
           "solver_ops": [ra.get("solver_ops"), rb.get("solver_ops")],
           "nodes": [len(na), len(nb)]}
    out["solver_ops_equal"] = ra.get("solver_ops") == rb.get("solver_ops")
    first_shape = None
    for k in range(min(len(na), len(nb))):
        if any(na[k].get(f) != nb[k].get(f) for f in SHAPE):
            first_shape = k
            break
    if first_shape is None and len(na) != len(nb):
        first_shape = min(len(na), len(nb))
    out["first_shape_divergence"] = first_shape
    # the run's final solver_ops closes the last node's charge
    na = na + [{"w_total": ra.get("solver_ops")}]
    nb = nb + [{"w_total": rb.get("solver_ops")}]
    first_word = None
    for k in range(min(len(na), len(nb))):
        if na[k].get("w_total") != nb[k].get("w_total"):
            first_word = k
            break
    out["first_word_divergence"] = first_word
    if first_word is not None:
        c = first_word - 1  # the node whose reduction ran before this entry
        out["culprit_node"] = c if c >= 0 else None
        if c >= 0:
            wa = na[first_word]["w_total"] - na[c]["w_total"]
            wb = nb[first_word]["w_total"] - nb[c]["w_total"]
            out["culprit_words"] = [wa, wb]
            out["culprit_shape"] = {f: [na[c].get(f), nb[c].get(f)] for f in SHAPE + ("terms",)}
        else:
            out["culprit_words"] = [na[first_word]["w_total"], nb[first_word]["w_total"]]
            out["culprit_shape"] = "work before the first dumped node (setup, not a node)"
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("--json", help="write the summary here")
    ap.add_argument("--all", action="store_true", help="print every run, not only the first diverging")
    args = ap.parse_args()
    ma, runs_a, nodes_a = load(args.a)
    mb, runs_b, nodes_b = load(args.b)
    for m, p in ((ma, args.a), (mb, args.b)):
        if not m:
            sys.exit(f"{p}: no meta line")
    print(f"A: {args.a}\n   {ma.get('arch')}-{ma.get('os')} commit {str(ma.get('commit'))[:12]} rustc {ma.get('rustc')}")
    print(f"B: {args.b}\n   {mb.get('arch')}-{mb.get('os')} commit {str(mb.get('commit'))[:12]} rustc {mb.get('rustc')}")
    warn = []
    for key in ("commit", "patch_sha256", "spec_sha256", "subset"):
        if ma.get(key) != mb.get(key):
            warn.append(f"{key} differs: {ma.get(key)!r} vs {mb.get(key)!r}")
    if warn:
        print("WARNING: dumps are not like for like: " + "; ".join(warn))
    common = sorted(set(runs_a) & set(runs_b))
    only_a, only_b = sorted(set(runs_a) - set(runs_b)), sorted(set(runs_b) - set(runs_a))
    if only_a or only_b:
        print(f"seqs only in A: {only_a}; only in B: {only_b}")
    results = [compare_seq(s, runs_a[s], runs_b[s], nodes_a.get(s, []), nodes_b.get(s, [])) for s in common]
    n_ops_eq = sum(r["solver_ops_equal"] for r in results)
    n_tree_eq = sum(r["first_shape_divergence"] is None for r in results)
    n_word_eq = sum(r["first_word_divergence"] is None for r in results)
    print(f"\nruns compared: {len(results)}; solver_ops equal: {n_ops_eq}; "
          f"node sequence (hash/shape) identical: {n_tree_eq}; node entry word counters identical: {n_word_eq}")
    first = next((r for r in results if r["first_shape_divergence"] is not None
                  or r["first_word_divergence"] is not None or not r["solver_ops_equal"]), None)
    for r in results:
        bad = (r["first_shape_divergence"] is not None or r["first_word_divergence"] is not None
               or not r["solver_ops_equal"])
        if args.all or r is first:
            tag = "FIRST DIVERGING RUN" if bad else "equal"
            print(f"\nseq {r['seq']} ({r['arm']}, target {r['target_index']}): {tag}")
            print(f"  solver_ops A/B: {r['solver_ops'][0]} / {r['solver_ops'][1]}  nodes A/B: {r['nodes'][0]} / {r['nodes'][1]}")
            print(f"  first node whose system differs (hash/shape): {r['first_shape_divergence']}")
            print(f"  first node whose entry word counter differs: {r['first_word_divergence']}")
            if r.get("culprit_words") is not None:
                print(f"  culprit node (its own reduction charged differently): {r['culprit_node']}")
                print(f"  words charged by it A/B: {r['culprit_words'][0]} / {r['culprit_words'][1]}")
                print(f"  its system (A, B) per field: {r['culprit_shape']}")
    if first is None:
        print("\nNo divergence: the hosts agree on every compared run.")
    elif first["first_shape_divergence"] is not None:
        print("\nReading: the splitting trees differ at a node (hash/shape), "
              "contradicting the archived equality of splits/propagations: look at the engine's node logic.")
    elif first["first_word_divergence"] is not None:
        print("\nReading: same node sequence, same systems; the word count first differs while reducing the "
              "culprit node above. Re-run both hosts with KIC_F4_NODE_DUMP_COMPACT unset (full systems) "
              "for that seq to inspect it.")
    elif not first["solver_ops_equal"]:
        print("\nReading: every node entry counter agrees but the run total differs: the extra words are charged "
              "after the last node (final completion or outside the node path).")
    summary = {"a": args.a, "b": args.b, "meta_a": ma, "meta_b": mb, "warnings": warn,
               "runs_compared": len(results), "solver_ops_equal": n_ops_eq,
               "tree_identical": n_tree_eq, "word_counters_identical": n_word_eq,
               "first_diverging": first, "runs": results if args.all else None}
    if args.json:
        with open(args.json, "w") as f:
            json.dump(summary, f, indent=1)
            f.write("\n")
        print(f"\nsummary written to {args.json}")
    sys.exit(0 if first is None else 1)


if __name__ == "__main__":
    main()
