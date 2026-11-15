//! Fresh, predeclared four-variable confirmation fixtures; no solver calls.
use relation_degree_trace::{projection,workload::generate,Poly};
use serde_json::json;
use std::{error::Error,fs,path::PathBuf};
fn expression(f:&Poly,names:&[String])->String{f.iter().map(|(m,c)|{let mut s=c.to_string();for(e,name)in m.iter().zip(names){if *e>0{s+=&format!("*{name}^{e}");}}s}).collect::<Vec<_>>().join("+")}
fn main()->Result<(),Box<dyn Error>>{
    let out=PathBuf::from(std::env::args().nth(1).ok_or("fresh fixture directory required")?);if out.exists(){return Err("output exists".into());}fs::create_dir_all(&out)?;let mut cases=Vec::new();
    for p in [2147483647,9007199254740881]{for seed in [1009,2003,3001]{let fixture=generate(p,4,seed)?;let root=out.join(&fixture.id);fs::create_dir(&root)?;
        fs::write(root.join("fixture.json"),serde_json::to_string_pretty(&fixture)?+"\n")?;let f=fixture.system.decode()?;let a=projection::augment(&f,4,p).0;
        for(arm,polys)in[("baseline",f),("candidate",a)]{
            let expr=polys.iter().map(|f|expression(f,&fixture.system.variables)).collect::<Vec<_>>().join(",\n");fs::write(root.join(format!("{arm}.ms")),format!("{}\n{p}\n{expr}\n",fixture.system.variables.join(",")))?;
            for algorithm in ["std","slimgb"]{let coeff=if p<2u64.pow(31){p.to_string()}else{format!("(integer,{p})")};
                let mut script=format!("ring r={coeff},({}),dp;\noption(redSB);\nsystem(\"--ticks-per-sec\",1000000);\nideal I={expr};\nideal G;int j;int start=timer;\nfor(j=0;j<100;j++){{G={algorithm}(I);}}\nprint(\"CPU_MICROSECONDS:\"+string(timer-start));\nint i;poly f;\n",fixture.system.variables.join(","));
                script+="for(i=1;i<=size(G);i++){print(\"POLY\");f=G[i];while(f!=0){print(\"TERM:\"+string(leadcoef(f))+\":\"+string(leadexp(f)));f=f-lead(f);}print(\"ENDPOLY\");}\nprint(\"DONE\");quit;\n";fs::write(root.join(format!("{arm}-{algorithm}.sing")),script)?;
            }
        }
        cases.push(json!({"id":fixture.id,"prime":p,"variables":4,"seed":seed,"fixture":format!("{}/fixture.json",fixture.id)}));
    }}
    fs::write(out.join("manifest.json"),serde_json::to_string_pretty(&json!({"schema":"relation-speed-confirmation/v1","cases":cases,"scope":"fresh four-variable panel; seeds frozen before final criterion timings"}))?+"\n")?;Ok(())
}
