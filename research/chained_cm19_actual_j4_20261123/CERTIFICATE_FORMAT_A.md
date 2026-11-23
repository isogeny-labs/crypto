# Complete original native certificate format A

Little endian. File created once at the sole output argv path, written
and flushed section by section. Header8bytes CTJ4A001, u32 version1,
u32 q117649 and6 u32 planned counts(16,11,4096,64,1331,15):40 bytes.
Header counts are prospective; each section declares its observed count.
Partial original files lack the final8byte COMPLETE footer and are never
represented as qualified complete data. [pcc:unverified]

Every section has4ASCII tag bytes,u32 recordcount,u32 recordwidth and
u32 payloadbytes, then the entire payload. Width0 means the explicitly
defined scan bitset. Planned full sections in order:

| Tag | Count | Width | Complete record |
|---|---:|---:|---|
| E4XS |117649|0| One bit per x encoding, little bit order in each byte;1 iff divisionfactor vanishes.14707 bytes, unused high padding bits0. |
| E4PT |16|9| O flag:u8(1 iff O), x:u32,y:u32; zeros for O. Sorted point order, O first. |
| E4AD |256|1| Actual point sum index for row-major16x16. |
| E4FR |16|3| Frobenius,negative,double indices. |
| E4WP |256|1| Actual Weil value exponent0..3 relative to i=3b, row-major16x16. |
| FIBR |15|166| Pindex:u8,piPindex:u8; f0,f1,f3,jet each degree:u8,constant:u32,9 coordinate coefficients:u32 ordered XYZ then t0,t1,t2, padding0 for degree2. |
| J4MP |4096|7| Sindex:u16,Rindex:u16,Frobeniusindex:u16,kernelFlag:u8; input index is row number. |
| KERN |64|2| Full sorted kernel input indices. |
| K4WP |4096|1| Actual H-Weil exponent for row-major kernel-pair list. |
| E11P |11|9| Actual rational E points, same point encoding. |
| E11A |121|1| Actual rational point sum table, row-major. |
| J11M |1331|6| Sindex:u16,Rindex:u16,[4]index:u16. |
| ELDL |11|6| Original ECpointindex:u8,embeddedJindex:u16,SimageJindex:u16,recoveredECpointindex:u8. |

Product index i0+n*i1+n2*i2; it always refers to the explicitly encoded
actual elliptic point table through the proven geometric beta map.
Planned complete byte size59325 includes header,13 section headers and
footer. Human-readable stdout also prints all torsion points/pairings,
kernel members, scalar checks, controls and summaries. Unexpected counts
or larger/partial files stay whole; no post-output splitting/truncation.
All metadata hashes/custody are captured by the existing runner, not by
a Python scientific verifier. [pcc:unverified]
