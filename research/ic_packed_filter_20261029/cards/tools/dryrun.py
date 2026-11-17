"""Runs the numbered steps of a card's code blocks exactly as an executor would, with the dispatch values substituted, except that a step can be left out.
usage: python3 -I dryrun.py CARD_PATH SECTION STEPS B [UNIT=x] [WALL=x] [COUNTED=x] [SESSION=x]
  SECTION  the heading text that starts the procedure (for example 'Procedure L'), ending at the next '## Procedure' heading or the end of the section list
  STEPS    comma-separated step labels, for example 0,1,2 (the label is the text after 'Step ' up to the first '.')
Prints each step's output and exit status. Used by the director to test the cards; it launches nothing unless a launching step is named."""
import re, subprocess, sys

card, section, steps, b = sys.argv[1:5]
extra = dict(a.split("=", 1) for a in sys.argv[5:])
text = open(card).read()
i = text.index("## " + section)
m = re.search(r"\n## (?!#)", text[i + 3:])
body = text[i: i + 3 + (m.start() if m else len(text))]
parts = re.split(r"\n(?=Step [A-Z]?[0-9]+\.)", body)
want = steps.split(",")
for p in parts:
    mm = re.match(r"Step ([A-Z]?[0-9]+)\.", p)
    if not mm or mm.group(1) not in want:
        continue
    code = re.search(r"```\n(.*?)\n```", p, re.S).group(1)
    code = code.replace("__B__", b)
    for k, v in extra.items():
        code = code.replace("__%s__" % k, v)
    print("#### STEP", mm.group(1))
    r = subprocess.run(["bash", "-c", code], capture_output=True, text=True)
    print(r.stdout, end="")
    print("STDERR:", r.stderr, end="")
    print("EXIT", r.returncode)
