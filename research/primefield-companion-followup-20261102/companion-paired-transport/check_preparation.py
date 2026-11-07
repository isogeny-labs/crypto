"""Full driver AST preservation with the single transport import replaced."""
from pathlib import Path
import ast
import hashlib
import json
D=Path(__file__).resolve().parent

def main():
    old=(D.parent/'companion-metric-solver/driver.py').read_text()
    new=(D/'driver_paired.py').read_text()
    a='from transport import move, permute_rhs, undo_vector'
    b='from transport import permute_rhs, undo_vector\nfrom paired import move'
    assert new.count(b)==1
    assert ast.dump(ast.parse(old))==ast.dump(ast.parse(new.replace(b,a)))
    files=list(D.rglob('*.py'))
    for p in files:ast.parse(p.read_text())
    refs=['companion-metric-pivot/transport.py','companion-metric-pivot/selection.py',
          'companion-pivot-transport/pivot.py','companion-rank-solver/solve.py',
          'companion-fused/multiply.py']
    print(json.dumps(dict(driver_AST_equal_after_import_replacement=True,
        syntax_files=len(files),numerical_trials=0,
        reference_driver_sha256=hashlib.sha256(old.encode()).hexdigest(),
        candidate_driver_sha256=hashlib.sha256(new.encode()).hexdigest(),
        dependencies={s:hashlib.sha256((D.parent/s).read_bytes()).hexdigest() for s in refs},
        scope='static integration preservation; no transport correctness execution')))
if __name__=='__main__':main()
