# Coefficient oracle and paired phase semantics

Let h=(r-1)/2. Starting a=[1], update b_i=-a_i+a_(i-1), treating absent entries as zero, exactly h times over the caller-certified prime field. Induction gives a_i=[X^i](X-1)^h. The selected mode e delivers a_(e+jm) in descending j. The oracle computes this using repeated polynomial multiplication by X-1, independently of the interval product recurrence and its inversions. It is outside capture and used only as a finite correctness reference.

Each arm begins with a new stride-m context. Initial capture ends after the three streams complete; doubled capture then delivers stride2m through the same engine, except the separate arm constructs a fresh engine. The count vectors are incremental, so component-wise addition produces a two-phase total. Cache storage survives capture and is never interpreted as peak memory. Native assertions require coefficient equality, A/A equality and exactly one added scalar product per phase for the planted control.

The strict analyzer validates200 raw phase rows. Canonical export groups adjacent initial/doubled rows into100 execution cells, preserving each event as a phase-qualified metric. Treating phases as replicates would incorrectly claim independent repeated runs; they are dependent stages of one execution. Frozen schema requirements and source/binary/receipt checks prevent ordinary accidental mixing, but producer records are unsigned and not independent authentication.

validation01 is a software qualification on r41,m5,cap2, disjoint from the production case list, plus synthetic row mutations. The source transformations are reversible. export-test02 qualifies the adapter on temporary synthetic records only. Independent signed verdict remains pending. [pcc:unverified]

The production protocol remains draft: actual production instrument, sizing preflight and execution admission are missing. No production counter vector is inferred from software tests or fabricated to fill these obligations.
