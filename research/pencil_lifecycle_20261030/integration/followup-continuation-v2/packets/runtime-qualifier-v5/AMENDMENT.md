# Qualifier v5: unique gate-receipt schema

Preserves sealed v4. Adds one assertion after removing and validating the first typed stdout line: the gate schema identifier must not occur in remaining stdout or stderr. Thus a second claimed gate record fails every active case.

Q1 still requires complete parsed JSON equality. Q2/Q3 deliberately require exact expected exit plus rejection substring and absence of the positive report schema; their complete negative output bytes are not claimed equal. This clarifies earlier prose using exact rejection. Gate failures126/127 cannot match payload exits2/125.

No native or Docker execution. AST parsing passed. Gate/final-image/admission dependencies and 300/30/20-second caps remain unchanged. The potentially visible file-bind parent /qualification could cause the existing empty-diff check to fail closed; no runtime observation establishes that behavior yet. No allowance is added here. Original v4 review remains separately preserved.
