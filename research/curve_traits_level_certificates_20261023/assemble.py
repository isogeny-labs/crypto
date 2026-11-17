"""Write docs/curves/level-certificates.json from certs.jsonl, sorted by
(slug, ell).  Layout only: curve_traits re-verifies every certificate."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
certs = [json.loads(l) for l in open(os.path.join(HERE, "certs.jsonl"))]
certs.sort(key=lambda c: (c["slug"], c["ell"]))
out = {
    "schema": "level-certificates/1",
    "what_this_is": "Stored proofs of depth-1 volcano levels of End(E), found by "
                    "research/curve_traits_level_certificates_20261023/search.gp and checked by "
                    "curve_traits (points.rs, verify_level_certificate); see docs/curves/TRAITS.md.",
    "certificates": certs,
}
with open(os.path.join(ROOT, "docs/curves/level-certificates.json"), "w") as f:
    json.dump(out, f, separators=(",", ":"))
    f.write("\n")
print(len(certs), "certificates")
