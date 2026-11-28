"""Capture only declared compiler/profile metadata, never environment values."""
from pathlib import Path
import json
import os

s = Path(__file__).resolve().parent
out = s / "COMPILER_V2.json"
if out.exists():
    raise SystemExit(0)
group = int((s / "raw/build-v2.pid").read_text())
for proc in Path("/proc").iterdir():
    if not proc.name.isdigit():
        continue
    try:
        pid = int(proc.name)
        if proc.stat().st_uid != os.getuid() or os.getpgid(pid) != group:
            continue
        args = (proc / "cmdline").read_bytes().decode().strip("\0").split("\0")
        if "--crate-name" not in args or args[args.index("--crate-name") + 1] != "crypto_lib":
            continue
        options = [args[i + 1] for i, a in enumerate(args[:-1]) if a == "-C"]
        options += [a[2:] for a in args if a.startswith("-C") and len(a) > 2]
        flags = [a for a in options if a.startswith(("opt-level=", "codegen-units=", "lto=", "debuginfo=", "embed-bitcode=", "panic="))]
        level = [a for a in flags if a.startswith("opt-level=")]
        libtest = "--test" in args
        receipt = {"schema": "observed-root-compiler-profile/v1", "pid": pid,
                   "process_group": group, "crate": "crypto_lib", "flags": flags,
                   "root_is_libtest": libtest,
                   "optimization_level": "0" if not level else level[-1].split("=", 1)[1],
                   "optimization_option_omitted_defaults_to_zero": not level,
                   "declared_profile_matches": all(a == "opt-level=0" for a in level) and "codegen-units=256" in flags and not libtest,
                   "environment_values_collected": False}
        with out.open("x") as f:
            json.dump(receipt, f, indent=2)
            f.write("\n")
        break
    except (FileNotFoundError, ProcessLookupError):
        continue
