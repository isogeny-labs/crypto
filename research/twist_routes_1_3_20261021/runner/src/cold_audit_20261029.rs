//! Independent raw trace/finite-output audit and GP certificate generation.
use crate::{
    driver::{read, sha, write},
    engine::Task,
    Result,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
    process::Command,
};
fn digest(b: &[u8]) -> String {
    hex::encode(crate::source_sha256::sha256(b))
}
fn payload(dir: &Path, index: &Value, name: &str) -> Result<Vec<u8>> {
    let entry = index["compressed_files"]
        .as_array()
        .ok_or("compression inventory")?
        .iter()
        .find(|v| v["uncompressed_name"] == name)
        .ok_or("compressed payload absent")?;
    let path = dir.join(entry["path"].as_str().unwrap());
    if sha(&path)? != entry["sha256"] {
        return Err("compressed bytes changed".into());
    }
    let out = Command::new("xz").args(["-dc"]).arg(&path).output()?;
    if !out.status.success() || digest(&out.stdout) != entry["uncompressed_sha256"] {
        return Err("decompressed payload hash changed".into());
    }
    Ok(out.stdout)
}
fn p(v: &Value) -> String {
    if v.is_null() {
        "[0]".into()
    } else {
        serde_json::to_string(v).unwrap()
    }
}
fn plist(v: &Value) -> String {
    format!(
        "[{}]",
        v.as_array()
            .unwrap()
            .iter()
            .map(p)
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn num(v: &Value) -> u64 {
    v.as_u64().unwrap()
}
const GP_HEADER: &str = r#"default(parisizemax,4000000000);
checked=0;
ck(c,s)={if(!c,error(s))};
cn(p)={if(p==[0],[0],lift(p))};
coldgcollect(ee,bb,targets)={my(mp=Map(),vals,sm,out,all=List(),rr,pa,pb);for(ii=1,#bb,for(jj=ii,#bb,for(ss=0,3,pa=if(bittest(ss,0),ellneg(ee,bb[ii]),bb[ii]);pb=if(bittest(ss,1),ellneg(ee,bb[jj]),bb[jj]);sm=cn(elladd(ee,pa,pb));if(!mapisdefined(mp,sm,&vals),vals=List());listput(vals,[ii-1,jj-1,ss]);mapput(mp,sm,vals))));for(qi=1,#targets,out=List();if(mapisdefined(mp,cn(targets[qi]),&vals),for(h=1,#vals,listput(out,[2,vals[h][1],vals[h][2],-1,vals[h][3]])));for(kk=1,#bb,for(ss=0,1,rr=cn(ellsub(ee,targets[qi],if(ss,ellneg(ee,bb[kk]),bb[kk])));if(mapisdefined(mp,rr,&vals),for(h=1,#vals,listput(out,[3,vals[h][1],vals[h][2],kk-1,vals[h][3]+4*ss])))));listput(all,vecsort(Vec(out))));Vec(all)};
coldrows(bb,ww,r)={my(out=List(),z,sg);for(qi=1,#ww,for(h=1,#ww[qi],z=vector(#bb+2);for(k=1,ww[qi][h][1],sg=if(bittest(ww[qi][h][5],k-1),-1,1);z[ww[qi][h][k+1]+1]=(z[ww[qi][h][k+1]+1]+r+sg)%r);z[#bb+1]=r-1;z[#bb+2]=(qi-1)%r;listput(out,z)));Set(Vec(out))};
cfiber(rows,n,r,part,kern,rank)={my(mm,rhs,kk,zz,rr);ck(#part==n&&#kern==n-rank,"fiber dimensions");if(#kern,kk=matrix(n,#kern,i,j,Mod(kern[j][i],r));ck(matrank(kk)==#kern,"kernel independent"));if(!#rows,ck(rank==0&&#kern==n,"empty equations");return(0));mm=matrix(#rows,n,i,j,Mod(rows[i][j],r));rhs=Col(vector(#rows,i,Mod(rows[i][n+1],r)));ck(matrank(mm)==rank,"rank");ck(mm*Col(part)==rhs,"particular");if(#kern,ck(mm*kk==matrix(#rows,#kern,i,j,Mod(0,r)),"kernel equations"));zz=matker(mm);rr=n-matrank(mm);sum(j=1,rr,lift(zz[n,j])!=0)==0};
crreduce(qr,cp,cols,r)={my(nn=#cp);vector(#qr,i,vector(#cols+2,j,lift(Mod(if(j<=#cols,sum(t=1,nn,qr[i][t]*cols[j][t]),if(j==#cols+1,qr[i][nn+1],qr[i][nn+2]-sum(t=1,nn,qr[i][t]*cp[t]))),r))))};
"#;
fn finite_case(
    script: &mut fs::File,
    d: &Value,
    case: usize,
    seen_census: &mut BTreeSet<String>,
    seen_alpha: &mut BTreeSet<String>,
) -> Result<()> {
    let t: Task = serde_json::from_value(d["task"].clone())?;
    let f = &t.fixture;
    let base = &d["base"];
    let n = base.as_array().map_or(0, Vec::len) + 1;
    writeln!(script,"/* finite case {case} */\nee=ellinit([{},{}],{});gg={};qq={};rr={};ck(isprime(rr)&&ellisoncurve(ee,gg)&&ellisoncurve(ee,qq)&&ellmul(ee,gg,rr)==[0],\"source subgroup\");",f.a,f.b,f.p,p(&serde_json::to_value(f.g())?),p(&serde_json::to_value(t.target)?),f.r)?;
    if d["status"] == "recovered" && base.is_null() {
        writeln!(
            script,
            "ck(ellmul(ee,gg,{})==qq,\"reference scalar\");checked++;",
            d["recovered_scalar"]
        )?;
        return Ok(());
    }
    if base.is_null() {
        writeln!(script, "checked++;\n")?;
        return Ok(());
    }
    writeln!(script,"bb={};ck(bb[1]==gg,\"generator anchor\");for(i=1,#bb,ck(ellisoncurve(ee,bb[i])&&ellmul(ee,bb[i],rr)==[0]&&bb[i][2]<=({}-1)/2,\"canonical subgroup base\"));",plist(base),f.p)?;
    if d["alpha"].is_object() {
        let a = &d["alpha"];
        let key = digest(&serde_json::to_vec(&json!([f.id, a]))?);
        if seen_alpha.insert(key) {
            writeln!(script,"roots={};tt=ellinit([{},{}],{});expn=ellgroup(tt)[1];hh=[0];for(xi=0,{}-1,ys=ellordinate(tt,xi);if(#ys,yy=lift(ys[1]);yy=min(yy,{}-yy);if(yy,pp=[Mod(xi,{}),Mod(yy,{})];if(ellorder(tt,pp,{})==expn,hh=pp;break))));ck(hh!=[0]&&expn%17==0,\"maximal seed / kernel support\");kk=ellmul(tt,hh,expn/17);ck(ellorder(tt,kk)==17,\"kernel order\");ck(Set(vector(16,i,lift(ellmul(tt,kk,i)[1]/{})))==roots,\"kernel x roots\");vv=ellisogeny(ee,prod(i=1,#roots,'x-Mod(roots[i],{})));ck(vv[1]==[0,0,0,Mod({},{}),Mod({},{})],\"Velu quotient\");",a["roots"],f.twist().a,f.twist().b,f.p,f.p,f.p,f.p,f.p,f.tn(),f.d,f.p,a["quotient"]["a"],f.p,a["quotient"]["b"],f.p)?;
            writeln!(script,"tv={};wv={};uu={};rmap=('x+sum(i=1,#roots,Mod(tv[i],{})/('x-roots[i])+Mod(wv[i],{})/('x-roots[i])^2))*uu^2;smap=(1-sum(i=1,#roots,Mod(tv[i],{})/('x-roots[i])^2+2*Mod(wv[i],{})/('x-roots[i])^3))*uu^3;ck(vv[2][1]/vv[2][3]^2*uu^2==rmap&&vv[2][2]/'y/vv[2][3]^3*uu^3==smap,\"generic partial-fraction map\");ck(Mod({},{} )*uu^4==ee.a4&&Mod({},{} )*uu^6==ee.a6,\"source endpoint isomorphism\");",a["t"],a["w"],a["scale"],f.p,f.p,f.p,f.p,a["quotient"]["a"],f.p,a["quotient"]["b"],f.p)?;
        }
        // Reinitialize from recorded roots for every case; the generic identity above
        // is cached only after byte-identical map data on the same model is checked.
        writeln!(script,"roots={};uu={};lambda={};vv=ellisogeny(ee,prod(i=1,#roots,'x-Mod(roots[i],{})));for(i=1,#bb,pp=ellisogenyapply(vv[2],bb[i]);if(pp!=[0],pp=[pp[1]*uu^2,pp[2]*uu^3]);ck(pp==ellmul(ee,bb[i],lambda),\"actual map scalar action\"));",a["roots"],a["scale"],a["lambda"],f.p)?;
    }
    if d["query_targets"].is_null() {
        writeln!(script, "checked++;\n")?;
        return Ok(());
    }
    let query_key = digest(&serde_json::to_vec(&json!([
        f.id,
        base,
        d["query_targets"],
        d["witnesses"]
    ]))?);
    writeln!(script,"targets={};ww={};for(j=1,#targets,ck(targets[j]==elladd(ee,qq,ellmul(ee,gg,j-1)),\"query sequence\"));",plist(&d["query_targets"]),d["witnesses"])?;
    if seen_census.insert(query_key) {
        writeln!(
            script,
            "ck(coldgcollect(ee,bb,targets)==ww,\"complete signed witness census\");"
        )?;
    }
    writeln!(
        script,
        "qr={};kr={};ck(coldrows(bb,ww,rr)==qr,\"deduplicated query rows\");allrows=concat(kr,qr);",
        d["query_rows"], d["known_rows"]
    )?;
    // Verify every known coefficient relation via point multiplication; this also
    // rejects a corrupt multiplier without consulting any factor-base log table.
    writeln!(script,"for(i=1,#kr,sm=[0];for(j=1,#bb,sm=elladd(ee,sm,ellmul(ee,bb[j],kr[i][j])));ck(sm==ellmul(ee,gg,kr[i][#bb+2]),\"known coefficient equation\"));")?;
    let fib = &d["fiber"];
    writeln!(script,"part={};kern={};fixed=cfiber(allrows,{},rr,part,kern,{});ck(fixed=={},\"target projection\");",fib["particular"],fib["kernel"],n,fib["rank"],if d["status"]=="recovered"{1}else{0})?;
    if d["status"] == "recovered" {
        writeln!(
            script,
            "ck(part[{}]=={}&&ellmul(ee,gg,{})==qq,\"original target recovery\");",
            n, d["recovered_scalar"], d["recovered_scalar"]
        )?;
    } else {
        writeln!(script,"alt={};ck(alt[{}]!=part[{}],\"distinct target witness\");mm=matrix(#allrows,{},i,j,Mod(allrows[i][j],rr));rhs=Col(vector(#allrows,i,Mod(allrows[i][{}+1],rr)));ck(mm*Col(alt)==rhs,\"ambiguity satisfies every equation\");",d["ambiguity_alternative"],n,n,n,n)?;
    }
    if d["compression"].is_object() {
        let c = &d["compression"];
        let b = n - 1;
        writeln!(script,"cp={};cols={};km=matrix(#kr,{},i,j,Mod(kr[i][j],rr));kb=Col(vector(#kr,i,Mod(kr[i][{}+2],rr)));ck(km*Col(cp)==kb,\"known affine particular\");ck(#cols=={}-matrank(km),\"known kernel dimension\");if(#cols,cm=matrix({},#cols,i,j,Mod(cols[j][i],rr));ck(matrank(cm)==#cols&&km*cm==matrix(#kr,#cols,i,j,Mod(0,rr)),\"known kernel spans\"));",c["base"],c["columns"],b,b,b,b)?;
        let rf = &d["reduced_fiber"];
        writeln!(script,"rpart={};rkern={};rrows=crreduce(qr,cp,cols,rr);ck(cfiber(rrows,#cols+1,rr,rpart,rkern,{})==fixed,\"reduced target fiber\");epart=concat(vector({},i,lift(Mod(cp[i]+sum(j=1,#cols,rpart[j]*cols[j][i]),rr))),[rpart[#cols+1]]);ekern=vector(#rkern,h,concat(vector({},i,lift(Mod(sum(j=1,#cols,rkern[h][j]*cols[j][i]),rr))),[rkern[h][#cols+1]]));ck(epart==part&&ekern==kern,\"full affine expansion\");",rf["particular"],rf["kernel"],rf["rank"],b,b)?;
    } else {
        writeln!(
            script,
            "ck({}==part&&{}==kern,\"uncompressed fiber retained\");",
            d["reduced_fiber"]["particular"], d["reduced_fiber"]["kernel"]
        )?;
    }
    writeln!(script, "checked++;\n")?;
    Ok(())
}
pub fn audit(plan: &Path, run: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh audit directory required".into());
    }
    fs::create_dir_all(out)?;
    let plan = read(plan)?;
    let lines = fs::read_to_string(run.join("ROWS.jsonl"))?;
    let mut gp = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out.join("verify.gp"))?;
    write!(gp, "{GP_HEADER}")?;
    let mut audited = vec![];
    let mut math_seen = BTreeMap::<String, usize>::new();
    let mut census = BTreeSet::new();
    let mut maps = BTreeSet::new();
    for line in lines.lines() {
        let row: Value = serde_json::from_str(line)?;
        let i = num(&row["index"]) as usize;
        let cell = &plan["cells"][i];
        if row["task_id"] != cell["task"]["id"] {
            return Err("row/plan identity".into());
        }
        if row["status"] == "not-started-global-cap" {
            audited.push(row);
            continue;
        }
        let dir = run.join(format!("cell-{i:05}"));
        if sha(dir.join("task.json"))? != row["task_sha256"] {
            return Err("task bytes".into());
        }
        let idx = read(dir.join("PROFILE_INDEX.json"))?;
        let mut data = None;
        let mut raw_traces = vec![];
        for rec in idx["compressed_files"].as_array().unwrap() {
            let name = rec["uncompressed_name"].as_str().unwrap();
            let bytes = payload(&dir, &idx, name)?;
            if name == "worker.json" {
                data = Some(serde_json::from_slice::<Value>(&bytes)?);
            }
            if name.starts_with("callgrind.") {
                let text = String::from_utf8(bytes)?;
                let mut pid = 0u32;
                let mut part = 0u64;
                let mut ir = None;
                let mut events = String::new();
                let mut trigger = String::new();
                for l in text.lines() {
                    if let Some(x) = l.strip_prefix("pid:") {
                        pid = x.trim().parse()?;
                    }
                    if let Some(x) = l.strip_prefix("part:") {
                        part = x.trim().parse()?;
                    }
                    if let Some(x) = l.strip_prefix("events:") {
                        events = x.trim().into();
                    }
                    if let Some(x) = l.strip_prefix("desc: Trigger:") {
                        trigger = x.trim().into();
                    }
                    if let Some(x) = l.strip_prefix("totals:") {
                        ir = Some(x.trim().parse::<u64>()?);
                    }
                }
                if events != "Ir" {
                    return Err("audit event set".into());
                }
                raw_traces.push((pid, part, trigger, ir.ok_or("audit total missing")?));
            }
        }
        if row["complete_attempt_cost"] == true {
            let data = data.as_ref().ok_or("worker data missing")?;
            let native = num(&data["native_pid"]) as u32;
            let backend = num(&data["gp_pid"]) as u32;
            let mut costs = BTreeMap::<String, u64>::new();
            let marks = data["result"]["markers"].as_array().unwrap();
            for (pid, part, label, ir) in raw_traces {
                if pid == native {
                    if let Some(name) = label.strip_prefix("Client Request: ") {
                        *costs.entry(name.into()).or_default() += ir;
                    } else if ir != 0 {
                        return Err("audit unassigned native cost".into());
                    }
                } else if pid == backend {
                    let mark = marks.iter().find(|m| {
                        part > num(&m["gp_calls_before"]) && part <= num(&m["gp_calls_after"])
                    });
                    if let Some(m) = mark {
                        *costs
                            .entry(m["phase"].as_str().unwrap().into())
                            .or_default() += ir;
                    } else if ir != 0 {
                        return Err("audit unassigned backend cost".into());
                    }
                } else if ir != 0 {
                    return Err("audit unknown process cost".into());
                }
            }
            for p in row["cost"]["exclusive_phases"].as_array().unwrap() {
                if costs.get(p["phase"].as_str().unwrap()).copied() != p["total_ir"].as_u64() {
                    return Err("independent phase counter mismatch".into());
                }
            }
            if costs.values().sum::<u64>() != num(&row["cost"]["total_ir"]) {
                return Err("independent total mismatch".into());
            }
            let d = &data["result"];
            let key = digest(&serde_json::to_vec(&json!([
                d["task"]["fixture"],
                d["task"]["target"],
                d["base"],
                d["alpha"],
                d["known_rows"],
                d["query_targets"],
                d["witnesses"],
                d["query_rows"],
                d["fiber"],
                d["reduced_fiber"],
                d["compression"],
                d["ambiguity_alternative"],
                d["recovered_scalar"]
            ]))?);
            let id = if let Some(id) = math_seen.get(&key) {
                *id
            } else {
                let id = math_seen.len();
                finite_case(&mut gp, d, id, &mut census, &mut maps)?;
                math_seen.insert(key.clone(), id);
                id
            };
            let mut z = row.clone();
            z["independent_counter_audit"] = json!(true);
            z["finite_GP_case"] = json!(id);
            z["finite_GP_payload_sha256"] = json!(key);
            audited.push(z);
        } else {
            audited.push(row);
        }
    }
    writeln!(gp,"ck(checked=={},\"finite proof population\");print(\"ALL_COLD_PIPELINE_REPLAY_OK \",checked);",math_seen.len())?;
    write(
        out.join("AUDIT.json"),
        &json!({"schema":"twist.cold-audit/v1","plan_sha256":digest(&serde_json::to_vec(&plan)?),"actual_rows":audited.len(),"planned_cells":plan["planned_cells"],"unique_GP_cases":math_seen.len(),"unique_complete_witness_checks":census.len(),"unique_generic_maps":maps.len(),"independent_counter_aggregation":true,"rows":audited,"GP_script_sha256":sha(out.join("verify.gp"))?,"mathematical_status":"awaiting strict independent GP process"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_gp_census_and_fiber_controls() {
        let script=format!("{GP_HEADER}\nee0=ellinit([1,13],59);g0=[1,29];ww0=coldgcollect(ee0,[g0],[ellmul(ee0,g0,2),[0],g0]);ck(ww0==[[[2,0,0,-1,0]],[[2,0,0,-1,1],[2,0,0,-1,2]],[[3,0,0,0,1],[3,0,0,0,2],[3,0,0,0,4]]],\"signed repeat / identity census\");ck(coldrows([g0],[ww0[1]],67)==[[2,66,0]],\"query equation\");ck(cfiber([[1,0,1],[2,66,0]],2,67,[1,2],[],2),\"fixed target\");ck(!cfiber([[1,0,1]],2,67,[1,0],[[0,1]],1),\"ambiguous target\");print(\"COLD_AUDIT_HEADER_CONTROLS_OK\");quit;\n");
        let mut child = Command::new(crate::driver::GP)
            .args(["-f", "-q", "-s", "64000000"])
            .env(
                "LD_LIBRARY_PATH",
                "/workspace/shared/twist-tools-20261023/cas/root/usr/lib/x86_64-linux-gnu",
            )
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(script.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        let stdout = String::from_utf8(out.stdout).unwrap();
        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(out.status.success());
        assert_eq!(stdout.trim(), "COLD_AUDIT_HEADER_CONTROLS_OK", "{stderr}");
        assert!(
            stderr.trim().is_empty()
                || stderr.trim()
                    == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes).",
            "{stderr}"
        );
    }
}
