"""Compare integration ASTs without importing/executing numerical code."""
from pathlib import Path
import ast
import hashlib
import json

D = Path(__file__).resolve().parent


def main():
    original = (D.parent/'companion-cached-pivot/solve_cached.py').read_text()
    candidate = (D/'solve_border.py').read_text()
    substitutions = [
        ('from border_schur import BorderSchur\n', ''),
        ("schur = BorderSchur(moved, inverse, permute(cache.column, 0, row))\n"
         "        trace.append(dict(kind='border_cache', dimension=size, profile=schur.profile))",
         'schur = SourceSchur(moved, 1, inverse)'),
        ('        schur.release_borders()\n', ''),
    ]
    restored = candidate
    for new, old in substitutions:
        if restored.count(new) != 1:
            raise ValueError('missing or repeated integration substitution')
        restored = restored.replace(new, old)
    a, b = ast.parse(original), ast.parse(restored)
    for tree in (a, b):
        first = tree.body[0]
        if not (isinstance(first, ast.Expr) and isinstance(first.value, ast.Constant)
                and isinstance(first.value.value, str)):
            raise ValueError('module docstring required')
        tree.body = tree.body[1:]
    if ast.dump(a) != ast.dump(b):
        raise ValueError('undeclared executable change')
    for p in D.glob('*.py'):
        ast.parse(p.read_text(), filename=str(p))
    print(json.dumps(dict(reference_sha256=hashlib.sha256(original.encode()).hexdigest(),
                          candidate_sha256=hashlib.sha256(candidate.encode()).hexdigest(),
                          normalized_AST_equal=True, numerical_trials=0)))


if __name__ == '__main__':
    main()
