import json, glob, collections
g = collections.defaultdict(lambda: [0, set(), set()])
for f in glob.glob('research/**/spec.json', recursive=True) + glob.glob('research/**/spec-*.json', recursive=True):
    try:
        d = json.load(open(f))
    except Exception:
        continue
    if not isinstance(d, dict) or not str(d.get('schema', '')).startswith('ecbench.spec'):
        continue
    ids = [a['method']['id'] for a in d['arms']]
    if not any(i.startswith('ic.') for i in ids):
        continue
    rt = {i for i in ids if i.startswith('rho.') and 'strong' not in i}
    rs = {i for i in ids if 'strong' in i}
    if not rt and not rs:
        continue
    top = f.split('/')[1]
    g[top][0] += 1
    g[top][1] |= rt
    g[top][2] |= rs
lines = ["dir\tspecs\tnon-strong rho arms\tstrong rho arms"]
for k, v in sorted(g.items()):
    lines.append(f"{k}\t{v[0]}\t{','.join(sorted(v[1])) or '-'}\t{','.join(sorted(v[2])) or '-'}")
open('/tmp/claude-501/r12/affected.tsv', 'w').write('\n'.join(lines) + '\n')
print(len(g), sum(v[0] for v in g.values()))
print(len([k for k, v in g.items() if v[1] and not v[2]]))
