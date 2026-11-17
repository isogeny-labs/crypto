#!/usr/bin/env python3
"""Run the frozen bounded BSGS matrix and retain every process record."""

import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "raw"
BIN = Path("/private/tmp/codex-eccp131-build-crypto/release/ecdlp-nist")
CELLS = [("ECCp-131", 16), ("ECC2K-130", 16), ("ECCp-131", 20), ("ECC2K-130", 20)]
ORDERS = [CELLS, CELLS[1:] + CELLS[:1], CELLS[2:] + CELLS[:2]]


def write_new(path, data):
    with path.open("x", encoding="utf-8") as out:
        out.write(data)


def main():
    RAW.mkdir(exist_ok=False)
    binary_hash = hashlib.sha256(BIN.read_bytes()).hexdigest()
    manifest = {
        "binary": str(BIN),
        "binary_sha256": binary_hash,
        "platform": platform.platform(),
        "python": sys.version,
        "source_head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "cwd": os.getcwd(),
        "round_order": ORDERS,
    }
    write_new(ROOT / "manifest.json", json.dumps(manifest, indent=2) + "\n")
    sequence = 0
    for round_index, cells in enumerate(ORDERS, 1):
        for curve, width in cells:
            for repeat in (("b", "a") if round_index == 2 else ("a", "b")):
                sequence += 1
                tag = f"{sequence:02d}-r{round_index}-{curve.lower()}-w{width}-{repeat}"
                args = [
                    str(BIN), "solve", curve, "--secret", "12345", "--interval-bits", str(width),
                    "--method", "bsgs", "--threads", "1", "--seed", "131", "--json",
                ]
                command = {"tag": tag, "argv": args, "binary_sha256": binary_hash}
                write_new(RAW / f"{tag}.command.json", json.dumps(command, indent=2) + "\n")
                started = time.monotonic()
                try:
                    p = subprocess.run(["/usr/bin/time", "-l", *args], text=True, capture_output=True, timeout=120)
                    code = p.returncode
                    stdout, stderr = p.stdout, p.stderr
                except subprocess.TimeoutExpired as exc:
                    code = 124
                    stdout = (exc.stdout or b"").decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
                    stderr = (exc.stderr or b"").decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
                wall_ms = (time.monotonic() - started) * 1000
                write_new(RAW / f"{tag}.json", stdout)
                write_new(RAW / f"{tag}.stderr", stderr)
                write_new(RAW / f"{tag}.exit", f"{code}\n")
                write_new(RAW / f"{tag}.wall-ms", f"{wall_ms:.6f}\n")
                try:
                    data = json.loads(stdout)
                    report = data["report"]
                    valid = (
                        code == 0 and data["planted_recovered"] is True
                        and report["verified"] is True and report["scalar"] == "12345"
                        and report["curve"] == curve and report["method"] == "bsgs"
                        and report["table_size"] > 0
                    )
                except (ValueError, KeyError, TypeError):
                    valid = False
                print(f"{tag} exit={code} valid={valid} wall_ms={wall_ms:.3f}", flush=True)
                if not valid:
                    raise SystemExit(f"invalid BSGS record: {tag}")
    write_new(ROOT / "COMPLETE", f"{sequence} verified process records\n")


if __name__ == "__main__":
    main()
