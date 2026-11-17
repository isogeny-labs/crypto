import json, subprocess, sys
img = json.load(open(sys.argv[1]))
digest = sys.argv[2]
if img["image_uri"].split("@")[1] != digest:
    print("FAIL image.json image_uri digest differs from", digest)
    sys.exit(1)
out = subprocess.run(["aws", "ecr", "describe-images", "--region", "us-east-2", "--repository-name", "isolab/tasks", "--image-ids", "imageDigest=" + digest, "--query", "imageDetails[0].imageDigest", "--output", "text"], capture_output=True, text=True)
print("aws exit", out.returncode)
print("aws stdout", out.stdout.strip())
if out.stderr.strip():
    print("aws stderr", out.stderr.strip())
if out.returncode != 0 or out.stdout.strip() != digest:
    print("FAIL digest not confirmed")
    sys.exit(1)
print("OK image digest present in ECR:", digest)
