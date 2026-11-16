# Reviewer fixed literal boundary controls before check

At p7, retained nonunit fixture a=(0,1,2,2),b=(0,4,0,0,3,2),a3=0: check exact generic/outer k equality as UInt64 and literal output w=1. At p5, all-zero coordinates except fourunit weights and a3=4: check None and canonical unused payload, demonstrating value-only total bundle despite early return. Reject a3=p as noncanonical. Existing producer full validity control remains unchanged. These are finite word-expression evaluations via Lean decide, not native collector runs.
