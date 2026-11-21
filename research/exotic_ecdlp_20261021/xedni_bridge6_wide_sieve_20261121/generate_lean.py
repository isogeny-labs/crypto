"""Generate concrete Lean certificates from frozen row records."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main():
    rows = sorted((HERE / "rows").glob("[0-9][0-9][0-9].jsonl"))
    assert len(rows) == 441
    raw = b"".join(path.read_bytes() for path in rows)
    result = json.loads((HERE / "result.json").read_text())
    assert hashlib.sha256(raw).hexdigest() == result["rows_raw_sha256"]
    survivor = json.loads((HERE / "survivor-result.json").read_text())
    assert survivor["extended_certificate_prime"] == 293
    out = ["import Mathlib.Data.ZMod.Basic\nimport Mathlib.Tactic\n\n",
           "set_option maxRecDepth 100000\nset_option maxHeartbeats 0\n\n",
           f"/-! Concrete first-rootless-prime certificates for all 441 cells.\n    Source rows SHA-256: {result['rows_raw_sha256']} -/\n\n",
           "namespace XedniBridge6WideSieve\n\n",
           "def evalInt : List ℤ → ℤ → ℤ\n"
           "  | [], _ => 0\n"
           "  | c :: cs, t => c + t * evalInt cs t\n\n",
           "def evalResidue (n : ℕ) : List ℤ → ZMod n → ZMod n\n"
           "  | [], _ => 0\n"
           "  | c :: cs, z => (c : ZMod n) + z * evalResidue n cs z\n\n",
           "theorem evalResidue_cast (n : ℕ) (cs : List ℤ) (t : ℤ) :\n"
           "    evalResidue n cs (t : ZMod n) = (evalInt cs t : ZMod n) := by\n"
           "  induction cs with\n"
           "  | nil => simp [evalInt, evalResidue]\n"
           "  | cons c cs ih => simp [evalInt, evalResidue, ih]\n\n",
           "theorem no_integer_root_if_rootless (n : ℕ) (cs : List ℤ)\n"
           "    (h : ∀ z : ZMod n, evalResidue n cs z ≠ 0) (t : ℤ) :\n"
           "    evalInt cs t ≠ 0 := by\n"
           "  intro hz\n"
           "  have ht := h (t : ZMod n)\n"
           "  rw [evalResidue_cast] at ht\n"
           "  exact ht (by simp [hz])\n\n"]
    for i, path in enumerate(rows):
        row = json.loads(path.read_text())
        assert row["status"] in ("excluded_all_integer_t", "unresolved")
        p = row["certificate_prime"] or 293
        assert p > 0 and (i != 107 or p == 293)
        coeffs = [int(c) for c in row["coefficients"]]
        name = f"coeffs{i:03d}"
        out.extend([f"def {name} : List ℤ := [\n",
                    "\n".join("  " + str(c) + ("," if j+1 < len(coeffs) else "")
                              for j, c in enumerate(coeffs)),
                    "\n]\n\n",
                    f"theorem rootless{i:03d} : ∀ z : ZMod {p}, "
                    f"evalResidue {p} {name} z ≠ 0 := by\n  decide\n\n",
                    f"theorem no_integer_root{i:03d} (t : ℤ) : "
                    f"evalInt {name} t ≠ 0 :=\n"
                    f"  no_integer_root_if_rootless {p} {name} rootless{i:03d} t\n\n",
                    f"#print axioms rootless{i:03d}\n"
                    f"#print axioms no_integer_root{i:03d}\n\n"])
    out.append("end XedniBridge6WideSieve\n")
    dest = HERE / "formal" / "WideSieve.lean"
    dest.parent.mkdir(exist_ok=True)
    dest.write_text("".join(out))
    print(json.dumps({"schema": "xedni-bridge6-wide-sieve-lean-source-v1",
                      "rows": len(rows), "bytes": dest.stat().st_size,
                      "source_rows_sha256": result["rows_raw_sha256"],
                      "lean_sha256": hashlib.sha256(dest.read_bytes()).hexdigest()},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
