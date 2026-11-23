#[cfg(test)]
mod kummer_driver_protocol_controls {
    use super::*;
    use serde_json::json;
    fn header()->serde_json::Value { json!({"kind":"header","schema":"kummer-driver-events/v1","r":7}) }
    fn identity()->Identity { Identity {source_revision:"a".repeat(40),source_sha256:"b".repeat(64),workload_id:"synthetic-protocol".into(),run_id:"fixed".into()} }
    fn begin()->serde_json::Value { json!({"kind":"driver_begin","operation":"lift","call":1,"input":{"labels":[0,1]}}) }
    fn end()->serde_json::Value { json!({"kind":"driver_end","operation":"lift","call":1,"divisor":{"degree":2,"u":[1,0,1],"v":[0,0]},"labels":[0,1],"status":"fixture","tested":1}) }
    #[test]
    fn missing_call_result_rejected() {
        let mut bytes=vec![];let mut trace=Trace::new(&mut bytes,identity(),20,10000).unwrap();
        {let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,header()).unwrap();
        adapter.observe(header()).unwrap();adapter.observe(begin()).unwrap();
        assert_eq!(adapter.observe(json!({"kind":"terminal","attempts":0,"driver_calls":1,"recovered":null})),Err(Error::State));}
        assert!(!trace.complete());
    }
    #[test]
    fn changed_call_number_rejected() {
        let mut bytes=vec![];let mut trace=Trace::new(&mut bytes,identity(),20,10000).unwrap();
        let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,header()).unwrap();
        adapter.observe(header()).unwrap();adapter.observe(begin()).unwrap();
        let mut event=end();event["call"]=json!(2);assert_eq!(adapter.observe(event),Err(Error::State));
    }
    #[test]
    fn changed_signed_labels_rejected() {
        let mut bytes=vec![];let mut trace=Trace::new(&mut bytes,identity(),20,10000).unwrap();
        let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,header()).unwrap();
        adapter.observe(header()).unwrap();adapter.observe(begin()).unwrap();
        let mut event=end();event["labels"]=json!([1,0]);assert_eq!(adapter.observe(event),Err(Error::State));
    }
    #[test]
    fn missing_lift_disposition_rejected() {
        let mut bytes=vec![];let mut trace=Trace::new(&mut bytes,identity(),20,10000).unwrap();
        let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,header()).unwrap();
        adapter.observe(header()).unwrap();adapter.observe(begin()).unwrap();adapter.observe(end()).unwrap();
        assert_eq!(adapter.observe(json!({"kind":"terminal","attempts":0,"driver_calls":1,"recovered":null})),Err(Error::State));
    }
}
