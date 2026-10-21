import json, sys
d = json.load(open(sys.argv[1]))
open(sys.argv[2], "w").write(json.dumps({"cell": d["cell"], "refuted": d["refuted"]}) + "\n")
