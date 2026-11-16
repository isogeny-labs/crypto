import json, sys
proto = json.load(open(sys.argv[1]))
req = json.load(open(sys.argv[2]))
freeze = proto.get("freeze", {})
arms = [a for a in proto.get("arms", []) if a.get("id") == "ctl-m2-n10"]
expected = arms[0]["parameters"]["expected_D"] if len(arms) == 1 else None
checks = [
    ("protocol.json version == 2", proto.get("version") == 2),
    ("freeze.status == frozen", freeze.get("status") == "frozen"),
    ("freeze.protocol_sha256 == request provenance.protocol_sha256", freeze.get("protocol_sha256") == req["provenance"]["protocol_sha256"]),
    ("exactly one arm ctl-m2-n10", len(arms) == 1),
    ("ctl-m2-n10 parameters.expected_D == 4", expected == 4),
]
print("protocol version", proto.get("version"))
print("freeze.status", freeze.get("status"))
print("freeze.protocol_sha256", freeze.get("protocol_sha256"))
print("request.provenance.protocol_sha256", req["provenance"]["protocol_sha256"])
print("ctl-m2-n10 expected_D", expected)
bad = 0
for name, ok in checks:
    print("OK  " if ok else "FAIL", name)
    bad += 0 if ok else 1
sys.exit(1 if bad else 0)
