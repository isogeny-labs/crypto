# MULT35 exact multiplication, prospective frozen source
Same4GiB/1200s per invocation. Native standalone Rust; no statistical/timing
claim. Exact input154word series from accepted jets35-01, bytes pinned below.
Following Khuri-Makdisi math/0409209v3 section2 RepresentationB, multiply every
unordered pair of the fixed324basis:52650products. Do not stop when708 is
reached. Require rank708, every product coordinate recombination, and every
pivot recombination. Each of52650products must agree between direct GF32
convolution and independent bit-plane binary convolution with descending
polynomial reduction by X5+X2+1. All1024field products independently checked.
Controls reject a corrupted field result and changed high product bit. Input
68/196/324ranks and all770coordinate pack/unpack values checked. Selected
products, allproduct coordinates, pivot certificates exported. Native exact
numbers only; completeness as L6D0 remains conditional on retained proper/RR
bridge and literal pole bounds, not implied solely by rank. No projected
class,order11action,targetfactor,ICrank or speed conclusion.
4ab10a2567608e39307002a5181c3287ed89d82a100a43e21c98d368748f665c  research/koblitz_modular_ic_20261018/rounds/35/runs/jets35-01/evaluation-words.txt
