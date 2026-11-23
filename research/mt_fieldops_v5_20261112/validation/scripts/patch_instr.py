#!/usr/bin/env python3
"""VALIDATOR instrumentation of examples/mt_fieldops.rs (v5).  Adds NO counted operation: a separate global
atomic tally XC (never read by the harness) counts, per category,
  0 HashMap probes/inserts on the counted tables (table, seen, db, pending, baby, ret inserts)
  1 the retained-table probe ret.get (ic5ks/ic5rw; one f.hash() covers ret.get + table.get)
  2 hidx calls (r-adding rho step index)      3 canon calls (rhodp/rhodpu)
  4 idx calls (rhodp/rhodpu step-index hash)  5 is_dp calls (rhodp/rhodpu DP test)
  6 W-step index computations (ic5w/ic5rw)    7 walk-restart RNG draws (rho family, uncounted by the harness)
  8 rhodpu setup HashSet probes (arm construction)
and, when MTFO_DUMP=<path> is set, writes {xs, qx, qy, found, xc} as JSON to <path>.
stdout is byte-identical in content to the pristine binary's except the ms field.
usage: patch_instr.py <pristine main.rs> <out main.rs>"""
import sys, re
src = open(sys.argv[1]).read().split("\n")
out = []
hdr = """use std::sync::atomic::{AtomicU64, Ordering};
static XC: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
fn xc(i: usize) {
    XC[i].fetch_add(1, Ordering::Relaxed);
}"""
done_hdr = False
for i, line in enumerate(src):
    s = line.strip()
    ind = line[: len(line) - len(line.lstrip())]
    if not done_hdr and s.startswith("type Pt ="):
        out.extend(hdr.split("\n")); done_hdr = True
    if re.search(r"\b(table|seen|seen_pairs|db|pending|baby|ret)\.(get|entry|insert)\(", s) and not s.startswith("//"):
        cat = 1 if re.search(r"\bret\.get\(", s) else 0
        out.append(f"{ind}xc({cat});")
    if "used.insert(" in s:
        # if i == j || !used.insert(...) : count only when evaluated (i != j); approximate by counting the line
        line = line.replace("!used.insert(", "!{ xc(8); used.insert(")
        line = line.replace(")) {", "))} {", 1) if "!{ xc(8)" in line else line
    if s.startswith("let k1 = rng.gen_range(0..BANK);"):
        out.append(f"{ind}xc(7);")
    out.append(line)
src = "\n".join(out)
n_h = src.count("let hidx = |pt: Pt| -> usize {\n                f.native();")
assert n_h == 4, n_h
src = src.replace("let hidx = |pt: Pt| -> usize {\n                f.native();", "let hidx = |pt: Pt| -> usize {\n                xc(2);\n                f.native();")
old_c = "let canon = |p: Pt| -> (Pt, i64) {\n                f.native();\n                f.native();"
assert src.count(old_c) == 2
src = src.replace(old_c, old_c + "\n                xc(3);")
old_i = "let idx = |p: Pt| -> usize { p.map("
assert src.count(old_i) == 2
src = src.replace(old_i, "let idx = |p: Pt| -> usize { xc(4); p.map(")
old_d = "|p: Pt| -> bool { p.map("
assert src.count(old_d) == 2
src = src.replace(old_d, "|p: Pt| -> bool { xc(5); p.map(")
old_w = "                    f.native();\n                    let j = {"
assert src.count(old_w) == 2
src = src.replace(old_w, "                    xc(6);\n" + old_w)
old_p = "    let phases: Vec<String> = snap"
assert src.count(old_p) == 1
dump = r'''    {
        let xcv: Vec<u64> = XC.iter().map(|a| a.load(Ordering::Relaxed)).collect();
        eprintln!("XC {:?}", xcv);
        if let Ok(path) = std::env::var("MTFO_DUMP") {
            let js = serde_json::json!({
                "xs": xs.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                "qx": qs.iter().map(|q| q.unwrap().0.to_string()).collect::<Vec<_>>(),
                "qy": qs.iter().map(|q| q.unwrap().1.to_string()).collect::<Vec<_>>(),
                "found": found.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                "xc": xcv,
            });
            std::fs::write(path, js.to_string()).unwrap();
        }
    }
'''
src = src.replace(old_p, dump + old_p)
open(sys.argv[2], "w").write(src)
