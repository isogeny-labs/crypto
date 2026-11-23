"""Produce a checked finite census of the saved outcome statuses."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main():
    files = sorted((HERE / "outcomes").glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(f.read_bytes() for f in files)
    result = json.loads((HERE / "result.json").read_text())
    assert hashlib.sha256(raw).hexdigest() == result["outcomes_raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == 42
    selected = [r["pair_index"] for r in rows]
    bridge = [r["status"] == "local_bridge" for r in rows]
    rational = [bool(r.get("crt_rational_relation")) for r in rows]
    text = """import Mathlib.Tactic

/-! Finite status census from the saved source-pair outcome rows.
    This module does not encode the elliptic-curve group-law replay. -/

namespace XedniBridge6SourcePolicy

"""
    text += "def selected : List ℕ := [" + ", ".join(map(str, selected)) + "]\n"
    text += "def bridge : List Bool := [" + ", ".join(str(x).lower() for x in bridge) + "]\n"
    text += "def rational : List Bool := [" + ", ".join(str(x).lower() for x in rational) + "]\n\n"
    text += """theorem selected_count : selected.length = 42 := by decide
theorem local_bridge_count : (bridge.filter id).length = 22 := by decide
theorem first_crt_rational_count : (rational.filter id).length = 0 := by decide

#print axioms selected_count
#print axioms local_bridge_count
#print axioms first_crt_rational_count

end XedniBridge6SourcePolicy
"""
    dest = HERE / "formal" / "Census.lean"
    dest.parent.mkdir(exist_ok=True)
    dest.write_text(text)
    print(json.dumps({"schema": "xedni-bridge6-source-policy-lean-source-v1",
                      "source_outcomes_sha256": result["outcomes_raw_sha256"],
                      "lean_sha256": hashlib.sha256(dest.read_bytes()).hexdigest()},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
