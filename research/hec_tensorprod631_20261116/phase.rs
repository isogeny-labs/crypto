use std::time::Instant;
pub struct Span{name:String,start:Option<Instant>}
impl Span{pub fn new(name:&str)->Self{Self{name:name.into(),start:std::env::var_os("HEC_PHASE_LOG").map(|_|Instant::now())}}}
impl Drop for Span{fn drop(&mut self){if let Some(t)=self.start{println!("TRACE_PHASE name={} wall_ns={} complete={} nested=true",self.name,t.elapsed().as_nanos(),!std::thread::panicking());}}}
