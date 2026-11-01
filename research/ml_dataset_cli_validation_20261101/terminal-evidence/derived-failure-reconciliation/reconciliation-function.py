def reconcile_failure(fixture, original_driver):
    # Called after failure receipt/inventory retention. No subprocess, cleanup,
    # arithmetic replay, solver resume, deadline change or remote I/O here.
    state=diagnostic_json(fixture/"prepared.json");protocol=diagnostic_json(fixture/"protocol.json")
    require(sha(fixture/"protocol.json")==state["protocol_file_sha256"],"failure protocol changed")
    require(sha(original_driver)==state["driver_sha256"],"original frozen driver changed")
    require([a["id"] for a in protocol["arms"]]==list(PHASE_ARMS),"failure adapter needs the frozen source/replay phases")
    failure=diagnostic_json(fixture/"execution-failed.json")
    settled=failure.get("cleanup",{}).get("settled") is True and not failure.get("cleanup",{}).get("unresolved")
    files=[];seen=set();cases=[];runs=[]
    def retain(path,kind="raw"):
        if path.exists() and path.is_file() and not path.is_symlink():
            row=diagnostic_file(fixture,path,kind)
            if row["path"] not in seen:seen.add(row["path"]);files.append(row)
            return row
        return None
    # Snapshot references preserve raw charges, caps, logs and failures even when
    # they cannot be projected into truthful numerical canonical measurements.
    inventory_path=fixture/"execution-failed-inventory.json"
    retain(inventory_path,"receipt");retain(fixture/"execution-failed.json","receipt")
    if inventory_path.exists():
        for item in diagnostic_json(inventory_path)["files"]:
            relative=Path(item["path"])
            if relative.is_absolute() or ".." in relative.parts:continue
            path=fixture/relative
            if path.is_file() and not path.is_symlink():retain(path)
    for case in failure_schedule(fixture):
        cell=case["root"]/"run/cells"/f"{case['index']:012}"
        observation={k:v for k,v in case.items() if k!="root"}
        observation.update(state="attempt_status_unknown",canonical_row=False,evidence=[],observed_outcome=None,reason=None)
        for name in ["attempt.json","worker-process.json","execution-context.json","result.json","manifest.json","config.json"]:
            artifact=retain(cell/name)
            if artifact:observation["evidence"].append(artifact)
        try:
            if case["panel"]=="native":name="generate";argv=expected_cli_argv(state,fixture)[name]
            else:
                panel=next(p for p in POLICY_PANELS if p["id"]==case["panel"])
                name="policy-"+case["panel"]+"-generate";argv=policy_commands(state,fixture,panel)["generate"]
            command=diagnostic_command(fixture,state,name,argv)
            if command is None:
                observation["state"]="not_attempted" if settled and not cell.exists() else "attempt_status_unknown"
                observation["reason"]="no generator command receipt; no worker state invented"
            elif not (cell/"attempt.json").exists():
                observation["state"]="not_attempted" if settled and not cell.exists() else "attempt_status_unknown"
                observation["reason"]="no retained attempt marker"
            else:
                attempt=diagnostic_json(cell/"attempt.json");config=diagnostic_json(cell/"config.json")
                expected={"family":case["family"],"n":case["n"],"m":case["m"],"twist":case["twist"],"curve_seed":0,"generator_index":0,"key_index":0,"seed":42,"factor_base_size":4,"degree_bound":3,"work_limit":16384,"solver":case["solver"]}
                if case["recipe"] is not None:expected["factor_base_policy"]=case["recipe"]
                require(config==expected,"attempt config differs from frozen case")
                binding=diagnostic_json(case["root"]/"run/run.json")
                require(binding["schema"]=="crypto.ml-dataset.run/v1" and binding["source_commit"]==state["source_commit"] and binding["binary_sha256"]==state["binary_sha256"],"run instrument substitution")
                require((binding["shard_index"],binding["shard_count"],binding["seconds_per_case"],binding["memory_mib"],binding["trace_mib"])==(0,1,60,256,128),"run limits changed")
                pre_name="preflight" if case["panel"]=="native" else "policy-"+case["panel"]+"-preflight"
                pre_argv=expected_cli_argv(state,fixture)["preflight"] if case["panel"]=="native" else policy_commands(state,fixture,panel)["preflight"]
                pre=diagnostic_command(fixture,state,pre_name,pre_argv)
                require(pre is not None and type(pre["exit_code"]) is int and pre["exit_code"]==0,"preflight missing or failed")
                pre_dir=next(iter(exact_command_directories(fixture,pre_name)));preflight=diagnostic_json(pre_dir/"stdout")
                require(preflight["plan_sha256"]==binding["plan_sha256"] and preflight["plan_file_sha256"]==sha(case["root"]/"plan.json") and preflight["protocol_file_sha256"]==state["protocol_file_sha256"] and preflight["source_commit"]==state["source_commit"] and preflight["binary_sha256"]==state["binary_sha256"],"preflight plan/instrument substitution")
                require(attempt["schema"]=="crypto.ml-dataset.attempt/v1" and attempt["cell_index"]==case["index"] and attempt["plan_sha256"]==binding["plan_sha256"] and attempt["status"]=="attempt_started","invalid attempt marker")
                observation["state"]="attempted_unverified"
                if (cell/"result.json").exists():observation["observed_result"]=diagnostic_json(cell/"result.json")
                if not settled:
                    observation["reason"]="unresolved cleanup; no terminal row claimed"
                elif not (cell/"manifest.json").exists():
                    observation["reason"]="attempt retained without terminal seal; command status not attributed to cell"
                else:
                    manifest=diagnostic_json(cell/"manifest.json")
                    require(manifest["schema"]=="crypto.ml-dataset.cell/v1" and manifest["config"]==config and manifest["cell_index"]==case["index"] and manifest["plan_sha256"]==binding["plan_sha256"],"manifest attempt/config mismatch")
                    outcome=manifest["outcome"];observation["observed_outcome"]=outcome
                    require(isinstance(outcome.get("supervisor"),dict) and "exit_code" in outcome["supervisor"],"terminal supervisor observation missing")
                    for artifact in manifest["artifacts"]:
                        relative=Path(artifact["path"])
                        require(not relative.is_absolute() and ".." not in relative.parts,"unsafe manifest member")
                        actual=retain(cell/relative)
                        require(actual and actual["sha256"]==artifact["sha256"] and actual["bytes"]==artifact["bytes"],"manifest member changed")
                        observation["evidence"].append(actual)
                    verified=diagnostic_checkpoint(fixture,state,case,sha(cell/"manifest.json"))
                    if verified:status="succeeded";classification="verified_completed"
                    elif outcome.get("status")=="timeout":status="timeout";classification="attempted_timeout"
                    elif outcome.get("solver_completed") is False:status="failed";classification="attempted_terminal_failure"
                    else:
                        observation["reason"]="sealed worker outcome lacks retained full contract checkpoint; no cancellation invented"
                        observation["phase_states"]={"producer-source":"attempted_unverified","verifier-replay":"attempt_status_unknown"}
                        cases.append(observation)
                        continue
                    observation["state"]=classification;observation["canonical_row"]=True
                    counters={k:v for k,v in outcome.get("counters",{}).items() if type(v) in [int,str]}
                    row={"run_id":"producer-source-"+case["id"],"arm":"producer-source","instance_id":case["id"],"seed":42,"replicate":0,"warmup":False,"status":status,"outcome_class":"completed" if status in ["succeeded","failed"] else "not_completed","attempts":1,"resumed":0,"checkpoint":None,
                        "measurements":{"counted":{"cell_contract_pass":1} if verified else {},"wall_seconds":None,"wall_valid":False,"cpu_user_seconds":None,"cpu_sys_seconds":None,"max_rss_bytes":None,"counters":counters},
                        "isolation":{"tier":None,"contended":None,"blockers":["failure diagnostics; no timing claim"]},"verification":{"status":"verified" if verified else "not_run","verifier":"retained source-bound driver checkpoint" if verified else None},"exclusion":{"excluded":not verified,"rule":"functional validation incomplete or failed" if not verified else None},"artifacts":observation["evidence"],"started_at":None,"finished_at":None,"worker":None,"backend_ref":str(cell.relative_to(fixture))}
                    runs.append(row)
                    observation["phase_states"]={"producer-source":classification,"verifier-replay":"attempt_status_unknown"}
                    if verified:
                        replay_artifacts=phase_replay_evidence(fixture,state,case)
                        replay=json.loads(json.dumps(row));replay.update(run_id="verifier-replay-"+case["id"],arm="verifier-replay",artifacts=replay_artifacts)
                        replay["measurements"]["counters"]={}
                        runs.append(replay);observation["phase_states"]["verifier-replay"]="verified_completed"
        except Exception as error:
            observation["reason"]=str(error)
            if not observation["canonical_row"]:observation["state"]="attempt_status_unknown"
        observation.setdefault("phase_states",{"producer-source":observation["state"],"verifier-replay":"attempt_status_unknown"})
        cases.append(observation)
    return {"cases":cases,"runs":runs,"referenced_files":files,"settled":settled}
