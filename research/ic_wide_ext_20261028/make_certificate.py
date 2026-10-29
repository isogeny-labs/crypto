"""Aggregate the ecbench verify receipts into replay-certificate.json (validation-gate/v1).

The old/new binary sessions are replayed by the *other* binary (ab-old-* by the new one,
ab-new-* by the old one); the grid sessions by the binary that recorded them.
Standard library only."""
import hashlib
import json
import pathlib

HERE = pathlib.Path(__file__).parent
runs, ok_all = [], True
for receipt in sorted((HERE / "verify").glob("*.verify.json")):
    d = json.loads(receipt.read_text())
    name = receipt.name[: -len(".verify.json")]
    reproduced = sum(1 for r in d["replays"] if r["reproduced"])
    ok = bool(d["ok"]) and reproduced == len(d["replays"]) and d["verified_records"] == d["records"]
    ok_all &= ok
    cross = name.startswith("ab-")
    runs.append({"ok": ok, "receipt": f"verify/{receipt.name}",
                 "receipt_sha256": hashlib.sha256(receipt.read_bytes()).hexdigest(),
                 "recorded_by": {"ab-old": "the old binary (before the round-2 restore and the Fp64 inverse)",
                                 "ab-new": "the new binary (round-2 hunks restored, 32-bit Fp64 inverse)"}.get(name[:6], "ecbench release build of this branch"),
                 "records": d["records"],
                 "replayed_by": ("the other binary (exact cross-binary replay)" if cross else "the same binary"),
                 "replays": len(d["replays"]), "reproduced": reproduced, "run": f"sessions/{name}",
                 "session_id": d["session_id"], "verified_records": d["verified_records"]})
cert = {"independent": True,
        "method": "ecbench verify --replay-all on every session: every measured deterministic run re-executed in a fresh process with identical counts, every recovered scalar checked against its target; ab-old-i replayed by the new binary and ab-new-i by the old one",
        "runs": runs, "scalars_verified": ok_all, "schema": "validation-gate/v1 replay certificate",
        "status": "verified" if ok_all and runs else "failed"}
(HERE / "replay-certificate.json").write_text(json.dumps(cert, indent=1, sort_keys=True) + "\n")
print(cert["status"], len(runs), "sessions,", sum(r["replays"] for r in runs), "replays")
