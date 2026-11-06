import re, subprocess, sys
b = sys.argv[1]; steps = sys.argv[2].split(",")
t = open("/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw/card-launch.md").read()
t = t.split("## Procedure W")[0]
parts = re.split(r"\n(?=Step \d)", t)
for p in parts:
    m = re.match(r"Step (\d)\.", p)
    if not m or m.group(1) not in steps: continue
    code = re.search(r"```\n(.*?)\n```", p, re.S).group(1).replace("__B__", b)
    print("#### STEP", m.group(1))
    r = subprocess.run(["bash", "-c", code], capture_output=True, text=True)
    print(r.stdout, end=""); print("STDERR:", r.stderr, end=""); print("EXIT", r.returncode)
