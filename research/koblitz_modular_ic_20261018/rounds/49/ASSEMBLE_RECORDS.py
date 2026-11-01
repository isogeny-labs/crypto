from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent
rows=[]
failed=json.loads((h/"failed-launches.json").read_text())
markers={"control49-01":"CONTROL49_ALL_CHECKS_PASS","prep49-01":"PREP49_ALL_CHECKS_PASS","prep49-02":"PREP49_ALL_CHECKS_PASS","normal49-01":"NORMAL49_PRODUCER_PASS","compact49-01":"COMPACT49_ALL_CHECKS_PASS","generic49-01":"GENERIC49_ALL_CHECKS_PASS","primitive49-01":"PRIMITIVE49_ALL_CHECKS_PASS","primitive49-02":"PRIMITIVE49_ALL_CHECKS_PASS","lambda49-01":"LAMBDA49_ALL_CHECKS_PASS","compact-normal49-01":"COMPACT_NORMAL49_PRODUCER_PASS"}
for p in sorted((h/"runs").iterdir()):
 if not p.is_dir():continue
 if p.name in failed:
  rows.append({"attempt":p.name,"kind":"Lean","status":"rejected",**failed[p.name]});continue
 row={"attempt":p.name,"source_revision":(p/"source-revision.txt").read_text().strip()}
 if p.name.startswith("lean"):
  assert (p/"compile.exit").exists(), "nonterminal Lean compile"
  c=int((p/"compile.exit").read_text()); a=int((p/"audit.exit").read_text()) if (p/"audit.exit").exists() else None
  log=(p/"audit.log").read_text() if (p/"audit.log").exists() else ""
  ok=c==0 and a==0 and "0 failing, 0 trusted" in log
  row.update(kind="Lean",compile_exit=c,audit_exit=a,status="accepted" if ok else "rejected",audited_declarations=int(re.search(r"audited (\d+) theorems",log).group(1)) if ok else 0)
 else:
  assert (p/"exit.txt").exists(), "nonterminal native process"
  c=int((p/"exit.txt").read_text()); log=(p/"native.log").read_text();errors=[l for l in log.splitlines() if re.match(r"^ *\?",l)]
  marker=markers.get(p.name,"VERIFY49_ALL_CHECKS_PASS" if p.name.startswith("verify49") else None);assert marker is not None
  present=marker in log.splitlines();ok=c==0 and not errors and present
  row.update(kind="Singular",native_exit=c,status="accepted" if ok else "rejected",timeout=c==124,native_errors=errors,late_marker_present=present,marker=marker)
  row["exports"]={f.name:hashlib.sha256(f.read_bytes()).hexdigest() for f in p.glob("*.sing") if f.name!="input.sing"}
 rows.append(row)
accepted={x["attempt"] for x in rows if x["status"]=="accepted"}
producer=bool({"normal49-01","compact-normal49-01"}&accepted)
verified=producer and any(x.startswith("verify49") for x in accepted)
normalLog=(h/"runs/normal49-01/native.log").read_text()
components=[{"index":int(a),"generic_degree":int(b)} for a,b in re.findall(r"^NORMAL49_COMPONENT index=(\d+) generic_degree=(-?\d+)$",normalLog,re.M)]
genericLog=(h/"runs/generic49-01/native.log").read_text()
primitiveLog=(h/"runs/primitive49-02/native.log").read_text()
lambdaLog=(h/"runs/lambda49-01/native.log").read_text()
r={"schema":"koblitz.results/v1","round":49,"runs":rows,"construction_counts":{"normalization_control_cases":2 if "control49-01" in accepted else 0,"source_preparation_cases":1 if "prep49-02" in accepted else 0,"registered_generic_units":8 if "prep49-02" in accepted else None,"generic_source_degree":32 if "prep49-02" in accepted else None,"generic_source_prime":int("generic49-01" in accepted and "original_ideal_prime=1" in genericLog),"integral_lambda_plane_models":int("lambda49-01" in accepted and "integral_plane_model=true" in lambdaLog),"primitive_presentations":int("primitive49-02" in accepted and "PRIMITIVE49_RESULT fixed_element_primitive=true" in primitiveLog),"compact_source_preparation_cases":int("compact49-01" in accepted),"normalization_outputs_computed":len({"normal49-01","compact-normal49-01"}&accepted),"verified_affine_normalizations":int(verified),"audited_conditional_declarations":sum(x.get("audited_declarations",0) for x in rows if x["status"]=="accepted")},"component_descriptors":components,"lambda_result_lines":[l for l in lambdaLog.splitlines() if l.startswith("LAMBDA49_RESULT ")],"primitive_result_lines":[l for l in primitiveLog.splitlines() if l.startswith("PRIMITIVE49_RESULT ") or l.startswith("PRIMITIVE49_SHAPE ")],"generic_source_result_lines":[l for l in (h/"runs/generic49-01/native.log").read_text().splitlines() if l.startswith("GENERIC49_RESULT ") or l.startswith("GENERIC49_COMPONENT ")],"failed_launches":failed,"normalization_output_computed":producer,"affine_normalization_verified":verified,"proper_projective_geometry_complete":False,"chosen_modular_factor_map_evaluated":False,"nonzero_target_order11_action_verified":False,"native_arrays_formalized":False,"independent_scientific_replay":False,"independent_semantic_review":False,"performance_measured":False,"larger_n_geometric_transfer":False,"IC_relation_rank":None}
(h/"results.json").write_text(json.dumps(r,indent=2)+"\n")
print(json.dumps({"attempts":len(rows),"accepted":len(accepted),"normalization_output_computed":producer,"affine_normalization_verified":verified}))
