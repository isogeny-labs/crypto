"""Pure proposed exact-diff parser; no execution or writer attribution."""
EXPECTED = frozenset({"/fixtures", "/qualification", "/qualification/exec-gate"})

def mount_diff(raw: bytes):
    if type(raw) is not bytes or len(raw) > 1024:
        raise ValueError("bytes/cap")
    try:
        text = raw.decode("ascii")
    except UnicodeDecodeError as error:
        raise ValueError("ASCII") from error
    if not text.endswith("\n") or text.endswith("\n\n"):
        raise ValueError("final newline")
    lines = text[:-1].split("\n")
    if len(lines) != 3 or len(set(lines)) != 3:
        raise ValueError("three unique changes required")
    if set(lines) != {"A " + path for path in EXPECTED}:
        raise ValueError("unexpected change")
    return {"schema": "software.exact-mount-diff/v1",
            "changes": [{"kind": "added", "path": path} for path in sorted(EXPECTED)],
            "writer_attribution": None, "no_writes_claim": False}
