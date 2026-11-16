#!/usr/bin/env python3
"""Paired timing of M4GB builds on a captured corpus, through `--serve`.

    ab_corpus.py CORPUS OUT.json ARM=BINDIR [ARM=BINDIR ...] [--repeats R]

CORPUS holds one directory per cell with the captured `NNNN.in` systems
(capture.sh).  Every arm gets one `--serve --loglevel 1 --nrthreads 1` process
per binary width (the narrowest solver_m4gb_n<N>_gf2 with N >= the system's
variables), warmed with one untimed request.  For every system and repeat the
arms run back to back in an order that rotates with the repeat and the system
index; each request is timed from the write of `SOLVE in out` to the reply
line.  The reply word and the SHA-256 of the output file are recorded, and a
system on which the arms disagree is flagged.  Standard library only."""
import hashlib, json, os, re, subprocess, sys, tempfile, time

def width_for(nvars, bindir):
    widths = sorted(int(m.group(1)) for f in os.listdir(bindir)
                    if (m := re.fullmatch(r'solver_m4gb_n(\d+)_gf2', f)))
    for w in widths:
        if w >= nvars:
            return w
    raise SystemExit(f'no binary in {bindir} takes {nvars} variables')

class Server:
    def __init__(self, path):
        self.p = subprocess.Popen([path, '--serve', '--loglevel', '1', '--nrthreads', '1'],
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                  stderr=subprocess.DEVNULL, text=True, bufsize=1)
        self.reply()  # READY
    def reply(self):
        while True:
            line = self.p.stdout.readline()
            if not line:
                raise RuntimeError('server exited')
            line = line.strip()
            if line in ('READY', 'OK', 'UNSAT') or line.startswith('ERR '):
                return line
    def solve(self, inp, out):
        t0 = time.perf_counter_ns()
        self.p.stdin.write(f'SOLVE {inp} {out}\n'); self.p.stdin.flush()
        r = self.reply()
        return r, time.perf_counter_ns() - t0
    def close(self):
        try:
            self.p.stdin.write('QUIT\n'); self.p.stdin.flush(); self.p.wait(10)
        except Exception:
            self.p.kill()

def main():
    args = sys.argv[1:]
    repeats = 3
    if '--repeats' in args:
        i = args.index('--repeats'); repeats = int(args[i + 1]); del args[i:i + 2]
    corpus, out_path, arm_specs = args[0], args[1], args[2:]
    arms = [tuple(a.split('=', 1)) for a in arm_specs]
    systems = []
    for cell in sorted(os.listdir(corpus)):
        cdir = os.path.join(corpus, cell)
        if not os.path.isdir(cdir):
            continue
        for f in sorted(os.listdir(cdir)):
            if f.endswith('.in'):
                text = open(os.path.join(cdir, f)).read(400)
                nv = int(re.search(r'\$vars (\d+)', text).group(1))
                systems.append((cell, f, os.path.join(cdir, f), nv))
    servers = {}
    def server(arm, bindir, nv):
        w = width_for(nv, bindir)
        key = (arm, w)
        if key not in servers:
            servers[key] = Server(os.path.join(bindir, f'solver_m4gb_n{w}_gf2'))
        return servers[key]
    tmp = tempfile.mkdtemp(prefix='m4gb-ab-')
    rows = []
    warmed = set()
    for idx, (cell, name, path, nv) in enumerate(systems):
        for arm, bindir in arms:
            s = server(arm, bindir, nv)
            if id(s) not in warmed:
                s.solve(path, os.path.join(tmp, 'warm.out')); warmed.add(id(s))
        runs = {arm: [] for arm, _ in arms}
        hashes = {}
        for r in range(repeats):
            k = (r + idx) % len(arms)
            for arm, bindir in arms[k:] + arms[:k]:
                outp = os.path.join(tmp, f'{arm}.out')
                if os.path.exists(outp):
                    os.remove(outp)
                reply, ns = server(arm, bindir, nv).solve(path, outp)
                h = hashlib.sha256(open(outp, 'rb').read()).hexdigest() if reply == 'OK' else None
                runs[arm].append({'wall_ns': ns, 'reply': reply})
                hashes.setdefault(arm, set()).add((reply, h))
        outcomes = set().union(*hashes.values())
        rows.append({'cell': cell, 'system': name, 'n_vars': nv, 'runs': runs,
                     'agree': len(outcomes) == 1, 'outcome': sorted(map(str, outcomes))})
        print(f'{cell} {name} vars={nv} ' + ' '.join(
            f'{a}={sorted(x["wall_ns"] for x in runs[a])[len(runs[a]) // 2] / 1e6:.2f}ms' for a, _ in arms)
            + ('' if rows[-1]['agree'] else '  DISAGREE'), flush=True)
    for s in servers.values():
        s.close()
    json.dump({'schema': 'm4gb-corpus-ab/1', 'arms': dict(arms), 'repeats': repeats,
               'systems': rows}, open(out_path, 'w'), indent=1)

if __name__ == '__main__':
    main()
