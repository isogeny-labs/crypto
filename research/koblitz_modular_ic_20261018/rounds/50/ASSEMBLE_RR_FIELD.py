"""Literal transcription only; all rational-field arithmetic is native CAS."""
from pathlib import Path
import hashlib
import json

here = Path(__file__).resolve().parent
donor = here.parent / "43" / "source-data"
sections_file = donor / "twist-sections.txt"
pairs_file = donor / "pairs2.txt"
lines = iter(sections_file.read_text().splitlines())
sections = []
for header in lines:
    tag, index, count = header.split()
    assert tag == "W" and int(index) == len(sections) + 1
    terms = [tuple(map(int, next(lines).split(","))) for _ in range(int(count))]
    assert all(len(e) == 4 and all(i >= 0 for i in e) for e in terms)
    sections.append(terms)
assert len(sections) == 68
pairs = [tuple(map(int, line.split())) for line in pairs_file.read_text().splitlines()]
assert len(pairs) == 196
assert all(index == i + 1 and 1 <= a <= 68 and 1 <= b <= 68
           for i, (index, a, b) in enumerate(pairs))
parts = [here.joinpath("source-data/rational-candidate50.sing").read_text(),
         "ring rrSections50=2,(N,L,V,U),dp;short=0;ideal rrNumerators50;"]
for i, terms in enumerate(sections, 1):
    monomials = ["*".join(f"{name}^{exponent}" for name, exponent in
                         zip(("N", "L", "V", "U"), term) if exponent) or "1"
                 for term in terms]
    parts.append(f"rrNumerators50[{i}]=" + "+".join(monomials) + ";")
parts += ["ring rrField50=(2,u),(l,v),dp;short=0;option(redSB);",
          "intmat rrPairs50[196][2];"]
for i, a, b in pairs:
    parts.append(f"rrPairs50[{i},1]={a};rrPairs50[{i},2]={b};")
parts.append(here.joinpath("RR_FIELD50.sing").read_text())
here.joinpath("source-data/rr-field-input50.sing").write_text("\n".join(parts))
here.joinpath("rr-field-donors50.json").write_text(json.dumps({
    str(path.relative_to(here.parent)): {
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "bytes": path.stat().st_size,
    } for path in (sections_file, pairs_file)
}, indent=2) + "\n")
