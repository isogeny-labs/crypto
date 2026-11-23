#!/usr/bin/env python3
"""Mechanical validator for the prime-field lead verdict records.

Reads every verdicts/W*.json (JSON array of lead-verdict records) under the
scratchpad, runs schema, citation and consistency checks, prints a Markdown
report and writes it to checks/validation.md next to this script.

Stdlib only. Read-only with respect to the worker files and to /home/user.
Exit status is 0 when the report was produced; pass --strict to exit 1 when
any schema, citation or consistency problem is found.

Usage:
    python3 checks/validate.py [--verdicts DIR] [--out FILE] [--strict]
"""

import argparse
import collections
import glob
import json
import os
import re
import sys

# ---------------------------------------------------------------- locations
SCRATCH = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRYPTO = "/home/user/crypto"
CRYPTANALYSIS = "/home/user/cryptanalysis"
AUTORESEARCHER = "/home/user/crypto-autoresearcher"  # vault-encrypted, not read
CORPUS = os.path.join(SCRATCH, "ideas-corpus", "files")
CORPUS_PARENT = os.path.dirname(CORPUS)  # holds idea_index.json, pr-triage-*.md

# ---------------------------------------------------------------- schema
REQUIRED = [
    "id", "label", "column", "source_kind", "verdict", "evidence_strength",
    "flagship", "exponent_delta", "distance_to_rho", "mechanism",
    "why_this_verdict", "cheapest_next_test", "sources_checked", "pcc",
]
VERDICTS = ["refuted", "engineering", "open", "blocked", "unexamined", "out-of-scope"]
EVIDENCE = ["proof", "measured", "toy", "desk", "none"]
COLUMNS = ["FB", "ORACLE", "LIFT", "TRANSFER", "BARRIER", "ENG"]
SOURCE_KINDS = ["atlas", "idea-pr", "crypto-study"]
MAX_LABEL = 38

# ---------------------------------------------------------------- citations
REPO_PREFIXES = ("crypto-autoresearcher", "cryptanalysis", "ideas-corpus", "scratchpad", "crypto")
PATH_EXTS = (".md", ".json", ".py", ".yaml", ".yml", ".tsv", ".txt", ".csv", ".html",
             ".pdf", ".lean", ".rs", ".toml", ".svg", ".png", ".log", ".sh", ".c",
             ".h", ".go", ".jsonl", ".cfg", ".ini")
# Words that show a verdict rests on something stronger than a toy run.
STRONG_WORDS = re.compile(
    r"\b(measured|replay|replayed|proof|proved|theorem|certificate|counted|charged|"
    r"machine-checked|lean)\b", re.I)
TOY_WORD = re.compile(r"\btoy\b", re.I)


def _strip_suffix(tok):
    """Remove a trailing '#section', ':line', ':L12' or sentence punctuation."""
    tok = re.sub(r"#.*$", "", tok)
    tok = re.sub(r"(:L?\d+(-\d+)?)+$", "", tok)
    tok = tok.rstrip(".,;)]'\"")
    return tok


def classify_citation(raw):
    """Return (prefix, token) for a path-like entry, or (None, reason) otherwise."""
    if not isinstance(raw, str):
        return None, "not a string"
    s = raw.strip()
    if not s:
        return None, "empty"
    for pre in REPO_PREFIXES:
        if s.startswith(pre + ":"):
            rest = s[len(pre) + 1:].strip()
            tok = _strip_suffix(rest.split(" ")[0]) if rest else ""
            if not tok or tok.startswith("("):
                return None, "prefix with no path"
            if "/" not in tok and not tok.endswith(PATH_EXTS):
                return None, "prefix with no path"
            if re.search(r"\.md\.\.", tok) or re.search(r"\.\.\d{4}", tok):
                return None, "range phrase"
            return pre, tok
    if s.startswith("/"):
        tok = _strip_suffix(s.split(" ")[0])
        return "/", tok
    first = s.split(" ")[0]
    tok = _strip_suffix(first)
    if "/" in tok or tok.endswith(PATH_EXTS):
        return "bare", tok
    return None, "free text"


def expand_braces(tok):
    """Expand {a,b} alternatives and {x..y} numeric ranges, one level at a time."""
    m = re.search(r"\{([^{}]*)\}", tok)
    if not m:
        return [tok]
    body = m.group(1)
    rng = re.fullmatch(r"(\d+)\.\.(\d+)", body)
    if rng:
        width = len(rng.group(1))
        alts = ["%0*d" % (width, n) for n in range(int(rng.group(1)), int(rng.group(2)) + 1)]
    else:
        alts = body.split(",")
    out = []
    for a in alts:
        out.extend(expand_braces(tok[:m.start()] + a + tok[m.end():]))
    return out


def resolve(prefix, tok):
    """Return (status, detail). status: resolved | vaulted | missing | nonpath."""
    if prefix is None:
        return "nonpath", tok
    if "{" in tok:
        for variant in expand_braces(tok):
            status, detail = _resolve_one(prefix, variant)
            if status != "missing":
                return status, detail
        return "missing", tok
    return _resolve_one(prefix, tok)


def _resolve_one(prefix, tok):
    if prefix == "crypto":
        cands = [os.path.join(CRYPTO, tok)]
    elif prefix == "cryptanalysis":
        cands = [os.path.join(CRYPTANALYSIS, tok)]
    elif prefix == "crypto-autoresearcher":
        cands = [os.path.join(CORPUS, tok), os.path.join(CORPUS_PARENT, tok)]
        for c in cands:
            if os.path.exists(c):
                return "resolved", c
        return "vaulted", "crypto-autoresearcher:" + tok
    elif prefix == "ideas-corpus":
        cands = [os.path.join(CORPUS, tok), os.path.join(CORPUS_PARENT, tok)]
    elif prefix == "scratchpad":
        cands = [os.path.join(SCRATCH, tok), os.path.join(CORPUS, tok),
                 os.path.join(CORPUS_PARENT, tok)]
    elif prefix == "/":
        cands = [tok]
        if tok.startswith(AUTORESEARCHER + "/") and not os.path.exists(tok):
            return "vaulted", tok
    else:  # bare relative path: try every root the workers cite from
        cands = [os.path.join(CRYPTO, tok), os.path.join(CRYPTANALYSIS, tok),
                 os.path.join(CORPUS, tok), os.path.join(CORPUS_PARENT, tok),
                 os.path.join(SCRATCH, tok)]
    for c in cands:
        if os.path.exists(c):
            return "resolved", c
    return "missing", tok


# ---------------------------------------------------------------- helpers
def is_blank(v):
    return v is None or (isinstance(v, str) and not v.strip())


def md_table(header, rows):
    out = ["| " + " | ".join(header) + " |",
           "|" + "|".join("---" for _ in header) + "|"]
    for r in rows:
        out.append("| " + " | ".join(str(x) for x in r) + " |")
    return out


def load_worker_files(verdict_dir):
    paths = sorted(glob.glob(os.path.join(verdict_dir, "W*.json")),
                   key=lambda p: (len(os.path.basename(p)), os.path.basename(p)))
    files = []
    for p in paths:
        name = os.path.splitext(os.path.basename(p))[0]
        entry = {"name": name, "path": p, "records": [], "status": "", "error": None}
        try:
            with open(p, encoding="utf-8") as fh:
                data = json.load(fh)
            if not isinstance(data, list):
                entry["status"] = "parse error"
                entry["error"] = "top level is %s, not a JSON array" % type(data).__name__
            else:
                entry["records"] = data
                entry["status"] = "ok"
        except (OSError, ValueError) as exc:
            entry["status"] = "parse error"
            entry["error"] = str(exc)
        files.append(entry)
    return files


# ---------------------------------------------------------------- main report
def build_report(verdict_dir):
    files = load_worker_files(verdict_dir)
    lines = ["# Lead verdict validation report", "",
             "Verdict directory: `%s`" % verdict_dir, ""]
    problems = 0

    # ---- 1. files
    lines += ["## 1. Files", ""]
    rows = []
    total_records = 0
    for f in files:
        n = len(f["records"]) if f["status"] == "ok" else 0
        total_records += n
        status = f["status"] if f["status"] == "ok" else "%s: %s" % (f["status"], f["error"])
        if f["status"] != "ok":
            problems += 1
        rows.append([f["name"], n, status])
    lines += md_table(["File", "Records", "Parse status"], rows)
    lines += ["", "Total records: **%d** across %d files." % (total_records, len(files)), ""]

    # ---- 2. schema
    lines += ["## 2. Schema", ""]
    missing_fields, bad_enums, long_labels, pcc_bad, non_dict = [], [], [], [], []
    id_locations = collections.defaultdict(list)
    all_records = []  # (file, index, record)
    for f in files:
        for i, rec in enumerate(f["records"]):
            where = "%s[%d]" % (f["name"], i)
            if not isinstance(rec, dict):
                non_dict.append(where)
                continue
            rid = rec.get("id")
            all_records.append((f["name"], i, rec))
            label = rid if not is_blank(rid) else where
            if not is_blank(rid):
                id_locations[rid].append(f["name"])
            miss = [k for k in REQUIRED if k not in rec or is_blank(rec.get(k))
                    and not isinstance(rec.get(k), bool)]
            # flagship is a boolean; False is a valid value, so it is never "missing".
            if "flagship" in rec and not isinstance(rec.get("flagship"), bool):
                if "flagship" not in miss:
                    bad_enums.append((f["name"], label, "flagship must be boolean, got %r" % (rec.get("flagship"),)))
            if "flagship" in miss and "flagship" in rec and isinstance(rec["flagship"], bool):
                miss.remove("flagship")
            if miss:
                missing_fields.append((f["name"], label, miss))
            for key, allowed in (("verdict", VERDICTS), ("evidence_strength", EVIDENCE),
                                 ("column", COLUMNS), ("source_kind", SOURCE_KINDS)):
                if key in rec and not is_blank(rec.get(key)) and rec.get(key) not in allowed:
                    bad_enums.append((f["name"], label, "%s=%r not in %s" % (key, rec[key], "/".join(allowed))))
            lab = rec.get("label")
            if isinstance(lab, str) and len(lab) > MAX_LABEL:
                long_labels.append((f["name"], label, len(lab), lab))
            if rec.get("pcc") != "unverified":
                pcc_bad.append((f["name"], label, rec.get("pcc")))
            if "sources_checked" in rec and not isinstance(rec["sources_checked"], list):
                bad_enums.append((f["name"], label, "sources_checked is not a list"))

    dups = {k: v for k, v in id_locations.items() if len(v) > 1}
    cross_file = {k: v for k, v in dups.items() if len(set(v)) > 1}
    within_file = {k: v for k, v in dups.items()
                   if any(v.count(name) > 1 for name in set(v))}

    def sec(title, items, fmt):
        lines.append("### %s (%d)" % (title, len(items)))
        if not items:
            lines.append("None.")
        else:
            for it in items:
                lines.append("- " + fmt(it))
        lines.append("")

    sec("Records missing required fields", missing_fields,
        lambda t: "`%s` `%s`: %s" % (t[0], t[1], ", ".join(t[2])))
    sec("Bad enum or type values", bad_enums,
        lambda t: "`%s` `%s`: %s" % (t[0], t[1], t[2]))
    sec("Labels longer than %d chars" % MAX_LABEL, long_labels,
        lambda t: "`%s` `%s`: %d chars: %r" % (t[0], t[1], t[2], t[3]))
    sec("Duplicate ids across files", list(cross_file.items()),
        lambda t: "`%s` in %s" % (t[0], ", ".join(t[1])))
    sec("Duplicate ids within a single file", list(within_file.items()),
        lambda t: "`%s` in %s" % (t[0], ", ".join(t[1])))
    sec("pcc field not equal to \"unverified\"", pcc_bad,
        lambda t: "`%s` `%s`: pcc=%r" % (t[0], t[1], t[2]))
    sec("Non-object entries in arrays", non_dict, lambda t: "`%s`" % t)

    schema_issues = (len(missing_fields) + len(bad_enums) + len(long_labels)
                     + len(cross_file) + len(within_file) + len(pcc_bad) + len(non_dict))
    problems += schema_issues

    # ---- 3. citations
    lines += ["## 3. Citation check", ""]
    status_counts = collections.Counter()
    per_worker = collections.defaultdict(lambda: collections.Counter())
    missing_by_worker = collections.defaultdict(lambda: collections.defaultdict(list))
    vaulted_by_worker = collections.Counter()
    vaulted_tokens = collections.Counter()
    nonpath_by_worker = collections.Counter()
    nonpath_samples = collections.Counter()
    for f in files:
        for i, rec in enumerate(f["records"]):
            if not isinstance(rec, dict):
                continue
            srcs = rec.get("sources_checked")
            if not isinstance(srcs, list):
                continue
            rid = rec.get("id", "?")
            for raw in srcs:
                prefix, tok = classify_citation(raw)
                if prefix is None:
                    status_counts["non-path note"] += 1
                    nonpath_by_worker[f["name"]] += 1
                    nonpath_samples[str(raw).strip()[:70]] += 1
                    continue
                status, detail = resolve(prefix, tok)
                status_counts[status] += 1
                per_worker[f["name"]][status] += 1
                if status == "missing":
                    missing_by_worker[f["name"]][str(raw).strip()].append(rid)
                elif status == "vaulted":
                    vaulted_by_worker[f["name"]] += 1
                    vaulted_tokens[detail] += 1
    lines += md_table(
        ["Outcome", "Citations"],
        [["resolved", status_counts["resolved"]],
         ["vaulted (expected unreadable)", status_counts["vaulted"]],
         ["missing (no file found)", status_counts["missing"]],
         ["non-path note (not checked)", status_counts["non-path note"]]])
    lines.append("")
    lines.append("### Per worker")
    lines += md_table(
        ["File", "Resolved", "Vaulted", "Missing", "Non-path"],
        [[f["name"], per_worker[f["name"]]["resolved"], per_worker[f["name"]]["vaulted"],
          per_worker[f["name"]]["missing"], nonpath_by_worker[f["name"]]]
         for f in files])
    lines.append("")
    lines.append("### Missing citations, grouped by worker")
    if status_counts["missing"] == 0:
        lines.append("None.")
    for f in files:
        miss = missing_by_worker.get(f["name"])
        if not miss:
            continue
        lines.append("")
        lines.append("**%s**: %d missing citation(s), %d distinct" % (
            f["name"], sum(len(v) for v in miss.values()), len(miss)))
        for raw, ids in sorted(miss.items(), key=lambda kv: (-len(kv[1]), kv[0])):
            lines.append("- `%s` (x%d; in %s)" % (raw, len(ids), ", ".join(sorted(set(ids)))))
    lines.append("")
    if vaulted_tokens:
        lines.append("### Vaulted citations (expected unreadable, not counted as missing)")
        lines.append("%d distinct vaulted path(s), %d citation(s)." % (
            len(vaulted_tokens), sum(vaulted_tokens.values())))
        lines.append("")
    lines.append("### Non-path notes (free text, not checked)")
    for txt, n in nonpath_samples.most_common(12):
        lines.append("- x%d `%s`" % (n, txt))
    lines.append("")
    problems += status_counts["missing"]

    # ---- 4. consistency
    lines += ["## 4. Consistency flags", ""]
    flags = collections.defaultdict(list)
    for fname, i, rec in all_records:
        rid = rec.get("id", "?")
        verdict = rec.get("verdict")
        ev = rec.get("evidence_strength")
        if verdict == "refuted" and ev == "none":
            flags["refuted with evidence_strength none"].append((fname, rid))
        if rec.get("flagship") is True and verdict in ("refuted", "engineering", "out-of-scope"):
            flags["flagship true but verdict %s" % verdict].append((fname, rid))
        cr = rec.get("check_run_today")
        if isinstance(cr, dict) and cr.get("ran") is True and is_blank(cr.get("command")):
            flags["check_run_today.ran true with empty command"].append((fname, rid))
        why = rec.get("why_this_verdict") or ""
        if ev in ("measured", "proof") and isinstance(why, str) and TOY_WORD.search(why) \
                and not STRONG_WORDS.search(why):
            flags["evidence %s but why_this_verdict cites only 'toy' (review)" % ev].append((fname, rid))
    # Informational: open/blocked flagships are fine by definition.
    fine = [rid for _, _, rec in all_records
            if rec.get("flagship") is True and rec.get("verdict") in ("open", "blocked")
            for rid in [rec.get("id", "?")]]
    if not flags:
        lines.append("None.")
    for key in sorted(flags):
        items = flags[key]
        lines.append("### %s (%d)" % (key, len(items)))
        for fname, rid in items:
            lines.append("- `%s` `%s`" % (rid, fname))
        lines.append("")
    lines.append("Info: %d flagship record(s) with verdict open or blocked (expected, not flagged)." % len(fine))
    lines.append("")
    problems += sum(len(v) for v in flags.values())

    # ---- 5. summary
    lines += ["## 5. Summary", ""]
    lines.append("### Verdict x column")
    vc = collections.Counter((r.get("verdict"), r.get("column")) for _, _, r in all_records)
    lines += md_table(["verdict \\ column"] + COLUMNS + ["total"],
                      [[v] + [vc[(v, c)] for c in COLUMNS] + [sum(vc[(v, c)] for c in COLUMNS)]
                       for v in VERDICTS]
                      + [["total"] + [sum(vc[(v, c)] for v in VERDICTS) for c in COLUMNS]
                         + [sum(vc.values())]])
    lines.append("")
    lines.append("### Verdict x evidence_strength")
    ve = collections.Counter((r.get("verdict"), r.get("evidence_strength")) for _, _, r in all_records)
    lines += md_table(["verdict \\ evidence"] + EVIDENCE + ["total"],
                      [[v] + [ve[(v, e)] for e in EVIDENCE] + [sum(ve[(v, e)] for e in EVIDENCE)]
                       for v in VERDICTS]
                      + [["total"] + [sum(ve[(v, e)] for v in VERDICTS) for e in EVIDENCE]
                         + [sum(ve.values())]])
    lines.append("")
    lines.append("### source_kind counts")
    sk = collections.Counter(r.get("source_kind") for _, _, r in all_records)
    lines += md_table(["source_kind", "count"],
                      [[k, sk.get(k, 0)] for k in SOURCE_KINDS]
                      + [[k, n] for k, n in sk.items() if k not in SOURCE_KINDS])
    lines.append("")
    flagships = [(fname, r.get("id"), r.get("verdict"), r.get("column"))
                 for fname, _, r in all_records if r.get("flagship") is True]
    lines.append("### Flagship list (%d)" % len(flagships))
    if not flagships:
        lines.append("None.")
    for fname, rid, v, c in flagships:
        lines.append("- `%s` (%s, %s, %s)" % (rid, v, c, fname))
    lines.append("")

    # ---- headline
    lines += ["## Headline", "",
              "- Records: %d; schema issues: %d; missing citations: %d; vaulted citations: %d; "
              "consistency flags: %d; file parse failures: %d." % (
                  len(all_records), schema_issues, status_counts["missing"],
                  status_counts["vaulted"], sum(len(v) for v in flags.values()),
                  sum(1 for f in files if f["status"] != "ok")),
              ""]
    return "\n".join(lines) + "\n", problems


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[1])
    ap.add_argument("--verdicts", default=os.path.join(SCRATCH, "verdicts"),
                    help="directory holding W*.json (default: scratchpad/verdicts)")
    ap.add_argument("--out", default=os.path.join(SCRATCH, "checks", "validation.md"),
                    help="where to write the Markdown report")
    ap.add_argument("--strict", action="store_true",
                    help="exit 1 if any problem is found (default: exit 0 after a successful run)")
    args = ap.parse_args(argv)

    report, problems = build_report(args.verdicts)
    sys.stdout.write(report)
    out_dir = os.path.dirname(os.path.abspath(args.out))
    os.makedirs(out_dir, exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as fh:
        fh.write(report)
    sys.stderr.write("wrote %s\n" % args.out)
    if args.strict and problems:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
