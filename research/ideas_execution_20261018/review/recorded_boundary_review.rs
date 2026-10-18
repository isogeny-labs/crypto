use serde_json::Value;
use std::{fs,io::Write};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let root="/workspace/crypto/research/ideas_execution_20261018";
    let fourier:Value=serde_json::from_str(&fs::read_to_string(format!("{root}/fourier_recovery.json"))?)?;
    let row=fourier["rows"].as_array().unwrap().iter().find(|row|
        row["r"]==5 && row["Q"]==16 && row["synthetic_secret"]==2).unwrap();
    let (r,q,d)=(5usize,16usize,2usize);
    let mut expected=vec![0.0;q*q];
    // Direct trigonometric sum, no FFT and no production decoder reuse.
    for u in 0..q {for v in 0..q {
        for z in 0..r {
            let(mut real,mut imaginary)=(0.0,0.0);
            for a in 0..q {for b in 0..q {
                if(a+d*b)%r==z {
                    let angle=2.0*std::f64::consts::PI*(u*a+v*b)as f64/q as f64;
                    real+=angle.cos();imaginary+=angle.sin();
                }
            }}
            expected[u*q+v]+=(real*real+imaginary*imaginary)/(q as f64).powi(4);
        }
    }}
    let actual=row["outcome_probabilities_row_major_u_v"].as_array().unwrap();
    assert_eq!(actual.len(),expected.len());
    let error=actual.iter().zip(&expected).map(|(a,b)|(a.as_f64().unwrap()-b).abs()).fold(0.0f64,f64::max);
    assert!(error<1e-12,"direct DFT probability disagreement: {error}");
    let sum:f64=expected.iter().sum();assert!((sum-1.0).abs()<1e-12);
    let work=(r*q*q*q.trailing_zeros()as usize)as u64;
    assert_eq!(row["fft_butterflies"].as_u64().unwrap(),work);
    // Independent exact zero-frequency probability from integer fiber sizes.
    let mut fibers=vec![0usize;r];for a in 0..q {for b in 0..q {fibers[(a+d*b)%r]+=1;}}
    let p00=fibers.iter().map(|x|x*x).sum::<usize>()as f64/(q*q*q*q)as f64;
    assert!((actual[0].as_f64().unwrap()-p00).abs()<1e-14);

    let quantum:Value=serde_json::from_str(&fs::read_to_string(format!("{root}/quantum_boundary.json"))?)?;
    let mut congruences=0;
    for row in quantum["diagnostics"].as_array().unwrap() {
        let r:i128=row["r"].as_str().unwrap().parse()?;
        let a:i128=row["a"].as_str().unwrap().parse()?;
        let b=row["b"].as_i64().unwrap()as i128;
        if let Some(answer)=row["recovered"].as_str() {
            let x:i128=answer.parse()?;
            assert_eq!((a+b*x).rem_euclid(r),0);
            assert_eq!(x,row["synthetic_secret"].as_i64().unwrap()as i128);
        } else {assert_eq!(b,0);}
        congruences+=1;
    }
    for row in quantum["resources"].as_array().unwrap() {
        let n=row["field_bits"].as_u64().unwrap();
        let ceil_log=64-(n-1).leading_zeros()as u64;
        assert_eq!(row["rnsl2017_logical_qubit_bound"].as_u64().unwrap(),9*n+2*ceil_log+10);
        assert_eq!(row["modeled_compressed_table_bytes"].as_u64().unwrap(),2*(n+1)*((n+7)/8+1));
        assert_eq!(row["classical_setup_point_doublings_once"].as_u64().unwrap(),2*n);
    }
    let report=serde_json::json!({"scope":"independent internal cross-track numerical/integer checks, not external formal review",
        "direct_dft_fixture":{"r":r,"Q":q,"d":d,"outcomes_checked":q*q,"maximum_probability_error":error,"normalization":sum,"zero_frequency_probability":p00,"fft_butterflies":work},
        "exact_congruences_checked":congruences,"resource_rows_checked":quantum["resources"].as_array().unwrap().len(),"passed":true});
    let output=std::env::args().nth(1).ok_or("review output required")?;
    let mut f=fs::OpenOptions::new().write(true).create_new(true).open(output)?;
    serde_json::to_writer_pretty(&mut f,&report)?;f.write_all(b"\n")?;
    println!("{report}");Ok(())
}
