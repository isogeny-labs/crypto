"""Apply the textual corrections in PATCHES.md to the program document.

Usage
    python3 apply_patches.py TARGET.md --dry-run
    python3 apply_patches.py TARGET.md

Matching is whitespace-insensitive (line wrapping in the source does not matter)
but otherwise literal, including unicode.  Each patch must match exactly once;
if any patch fails the file is left untouched.  Patches whose replacement is not
a blockquote (P8, the summary table) are reported as MANUAL.
"""
import re, sys, textwrap

PATCH_FILE = "PATCHES.md"

def parse(path):
    doc = open(path, encoding="utf-8").read()
    sections = re.split(r"\n---+\n", doc)
    patches = []
    for sec in sections:
        pid = re.search(r"^## (P\d+)", sec, re.M)
        pid = pid.group(1) if pid else "?"
        pairs = re.findall(
            r"\*\*Original[^*]*\*\*\s*\n((?:>.*\n?)+?)\s*\n\*\*Replacement\*\*\s*\n((?:>.*\n?)+)",
            sec)
        for old, new in pairs:
            strip = lambda s: "\n".join(l[2:] if l.startswith("> ") else l.lstrip(">")
                                        for l in s.strip().splitlines())
            patches.append((pid, strip(old), strip(new)))
        if "MANUAL" not in sec and "| # | Direction" in sec:
            patches.append((pid, None, None))
    return patches

def flexible(old):
    words = re.split(r"\s+", old.strip())
    return r"\s+".join(re.escape(w) for w in words)

def main():
    target = sys.argv[1]
    dry = "--dry-run" in sys.argv
    doc = open(target, encoding="utf-8").read()
    patches = parse(PATCH_FILE)
    applied, failed, manual = [], [], []
    for pid, old, new in patches:
        if old is None:
            manual.append(pid); continue
        pat = flexible(old)
        n = len(re.findall(pat, doc))
        if n != 1:
            failed.append((pid, n, old.split(".")[0][:70]))
            continue
        wrapped = textwrap.fill(new, width=95)
        doc = re.sub(pat, lambda m: wrapped, doc, count=1)
        applied.append(pid)
    print("applied:", applied)
    print("manual :", manual)
    if failed:
        print("FAILED (file not written):")
        for pid, n, head in failed:
            print(f"  {pid}: {n} matches for {head!r}...")
        sys.exit(1)
    if not dry:
        open(target, "w", encoding="utf-8").write(doc)
        print("wrote", target)
    else:
        print("dry run: no write")

main()
