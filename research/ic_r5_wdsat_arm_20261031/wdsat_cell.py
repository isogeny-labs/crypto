#!/usr/bin/env python3
"""R5: size WDSat's static allocation for a cell and build the per-size binaries.

WDSat (github.com/mtrimoska/WDSat, 61c6ff3f49445af2729274819edb27b85a89efc1, GPL-3.0) fixes its
array sizes in src/config.h at compile time.  This script never copies WDSat's source into the
repository: it works on a copy of an external clone given by --src, writes a generated config.h
into that copy (outside the repo) and runs the upstream makefile there.

  sizing  <anf_dir>                      constants required by every emitted system (from the ANF text,
                                         following upstream dimacs.c's counting rules)
  build   <anf_dir> --src S --out O      one binary per distinct number of Boolean variables
                                         (MAX_ID exact, the other constants at the cell maximum)

Counting rules (upstream src/dimacs.c, ANF reader): every distinct monomial of degree d >= 2 becomes
one new variable and d+1 OR-clauses (2d binary-clause atoms + d+1 atoms); every XOR line is one XOR
equation of as many atoms as it has terms.
"""
import argparse, glob, json, os, re, shutil, subprocess, sys

def parse_anf(path):
    lines = open(path).read().split('\n')
    hdr = lines[0].split()
    nunary = int(hdr[2])
    mons = set()
    maxdeg = 1
    xlens = []
    neq = 0
    for ln in lines[1:]:
        if not ln.startswith('x'):
            continue
        neq += 1
        toks = ln.split()[1:]
        k = 0
        n_atoms = 0
        while k < len(toks):
            t = toks[k]
            if t == '0':
                break
            if t == 'T':
                k += 1
                continue
            if t[0] == '.':
                d = int(t[1:])
                vs = tuple(sorted(int(v) for v in toks[k + 1:k + 1 + d]))
                mons.add(vs)
                maxdeg = max(maxdeg, d)
                k += 1 + d
            else:
                k += 1
            n_atoms += 1
        xlens.append(n_atoms)
    nvars = nunary + len(mons)
    clauses = sum(len(m) + 1 for m in mons)
    buf_be = sum(3 * len(m) + 1 for m in mons)
    buf_xe = sum(xlens)
    return dict(nunary=nunary, nvars=nvars, clauses=clauses, maxdeg=maxdeg, buf_be=buf_be,
                buf_xe=buf_xe, maxlen=max(xlens) if xlens else 0, neq=neq)

def sizing(*anf_dirs):
    rows = [parse_anf(p) for d in anf_dirs for p in sorted(glob.glob(os.path.join(d, '*.anf')))]
    assert rows, 'no .anf files'
    cell = dict(
        files=len(rows),
        nvars_values=sorted(set(r['nvars'] for r in rows)),
        nunary=max(r['nunary'] for r in rows),
        maxdeg=max(r['maxdeg'] for r in rows),
        clauses=int(1.3 * max(r['clauses'] for r in rows)) + 10,
        buf=max(max(r['buf_be'], r['buf_xe']) for r in rows),
        maxlen=max(r['maxlen'] for r in rows),
        neq=max(r['neq'] for r in rows) + 6,
    )
    return cell

def config_text(cell, max_id, find_all=True):
    # Strict `<` asserts in upstream dimacs.c => +1; MAX_ID exact (upstream prints a warning otherwise).
    fa = '#define __FIND_ALL_SOLUTIONS__\n' if find_all else ''
    return f"""#define __XG_ENHANCED__
{fa}#ifdef __XG_ENHANCED__
#define __MAX_ANF_ID__ {cell['nunary'] + 1}
#define __MAX_DEGREE__ {cell['maxdeg'] + 1}
#endif
#define __MAX_ID__ {max_id}
#define __MAX_BUFFER_SIZE__ {2 * cell['buf'] + 100}
#define __MAX_EQ__ {cell['clauses'] + 2}
#define __MAX_EQ_SIZE__ {cell['maxdeg'] + 2}
#define __MAX_XEQ__ {cell['neq'] + 2}
#define __MAX_XEQ_SIZE__ {max(cell['maxlen'] + 2, max_id + 1)}
"""

def build(anf_dirs, src, out, only=None, pad=0, find_all=True):
    cell = sizing(*anf_dirs)
    vals = cell['nvars_values']
    if pad:
        vals = list(range(min(vals) - pad, max(vals) + pad + 1))
        cell['nvars_values'] = vals
    os.makedirs(out, exist_ok=True)
    built = []
    for nv in (only or cell['nvars_values']):
        work = os.path.join(out, f'build_id{nv}')
        if os.path.exists(work):
            shutil.rmtree(work)
        os.makedirs(work)
        shutil.copytree(os.path.join(src, 'src'), os.path.join(work, 'src'))
        with open(os.path.join(work, 'src', 'config.h'), 'w') as f:
            f.write(config_text(cell, nv, find_all))
        r = subprocess.run(['make', '-C', os.path.join(work, 'src')], capture_output=True, text=True)
        if r.returncode != 0:
            sys.exit(r.stderr[-2000:])
        shutil.move(os.path.join(work, 'wdsat_solver'), os.path.join(out, f'wdsat_id{nv}'))
        shutil.rmtree(work)
        built.append(nv)
    json.dump(dict(cell=cell, built=built), open(os.path.join(out, 'cell.json'), 'w'), indent=1)
    return cell, built

if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd', choices=['sizing', 'build'])
    ap.add_argument('anf_dir', nargs='+')
    ap.add_argument('--pad', type=int, default=0)
    ap.add_argument('--first', action='store_true', help='first-solution binaries (no __FIND_ALL_SOLUTIONS__)')
    ap.add_argument('--src')
    ap.add_argument('--out')
    ap.add_argument('--only', type=int, nargs='*')
    a = ap.parse_args()
    if a.cmd == 'sizing':
        print(json.dumps(sizing(*a.anf_dir), indent=1))
    else:
        print(json.dumps(build(a.anf_dir, a.src, a.out, a.only, a.pad, not a.first), indent=1))
