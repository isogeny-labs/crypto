#!/usr/bin/env python3
"""Regenerate the generated block of .github/branch-protect.txt.

Run this by hand (it scans every tracked text file, including *.json, *.csv,
*.log and *.txt, so it takes minutes on the full tree) from a full clone:

    git fetch origin
    python3 .github/scripts/gen_branch_protect.py --rev origin/main

and commit the result.  pr_hygiene.py refuses to delete branches when the
``scan-date`` stamp this script writes is missing or older than 14 days.

What it protects: every remote branch in a deletable lane (codex/ claude/
cursor/ feat/ fix/) whose exact name occurs anywhere in a tracked file at
``--rev`` (substring match, so ``refs/heads/<name>`` in an inventory file counts,
and a name that is a prefix of a longer cited name is protected too).  Hand
entries outside the BEGIN/END markers are kept untouched.

Standard library only; reads git, writes only --manifest.
"""
from __future__ import annotations

import argparse
import datetime
import os
import re
import subprocess
import sys
import tempfile

BEGIN = "# BEGIN GENERATED (gen_branch_protect.py; do not edit by hand)"
END = "# END GENERATED"
LANES = ("codex/", "claude/", "cursor/", "feat/", "fix/")
NAME_RE = re.compile(r"^[A-Za-z0-9._/-]{1,200}$")
MANIFEST_REL = ".github/branch-protect.txt"


def git(args, cwd):
    proc = subprocess.run(["git", "-C", cwd] + list(args), capture_output=True, text=True)
    if proc.returncode not in (0, 1):  # grep exits 1 for "no match"
        raise SystemExit("git %s failed: %s" % (args[0], proc.stderr.strip()[:300]))
    return proc.stdout


def remote_branches(cwd, remote):
    prefix = "refs/remotes/%s/" % remote
    out = git(["for-each-ref", "--format=%(refname)", prefix], cwd)
    names = set()
    for ln in out.splitlines():
        name = ln[len(prefix):]
        if name != "HEAD" and name.startswith(LANES) and NAME_RE.match(name):
            names.add(name)
    return names


def cited_names(cwd, rev, names):
    with tempfile.NamedTemporaryFile("w", suffix=".pat", delete=False) as fh:
        fh.write("\n".join(sorted(names)) + "\n")
        pat = fh.name
    try:
        out = git(["grep", "-h", "-I", "-o", "-F", "-f", pat, rev, "--", ".",
                   ":(exclude)" + MANIFEST_REL], cwd)
    finally:
        os.unlink(pat)
    return {ln.strip() for ln in out.splitlines() if ln.strip() in names}


def split_manifest(text):
    """(before, after) the generated block; whole text is `before` if no block yet."""
    if BEGIN in text and END in text:
        head, rest = text.split(BEGIN, 1)
        _, tail = rest.split(END, 1)
        return head, tail.lstrip("\n")
    return text, ""


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--rev", default="origin/main")
    ap.add_argument("--git-dir", default=".")
    ap.add_argument("--remote", default="origin")
    ap.add_argument("--manifest", default=MANIFEST_REL)
    ap.add_argument("--date", default=None, help="YYYY-MM-DD stamp (default: today, UTC)")
    args = ap.parse_args(argv)

    sha = git(["rev-parse", "--verify", args.rev + "^{commit}"], args.git_dir).strip()
    if not re.fullmatch(r"[0-9a-f]{40}", sha):
        raise SystemExit("cannot resolve %s" % args.rev)
    names = remote_branches(args.git_dir, args.remote)
    if not names:
        raise SystemExit("no remote branches under %s: fetch first" % args.remote)
    cited = sorted(cited_names(args.git_dir, sha, names))
    date = args.date or datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")

    with open(args.manifest, encoding="utf-8") as fh:
        before, after = split_manifest(fh.read())
    manual = {ln.split("#", 1)[0].strip() for ln in (before + after).splitlines()}
    block = [BEGIN,
             "# scan-date: %s" % date,
             "# scan-rev: %s" % sha,
             "# %d of %d deletable-lane remote branches are named in a tracked file at that"
             % (len(cited), len(names)),
             "# revision (every file type, including json/csv/log/txt)."]
    block += [n for n in cited if n not in manual]
    block.append(END)
    new = before.rstrip("\n") + "\n" + "\n".join(block) + "\n" + (("\n" + after) if after.strip() else "")
    with open(args.manifest, "w", encoding="utf-8") as fh:
        fh.write(new)
    sys.stdout.write("%s: %d cited branches (%d already listed by hand), rev %s\n"
                     % (args.manifest, len(cited), sum(1 for n in cited if n in manual), sha[:12]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
