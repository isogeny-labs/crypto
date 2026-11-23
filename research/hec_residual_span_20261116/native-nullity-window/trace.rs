use std::cell::Cell;
thread_local!{static ANCHOR:Cell<usize>=const{Cell::new(0)};static CONTEXT:Cell<Option<(usize,u64,u64,usize,usize)>>=const{Cell::new(None)};}
pub fn context(v:(usize,u64,u64,usize,usize)){CONTEXT.with(|c|c.set(Some(v)));}
pub fn emit(l:&crate::linear::Linear,anchor:usize,done:bool){ANCHOR.with(|a|a.set(anchor));if let Some((g,p,sm,case,rep))=CONTEXT.with(Cell::get){let(rho,q,grounded,cells)=l.topology();println!("TRACE g={g} p={p} sm={sm} case={case} rep={rep} anchor={anchor} final={done} rho={rho} free_components={q} grounded={grounded} payload_cells={cells} admitted={}",l.admitted);}}

pub fn row(l:&crate::linear::Linear,determined:bool){let(rho,q,grounded,cells)=l.topology();if rho>2{return;}if let Some((g,p,sm,case,rep))=CONTEXT.with(Cell::get){let anchor=ANCHOR.with(Cell::get);println!("ROWTRACE g={g} p={p} sm={sm} case={case} rep={rep} anchor={anchor} rho={rho} free_components={q} grounded={grounded} payload_cells={cells} admitted={} determined={determined}",l.admitted);}}
