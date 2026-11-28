import json, os, sys
frozen, root = sys.argv[1], sys.argv[2]
hits = sorted(os.path.join(d, f) for d, _, fs in os.walk(root) for f in fs if f == "input-spec.json")
if len(hits) != 1:
    print("STOP: input-spec.json files found:", len(hits), hits)
    sys.exit(1)
a = json.load(open(hits[0]))
b = json.load(open(frozen))
if a != b:
    print("STOP: SPEC_NOT_EQUAL", hits[0], frozen)
    sys.exit(1)
print("SPEC_EQUAL", hits[0], frozen)
