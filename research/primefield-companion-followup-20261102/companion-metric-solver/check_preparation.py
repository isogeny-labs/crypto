"""Static preservation audit; no numerical module imports."""
from pathlib import Path
import ast
import hashlib
import json
D=Path(__file__).resolve().parent

def function(path):
    t=ast.parse(path.read_text())
    return next(x for x in t.body if isinstance(x,ast.FunctionDef) and x.name=='solve_or_witness')

def main():
    old=function(D.parent/'companion-rank-solver/solve.py')
    new=function(D/'driver.py')
    # Final source-equation and witness-normalization verification are exact copies.
    assert ast.dump(old.body[-2])==ast.dump(new.body[-2])
    def zero_branch(f):
        loop=next(x for x in f.body if isinstance(x,ast.While))
        return next(x for x in loop.body if isinstance(x,ast.If))
    assert ast.dump(zero_branch(old))==ast.dump(zero_branch(new))
    files=list(D.rglob('*.py'))
    for p in files:ast.parse(p.read_text())
    frame=ast.parse((D/'block_frame.py').read_text())
    cls=next(x for x in frame.body if isinstance(x,ast.ClassDef))
    slots=ast.literal_eval(cls.body[0].value)
    assert not set(slots)&{'source','schur','rows','full_rhs'}
    refs=['companion-rank-solver/solve.py','companion-metric-pivot/selection.py',
          'companion-metric-pivot/transport.py','companion-metric-pivot/metric.py',
          'companion-recursive-inverse/recursive.py','companion-schur/schur.py',
          'companion-schur-lift/lift.py','companion-generator-compression/compress.py']
    print(json.dumps(dict(syntax_files=len(files),numerical_trials=0,
        zero_terminal_AST_equal=True,root_verification_AST_equal=True,
        dependencies={r:hashlib.sha256((D.parent/r).read_bytes()).hexdigest() for r in refs},
        frame_slots=list(slots),scope='static preservation, not numerical correctness')))
if __name__=='__main__':main()
