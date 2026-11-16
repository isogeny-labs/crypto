"""P4 scan: reads each blob oid listed on stdin through `git cat-file --batch` in the repo given as argv[1]
and prints `oid needle` for every fixed needle (argv[2], one per line) found in the blob bytes.
Also prints the totals. usage: python3 -I p4scan.py REPO NEEDLEFILE < oids"""
import subprocess, sys

repo, needle_file = sys.argv[1], sys.argv[2]
needles = [l.strip().encode() for l in open(needle_file) if l.strip()]
oids = [l.strip() for l in sys.stdin if l.strip()]
p = subprocess.Popen(["git", "-C", repo, "cat-file", "--batch"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
n = 0
nbytes = 0
missing = 0
import threading


def feed():
    for o in oids:
        p.stdin.write((o + "\n").encode())
    p.stdin.close()


t = threading.Thread(target=feed)
t.start()
for o in oids:
    hdr = p.stdout.readline().split()
    if len(hdr) == 2 and hdr[1] == b"missing":
        missing += 1
        print("MISSING", o)
        continue
    size = int(hdr[2])
    data = p.stdout.read(size)
    p.stdout.read(1)
    n += 1
    nbytes += size
    for nd in needles:
        if nd in data:
            print("HIT", o, nd.decode())
t.join()
print("SCANNED_BLOBS", n, "BYTES", nbytes, "MISSING", missing)
