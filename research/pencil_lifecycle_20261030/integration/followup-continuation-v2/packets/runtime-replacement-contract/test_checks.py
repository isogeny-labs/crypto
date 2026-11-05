import ast,json,tempfile,unittest
from pathlib import Path
p=Path(__file__).parent
# Load only pure helpers, never the top-level input scan or a Docker command.
tree=ast.parse((p/'check_inputs.py').read_text()); nodes=[n for n in tree.body if isinstance(n,(ast.Import,ast.ImportFrom,ast.FunctionDef))]
ns={'checks':[]};exec(compile(ast.Module(body=nodes,type_ignores=[]),str(p/'check_inputs.py'),'exec'),ns)
class Controls(unittest.TestCase):
 def test_duplicate_key_rejected(self):
  with self.assertRaisesRegex(ValueError,'duplicate key'):ns['strict']('{"admitted":false,"admitted":true}')
 def test_wrong_digest_rejected(self):
  with tempfile.TemporaryDirectory() as d:
   f=Path(d)/'input';f.write_bytes(b'input')
   with self.assertRaisesRegex(ValueError,'digest'):ns['pin'](f,'0'*64,'control')
 def test_symlink_rejected(self):
  with tempfile.TemporaryDirectory() as d:
   f=Path(d)/'input';f.write_bytes(b'input');link=Path(d)/'link';link.symlink_to(f)
   with self.assertRaisesRegex(ValueError,'symlink'):ns['pin'](link,ns['digest'](b'input'),'control')
 def test_exact_bytes_accepted(self):
  with tempfile.TemporaryDirectory() as d:
   f=Path(d)/'input';f.write_bytes(b'input');self.assertEqual(ns['pin'](f,ns['digest'](b'input'),'control'),b'input')
if __name__=='__main__':unittest.main()
