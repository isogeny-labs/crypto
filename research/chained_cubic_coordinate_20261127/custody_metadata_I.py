"""File/Git/hash/JSON custody only. The shell launches native Rust calls."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import os
import shutil
import subprocess
import sys
STUDY = Path(__file__).resolve().parent
ROOT = STUDY.parents[1]
LOCK = STUDY / "SOURCE_LOCK_I.json"

def write(path, value):
    if path.exists():
        raise RuntimeError("write-once record exists: " + str(path))
    path.write_text(json.dumps(value, indent=2) + "\n")

def digest(path):
    b = path.read_bytes()
    return {"bytes": len(b), "sha256": hashlib.sha256(b).hexdigest(),
            "mode": path.stat().st_mode & 0o777}

def now():
    return datetime.now(timezone.utc).isoformat()

def snap(out):
    lock = json.loads(LOCK.read_text())
    paths = list(lock["files"]) + [str(LOCK.relative_to(ROOT))]
    dep = json.loads((STUDY / "DEPENDENCY_INPUTS_H.json").read_text())
    built = out / "target/release/chained-cubic-identity-record-i"
    return {
        "original": {p: digest(ROOT / p) for p in paths},
        "copied": {p: digest(out / "source" / p) for p in paths},
        "tools": {p: digest(Path(p)) for p in lock["tools"]},
        "runtime": {p: digest(Path(p)) for p in lock["runtime"]},
        "dependency_originals": {p: digest(Path(p)) for p in dep["files"]},
        "built": digest(built) if built.exists() else None,
        "qualifier": digest(out / "qualifier") if (out / "qualifier").exists() else None,
    }

action = sys.argv[1]
out = Path(sys.argv[3] if action == "init" else sys.argv[2])
if action == "init":
    source = sys.argv[2]
    lock = json.loads(LOCK.read_text())
    dep = json.loads((STUDY / "DEPENDENCY_INPUTS_H.json").read_text())
    for path, expected in lock["files"].items():
        original = ROOT / path
        if digest(original) != expected:
            raise RuntimeError("original locked input differs: " + path)
        proc = subprocess.run(["git", "cat-file", "blob", source + ":" + path],
                              cwd=ROOT, capture_output=True)
        if proc.returncode or proc.stdout != original.read_bytes():
            write(out / "failed-git.json", {"path": path, "exit": proc.returncode,
                  "stdout_hex": proc.stdout.hex(), "stderr": proc.stderr.decode(errors="replace")})
            raise RuntimeError("whole published source differs: " + path)
        dest = out / "source" / path
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(original, dest)
    published = subprocess.check_output(["git", "cat-file", "blob",
        source + ":" + str(LOCK.relative_to(ROOT))], cwd=ROOT)
    if published != LOCK.read_bytes():
        raise RuntimeError("published source lock differs")
    shutil.copy2(LOCK, out / "source" / LOCK.relative_to(ROOT))
    for key in ["tools", "runtime"]:
        for path, expected in lock[key].items():
            if digest(Path(path)) != expected:
                raise RuntimeError("locked installed " + key + " differs: " + path)
    for path, expected in dep["files"].items():
        if digest(Path(path)) != expected:
            raise RuntimeError("whole official dependency input differs: " + path)
    write(out / "CONTEXT.json", {"recorded": now(), "source": source,
        "limits": lock["limits"], "classification": lock["classification"],
        "cargo_home": dep["cargo_home"],
        "environment": {k: os.environ.get(k) for k in ["RUSTUP_TOOLCHAIN", "RUSTC",
            "CARGO_HOME", "CARGO_BUILD_JOBS", "RAYON_NUM_THREADS", "RUSTFLAGS", "CHECKPOINT_SKIP"]},
        "boundary": "Same unsigned producer; exact official dependency archives/extracted bytes and canonical instrument. OS ambient; no timing/independent signature."})
    write(out / "BUILD_MANIFEST.json", snap(out))
elif action == "phase":
    name, status = sys.argv[3], int(sys.argv[4])
    write(out / (name + ".phase.json"), {"recorded": now(), "name": name,
        "exit": status, "expected_exit": 0,
        "stdout": digest(out / (name + ".stdout")),
        "stderr": digest(out / (name + ".stderr"))})
elif action == "pre":
    write(out / "PRE_CALL_MANIFEST.json", snap(out))
    write(out / "WHOLE_BUILD_OUTPUTS.json", {str(p.relative_to(out)): digest(p)
        for p in (out / "target").rglob("*") if p.is_file()})
elif action == "finish":
    status = int(sys.argv[3])
    if not (out / "BUILD_MANIFEST.json").exists():
        write(out / "POST_CALL_MANIFEST.json", {"recorded": now(), "outer_exit": status,
              "initialization_incomplete": True})
    else:
        before = json.loads((out / "BUILD_MANIFEST.json").read_text())
        post = snap(out)
        failures = [key for key in ["original", "copied", "tools", "runtime",
            "dependency_originals"] if before[key] != post[key]]
        if (out / "PRE_CALL_MANIFEST.json").exists():
            pre = json.loads((out / "PRE_CALL_MANIFEST.json").read_text())
            failures += [key for key in ["built", "qualifier"] if pre[key] != post[key]]
            outputs = json.loads((out / "WHOLE_BUILD_OUTPUTS.json").read_text())
            for path, expected in outputs.items():
                if digest(out / path) != expected:
                    failures.append("build-output:" + path)
        elif status == 0:
            failures.append("missing pre-call manifest")
        post.update({"recorded": now(), "outer_exit": status, "guard_failures": failures})
        write(out / "POST_CALL_MANIFEST.json", post)
        if failures:
            raise RuntimeError("post-call guards failed: " + repr(failures))
else:
    raise RuntimeError("unknown metadata action")
