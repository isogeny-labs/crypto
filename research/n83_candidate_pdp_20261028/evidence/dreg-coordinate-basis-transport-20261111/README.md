# N83 coordinate-basis Dreg transport map

This artifact joins the existing exact chain-Dreg reference certificates to the
saved candidate-base metadata. It makes the theorem's conditional implication
visible across coordinate selector bases without presenting it as fresh
measurements on those candidates.

The reference proof uses `dreg = 83(m-2)+27+1` for the original individual
highest-form ideal, including Boolean squares, when its explicit source-shape
and rank hypotheses hold. Its 84 saved Koblitz/binary cases cover original,
baseline and spike40 for `m=3..9` and both target kinds. The exact result is
base-invariant across those three tested arms: 111, 194, 277, 360, 443, 526,
and 609 for `m=3..9` respectively.

The generated map records candidate coordinate-bit bases spike27 through
spike39, available family manifests, and saved case metadata. For binary and
Koblitz `m>=3`, it records the value implied **if** each concrete candidate
system satisfies the chain theorem's hypotheses. Candidate-specific exact Dreg
fields remain null because these candidates were not passed through the
chain-exact source parser. Prime and `m=2` rows are explicitly outside this
theorem's scope. Missing local manifests or case inputs are marked as missing;
they are not interpreted as proof that no S3 object exists.

The next evidence needed for a case-specific conclusion is a strict source
parse and certificate binding for each candidate's actual top forms, followed
by independent scientific review/formalization. This map does not imply FFD,
LFD, affine solving degree, or attack-cost improvement. The exact serialized
record meanings and null conventions are in `FORMAT.json`.

Regenerate from the repository root:

```sh
python3 research/n83_candidate_pdp_20261028/evidence/dreg-coordinate-basis-transport-20261111/build_dreg_coordinate_basis_map.py
```

The source summary hash, candidate input/manifest hashes, and file layout are
preserved in `coordinate-basis-dreg-map.json`. No input or prior run is
overwritten.
