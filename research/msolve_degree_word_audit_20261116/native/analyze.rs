// Exact known-answer analysis. No timing or statistical claims.
use std::{collections::BTreeMap, env, fs, path::Path};
type Poly = BTreeMap<(u32,u32),u32>;
fn number(s:&str)->Result<u32,String>{s.parse().map_err(|_|format!("invalid integer {s:?}"))}
fn poly(s:&str)->Result<Poly,String>{
    let s:String=s.chars().filter(|c|!c.is_whitespace()).collect();
    if !s.is_ascii(){return Err("non-ASCII polynomial".into())}
    if s.is_empty(){return Err("empty polynomial".into())}
    let mut p=Poly::new(); let mut i=0;
    while i<s.len(){
        let neg=if s.as_bytes()[i]==b'-'{i+=1;true}else{if s.as_bytes()[i]==b'+'{i+=1;}false};
        let begin=i; while i<s.len()&&!matches!(s.as_bytes()[i],b'+'|b'-'){i+=1;}
        let term=&s[begin..i]; if term.is_empty(){return Err("empty term".into())}
        let(mut c,mut x,mut y)=(1u32,0u32,0u32);let(mut seen_c,mut seen_x,mut seen_y)=(false,false,false);
        for f in term.split('*'){
            if f.starts_with('x')||f.starts_with('y'){
                let e=if f.len()==1{1}else if &f[1..2]=="^"{number(&f[2..])?}else{return Err(format!("bad factor {f}"))};
                if f.starts_with('x'){if seen_x{return Err("repeated x".into())}seen_x=true;x=e;}
                else{if seen_y{return Err("repeated y".into())}seen_y=true;y=e;}
            }else{if seen_c{return Err("repeated coefficient".into())}seen_c=true;c=number(f)?%101;}
        }
        if neg&&c!=0{c=101-c;}
        let v=(p.get(&(x,y)).copied().unwrap_or(0)+c)%101;
        if v==0{p.remove(&(x,y));}else{p.insert((x,y),v);}
    }
    Ok(p)
}
fn expected(n:u32)->Vec<Poly>{vec![BTreeMap::from([((0,0),1),((n,0),1)]),BTreeMap::from([((0,1),1)])]}
fn inputs(path:&Path,n:u32)->Result<Vec<Poly>,String>{
    let text=fs::read_to_string(path).map_err(|e|e.to_string())?;
    let mut lines=text.lines(); if lines.next()!=Some("x,y")||lines.next()!=Some("101"){return Err("input header".into())}
    let body=lines.collect::<Vec<_>>().join("");let p:Vec<_>=body.split(',').map(poly).collect::<Result<_,_>>()?;
    let wanted=vec![BTreeMap::from([((0,0),1),((n,0),1)]),BTreeMap::from([((n-1,1),1)])];
    if p!=wanted{return Err("input polynomial mismatch".into())}Ok(p)
}
fn basis(path:&Path)->Result<Vec<Poly>,String>{
    let text=fs::read_to_string(path).map_err(|e|e.to_string())?;
    if !text.contains("#field characteristic: 101")||!text.contains("#variable order:       x, y")||!text.contains("#monomial order:       graded reverse lexicographical"){return Err("basis field/order header".into())}
    let start=text.find('[').ok_or("missing basis start")?;
    let end=text.rfind(']').ok_or("missing basis end")?;
    if end<=start||text[end+1..].trim()!=":"{return Err("basis terminator".into())}
    text[start+1..end].split(',').map(poly).collect()
}
fn lean_polys(p:&[Poly])->String{
    format!("[{}]",p.iter().map(|p|format!("[{}]",p.iter().map(|(&(x,y),&c)|format!("({x}, {y}, {c})")).collect::<Vec<_>>().join(", "))).collect::<Vec<_>>().join(", "))
}
fn lean_rows(rows:&[Vec<u64>])->String{format!("[{}]",rows.iter().map(|r|format!("[{}]",r.iter().map(u64::to_string).collect::<Vec<_>>().join(", "))).collect::<Vec<_>>().join(", "))}
fn run()->Result<(),String>{
    let a:Vec<String>=env::args().collect();if a.len()!=5{return Err("usage: analyze CASE INPUT PREFIX OUTPUT_DIR".into())}
    let name=&a[1];let n=match name.as_str(){"control"=>2,"boundary"=>65535,_=>return Err("case".into())};
    let pre=&a[3];let out=Path::new(&a[4]);fs::create_dir_all(out).map_err(|e|e.to_string())?;
    let mut errors=Vec::new();
    let inp=match inputs(Path::new(&a[2]),n){Ok(p)=>p,Err(e)=>{errors.push(e);vec![]}};
    let exit=fs::read_to_string(format!("{pre}.exit")).unwrap_or_else(|_|"missing".into());
    if exit.trim()!="0"{errors.push(format!("native exit {}",exit.trim()));}
    let p=match basis(Path::new(&format!("{pre}.out"))){Ok(p)=>p,Err(e)=>{errors.push(e);vec![]}};
    let mut sorted=p.clone();sorted.sort();let mut want=expected(n);want.sort();
    if sorted!=want{errors.push("basis differs from exact registered answer".into());}
    let stderr=fs::read_to_string(format!("{pre}.stderr")).map_err(|e|e.to_string())?;
    let(mut lc,mut rows,mut sels)=(Vec::<Vec<u64>>::new(),Vec::<Vec<u64>>::new(),Vec::<Vec<u64>>::new());
    for line in stderr.lines(){
        if line.starts_with("WORD_LOG_CAP"){errors.push("degree logging cap hit".into());continue}
        let(tag,len)=if line.starts_with("WORD_LCM "){("WORD_LCM",8)}else if line.starts_with("WORD_ROW "){("WORD_ROW",4)}else if line.starts_with("WORD_SELECT "){("WORD_SELECT",2)}else{continue};
        let v:Vec<u64>=line.split_whitespace().skip(1).map(|x|x.parse::<u64>().map_err(|_|"log integer".to_string())).collect::<Result<_,_>>()?;
        if v.len()!=len{return Err("log arity".into())}
        match tag{
            "WORD_LCM"=>{if v[4]!=v[0].max(v[2])||v[5]!=v[1].max(v[3])||v[7]!=v[4]+v[5]||v[6]!=v[7]%65536{errors.push("LCM coordinate/word invariant".into());}lc.push(v);},
            "WORD_ROW"=>{if v[0]>65535||v[1]>65535||v[2]>131070||v[3]==0{errors.push("row range/empty".into());}rows.push(v);},
            _=>{if v[1]!=2147483647{errors.push("unexpected max degree setting".into());}sels.push(v);}
        }
    }
    let true_degree=u64::from(n)+1;
    let stored_degree=true_degree%65536;
    if !lc.iter().any(|v|v[4]==u64::from(n)&&v[5]==1&&v[6]==stored_degree&&v[7]==true_degree){errors.push("registered LCM event absent".into());}
    if !rows.iter().any(|v|v[1]<=u64::from(n).max(stored_degree)&&v[2]==true_degree){errors.push("registered multiplied row event absent".into());}
    if !sels.iter().any(|v|v[0]==stored_degree){errors.push("registered selection event absent".into());}
    if stderr.contains("AddressSanitizer")||stderr.contains("runtime error:"){errors.push("sanitizer diagnostic".into());}
    if lc.is_empty()||rows.is_empty()||sels.is_empty(){errors.push("missing required degree observations".into());}
    let bound=rows.iter().map(|r|r[1]).chain(sels.iter().map(|s|s[0])).fold(u64::from(n),u64::max);
    if bound!=if n==2{3}else{65535}{errors.push("unexpected observed stored bound".into());}
    let stem=if n==2{"Control"}else{"Boundary"};
    let constants=format!("def native{stem}Inputs : List (List (Nat × Nat × Nat)) := {}\ndef native{stem}Basis : List (List (Nat × Nat × Nat)) := {}\ndef native{stem}LCM : List (List Nat) := {}\ndef native{stem}Rows : List (List Nat) := {}\ndef native{stem}Selections : List (List Nat) := {}\ndef native{stem}StoredBound : Nat := {bound}\n",lean_polys(&inp),lean_polys(&p),lean_rows(&lc),lean_rows(&rows),lean_rows(&sels));
    fs::write(out.join(format!("{name}-constants.lean.txt")),constants).map_err(|e|e.to_string())?;
    let errs=errors.iter().map(|e|format!("{:?}",e)).collect::<Vec<_>>().join(",");
    let result=format!("{{\"case\":\"{name}\",\"accepted\":{},\"native_exit\":{:?},\"basis_count\":{},\"lcm_records\":{},\"row_records\":{},\"selection_records\":{},\"observed_stored_bound\":{},\"errors\":[{}]}}\n",errors.is_empty(),exit.trim(),p.len(),lc.len(),rows.len(),sels.len(),bound,errs);
    fs::write(out.join(format!("{name}-analysis.json")),&result).map_err(|e|e.to_string())?;print!("{result}");
    if errors.is_empty(){Ok(())}else{Err("registered exact checks failed".into())}
}
fn main(){if let Err(e)=run(){eprintln!("analysis failure: {e}");std::process::exit(1)}}
