import json, re, subprocess, sys
req = json.load(open(sys.argv[1]))
img = json.load(open(sys.argv[2]))
frozen = sys.argv[3]
head = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout.strip()
status = subprocess.run(["git", "status", "--porcelain"], capture_output=True, text=True, check=True).stdout
checks = [
    ("git HEAD is 40 hex", re.fullmatch("[0-9a-f]{40}", head) is not None),
    ("git status --porcelain is empty", status == ""),
    ("request provenance.source_revision == image.json source_revision", req["provenance"]["source_revision"] == img["source_revision"]),
    ("image.json source_revision == frozen commit", img["source_revision"] == frozen),
    ("request job.image == image.json image_uri", req["job"]["image"] == img["image_uri"]),
]
print("HEAD", head)
print("request.provenance.source_revision", req["provenance"]["source_revision"])
print("image.source_revision", img["source_revision"])
print("request.job.image", req["job"]["image"])
print("image.image_uri", img["image_uri"])
if status:
    print("git status --porcelain:")
    print(status)
bad = 0
for name, ok in checks:
    print("OK  " if ok else "FAIL", name)
    bad += 0 if ok else 1
sys.exit(1 if bad else 0)
