"""Capture existing frozen extractor output as source-processing metadata; no comparison verdicts."""
from pathlib import Path
import importlib.util,sys,json,hashlib
r=Path(__file__).resolve().parents[2];tool=r/'tools/pcc/pcc.py';spec=importlib.util.spec_from_file_location('frozen_pcc_capture',tool);m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
src=r/'research/statement_binding_audit_20261114/lean/HeaderControls.lean';rows=[]
for name in ['bar_control','named_argument_control','default_proof_control','inline_type_control','delimiter_model_not_injective','comment_control','string_control']:
 d=m.lean_find_declaration([src],'StatementBindingAudit.'+name)
 rows.append(dict(theorem='StatementBindingAudit.'+name,recorded_statement=d[1]if d else None,decl_sha256=hashlib.sha256(d[1].encode()).hexdigest()if d else None))
x=dict(schema='frozen-extractor-control-capture/v1',frozen_tool_path=str(tool.relative_to(r)),frozen_tool_sha256=hashlib.sha256(tool.read_bytes()).hexdigest(),source_path=str(src.relative_to(r)),source_sha256=hashlib.sha256(src.read_bytes()).hexdigest(),outputs=rows,scope='Observed output of existing Python source extractor as system under test. No reimplementation or comparison verdict in this metadata capture.')
p=Path(sys.argv[1]);p.write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n');print(json.dumps(x,ensure_ascii=False))
