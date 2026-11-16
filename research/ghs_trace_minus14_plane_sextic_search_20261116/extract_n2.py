#!/usr/bin/env sage -python
"""Capture F9 survivors from the frozen first screen without editing it."""
from pathlib import Path

source = Path(__file__).with_name("search.py")
text = source.read_text()
needle = "counts['N2']+=len(C)\n if len(C)==0:continue"
assert text.count(needle) == 1
text = text.replace(needle, "counts['N2']+=len(C)\n near.extend([[int(v) for v in row] for row in C])\n if len(C)==0:continue")
exec(compile(text, str(source), "exec"), {"__name__": "__main__", "__file__": str(source)})
