"""Static integration audit only; imports no numerical component."""
from pathlib import Path
import ast
import hashlib
import json

D = Path(__file__).resolve().parent


def main():
    original = (D.parent/'companion-rank-solver/solve.py').read_text()
    candidate = (D/'solve_cached.py').read_text()
    replacements = [
        ('from cached_transport import find_pivot, swap_from_search\n', ''),
        ('row, column, value, queried, cache = find_pivot(source)',
         'row, column, value, queried = _find_pivot(source)'),
        ('moved, column_receipt = swap_from_search(source, cache)',
         'moved, column_receipt = swap_columns(source, 0, column)'),
    ]
    restored = candidate
    for new, old in replacements:
        if restored.count(new) != 1:
            raise ValueError('declared integration substitution missing or repeated')
        restored = restored.replace(new, old)
    a, b = ast.parse(original), ast.parse(restored)
    for tree in (a, b):
        first = tree.body[0]
        if not (isinstance(first, ast.Expr) and isinstance(first.value, ast.Constant)
                and isinstance(first.value.value, str)):
            raise ValueError('module docstring expected')
        tree.body = tree.body[1:]
    if ast.dump(a) != ast.dump(b):
        raise ValueError('undeclared executable source change')
    for path in D.glob('*.py'):
        ast.parse(path.read_text(), filename=str(path))
    print(json.dumps(dict(reference_sha256=hashlib.sha256(original.encode()).hexdigest(),
                          candidate_sha256=hashlib.sha256(candidate.encode()).hexdigest(),
                          normalized_AST_equal=True, numerical_trials=0)))


if __name__ == '__main__':
    main()
