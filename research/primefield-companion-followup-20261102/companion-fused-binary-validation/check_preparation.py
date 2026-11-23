"""Metadata-only audit of preserved numerical code and bounded output routes."""
from pathlib import Path
import ast
import hashlib
import json

D = Path(__file__).resolve().parent
ROOT = D.parent


class NormalizeOutput(ast.NodeTransformer):
    def visit_Call(self, node):
        self.generic_visit(node)
        f = node.func
        if isinstance(f, ast.Attribute) and isinstance(f.value, ast.Name) and f.value.id == 'budget':
            name = node.args[0]
            assert isinstance(name, ast.Constant) and name.value in {
                'host.json', 'carry-certificate.json', 'cases.jsonl', 'summary.json', 'row-index.json'}
            path = ast.BinOp(left=ast.Name(id='OUT', ctx=ast.Load()), op=ast.Div(), right=name)
            if f.attr == 'write_text':
                return ast.Call(func=ast.Attribute(value=path, attr='write_text', ctx=ast.Load()),
                                args=node.args[1:], keywords=node.keywords)
            assert f.attr == 'open' and name.value == 'cases.jsonl'
            return ast.Call(func=ast.Attribute(value=path, attr='open', ctx=ast.Load()),
                            args=[ast.Constant(value='w')], keywords=[])
        return node


def main():
    assert __debug__
    original = (ROOT/'companion-fused-validation/check.py').read_text()
    old_lock = json.loads((ROOT/'companion-fused-validation/source-lock.json').read_text())
    assert hashlib.sha256(original.encode()).hexdigest() == old_lock['companion-fused-validation/check.py']
    prepared = (D/'check.py').read_text()
    def functions(source):
        return {n.name:n for n in ast.parse(source).body if isinstance(n,ast.FunctionDef)}
    before, after = functions(original), functions(prepared)
    guards = ast.parse("if not __debug__:raise SystemExit('Assertions must remain enabled')\nassert json.loads((D/'protocol.json').read_text())['freeze']['status']=='frozen','A new frozen protocol is required'").body
    assert [ast.dump(n) for n in after['main'].body[:2]] == [ast.dump(n) for n in guards]
    body = []
    removed = 0
    for n in after['main'].body[2:]:
        if ast.dump(n) == ast.dump(ast.parse('budget=DataBudget(OUT)').body[0]):
            removed += 1
        else: body.append(NormalizeOutput().visit(n))
    assert removed == 1
    checks = dict(stages=ast.dump(before['stages'])==ast.dump(after['stages']),
                  verify_before_return=[ast.dump(n) for n in before['verify'].body[:-1]] ==
                                       [ast.dump(n) for n in after['verify'].body[:-1]],
                  main_after_freeze_guard_and_output_routing=
                      [ast.dump(n) for n in before['main'].body]==[ast.dump(n) for n in body])
    assert all(checks.values())
    (D/'source-preservation.json').write_text(json.dumps(checks,indent=2)+'\n')
    print(checks)


if __name__=='__main__':main()
