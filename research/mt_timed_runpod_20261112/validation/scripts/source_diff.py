#!/usr/bin/env python3
"""VALIDATOR source check: is the only change to examples/mt_fieldops.rs between the validated timed study's commit and
the frozen commit the hasher alias / feature?  Applies the allowed rewrites to the old file, strips the alias block
from the new one, normalises whitespace and compares; prints every residual difference.  Also diffs Cargo.toml at the
parent of the protocol commit vs the frozen commit (only the `mt-fasthash = []` feature and its comment allowed).
usage: source_diff.py <repo> <old commit> <protocol parent commit> <frozen commit> <out.json>"""
import difflib
import json
import re
import subprocess
import sys

REPO, OLD, PARENT, NEW, OUT = sys.argv[1:6]


def show(c, p):
    return subprocess.run(["git", "-C", REPO, "show", f"{c}:{p}"], capture_output=True, text=True, check=True).stdout


old = show(OLD, "examples/mt_fieldops.rs")
new = show(NEW, "examples/mt_fieldops.rs")
res = dict(old_commit=OLD, new_commit=NEW)
# alias block in the new file: from the first cfg line to the end of `impl std::hash::Hasher for MtFastH { ... }`
m = re.search(r"// The hash tables of every arm\..*?\nimpl std::hash::Hasher for MtFastH \{.*?\n\}\n", new, re.S)
res["alias_block_found"] = bool(m)
alias = m.group(0) if m else ""
res["alias_block"] = alias
new_wo = new.replace(alias, "")
# allowed rewrites of the old file
o = old.replace("use std::collections::HashMap;\n", "")
o = o.replace("HashMap::new()", "HashMap::default()")
o = re.sub(r"HashMap::with_capacity\(((?:[^()]|\([^()]*\))*)\)",
           lambda mm: mm.group(0) if mm.group(1).strip() == "size * 2" else "HashMap::with_capacity_and_hasher(%s, Default::default())" % mm.group(1), o)
# (the opcost std-table site `HashMap::with_capacity(size * 2)` stays std in the new file, written with explicit std types)
# opcost: explicit std types in the new file are the old file's HashMap (std, before the alias existed)
n = new_wo.replace("std::collections::HashMap<", "HashMap<").replace("std::collections::HashMap::with_capacity_and_hasher(", "HashMap::with_capacity_and_hasher(")
n = n.replace("std::collections::HashMap::with_capacity(", "HashMap::with_capacity(")
norm = lambda s: re.sub(r"\s+", "", s)
a, b = norm(o), norm(n)
res["normalised_equal"] = a == b
if a != b:
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    res["residual"] = [[tag, a[i1 - 40:i2 + 40], b[j1 - 40:j2 + 40]] for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != "equal"]
# alias semantics: default build = RandomState (SipHash), feature build = BuildHasherDefault<MtFastH>
res["alias_default_is_RandomState"] = '#[cfg(not(feature = "mt-fasthash"))]\ntype HState = std::collections::hash_map::RandomState;' in alias
res["alias_feature_is_MtFastH"] = '#[cfg(feature = "mt-fasthash")]\ntype HState = std::hash::BuildHasherDefault<MtFastH>;' in alias
res["remaining_std_hashmap_uses_in_new"] = [l.strip() for l in new.splitlines() if "std::collections::HashMap" in l and "type HashMap" not in l]
res["hashmap_new_or_with_capacity_left_in_new"] = [l.strip() for l in new.splitlines() if re.search(r"\bHashMap::(new|with_capacity)\(", l)]
# Cargo.toml
c0, c1 = show(PARENT, "Cargo.toml"), show(NEW, "Cargo.toml")
d = [l for l in difflib.unified_diff(c0.splitlines(), c1.splitlines(), lineterm="", n=0) if l[:1] in "+-" and not l.startswith(("+++", "---"))]
res["cargo_toml_diff"] = d
res["cargo_toml_only_feature"] = all(l.startswith("+") for l in d) and any(l == "+mt-fasthash = []" for l in d) and all(l.startswith("+#") or l == "+mt-fasthash = []" for l in d)
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in res.items() if k != "alias_block"}, indent=1))
