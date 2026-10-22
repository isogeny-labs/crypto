#!/usr/bin/env python3
"""Independent validator recompute: item 14's negation-map matched rho (`rho-neg`).

Standard library only.  Reads committed raw data of snapshot 41155e5554 (card
CARD-BLINDSPOT-14-RHONEG-01) and writes, with --write, exactly two files next to
this script: rho-reference-neg.json and validator-report-rhoneg.json.

    python3 -I benchmarks/20261018-blindspots-prime-m4-cells/validator_recompute_rhoneg.py [--write]

What it does (validator, not producer; nothing here repairs or rewrites an artifact):
  1. binds every file read to the snapshot (git blob identity) and pins its sha256;
  2. checks the receipt against the frozen card (card sha, argv, exit codes, output,
     stdout and stderr digests, commit, dirty state, instrument sha) and the
     commit order addendum -> card -> run;
  3. checks the continuity controls (cont-* rows == recorded rows except `ms`);
  4. checks the curve facts behind `strongest_available` (prime field, a, b != 0,
     j != 0, 1728, prime order n = #E(F_p));
  5. re-runs every rho-neg and every continuity rho run with its own Python port of
     the instrument's rho walks (from the seed formulas in the addendum) and requires
     every emitted field except `ms` to match; also counter closure identities per row;
  6. recomputes rho-neg(w), plain rho(w), arm (a) S(w) at w = 0, 0.1, 1 (mean per run of
     gae + w * native, setup included) and both gaps, with 95% t intervals;
  7. a validator-side null control: the same port with the class map switched off.
"""
from __future__ import annotations

import hashlib
import json
import math
import platform
import re
import statistics
import subprocess
import sys
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
EVID = "benchmarks/20261018-blindspots-prime-m4-cells"
ITEM = "research/blindspots_20261018/14-prime-m4-cells"
RAW = ITEM + "/raw"
NEG = RAW + "/rho-neg"
INSTR = "research/blindspots_20261018/instruments/blindspots_14_prime_m4_neg.rs"
INSTR_ORIG = "research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs"
ADDENDUM = ITEM + "/PROTOCOL-ADDENDUM-rho-neg.md"
CARD = ITEM + "/rho-neg-cards/card-n01.yaml"
RECEIPT = NEG + "/receipt-n01.json"
SNAP = "41155e55547b49baa2f1d8b077724a0c49d9da76"
CARD_COMMIT = "84e5b2c8592a97fffe0245602d18139fc140c45b"
ADD_COMMIT = "9b2f5e1b36467d7b93c72c09072f68dff6f83d2e"
INSTR_SHA = "1a7d51c3adcd7108de335cd7ac3b2d83a11f5ef1246b8b959271ced1f5e8710e"
ORIG_SHA = "f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376"
REPORT_ID = "VAL-20261022-0549c2"
DESIGN = (16, 20, 24, 28)
WS = (Fraction(0), Fraction(1, 10), Fraction(1))
WK = ("0", "0.1", "1")
T15 = 2.131449545559323  # two-sided 95% Student t, 15 degrees of freedom
REL_TOL = 1e-12  # exact integer arithmetic on both sides; only float representation may differ

READ: dict[str, dict] = {}  # rel path -> {sha256, role}
GENERIC_ROLES = {"receipt output", "stdout log", "stderr log", "card-bound file"}
CHECKS: list[dict] = []


def die(msg):
    raise SystemExit(f"validator_recompute_rhoneg: {msg}")


def git(*args) -> str:
    return subprocess.run(["git", *args], cwd=REPO, check=True, capture_output=True, text=True).stdout


def sha256b(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def read(rel: str, role: str, at_snapshot: bool = True) -> bytes:
    """Read a repository file, pin its sha256, and require it to be the snapshot's blob."""
    data = (REPO / rel).read_bytes()
    if at_snapshot:
        blob = git("rev-parse", f"{SNAP}:{rel}").strip()
        here = git("hash-object", rel).strip()
        if blob != here:
            die(f"{rel} differs from snapshot {SNAP[:10]} ({here} != {blob})")
    if rel not in READ or READ[rel]["role"] in GENERIC_ROLES:
        READ[rel] = {"sha256": sha256b(data), "role": role}
    return data


def check(name, ok, detail):
    CHECKS.append({"check": name, "ok": bool(ok), "detail": detail})
    if not ok:
        print(f"CHECK FAILED: {name}: {detail}", file=sys.stderr)


def jsonl(rel, role):
    rows = [json.loads(l) for l in read(rel, role).decode().splitlines() if l.strip()]
    return rows


# ───────────────────────── 1-2. snapshot, commit order, receipt ─────────────────────────

def is_ancestor(a, b) -> bool:
    return subprocess.run(["git", "merge-base", "--is-ancestor", a, b], cwd=REPO).returncode == 0


def commit_time(c) -> str:
    return git("show", "-s", "--format=%cI", c).strip()


def parse_card(text: str) -> dict:
    """Minimal parser for this card's fixed layout (no YAML library in the stdlib)."""
    binds = re.findall(r"- \{path: (\S+), sha256: ([0-9a-f]{64})\}", text)
    mne_block = text.split("must_not_exist:")[1].split("commands:")[0]
    must_not_exist = re.findall(r"^\s+- (\S+)$", mne_block, re.M)
    cmds = []
    for chunk in text.split("commands:")[1].split("\nbus_address:")[0].split("  - id: ")[1:]:
        lines = chunk.splitlines()
        c = {"id": lines[0].strip()}
        for ln in lines[1:]:
            m = re.match(r"\s+(\w+): (.*)$", ln)
            if not m:
                continue
            k, v = m.group(1), m.group(2).strip()
            if k in ("argv", "success_exit_codes", "outputs"):
                c[k] = json.loads(v)
            elif k == "timeout_seconds":
                c[k] = int(v)
            else:
                c[k] = v
        cmds.append(c)
    ws = re.findall(r"write_scope:\n\s+- (\S+)", text)
    return {"binds": binds, "must_not_exist": must_not_exist, "commands": cmds, "write_scope": ws,
            "card_id": re.search(r"^card_id: (\S+)", text, re.M).group(1)}


def check_receipt():
    card_bytes = read(CARD, "frozen command card (CARD-BLINDSPOT-14-RHONEG-01)")
    card = parse_card(card_bytes.decode())
    rec = json.loads(read(RECEIPT, "execution receipt of card n01"))
    out = {}
    check("receipt card binding", rec["card_id"] == card["card_id"] and rec["card_path"] == CARD
          and rec["card_sha256"] == sha256b(card_bytes),
          f"card_id {rec['card_id']}, card_sha256 {rec['card_sha256']} == sha256(card-n01.yaml) "
          f"{sha256b(card_bytes) == rec['card_sha256']}")
    check("receipt status", rec["status"] == "completed" and rec["commands_total"] == len(rec["commands"]) == 17
          == len(card["commands"]), f"status {rec['status']}, {len(rec['commands'])} commands, card lists "
          f"{len(card['commands'])}")
    bad = []
    for i, (rc, cc) in enumerate(zip(rec["commands"], card["commands"])):
        if rc["id"] != cc["id"] or rc["argv"] != cc["argv"] or rc["index"] != i + 1:
            bad.append(f"{cc['id']}: argv/id/index differ")
        if rc["exit_code"] not in cc["success_exit_codes"] or not rc["succeeded"] or rc["timed_out"] \
                or rc["attempts"] != 1 or [a["exit_code"] for a in rc["attempt_log"]] != [rc["exit_code"]]:
            bad.append(f"{cc['id']}: exit {rc['exit_code']} attempts {rc['attempts']}")
        if sorted(rc["outputs"]) != sorted(cc.get("outputs", [])):
            bad.append(f"{cc['id']}: outputs differ from card")
        for o, dig in rc["outputs"].items():
            if sha256b(read(o, "receipt output")) != dig:
                bad.append(f"{o}: digest mismatch")
        for stream in ("stdout", "stderr"):
            path = cc.get(stream)
            want = rc[f"{stream}_sha256"]
            got = sha256b(read(path, f"{stream} log")) if path else sha256b(b"")
            if got != want:
                bad.append(f"{cc['id']}: {stream} digest mismatch")
    check("receipt commands vs card", not bad,
          "17/17 commands: id, argv and order equal the card; exit 0 in success_exit_codes, one attempt, "
          "no timeout; 12 output digests and 34 stdout/stderr digests equal sha256 of the committed files"
          if not bad else "; ".join(bad))
    out["exit_codes"] = {c["id"]: c["exit_code"] for c in rec["commands"]}
    out["seconds"] = {c["id"]: c["seconds"] for c in rec["commands"]}
    # commit, dirty state, instrument
    instr_at_commit = sha256b(subprocess.run(["git", "show", f"{rec['commit']}:{INSTR}"], cwd=REPO,
                                             check=True, capture_output=True).stdout)
    check("receipt commit and instrument", rec["commit"] == CARD_COMMIT and rec["dirty_before"] == []
          and instr_at_commit == INSTR_SHA and sha256b(read(INSTR, "instrument source")) == INSTR_SHA,
          f"commit {rec['commit']}, dirty_before {rec['dirty_before']}, instrument sha256 at that commit "
          f"{instr_at_commit}; working tree and snapshot identical")
    binds_bad = []
    for path, sha in card["binds"]:
        at = sha256b(subprocess.run(["git", "show", f"{CARD_COMMIT}:{path}"], cwd=REPO, check=True,
                                    capture_output=True).stdout)
        if at != sha or sha256b(read(path, "card-bound file")) != sha:
            binds_bad.append(path)
    check("card binds", not binds_bad and len(card["binds"]) == 9,
          f"{len(card['binds'])} bound files (instrument + 8 recorded raw files) hash to the card's sha256 at "
          f"the run commit and now" if not binds_bad else f"mismatch: {binds_bad}")
    tree_now = set(git("ls-tree", "-r", "--name-only", SNAP, "--", NEG + "/").split())
    tree_before = git("ls-tree", "-r", "--name-only", CARD_COMMIT, "--", NEG + "/").split()
    mne_before = [p for p in card["must_not_exist"]
                  if subprocess.run(["git", "cat-file", "-e", f"{CARD_COMMIT}:{p}"], cwd=REPO,
                                    capture_output=True).returncode == 0]
    check("dirty_after and write scope", set(rec["dirty_after"]) | {RECEIPT} == tree_now
          and all(p.startswith(card["write_scope"][0]) for p in rec["dirty_after"])
          and not tree_before and not mne_before and not any("/bin/" in p for p in tree_now),
          f"dirty_after lists {len(rec['dirty_after'])} files, all under {card['write_scope'][0]}; with the "
          f"receipt they are exactly the {len(tree_now)} files of raw/rho-neg at the snapshot; raw/rho-neg "
          f"absent at the run commit; no must_not_exist path existed there; bin/b14neg removed")
    # selftest
    st_lines = read(NEG + "/selftest.stdout.log", "selftest stdout").decode().splitlines()
    sel = next(c for c in rec["commands"] if c["id"] == "selftest")
    check("selftest", re.fullmatch(r"SELFTEST OK \(\d+\.\d+s\)", st_lines[-1]) is not None
          and not any(re.search(r"FAIL|panick|error", l, re.I) for l in st_lines)
          and (REPO / NEG / "selftest.stderr.log").stat().st_size == 0 and sel["stdout_tail"] == st_lines[-20:],
          f"{len(st_lines)} lines, last '{st_lines[-1]}', stderr empty, receipt stdout_tail equals the last 20 lines")
    # commit order: addendum committed before the run
    add_blobs = {c: git("rev-parse", f"{c}:{ADDENDUM}").strip() for c in (ADD_COMMIT, CARD_COMMIT, SNAP)}
    order_ok = (is_ancestor(ADD_COMMIT, CARD_COMMIT) and git("rev-parse", f"{SNAP}^").strip() == CARD_COMMIT
                and len(set(add_blobs.values())) == 1 and rec["commit"] == CARD_COMMIT
                and git("rev-parse", f"{ADD_COMMIT}:{INSTR}").strip() == git("rev-parse", f"{SNAP}:{INSTR}").strip())
    out["commit_times"] = {c[:10]: commit_time(c) for c in (ADD_COMMIT, CARD_COMMIT, SNAP)}
    check("addendum precedes the run", order_ok,
          f"addendum and instrument committed in {ADD_COMMIT[:10]} ({out['commit_times'][ADD_COMMIT[:10]]}), an "
          f"ancestor of the card commit {CARD_COMMIT[:10]} ({out['commit_times'][CARD_COMMIT[:10]]}) at which the "
          f"receipt says the card ran with a clean tree; outputs committed in {SNAP[:10]} "
          f"({out['commit_times'][SNAP[:10]]}), child of the card commit; the addendum blob is the same in all three")
    # the binary sha256 the addendum says the receipt records
    has_bin_sha = "b14neg" in json.dumps({k: v for k, v in rec.items() if k != "commands"}) or any(
        "sha256sum" in " ".join(c["argv"]) for c in rec["commands"])
    out["binary_sha256_recorded"] = has_bin_sha
    out["selftest_lines"] = st_lines
    out["card"] = card
    out["receipt"] = rec
    return out


# ───────────────────────── 3. continuity ─────────────────────────

def strip_ms(line: str) -> str:
    return re.sub(r',"ms":[0-9.]+', "", line)


def check_continuity():
    res = {}
    for arm, rec_name in (("rho", "rho-b{}.jsonl"), ("fullic-a", "fullic-a-b{}.jsonl")):
        for b in DESIGN:
            new_rel = f"{NEG}/cont-{arm}-b{b}.jsonl"
            old_rel = f"{RAW}/{rec_name.format(b)}"
            new_t = read(new_rel, f"continuity control output ({arm})").decode().splitlines()
            old_t = read(old_rel, f"recorded {arm} rows (card-01)").decode().splitlines()
            new = [json.loads(l) for l in new_t]
            old = [json.loads(l) for l in old_t]
            fields_ok = len(new) == len(old) == 16 and all(
                {k: v for k, v in x.items() if k != "ms"} == {k: v for k, v in y.items() if k != "ms"}
                and list(x) == list(y) for x, y in zip(new, old))
            bytes_ok = [strip_ms(l) for l in new_t] == [strip_ms(l) for l in old_t]
            ms_diff = sum(x["ms"] != y["ms"] for x, y in zip(new, old))
            nfields = sum(len(x) - 1 for x in new)
            res[f"{arm}-b{b}"] = {"rows": len(new), "fields_equal_except_ms": fields_ok,
                                  "bytes_equal_after_removing_ms": bytes_ok, "rows_with_different_ms": ms_diff,
                                  "fields_compared": nfields}
            check(f"continuity {arm} b{b}", fields_ok and bytes_ok,
                  f"16 rows, {nfields} top-level fields (nested counters compared whole) equal to {old_rel} in "
                  f"value and order except ms; byte-identical after removing ms; ms differs in {ms_diff} rows")
    return res


# ───────────────────────── 4. curve facts ─────────────────────────

def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def parse_cells(src: str) -> list[dict]:
    block = src.split("const CELLS: [Cell; 4] = [")[1].split("];")[0]
    cells = []
    for m in re.finditer(r"Cell \{(.*?)\}", block, re.S):
        kv = dict(re.findall(r"(\w+): (\d+)", m.group(1)))
        cells.append({k: int(v) for k, v in kv.items()})
    return cells


def check_curves(cells):
    out = []
    for c in cells:
        p, a, b, n = c["p"], c["a"], c["b"], c["n"]
        A, B = 4 * pow(a, 3, p) % p, 27 * b * b % p
        disc = (A + B) % p
        j = 1728 * A * pow(disc, -1, p) % p if disc else None
        cv = Curve(p, a, b)
        g = (c["gx"], c["gy"])
        on = (g[1] * g[1] - (g[0] ** 3 + a * g[0] + b)) % p == 0
        ng = cv.mul(n, g)
        hasse = abs(p + 1 - n) <= 2 * math.isqrt(p) + 2
        unique = n > 4 * math.isqrt(p) + 4  # one multiple of n in the Hasse interval
        r = {"bits": c["bits"], "p": p, "a": a, "b": b, "n": n, "p_prime": is_prime(p), "p_mod_4": p % 4,
             "a_nonzero": a % p != 0, "b_nonzero": b % p != 0, "discriminant_nonzero": disc != 0, "j": j,
             "j_not_0": j not in (None, 0), "j_not_1728": j not in (None, 1728 % p), "G_on_curve": on,
             "nG_is_identity": ng is None, "n_prime": is_prime(n), "n_in_hasse_interval": hasse,
             "order_equals_n": hasse and unique and is_prime(n) and ng is None, "anomalous": n == p}
        r["negation_is_strongest"] = all(r[k] for k in ("p_prime", "a_nonzero", "b_nonzero", "discriminant_nonzero",
                                                       "j_not_0", "j_not_1728", "order_equals_n")) and p > 3
        out.append(r)
        check(f"curve {c['bits']} bits", r["negation_is_strongest"],
              f"p={p} prime ({p % 4} mod 4), a, b != 0, disc != 0, j={j} (not 0, not 1728); G on curve, nG = O, "
              f"n={n} prime in the Hasse interval and > 4 sqrt p, so #E(F_p) = n (odd: no 2-torsion, classes "
              f"{{P,-P}} have size 2); Aut(E) = {{+-1}}; Frobenius is the identity on E(F_p); n != p")
    return out


# ───────────────────────── 5. Python port of the rho walks ─────────────────────────

M64 = (1 << 64) - 1
NAMES = ["gadd", "lookup", "insert", "legendre", "sqrt", "rowop", "madd", "dptest", "fmul", "fsqr",
         "finv", "build", "canon", "lookahead", "cyctest", "x15"]
(GADD, LOOKUP, INSERT, LEG, SQRT, ROWOP, MADD, HASHT, FMUL, FSQR, FINV, BUILD, CANON, LOOKA, CYCT) = range(15)
NATIVE = ("lookup", "insert", "legendre", "sqrt", "rowop", "madd", "dptest", "build",
          "canon", "lookahead", "cyctest")  # fmul, fsqr, finv are inside gadd


class Rng:
    __slots__ = ("s",)

    def __init__(self, s):
        self.s = s & M64

    def next(self):
        self.s = (self.s + 0x9E3779B97F4A7C15) & M64
        z = self.s
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
        return z ^ (z >> 31)

    def below(self, n):
        lim = M64 - (M64 % n)
        while True:
            v = self.next()
            if v < lim:
                return v % n


class Curve:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b
        self.ct = [0] * 16

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p, ct = self.p, self.ct
        ct[GADD] += 1
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1 % p, -1, p) % p
            ct[FMUL] += 2
            ct[FSQR] += 2
            ct[FINV] += 1
        else:
            lam = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
            ct[FMUL] += 2
            ct[FSQR] += 1
            ct[FINV] += 1
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        acc, base = None, P
        while k > 0:
            if k & 1:
                acc = self.add(acc, base)
            k >>= 1
            if k > 0:
                base = self.add(base, base)
        return acc

    def on_curve(self, P):
        return P is None or (P[1] * P[1] - (P[0] ** 3 + self.a * P[0] + self.b)) % self.p == 0


def part(x):
    return (((x * 0x9E3779B1) & M64) >> 9) % 16


def key(P):
    return M64 if P is None else (P[0] << 32) | P[1]


def make_jumps(cv, bits, n, g, q, rng):
    gp, qp = [], []
    x, y = g, q
    for _ in range(bits):
        gp.append(x)
        qp.append(y)
        x = cv.add(x, x)
        y = cv.add(y, y)
    out = []
    for _ in range(16):
        def pick():
            s = []
            while len(s) < 4:
                i = rng.below(bits)
                if i not in s:
                    s.append(i)
            return s
        sa = pick()
        sb = pick()
        al = be = 0
        t = None
        for i in sa:
            al += 1 << i
            t = cv.add(t, gp[i])
        for i in sb:
            be += 1 << i
            t = cv.add(t, qp[i])
        out.append((al % n, be % n, t))
    return out


def setup(cell, d, seed):
    cv = Curve(cell["p"], cell["a"], cell["b"])
    g = (cell["gx"], cell["gy"])
    q = cv.mul(d, g)  # the instance, not charged
    cv.ct = [0] * 16
    rng = Rng(seed)
    n = cell["n"]
    jumps = make_jumps(cv, cell["bits"], n, g, q, rng)
    theta = max(cell["bits"] // 4, 2)
    a = 1 + rng.below(n - 1)
    b = 1 + rng.below(n - 1)
    r = cv.add(cv.mul(a, g), cv.mul(b, q))
    return cv, g, q, rng, jumps, (1 << theta) - 1, 20 * (1 << theta) + 64, a, b, r


def rho_run(cell, d, seed):
    """Port of rho_run (plain r-adding walk)."""
    cv, g, q, rng, jumps, mask, cap, a, b, r = setup(cell, d, seed)
    n, ct = cell["n"], cv.ct
    setup_gadd = ct[GADD]
    table, walks, found = {}, 0, None
    while True:
        for _ in range(2):
            j = rng.below(16)
            r = cv.add(r, jumps[j][2])
            a = (a + jumps[j][0]) % n
            b = (b + jumps[j][1]) % n
        walks += 1
        wa, wb, wr = a, b, r
        steps, done = 0, False
        while wr is not None:
            j = part(wr[0])
            wr = cv.add(wr, jumps[j][2])
            wa = (wa + jumps[j][0]) % n
            wb = (wb + jumps[j][1]) % n
            steps += 1
            ct[HASHT] += 1
            if wr is not None and wr[0] & mask == 0:
                ct[LOOKUP] += 1
                if wr[0] in table:
                    pa, pb, py = table[wr[0]]
                    if py == wr[1]:
                        num, den = (pa + n - wa) % n, (wb + n - pb) % n
                    else:
                        num, den = (n - (wa + pa) % n) % n, (wb + pb) % n
                    if den != 0:
                        cand = num * pow(den, -1, n) % n
                        if cv.mul(cand, g) == q:
                            found, done = cand, True
                            break
                else:
                    ct[INSERT] += 1
                    table[wr[0]] = (wa, wb, wr[1])
                break
            if steps > cap:
                break
        if done or walks > 50_000_000:
            break
    return {"ok": found == d, "setup_gadd": setup_gadd, "walks": walks, "counts": list(ct)}


def rho_neg_run(cell, d, seed, lookahead=True, use_neg=True, check_level=0, audit=None):
    """Port of rho_neg_run.  use_neg=False is the validator's null control (class map off).
    check_level 1: canonical/on-curve after every transition and wa G + wb Q == w at every DP,
    escape and cycle detection; 2: the coefficient invariant after every transition too.
    The checks use a separate uncounted curve object."""
    cv, g, q, rng, jumps, mask, cap, a, b, r = setup(cell, d, seed)
    p, n, ct = cell["p"], cell["n"], cv.ct
    chk = Curve(cell["p"], cell["a"], cell["b"])
    setup_gadd = ct[GADD]
    st = dict(la_rejects=0, la_exhausted=0, cycles=0, two_cycles=0, cycle_len_sum=0, escape_gadd=0,
              abandoned=0)
    table, walks, steps_total, found = {}, 0, 0, None

    def canon(P):
        ct[CANON] += 1
        if P is None or not use_neg:
            return P, False
        x, y = P
        ny = p - y if y else 0
        return ((x, ny), True) if ny < y else (P, False)

    def neg_step(w, wa, wb):
        j0 = part(w[0])
        for t in range(16):
            j = (j0 + t) % 16
            c, ng = canon(cv.add(w, jumps[j][2]))
            ca, cb = (wa + jumps[j][0]) % n, (wb + jumps[j][1]) % n
            if ng:
                ca, cb = (n - ca) % n, (n - cb) % n
            if lookahead and ng and c is not None:
                ct[LOOKA] += 1
                if part(c[0]) == j:
                    st["la_rejects"] += 1
                    continue
            return c, ca, cb
        st["la_exhausted"] += 1
        c, ng = canon(cv.add(w, w))
        ca, cb = 2 * wa % n, 2 * wb % n
        if ng:
            ca, cb = (n - ca) % n, (n - cb) % n
        return c, ca, cb

    def solve(num, den):
        if den == 0:
            return None
        cand = num * pow(den, -1, n) % n
        return cand if cv.mul(cand, g) == q else None

    def invariant(w, wa, wb):
        return chk.add(chk.mul(wa, g), chk.mul(wb, q)) == w

    while True:
        for _ in range(2):
            j = rng.below(16)
            r = cv.add(r, jumps[j][2])
            a = (a + jumps[j][0]) % n
            b = (b + jumps[j][1]) % n
        walks += 1
        c0, ng0 = canon(r)
        wa, wb, wr = ((n - a) % n, (n - b) % n, c0) if ng0 else (a, b, c0)
        steps = 0
        sv, sva, svb = wr, wa, wb
        power, lam = 1, 0
        escape, done = None, False
        while wr is not None:
            if escape is not None:
                ln, escape = escape, None
                g0 = ct[GADD]
                mr, ma, mb = wr, wa, wb
                cr, ca, cb = wr, wa, wb
                for _ in range(1, ln):
                    cr, ca, cb = neg_step(cr, ca, cb)
                    ct[CYCT] += 1
                    if key(cr) < key(mr):
                        mr, ma, mb = cr, ca, cb
                c2, ng2 = canon(cv.add(mr, mr))
                na, nb = 2 * ma % n, 2 * mb % n
                if ng2:
                    na, nb = (n - na) % n, (n - nb) % n
                wr, wa, wb = c2, na, nb
                st["escape_gadd"] += ct[GADD] - g0
                escaped = True
            else:
                wr, wa, wb = neg_step(wr, wa, wb)
                steps += 1
                steps_total += 1
                escaped = False
            if check_level and audit is not None and wr is not None:
                audit["transitions"] += 1
                if use_neg and not (wr[1] <= p - wr[1]):
                    audit["noncanonical"] += 1
                if not cv.on_curve(wr):
                    audit["off_curve"] += 1
                if check_level >= 2 or escaped:
                    audit["invariant_checked"] += 1
                    audit["invariant_failed"] += not invariant(wr, wa, wb)
            ct[HASHT] += 1
            if wr is not None and wr[0] & mask == 0:
                ct[LOOKUP] += 1
                if check_level and audit is not None:
                    audit["invariant_checked"] += 1
                    audit["invariant_failed"] += not invariant(wr, wa, wb)
                if wr[0] in table:
                    pa, pb, py = table[wr[0]]
                    if py == wr[1]:
                        num, den = (pa + n - wa) % n, (wb + n - pb) % n
                    else:
                        num, den = (n - (wa + pa) % n) % n, (wb + pb) % n
                    cand = solve(num, den)
                    if cand is not None:
                        found, done = cand, True
                        break
                else:
                    ct[INSERT] += 1
                    table[wr[0]] = (wa, wb, wr[1])
                break
            if steps > cap:
                st["abandoned"] += 1
                break
            if escaped:
                sv, sva, svb = wr, wa, wb
                power, lam = 1, 0
                continue
            ct[CYCT] += 1
            lam += 1
            if wr == sv:
                if (wa, wb) != (sva, svb):
                    if audit is not None:
                        audit["useful_self_collisions"] += 1
                    cand = solve((sva + n - wa) % n, (wb + n - svb) % n)
                    if cand is not None:
                        found, done = cand, True
                        break
                elif audit is not None:
                    audit["fruitless_same_coefficients"] += 1
                    audit["cycle_lengths"][lam] = audit["cycle_lengths"].get(lam, 0) + 1
                st["cycles"] += 1
                st["cycle_len_sum"] += lam
                if lam == 2:
                    st["two_cycles"] += 1
                escape = lam
                continue
            if lam == power:
                sv, sva, svb = wr, wa, wb
                power *= 2
                lam = 0
        if done or walks > 50_000_000:
            break
    return {"ok": found == d, "setup_gadd": setup_gadd, "walks": walks, "counts": list(ct),
            "steps": steps_total, "st": st}


def counts_json(ct):
    return {NAMES[i]: ct[i] for i in range(16) if i < CANON or ct[i] != 0}


def native_of(counts: dict) -> int:
    return sum(counts.get(k, 0) for k in NATIVE)


def target_rho(bits, n, run, seed=20261018):
    return 1 + Rng(seed ^ (bits << 32) ^ 0x4410 ^ ((run * 0x9E3779B9) & M64)).below(n - 1)


def target_neg(bits, n, run, seed=20261018):
    return 1 + Rng(seed ^ (bits << 32) ^ 0x4E47 ^ ((run * 0x9E3779B9) & M64)).below(n - 1)


def seed_rho(run, seed=20261018):
    return seed + run * 104729 + 3


def seed_neg(run, seed=20261018):
    return seed + run * 104729 + 0x4E4547


def f1(x):
    return float(f"{x:.1f}")


def row_neg(cell, run, res):
    n = cell["n"]
    c = counts_json(res["counts"])
    st = res["st"]
    return {"kind": "rho-neg", "bits": cell["bits"], "run": run, "ok": res["ok"], "setup_gadd": res["setup_gadd"],
            "walks": res["walks"], "counts": c, "gae": res["counts"][GADD], "native": native_of(c), "n": n,
            "sqrt_pi_n_over_2": f1(math.sqrt(math.pi * n / 2.0)),
            "sqrt_pi_n_over_4": f1(math.sqrt(math.pi * n / 4.0)), "lookahead": True, "steps": res["steps"],
            "la_rejects": st["la_rejects"], "la_exhausted": st["la_exhausted"], "fruitless_cycles": st["cycles"],
            "fruitless_2cycles": st["two_cycles"], "cycle_len_sum": st["cycle_len_sum"],
            "escape_gadd": st["escape_gadd"], "abandoned_walks": st["abandoned"]}


def row_rho(cell, run, res):
    n = cell["n"]
    c = counts_json(res["counts"])
    return {"kind": "rho", "bits": cell["bits"], "run": run, "ok": res["ok"], "setup_gadd": res["setup_gadd"],
            "walks": res["walks"], "counts": c, "gae": res["counts"][GADD], "native": native_of(c), "n": n,
            "sqrt_pi_n_over_2": f1(math.sqrt(math.pi * n / 2.0))}


def new_audit():
    return {"transitions": 0, "noncanonical": 0, "off_curve": 0, "invariant_checked": 0, "invariant_failed": 0,
            "useful_self_collisions": 0, "fruitless_same_coefficients": 0, "cycle_lengths": {}}


def verify_cost(k: int) -> int:
    """Counted additions of mul(k, G): bitlen-1 doublings and popcount-1 additions."""
    return (k.bit_length() - 1) + (bin(k).count("1") - 1)


def reimplement(cells_by_bits):
    out = {"rho-neg": {}, "cont-rho": {}}
    audit_tot = new_audit()
    for b in DESIGN:
        cell = cells_by_bits[b]
        rows = jsonl(f"{NEG}/rho-neg-b{b}.jsonl", "rho-neg rows (measured reference)")
        mism, closure_bad = [], []
        level = 2 if b <= 20 else 1
        for row in rows:
            run = row["run"]
            d = target_neg(b, cell["n"], run)
            res = rho_neg_run(cell, d, seed_neg(run), check_level=level, audit=audit_tot)
            mine = row_neg(cell, run, res)
            theirs = {k: v for k, v in row.items() if k != "ms"}
            if mine != theirs or list(mine) != list(theirs):
                mism.append({"run": run, "fields": sorted(k for k in theirs if mine.get(k) != theirs.get(k))})
            # counter closure identities (from the algorithm; independent of the port)
            c = row["counts"]
            ver = c["gadd"] - row["setup_gadd"] - 2 * row["walks"] - row["steps"] - row["cycle_len_sum"] \
                - row["la_rejects"]
            e_lo = row["steps"] - c["lookup"] - row["abandoned_walks"] + row["cycle_len_sum"] - row["fruitless_cycles"]
            conds = {
                "gae == counts.gadd": row["gae"] == c["gadd"],
                "native == native counters": row["native"] == native_of(c),
                "other native counters zero": all(c[k] == 0 for k in ("legendre", "sqrt", "rowop", "madd", "build")),
                "dptest == steps + fruitless_cycles": c["dptest"] == row["steps"] + row["fruitless_cycles"],
                "canon == walks + steps + cycle_len_sum + la_rejects":
                    c["canon"] == row["walks"] + row["steps"] + row["cycle_len_sum"] + row["la_rejects"],
                "gadd - setup - 2 walks - steps - cycle_len_sum - la_rejects == verification of d":
                    ver == verify_cost(d),
                "lookup + abandoned in {walks, walks - 1}":
                    row["walks"] - c["lookup"] - row["abandoned_walks"] in (0, 1),
                "insert <= lookup - (1 if solved at a DP)":
                    c["insert"] <= c["lookup"] - (1 if row["walks"] - c["lookup"] - row["abandoned_walks"] == 0 else 0),
                "cyctest in closure bounds": e_lo <= c["cyctest"] <= e_lo + min(row["fruitless_cycles"], c["lookup"]),
                "la_rejects <= lookahead <= canon": row["la_rejects"] <= c["lookahead"] <= c["canon"],
                "escape_gadd >= cycle_len_sum": row["escape_gadd"] >= row["cycle_len_sum"],
                "setup_gadd <= gadd": row["setup_gadd"] <= c["gadd"],
                "ok": row["ok"] is True,
            }
            if not all(conds.values()):
                closure_bad.append({"run": run, "failed": [k for k, v in conds.items() if not v]})
        out["rho-neg"][b] = {"rows": len(rows), "mismatched_rows": mism, "closure_failures": closure_bad,
                             "invariant_level": level}
        check(f"rho-neg b{b}: Python port reproduces every row", not mism and len(rows) == 16,
              f"16/16 rows equal in every field except ms (targets d and walk seeds from the addendum's formulas)"
              if not mism else f"mismatches {mism}")
        check(f"rho-neg b{b}: counter closure identities", not closure_bad,
              "per row: gae == gadd; native == lookup+insert+dptest+canon+lookahead+cyctest; legendre, sqrt, rowop, "
              "madd, build zero; dptest == steps + fruitless_cycles; canon == walks + steps + cycle_len_sum + "
              "la_rejects; gadd - setup_gadd - 2 walks - steps - cycle_len_sum - la_rejects == the counted cost of "
              "one verification mul(d, G) for the addendum's target d; cyctest within its closure bounds; ok"
              if not closure_bad else f"failures {closure_bad}")
        crows = jsonl(f"{NEG}/cont-rho-b{b}.jsonl", "continuity control output (rho)")
        cm = []
        for row in crows:
            run = row["run"]
            res = rho_run(cell, target_rho(b, cell["n"], run), seed_rho(run))
            mine = row_rho(cell, run, res)
            theirs = {k: v for k, v in row.items() if k != "ms"}
            if mine != theirs or list(mine) != list(theirs):
                cm.append(run)
        out["cont-rho"][b] = {"rows": len(crows), "mismatched_rows": cm}
        check(f"plain rho b{b}: Python port reproduces every continuity row", not cm,
              "16/16 rows equal except ms (rho seeds: tag 0x4410, walk seed + 3)" if not cm else f"runs {cm}")
    a = audit_tot
    check("rho-neg walk semantics (port, all 64 measured runs)",
          a["noncanonical"] == 0 and a["off_curve"] == 0 and a["invariant_failed"] == 0,
          f"{a['transitions']} transitions: every walk point canonical (y <= p - y) and on the curve; coefficient "
          f"invariant wa G + wb Q == w checked {a['invariant_checked']} times (every transition at 16 and 20 bits, "
          f"every DP and escape at 24 and 28 bits), {a['invariant_failed']} failures; detected cycles with equal "
          f"coefficients (fruitless) {a['fruitless_same_coefficients']}, lengths "
          f"{dict(sorted(a['cycle_lengths'].items()))}; useful self-collisions (solved inside a walk) "
          f"{a['useful_self_collisions']}")
    out["audit"] = dict(a)
    out["audit"]["cycle_lengths"] = {str(k): v for k, v in sorted(a["cycle_lengths"].items())}
    return out


# ───────────────────────── 6. costs ─────────────────────────

def fmean(xs):
    return Fraction(sum(xs), len(xs))


def band(rows, gae_key, nat_fn):
    gae = [gae_key(r) for r in rows]
    nat = [nat_fn(r) for r in rows]
    out = {}
    for w, k in zip(WS, WK):
        per = [Fraction(g) + w * x for g, x in zip(gae, nat)]
        m = fmean(per)
        sd = statistics.stdev([float(v) for v in per])
        se = sd / math.sqrt(len(per))
        out[k] = {"mean": m, "sd": sd, "se": se, "lo": float(m) - T15 * se, "hi": float(m) + T15 * se}
    return out


def costs():
    res = {}
    for b in DESIGN:
        neg = jsonl(f"{NEG}/rho-neg-b{b}.jsonl", "rho-neg rows (measured reference)")
        rho = jsonl(f"{RAW}/rho-b{b}.jsonl", "recorded rows (card-01)")
        ic = jsonl(f"{RAW}/fullic-a-b{b}.jsonl", "recorded rows (card-01)")
        assert len(neg) == len(rho) == len(ic) == 16
        assert all(r["bits"] == b and r["kind"] == "rho-neg" for r in neg)
        assert all(r["bits"] == b and r["kind"] == "rho" and r["gae"] == r["counts"]["gadd"] for r in rho)
        assert all(r["bits"] == b and r["arm"] == "a" for r in ic)
        ic_ok = all(r["gae_total"] == r["total"]["gadd"] and r["native_total"] == native_of(r["total"])
                    and all(r["total"][k] == r["setup"][k] + r["online"][k] + r["verify"][k] for k in r["total"])
                    for r in ic)
        rho_ok = all(r["native"] == native_of(r["counts"]) and r["setup_gadd"] <= r["gae"] for r in rho)
        check(f"cost fields b{b}", ic_ok and rho_ok,
              "arm (a): gae_total == total.gadd, native_total == lookup+insert+legendre+sqrt+rowop+madd+dptest+build "
              "of total, total == setup + online + verify per counter; plain rho: native == lookup+insert+dptest, "
              "setup_gadd <= gae")
        # claimed = emitted totals; recomputed = from the per-counter fields
        cl = {"neg": band(neg, lambda r: r["gae"], lambda r: r["native"]),
              "rho": band(rho, lambda r: r["gae"], lambda r: r["native"]),
              "S": band(ic, lambda r: r["gae_total"], lambda r: r["native_total"])}
        rc = {"neg": band(neg, lambda r: r["counts"]["gadd"], lambda r: native_of(r["counts"])),
              "rho": band(rho, lambda r: r["counts"]["gadd"], lambda r: native_of(r["counts"])),
              "S": band(ic, lambda r: r["total"]["gadd"], lambda r: native_of(r["total"]))}
        cell = {"bits": b, "n": neg[0]["n"], "bx": ic[0]["bx"],
                "ok": {"rho_neg": sum(r["ok"] for r in neg), "rho": sum(r["ok"] for r in rho),
                       "ic_a": sum(r["ok"] for r in ic)},
                "claimed": cl, "recomputed": rc}
        # diagnostics
        e4 = math.sqrt(math.pi * neg[0]["n"] / 4.0)
        e2 = math.sqrt(math.pi * neg[0]["n"] / 2.0)
        cell["neg_mean_steps"] = statistics.mean(r["steps"] for r in neg)
        cell["neg_steps_over_sqrt_pi_n_4"] = cell["neg_mean_steps"] / e4
        cell["rho_mean_steps"] = statistics.mean(r["counts"]["dptest"] for r in rho)
        cell["rho_steps_over_sqrt_pi_n_2"] = cell["rho_mean_steps"] / e2
        cell["online_gae_plain_over_neg"] = (statistics.mean(r["gae"] - r["setup_gadd"] for r in rho)
                                             / statistics.mean(r["gae"] - r["setup_gadd"] for r in neg))
        cell["neg_native_per_step"] = (statistics.mean(native_of(r["counts"]) for r in neg)
                                       / cell["neg_mean_steps"])
        cell["neg_totals"] = {k: sum(r[k] for r in neg) for k in
                              ("walks", "steps", "la_rejects", "la_exhausted", "fruitless_cycles",
                               "fruitless_2cycles", "cycle_len_sum", "escape_gadd", "abandoned_walks")}
        cell["neg_totals"]["gadd"] = sum(r["counts"]["gadd"] for r in neg)
        cell["neg_mean_setup_gadd"] = statistics.mean(r["setup_gadd"] for r in neg)
        cell["rho_mean_setup_gadd"] = statistics.mean(r["setup_gadd"] for r in rho)
        cell["ic_mean_setup_gadd"] = statistics.mean(r["setup"]["gadd"] for r in ic)
        res[b] = cell
    return res


# ───────────────────────── 7. null control (validator-side) ─────────────────────────

def null_control(cells_by_bits):
    """The class map is what should halve the class space; switching it off must bring the
    accepted steps back from about sqrt(pi n / 4) to about sqrt(pi n / 2).  Paired seeds."""
    out = {}
    plan = {16: 400, 20: 200, 24: 16, 28: 16}
    for b, runs in plan.items():
        cell = cells_by_bits[b]
        n = cell["n"]
        on, off = [], []
        for run in range(runs):
            if runs == 16:  # the measured targets and walk seeds
                d, s = target_neg(b, n, run), seed_neg(run)
            else:
                d, s = 1 + Rng(0x56414C ^ (b << 32) ^ ((run * 0x9E3779B9) & M64)).below(n - 1), 777_001 + 13 * run
            r1 = rho_neg_run(cell, d, s, use_neg=True)
            r0 = rho_neg_run(cell, d, s, use_neg=False)
            assert r1["ok"] and r0["ok"]
            on.append(r1["steps"])
            off.append(r0["steps"])
        e4 = math.sqrt(math.pi * n / 4.0)
        out[b] = {"runs": runs, "seeds": "measured rho-neg targets and walk seeds" if runs == 16 else
                  "validator seeds (target tag 0x56414C, walk seed 777001 + 13 run)",
                  "steps_on_over_sqrt_pi_n_4": statistics.mean(on) / e4,
                  "steps_off_over_sqrt_pi_n_4": statistics.mean(off) / e4,
                  "off_over_on": statistics.mean(off) / statistics.mean(on)}
    return out


# ───────────────────────── report assembly ─────────────────────────

def ff(x):
    return float(x)


def main(argv):
    write = "--write" in argv
    read("docs/VALIDATION_GATE.md", "gate schema (policy, read for the schema)", at_snapshot=True)
    read("scripts/validation_gate.py", "gate module (check_rho_reference, check_validator_report)", at_snapshot=True)
    read(f"{EVID}/rho-reference.json", "earlier rho reference (plain rho; strongest_available false)")
    read(f"{EVID}/validator-report.json", "earlier validator report (plain-rho numbers cross-checked)")
    read(f"{EVID}/validator_recompute.py", "earlier validator script (grepped for the cost definition only)")
    read(ADDENDUM, "protocol addendum B (committed before the run)")
    src = read(INSTR, "instrument source (rho-neg)").decode()
    orig = read(INSTR_ORIG, "original instrument (diffed against the copy)").decode()
    check("original instrument untouched", sha256b(orig.encode()) == ORIG_SHA, f"sha256 {ORIG_SHA}")
    cells = parse_cells(src)
    check("cells unchanged in the copy", cells == parse_cells(orig) and [c["bits"] for c in cells] == list(DESIGN),
          "CELLS block parses identically from both instrument files")
    cells_by_bits = {c["bits"]: c for c in cells}

    rcpt = check_receipt()
    cont = check_continuity()
    curves = check_curves(cells)
    reimpl = reimplement(cells_by_bits)
    cost = costs()
    null = null_control(cells_by_bits)

    # addendum pre-run expectations vs the measured build's selftest log
    add_text = " ".join((REPO / ADDENDUM).read_text().split())
    add_lines = (REPO / ADDENDUM).read_text()
    st = "\n".join(rcpt["selftest_lines"])
    exp_rows = re.findall(r"^\| (\d+) \| (\d+) \| ([0-9.]+) \|$", add_lines, re.M)
    st_ratio = dict(re.findall(r"rho-neg (\d+) bits, \d+ runs: mean steps [0-9.]+ = ([0-9.]+) x", st))
    st_pg = dict(re.findall(r"rho-neg (\d+) bits, \d+ runs: .*plain/neg online GAE ([0-9.]+)", st))
    pg_add = re.search(r"Plain-over-negation online GAE was ([0-9.]+), ([0-9.]+) and ([0-9.]+)", add_text).groups()
    pre_ok = (len(exp_rows) == 3 and all(st_ratio.get(bb) == r for bb, _, r in exp_rows)
              and [st_pg.get(x) for x in ("16", "20", "24")] == list(pg_add)
              and "140 cycles (4 of length 2)" in st and "574 escape GAE" in st
              and "289 fruitless 2-cycles detected and escaped, 50/50 solved" in st
              and "140 fruitless cycles (4 of length 2), with 574 escape GAE" in add_text
              and "289 2-cycles were detected and escaped, and all 50 logarithms were recovered" in add_text)
    check("addendum expectations reproduce in the measured build's selftest", pre_ok,
          f"addendum table {exp_rows}, selftest {st_ratio}; plain/neg online GAE addendum {pg_add}, selftest "
          f"{st_pg}; 140 cycles (4 of length 2), 574 escape GAE, 289 2-cycles 50/50: present in both")

    # gaps and order
    cells_out, heads = {}, []
    for b in DESIGN:
        c = cost[b]
        cl, rc = c["claimed"], c["recomputed"]
        per = {}
        for k in WK:
            S, N, R = rc["S"][k]["mean"], rc["neg"][k]["mean"], rc["rho"][k]["mean"]
            per[k] = {"S_a": ff(S), "rho_neg": ff(N), "rho_plain": ff(R), "gap_S_over_rho_neg": ff(S / N),
                      "gap_S_over_rho_plain": ff(S / R), "rho_plain_over_rho_neg": ff(R / N),
                      "gap_S_over_cheaper_rho": ff(S / min(N, R)),
                      "cheaper_rho": "rho_neg" if N < R else "rho_plain",
                      "ci95": {"S_a": [rc["S"][k]["lo"], rc["S"][k]["hi"]],
                               "rho_neg": [rc["neg"][k]["lo"], rc["neg"][k]["hi"]],
                               "rho_plain": [rc["rho"][k]["lo"], rc["rho"][k]["hi"]],
                               "gap_S_over_rho_neg_conservative": [rc["S"][k]["lo"] / rc["neg"][k]["hi"],
                                                                   rc["S"][k]["hi"] / rc["neg"][k]["lo"]],
                               "gap_S_over_rho_plain_conservative": [rc["S"][k]["lo"] / rc["rho"][k]["hi"],
                                                                     rc["S"][k]["hi"] / rc["rho"][k]["lo"]]}}
        neg_dir = {per[k]["S_a"] > per[k]["rho_neg"] for k in WK}
        pl_dir = {per[k]["S_a"] > per[k]["rho_plain"] for k in WK}
        rr_dir = {per[k]["rho_neg"] < per[k]["rho_plain"] for k in WK}
        ci_sep = all(per[k]["ci95"]["S_a"][0] > max(per[k]["ci95"]["rho_neg"][1], per[k]["ci95"]["rho_plain"][1])
                     for k in WK)
        cells_out[b] = {"bits": b, "n": c["n"], "bx_arm_a": c["bx"], "runs": 16, "ok": c["ok"], "band": per,
                        "order_rho_neg_cheaper_than_arm_a_all_w": neg_dir == {True},
                        "order_rho_plain_cheaper_than_arm_a_all_w": pl_dir == {True},
                        "order_holds_against_both": neg_dir == {True} and pl_dir == {True},
                        "ci95_separated_at_all_w": ci_sep,
                        "rho_neg_vs_rho_plain": ("rho_neg cheaper at all w" if rr_dir == {True} else
                                                 "rho_plain cheaper at all w" if rr_dir == {False} else
                                                 "unresolved at the boundary (order flips inside the band)"),
                        "diagnostics": {k: c[k] for k in (
                            "neg_mean_steps", "neg_steps_over_sqrt_pi_n_4", "rho_mean_steps",
                            "rho_steps_over_sqrt_pi_n_2", "online_gae_plain_over_neg", "neg_native_per_step",
                            "neg_totals", "neg_mean_setup_gadd", "rho_mean_setup_gadd", "ic_mean_setup_gadd")}}
        for k, w in zip(WK, ("0", "0.1", "1")):
            for label, arm in (("rho-neg", "neg"), ("plain rho", "rho"), ("arm (a) full IC S", "S")):
                heads.append({"name": f"{b} bits: {label}(w={w}) (mean per run of gae + w*native, setup included)",
                              "claimed": ff(cl[arm][k]["mean"]), "recomputed": ff(rc[arm][k]["mean"]),
                              "rel_tol": REL_TOL})
            heads.append({"name": f"{b} bits: gap S/rho-neg at w={w}",
                          "claimed": ff(cl["S"][k]["mean"] / cl["neg"][k]["mean"]),
                          "recomputed": ff(rc["S"][k]["mean"] / rc["neg"][k]["mean"]), "rel_tol": REL_TOL})
            heads.append({"name": f"{b} bits: gap S/plain-rho at w={w}",
                          "claimed": ff(cl["S"][k]["mean"] / cl["rho"][k]["mean"]),
                          "recomputed": ff(rc["S"][k]["mean"] / rc["rho"][k]["mean"]), "rel_tol": REL_TOL})
    # the port re-ran every rho-neg run: its means equal the recomputed ones exactly when all rows matched
    port_ok = all(not reimpl["rho-neg"][b]["mismatched_rows"] for b in DESIGN)
    for h in heads:
        if h["name"].split(": ")[1].startswith("rho-neg(") and port_ok:
            h["reimplemented"] = h["recomputed"]
    # cross-check with the earlier validator's plain rho and S numbers
    prev = json.loads((REPO / EVID / "validator-report.json").read_text())
    prev_vals = {h["name"]: h["recomputed"] for h in prev["headline"]}
    xbad = []
    for b in DESIGN:
        for k, w in zip(WK, ("0", "0.1", "1")):
            pr = prev_vals.get(f"matched rho {b} bits: rho({w})")
            ps = prev_vals.get(f"arm (a) full IC {b} bits: S({w})")
            if pr is None or ps is None or abs(pr - cells_out[b]["band"][k]["rho_plain"]) > 1e-9 * pr \
                    or abs(ps - cells_out[b]["band"][k]["S_a"]) > 1e-9 * ps:
                xbad.append((b, w))
    check("agreement with the earlier validator (plain rho(w), S(w))", not xbad,
          "24 values equal to VAL-20261022-e873ba's recomputed numbers" if not xbad else f"differ at {xbad}")

    order_all = all(cells_out[b]["order_holds_against_both"] for b in DESIGN)
    all_ok = all(c["ok"] for c in CHECKS)
    max_rel = max(abs(h["claimed"] - h["recomputed"]) / max(abs(h["claimed"]), 1e-300) for h in heads)

    summary = {"checks_passed": sum(c["ok"] for c in CHECKS), "checks": len(CHECKS), "all_ok": all_ok,
               "order_holds_all_cells_both_rhos": order_all, "max_rel_diff_claimed_vs_recomputed": max_rel,
               "binary_sha256_recorded": rcpt["binary_sha256_recorded"]}
    print(json.dumps(summary, indent=1))
    for b in DESIGN:
        for k in WK:
            x = cells_out[b]["band"][k]
            print(f"{b} w={k:>3}: rho_neg {x['rho_neg']:12.4f} rho_plain {x['rho_plain']:12.4f} S {x['S_a']:14.4f} "
                  f"S/neg {x['gap_S_over_rho_neg']:8.4f} S/plain {x['gap_S_over_rho_plain']:8.4f} "
                  f"plain/neg {x['rho_plain_over_rho_neg']:.4f}")
        print(f"   {cells_out[b]['rho_neg_vs_rho_plain']}; steps/sqrt(pi n/4) "
              f"{cells_out[b]['diagnostics']['neg_steps_over_sqrt_pi_n_4']:.3f}; null {null[b]}")
    if not write:
        return 0 if all_ok else 1
    cost_hw = {arm: max(T15 * cost[b]["recomputed"][arm][k]["se"] / float(cost[b]["recomputed"][arm][k]["mean"])
                        for b in DESIGN for k in WK) for arm in ("neg", "rho", "S")}
    build_outputs(rcpt, cont, curves, reimpl, cells_out, heads, null, order_all, all_ok, max_rel, cost_hw)
    return 0 if all_ok else 1


def build_outputs(rcpt, cont, curves, reimpl, cells_out, heads, null, order_all, all_ok, max_rel, cost_hw):
    c28 = cells_out[28]["band"]
    neg_cheaper = where(cells_out, lambda x: x["rho_neg"] < x["rho_plain"])
    plain_cheaper = where(cells_out, lambda x: x["rho_plain"] < x["rho_neg"])
    rel = lambda p: "../../" + p  # paths relative to the evidence directory
    binary = (f"{NEG}/bin/b14neg: one rustc build (prlimit --as=4294967296 rustc --edition 2021 -C opt-level=3 "
              f"-C codegen-units=1 -C target-cpu=native) of {INSTR} (sha256 {INSTR_SHA}, committed in "
              f"{ADD_COMMIT[:10]}), command 'build' (index 2) of card CARD-BLINDSPOT-14-RHONEG-01 "
              f"({CARD}, sha256 {rcpt['receipt']['card_sha256']}), run at commit {CARD_COMMIT} with dirty_before "
              f"empty; receipt {RECEIPT} (sha256 {READ[RECEIPT]['sha256']}), outputs committed in {SNAP[:10]}. From "
              f"that one binary, as separate processes: selftest --deep (3), continuity controls cont-rho-b16..b28 "
              f"(4-7) and cont-fullic-a-b16..b28 (8-11), rho-neg-b16..b28 (12-15); remove-bin (16). Continuity: "
              f"cont-rho-bB.jsonl equals raw/rho-bB.jsonl and cont-fullic-a-bB.jsonl equals raw/fullic-a-bB.jsonl in "
              f"every field except ms at B = 16, 20, 24, 28 (validator-checked), so arm (a)'s recorded rows (card-01 "
              f"binary, built from {INSTR_ORIG}, sha256 {ORIG_SHA[:12]}...) are what this binary computes, and the "
              f"gap uses them. The binary's own sha256 is not recorded (deleted by remove-bin; the receipt has no "
              f"binary digest).")
    ref = {
        "schema": "validation-gate/v1 rho_reference",
        "descriptor_for": "the rho_reference key of a VALIDATION.json for item 14 arm (a); VALIDATION.json itself "
                          "is not written by the validator",
        "validator_report": "validator-report-rhoneg.json",
        "supersedes_for_the_gate": "rho-reference.json (plain rho, strongest_available false); that file is unchanged",
        "scope": "arm (a) full index calculus (default bx per cell) vs the negation-map matched rho (rho-neg), the "
                 "four design cells 16, 20, 24, 28 bits; gae_band at the largest measured cell, 28 bits (bx = 63). "
                 "Arm (b) full-IC rows are outside this reference (separate builds, cards 09-12).",
        "kind": "negation",
        "kind_note": "Pollard rho on classes {P, -P} (representative: smaller ordinate), the 16-jump r-adding walk "
                     "with BKL look-ahead against fruitless 2-cycles, Brent detection of longer fruitless cycles "
                     "and a deterministic escape (double the cycle minimum). Validator: code read, and a Python "
                     "port re-ran all 64 measured runs and reproduced every emitted field except ms.",
        "protocol": f"{ADDENDUM} (addendum B to PROTOCOL.md, committed in {ADD_COMMIT[:10]} before the run): "
                    "'The matched reference for the gate is rho-neg (kind: negation). Plain rho is still reported "
                    "beside it, priced at the same three weights.' Everything not changed by the addendum is "
                    "PROTOCOL.md's matched rho: same make_jumps (16 jumps, four random bits per scalar, counted "
                    "doubling chains), same partition, theta = max(bits/4, 2), DP rule x mod 2^theta = 0, cap "
                    "20*2^theta + 64, x-keyed table, counted verification mul(cand, G) == Q, 16 logarithms per "
                    "cell, one thread, setup counted in gae.",
        "unit": "GAE: one counted affine group addition (doubling and P + (-P) included) = 1, plus w x native "
                "operations, w in {0, 0.1, 1}. Native set (same counters on every arm): lookup, insert, legendre, "
                "sqrt, rowop, madd, dptest, build, and for rho-neg canon, lookahead, cyctest (zero on every other "
                "arm). Recomputed natives: rho-neg = lookup + insert + dptest + canon + lookahead + cyctest; plain "
                "rho = lookup + insert + dptest; arm (a) = lookup + insert + legendre + sqrt + rowop. "
                "Cost(w) = mean over 16 runs of (gae + w * native), setup included.",
        "binary_or_process": binary,
        "matches_protocol": True,
        "strongest_available": True,
        "same_unit": True,
        "same_binary_or_process": True,
        "evidence": {
            "matches_protocol": "rho_neg_run/neg_step/canon/cmd_rho_neg read against the addendum's 'rho-neg "
                                "algorithm (exact)': classes, look-ahead, Brent, escape, counting, row fields and "
                                "seeds as specified; everything else is rho_run's code (make_jumps, start chain, "
                                "theta, mask, cap, table, collision algebra, verification). The validator's port "
                                "reproduced all 64 rho-neg rows and all 64 continuity rho rows field by field "
                                "(except ms) from the addendum's target and seed formulas; 16/16 ok per cell.",
            "strongest_available": "every cell: p prime > 3, a, b != 0, j not in {0, 1728} => Aut(E) = {+-1}; "
                                   "E(F_p) has prime order n = #E(F_p) (G of order n, n prime in the Hasse "
                                   "interval and > 4 sqrt p), so no 2-torsion; the p-power Frobenius fixes E(F_p) "
                                   "pointwise and gives no class reduction. Negation (classes of size 2) is the "
                                   "largest usable automorphism group. Per-cell facts in validator-report-rhoneg.json.",
            "same_unit": "one instrument, one counter set: gae = counts.gadd for both rhos and total.gadd for arm "
                         "(a); the new rho-neg counters are priced as native operations exactly like dptest (the "
                         "addendum's pricing note; conservative against rho-neg at high w).",
            "same_binary_or_process": "see binary_or_process; continuity rows equal the recorded rows except ms at "
                                      "all four cells.",
        },
        "gae_band": {k: {"ic": c28[k]["S_a"], "rho": c28[k]["rho_neg"]} for k in WK},
        "gae_band_cell": {"bits": 28, "p": 248685287, "n": 248685841, "bx": 63, "runs_ic": 16, "runs_rho": 16,
                          "ic_over_rho": {k: c28[k]["gap_S_over_rho_neg"] for k in WK},
                          "definition": "ic = mean(gae_total) + w * mean(native_total) over 16 arm (a) full-IC runs "
                                        "(raw/fullic-a-b28.jsonl); rho = mean(gae) + w * mean(native) over 16 rho-neg "
                                        "runs (raw/rho-neg/rho-neg-b28.jsonl)"},
        "order_holds_across_band": order_all,
        "order_note": ("At every design cell and every w in {0, 0.1, 1}, rho-neg AND plain rho are both cheaper "
                       "than arm (a) full IC (the addendum requires an order statement to hold against both). "
                       f"The two rhos change order inside the band: rho-neg is cheaper at {neg_cheaper}, plain rho "
                       f"at {plain_cheaper} (rho-neg's canon, lookahead and cyctest are priced like dptest), so "
                       "which rho is cheaper is unresolved at that boundary; the distance remaining for arm (a) is "
                       "S / min(rho-neg, plain rho), reported per cell as gap_S_over_cheaper_rho.")
        if order_all else "the order does not hold against both rhos at every cell; see per_cell",
        "per_cell": {str(b): {
            "bits": b, "n": cells_out[b]["n"], "bx_arm_a": cells_out[b]["bx_arm_a"],
            "band": {k: {kk: vv for kk, vv in cells_out[b]["band"][k].items() if kk != "ci95"} for k in WK},
            "ci95": {k: cells_out[b]["band"][k]["ci95"] for k in WK},
            "order_holds_against_both": cells_out[b]["order_holds_against_both"],
            "ci95_separated_at_all_w": cells_out[b]["ci95_separated_at_all_w"],
            "rho_neg_vs_rho_plain": cells_out[b]["rho_neg_vs_rho_plain"],
        } for b in DESIGN},
        "plain_rho_comparison": {
            "source": "raw/rho-bB.jsonl (card-01; reproduced in this binary by cont-rho-bB.jsonl)",
            "plain_over_neg": {str(b): {k: cells_out[b]["band"][k]["rho_plain_over_rho_neg"] for k in WK}
                               for b in DESIGN},
            "online_gae_plain_over_neg": {str(b): cells_out[b]["diagnostics"]["online_gae_plain_over_neg"]
                                          for b in DESIGN},
            "note": "plain/neg > 1 means rho-neg is cheaper. Expected online-GAE ratio about sqrt 2 = 1.414 "
                    "(heuristic); independent 16-run samples, so a per-cell ratio carries about 11% (16 bits) to "
                    "20% (28 bits) relative standard error.",
        },
    }
    raw_data = [{"path": rel(p), "sha256": v["sha256"], "role": v["role"]} for p, v in sorted(READ.items())]
    rep = {
        "schema": "validation-gate/v1 validator-report",
        "id": REPORT_ID,
        "status": "pass" if all_ok else "fail",
        "independent": True,
        "independence": ("Validator subagent; did not write the rho-neg instrument, addendum, card or runs, and "
                         "took no number from a producer summary (none exists at the snapshot). Every value is "
                         "recomputed by validator_recompute_rhoneg.py (stdlib Python) from committed raw rows; the "
                         "rho walks were re-run by the validator's own Python port of the instrument (written after "
                         "reading the instrument source, so not a blind re-derivation). The producing commits carry "
                         "the same Claude-Session trailer as this validator's parent session: independence is a "
                         "separate subagent context with no producer messages, not a separate model or session. The "
                         "replay lane's outputs (raw/rho-neg-replay/, replay-certificate-v2.json, commits d63dea4372 "
                         "and 043f1eb98c) were not read."),
        "subject": {"item": ITEM, "evidence_directory": EVID, "snapshot_validated": SNAP,
                    "card": "CARD-BLINDSPOT-14-RHONEG-01", "run_commit": CARD_COMMIT, "addendum_commit": ADD_COMMIT,
                    "instrument_sha256": INSTR_SHA, "files_read_equal_snapshot_blobs": True,
                    "commit_times": rcpt["commit_times"]},
        "script": {"path": "validator_recompute_rhoneg.py",
                   "run": f"python3 -I {EVID}/validator_recompute_rhoneg.py --write",
                   "python": platform.python_version()},
        "tolerance_policy": ("claimed = the instrument-emitted per-row totals (gae, native; arm (a) gae_total, "
                             "native_total), aggregated exactly; recomputed = the per-counter fields, natives summed "
                             "by the validator, aggregated exactly (Fractions, one float conversion). Both sides are "
                             "exact integer means, so rel_tol is 1e-12 (float representation only); observed "
                             f"maximum relative difference {max_rel}. rho-neg entries also carry 'reimplemented': "
                             "the same mean from the validator's Python port, which reproduced every row."),
        "raw_data": raw_data,
        "headline": heads,
        "headline_failures": [h["name"] for h in heads
                              if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]),
                                                                                         abs(h["recomputed"]))],
        "per_cell": {str(b): cells_out[b] for b in DESIGN},
        "artifact_and_control_checks": CHECKS,
        "continuity": cont,
        "curves": curves,
        "reimplementation": {"rho-neg": {str(b): v for b, v in reimpl["rho-neg"].items()},
                             "cont-rho": {str(b): v for b, v in reimpl["cont-rho"].items()},
                             "walk_audit": reimpl["audit"]},
        "null_control": {"what": "validator-side: the same port with the class map off (canon returns its input), "
                                 "paired targets and seeds; accepted steps should rise by about sqrt 2",
                         "cells": {str(b): v for b, v in null.items()}},
        "receipt": {"exit_codes": rcpt["exit_codes"], "seconds": rcpt["seconds"],
                    "binary_sha256_recorded": rcpt["binary_sha256_recorded"]},
        "discrepancies": discrepancies(rcpt, cells_out),
        "limitations": limitations(cells_out, cost_hw, reimpl["audit"]["useful_self_collisions"]),
        "validation_report": {
            "id": REPORT_ID,
            "task_id": "validation-gate item 14 (blind-spot 20261018) negation-map matched rho; no Conductor task "
                       "card supplied",
            "run_ids": ["CARD-BLINDSPOT-14-RHONEG-01"],
            "heuristic_validation_checks": heuristic(null),
            "cost_model_checks": COSTMODEL,
            "proof_architecture_checks": [],
            "verdict": "passed" if all_ok else "failed",
            "artifact_paths": ["validator-report-rhoneg.json", "rho-reference-neg.json",
                               "validator_recompute_rhoneg.py"],
        },
        "review_attestation": {
            "joints_owned": ["receipt and commit-order binding of card n01",
                             "continuity controls (rho, fullic --arm a) at 16-28 bits",
                             "rho-neg algorithm vs addendum B and vs plain rho (code and port)",
                             "strongest-available symmetry on the four cells",
                             "rho-neg(w), plain rho(w), S(w) and both gaps at w = 0, 0.1, 1"],
            "sources_read": [rel(p) for p in sorted(READ)] + SOURCES_OUTSIDE,
            "read_sibling_reports": False,
            "sibling_note": "the earlier validator report and rho-reference.json for this item were read (context "
                            "named by the task, and the plain-rho cross-check); the concurrent replay certificate "
                            "v2 and the replay outputs were not read",
            "verdict": "holds" if all_ok else "breaks",
        },
    }
    (HERE / "rho-reference-neg.json").write_text(json.dumps(ref, indent=2) + "\n")
    (HERE / "validator-report-rhoneg.json").write_text(json.dumps(rep, indent=2) + "\n")
    print("wrote", HERE / "rho-reference-neg.json", HERE / "validator-report-rhoneg.json")


SOURCES_OUTSIDE = [
    "research/blindspots_20261018/14-prime-m4-cells/rho-neg-cards/PLAN.md at 84e5b2c859 (git show; the working-tree "
    "file was appended later by d63dea4372 and that part was not read)",
    "AGENTS.md (crypto) sections 'Standing workflow rules' pointer and 5a (pricing band)",
    "/home/user/crypto-autoresearcher/agents/validator.md, docs/agent-runtime-core.md, AGENTS.md 'Review "
    "architecture' and 'Curve identity and measured bounds' (role contract; outside this repository)",
]
def where(cells_out, pred):
    return [f"{b} bits w={k}" for b in DESIGN for k in WK if pred(cells_out[b]["band"][k])]


def discrepancies(rcpt, cells_out):
    out = []
    if not rcpt["binary_sha256_recorded"]:
        out.append({
            "what": "binary sha256 not recorded",
            "detail": "PROTOCOL-ADDENDUM-rho-neg.md says 'The card records the source and binary sha256 in its "
                      "receipt.' The card has no hashing command and receipt-n01.json has no binary digest (the "
                      "binary was removed by remove-bin). The source is bound (card binds, clean tree at the run "
                      "commit); binary identity rests on the command sequence of one card (build once, every run "
                      "command invokes that path) and on the continuity controls, which reproduce the recorded rows. "
                      "Same limitation as card-01.",
            "material": False})
    neg_cheaper = where(cells_out, lambda x: x["rho_neg"] < x["rho_plain"])
    plain_cheaper = where(cells_out, lambda x: x["rho_plain"] < x["rho_neg"])
    nps = {b: round(cells_out[b]["diagnostics"]["neg_native_per_step"], 2) for b in DESIGN}
    if plain_cheaper:
        out.append({
            "what": "plain rho is cheaper than rho-neg inside the band",
            "detail": f"rho-neg cheaper at {neg_cheaper}; plain rho cheaper at {plain_cheaper}. rho-neg emits "
                      f"{nps} native operations per accepted step (dptest, canon, lookahead on negated candidates, "
                      "cyctest) against about 1.1 for plain rho, each charged w GAE. The rho order therefore flips "
                      "inside the band at every cell (unresolved at that boundary). The addendum foresaw this and "
                      "fixed the rule (an order statement must hold against both rhos); arm (a)'s order holds against "
                      "both. Not a defect; but S/rho-neg understates the distance to the cheapest measured rho "
                      "wherever plain rho is cheaper (per_cell gap_S_over_cheaper_rho).",
            "material": False})
    pn0 = {b: round(cells_out[b]["band"]["0"]["rho_plain_over_rho_neg"], 3) for b in DESIGN}
    st_neg = {b: round(cells_out[b]["diagnostics"]["neg_steps_over_sqrt_pi_n_4"], 3) for b in DESIGN}
    st_pl = {b: round(cells_out[b]["diagnostics"]["rho_steps_over_sqrt_pi_n_2"], 3) for b in DESIGN}
    out.append({
        "what": "addendum expectation 'the gap is expected to shrink by about sqrt 2 at w = 0' is inverted",
        "detail": "a rho that is sqrt 2 cheaper makes S/rho grow by sqrt 2, not shrink. Measured at w = 0, "
                  f"S/rho-neg = (S/plain rho) x {pn0} (plain/neg). The ratio is below sqrt 2 at 20-28 bits partly "
                  f"because the recorded plain-rho sample ran short (mean steps / sqrt(pi n/2) = {st_pl}) while "
                  f"rho-neg ran near expectation (mean steps / sqrt(pi n/4) = {st_neg}); independent 16-run samples, "
                  "within sampling error. The other half of the expectation, gap above 1 at every cell and weight, "
                  f"holds (smallest S/rho-neg {min(cells_out[b]['band'][k]['gap_S_over_rho_neg'] for b in DESIGN for k in WK):.3f}).",
        "material": False})
    return out


def limitations(cells_out, cost_hw, n_self):
    sep = all(cells_out[b]["ci95_separated_at_all_w"] for b in DESIGN)
    return [
        f"One curve per size and 16 logarithms per cell. The 95% t half-width of a cell mean reaches "
        f"{cost_hw['neg']:.0%} for rho-neg and {cost_hw['rho']:.0%} for plain rho (largest over cells and w), "
        f"{cost_hw['S']:.0%} for arm (a). " + (
            "The order is robust to it: arm (a)'s lower 95% bound exceeds both rhos' upper 95% bounds at every cell "
            "and weight (per_cell ci95_separated_at_all_w)." if sep else
            "At some cell the intervals overlap (per_cell ci95_separated_at_all_w)."),
        "The rho-neg vs plain rho comparison is between independent samples (distinct targets and walk seeds by "
        "design), not paired runs; per-cell plain/neg ratios carry about 11% (16 bits) to 20% (28 bits) relative "
        "standard error, from the per-run coefficients of variation.",
        "Partition hashes (part(x) per step), coefficient updates mod n and Brent's saved-class copies are not counted "
        "on either rho, as in the original protocol; listed per AGENTS.md 5a.",
        "theta = floor(bits/4) uses the cell label, as PROTOCOL.md does; the new walk inherits it unchanged.",
        f"rho-neg can solve inside a walk when a detected cycle closes with different coefficients ({n_self} of "
        "the 64 measured runs ended that way, per the validator's port); plain rho has no cycle check. This is part "
        "of the addendum's specified fruitless-cycle handling and is counted, not an uncounted shortcut.",
        "The receipt records no environment (rustc version, CPU for target-cpu=native) and no timestamps; counts do "
        "not depend on either, and wall time (ms) is not used.",
        "The Python port was written after reading the instrument; it shows the rows follow from the code and the "
        "addendum's seed formulas, not that an independent reading of the addendum alone yields the same walk.",
        "Validated content is snapshot 41155e5554; HEAD has since gained the replay lane's commits d63dea4372 and "
        "043f1eb98c, which change none of the files read (each read file equals its snapshot blob).",
        "This report makes no ECDLP claim and authorizes no promotion; it admits the rho-neg rows as the gate's "
        "matched rho reference for arm (a) at the four design cells.",
    ]


def heuristic(null):
    big = [null[b]["off_over_on"] for b in (16, 20)]
    ok = all(x > 1.25 for x in big) and all(null[b]["off_over_on"] > 1.0 for b in DESIGN)
    return [
        {"check": "pre-registered prediction", "ok": True,
         "detail": "addendum committed in 9b2f5e1b36 before the card commit at which the run executed: steps near "
                   "sqrt(pi n / 4) instead of sqrt(pi n / 2), gap above 1 at every cell and weight (the 'shrink by "
                   "sqrt 2' wording is inverted; see discrepancies). Its numeric expectations came from selftest seeds "
                   "(target tag 0x5E1F, walk seeds 900001 + 7 run), disjoint from the measured streams (0x4E47, "
                   "+ 0x4E4547), and reproduce in the measured build's selftest log."},
        {"check": "sample integrity", "ok": True,
         "detail": "16 runs per cell, targets and walk seeds from the addendum's formulas, every row reproduced by "
                   "the validator's port; mean steps / sqrt(pi n/4) and online-GAE plain/neg recomputed from rows "
                   "(per_cell.diagnostics)."},
        {"check": "null control (class map off)", "ok": ok,
         "detail": "validator's port with canon returning its input, paired targets and seeds: accepted steps "
                   f"off/on = { {b: round(null[b]['off_over_on'], 3) for b in DESIGN} } (runs "
                   f"{ {b: null[b]['runs'] for b in DESIGN} }); expected about sqrt 2 = 1.414 less DP overhead. "
                   "The step reduction is caused by the class map."},
        {"check": "scale binding", "ok": True,
         "detail": "p of 16, 20, 24, 28 bits only; no transfer to larger p is claimed or validated."},
    ]


COSTMODEL = [
    {"check": "unit declared", "ok": True,
     "detail": "GAE plus w x native, w in {0, 0.1, 1}, natives listed per arm; no memory figure is emitted by "
               "either rho or arm (a) in these rows (rho table size = insert count is the only proxy)."},
    {"check": "optimistic assumptions flagged", "ok": True,
     "detail": "canon, lookahead and cyctest priced like dptest (pessimistic for rho-neg at w = 1); setup included "
               "on every arm; per-run cost includes the counted verification."},
    {"check": "expected-cost bookkeeping", "ok": True,
     "detail": "costs are means over runs that each ran to a verified logarithm (16/16 ok on both rhos and arm (a)), "
               "not per-attempt costs scaled by a success probability."},
]


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
