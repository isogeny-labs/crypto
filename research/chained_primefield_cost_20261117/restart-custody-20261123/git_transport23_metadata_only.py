"""Git-object transport metadata only; no scientific execution or arithmetic."""
from pathlib import Path
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)

def prepare(base, head):
    rows = []
    for raw in git("diff", "--raw", "--no-abbrev", "--no-renames", base, head).decode().splitlines():
        meta, path = raw.split("\t", 1)
        oldmode, mode, oldblob, blob, status = meta.split()
        if status == "D":
            raise RuntimeError("Deletion requires separate review: " + path)
        append = None
        if path in ("pcc/ledger.jsonl", "docs/session-logs/2026-11-23.md"):
            before = git("cat-file", "blob", base + ":" + path)
            after = git("cat-file", "blob", blob)
            if not after.startswith(before):
                raise RuntimeError("Canonical prefix changed: " + path)
            append = after[len(before):].decode()
        rows.append({"path": path, "mode": mode, "type": "blob", "sha": blob, "append": append})
    return {"base": base, "head": head, "base_tree": git("rev-parse", base + "^{tree}").decode().strip(), "tree": git("rev-parse", head + "^{tree}").decode().strip(), "files": rows}

if __name__ == "__main__":
    if sys.argv[1] == "prepare":
        value = prepare(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == "chunk":
        blob, offset = sys.argv[2], int(sys.argv[3])
        data = git("cat-file", "blob", blob)
        import base64
        value = {"chunk": base64.b64encode(data[offset:offset + 3000]).decode(), "done": len(data) <= offset + 3000}
    else:
        raise SystemExit("prepare BASE HEAD | chunk BLOB BYTE_OFFSET")
    print(json.dumps(value))
