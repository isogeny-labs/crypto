"""Stage a private, exact context. No Docker or payload execution.
Only use with the reviewed local ledger and trusted source ancestors.
This is a single-writer snapshot, not protection against hostile concurrent mutation.
"""
import argparse, hashlib, json, os, stat, tempfile
from pathlib import Path, PurePosixPath
from datetime import datetime, timezone

def digest(b): return hashlib.sha256(b).hexdigest()
def read_file(p):
    fd = os.open(p, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, "rb") as f:
        st = os.fstat(f.fileno())
        if not stat.S_ISREG(st.st_mode): raise ValueError(f"not regular: {p}")
        if st.st_size > 16*1024*1024: raise ValueError("file limit")
        data = f.read(16*1024*1024+1)
        if len(data) != st.st_size: raise ValueError("unstable file length")
    return data, stat.S_IMODE(st.st_mode)
def inventory(root):
    out = []
    for p in sorted(root.rglob("*")):
        st = p.lstat()
        if stat.S_ISDIR(st.st_mode): kind, data = "directory", None
        elif stat.S_ISREG(st.st_mode): kind, data = "file", read_file(p)[0]
        else: raise ValueError(f"unexpected member: {p}")
        out.append(dict(context_path=str(p.relative_to(root)), kind=kind,
                        mode=oct(stat.S_IMODE(st.st_mode)),
                        bytes=None if data is None else len(data),
                        sha256=None if data is None else digest(data)))
    return out

def stage(ledger_path, recipe_path, recipe_sha256, output):
    ledger_bytes = ledger_path.read_bytes()
    ledger = json.loads(ledger_bytes)
    if ledger.get("errors") != []: raise ValueError("ledger errors")
    expected, payloads = {}, {}
    for m in ledger["members"]:
        rel = m["context_path"]
        q = PurePosixPath(rel)
        if q.is_absolute() or ".." in q.parts or str(q) != rel or rel in expected:
            raise ValueError("invalid or repeated context path")
        expected[rel] = {k:m[k] for k in ("context_path","kind","mode","bytes","sha256")}
        src = Path(m["source_path"])
        st = src.lstat()
        if stat.S_IMODE(st.st_mode) != int(m["mode"],8): raise ValueError(f"source mode: {rel}")
        if m["kind"] == "directory":
            if not stat.S_ISDIR(st.st_mode): raise ValueError("source directory")
        elif m["kind"] == "file":
            data, mode = read_file(src)
            if (len(data), digest(data), mode) != (m["bytes"], m["sha256"], int(m["mode"],8)):
                raise ValueError(f"source bytes or mode: {rel}")
            payloads[rel] = data
        else: raise ValueError("unknown member kind")
    recipe, mode = read_file(recipe_path)
    if digest(recipe) != recipe_sha256: raise ValueError("recipe digest")
    if "Dockerfile" in expected or "mountpoints" in expected: raise ValueError("reserved member")
    expected["Dockerfile"] = dict(context_path="Dockerfile",kind="file",mode="0o644",bytes=len(recipe),sha256=digest(recipe))
    expected["mountpoints"] = dict(context_path="mountpoints",kind="directory",mode="0o755",bytes=None,sha256=None)
    payloads["Dockerfile"] = recipe
    for rel in expected:
        for parent in PurePosixPath(rel).parents:
            if str(parent) != "." and (str(parent) not in expected or expected[str(parent)]["kind"] != "directory"):
                raise ValueError("undeclared parent")
    if sum(map(len,payloads.values())) > 32*1024*1024: raise ValueError("context byte limit")
    # tempfile's parent is private. No existing context or previous receipt is overwritten.
    private = Path(tempfile.mkdtemp(prefix="pencil-context-staging-",dir="/var/tmp"))
    root = private / "context"
    root.mkdir(mode=0o755)
    root.chmod(0o755)
    for rel, m in sorted(expected.items(),key=lambda kv:(len(PurePosixPath(kv[0]).parts),kv[0])):
        dst = root / rel
        if m["kind"] == "directory": dst.mkdir(mode=int(m["mode"],8))
        else:
            with dst.open("xb") as f: f.write(payloads[rel])
        dst.chmod(int(m["mode"],8))
    got = inventory(root)
    want = [expected[k] for k in sorted(expected)]
    if got != want: raise ValueError("staged inventory mismatch")
    result = dict(schema="runtime.staged-context/v1",observed_utc=datetime.now(timezone.utc).isoformat(),
        ledger_sha256=digest(ledger_bytes),recipe_sha256=digest(recipe),
        context_path=str(root),root_mode=oct(stat.S_IMODE(root.stat().st_mode)),
        private_parent_mode=oct(stat.S_IMODE(private.stat().st_mode)),members=got,
        member_count=len(got),file_bytes=sum(map(len,payloads.values())),
        context_staged=True,exact_inventory_match=True,
        docker_actions=0,payload_executions=0,scientific_execution=False,
        new_image_id=None,qualification_receipt=None,admitted=False,
        remaining=["exact base parent-path types/modes inspection", "bounded assembly and qualification admission", "independent review of completed bindings", "new image identity and unchanged qualifier checks"])
    with output.open("x") as f: json.dump(result,f,indent=2); f.write("\n")
    print(json.dumps({k:result[k] for k in ("context_path","member_count","file_bytes","exact_inventory_match")}))
if __name__ == "__main__":
    a=argparse.ArgumentParser(); a.add_argument("ledger",type=Path); a.add_argument("recipe",type=Path); a.add_argument("recipe_sha256"); a.add_argument("output",type=Path)
    args=a.parse_args(); stage(args.ledger,args.recipe,args.recipe_sha256,args.output)
