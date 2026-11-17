#!/usr/bin/env python3
"""Post-process pandoc's body.typ: strip the directory prefix from image paths and
put each figure on its own landscape page at full width so the SVG text stays legible."""
import pathlib, re
p = pathlib.Path(__file__).resolve().parent / "body.typ"
s = p.read_text().replace("lattice_ecdlp_routes_20261023/", "")
captions = {
    "routes.svg": "The lattices attached to a prime-field ECDLP instance, split by whether a basis can be written without the log (source: routes.svg).",
    "exponents.svg": "Derived cost exponents of the small-x Coppersmith decomposition route against Pollard rho (left) and the root-region exponent a lattice reaches against the one rho parity needs (right). Values from exponents.json; nothing measured.",
}
def rep(m):
    name = m.group(1)
    return ('#page(flipped: true, margin: 1.2cm)[#figure(image("%s", width: 100%%), caption: [%s])]'
            % (name, captions[name]))
s, n = re.subn(r'#image\("([^"]+\.svg)"\)', rep, s)
assert n == 2, n
p.write_text(s)
print("fixed", n, "figures")
