# Per-phase medians

Each entry is the median of25 main calls in milliseconds. Phase medians do not add to the median work cost. The frontier's constructor and full replay are inside its native phase; extra preprocessing is not its total construction cost. Ratios use paired block observations in the main report, not ratios of these medians. [pcc:unverified]

|n|Arm|Extra preprocessing|Own preparation|Exact identity check|Native solve/check|Work|Full paid|
|---|---|---:|---:|---:|---:|---:|---:|
|3|prefix-singular|0.085107|0.211677|9.808425|3.638091|13.686317|39.722843|
|3|prefix-rust|0.085168|0.209655|0.080170|3.674957|4.158382|30.358585|
|3|frontier-singular|0.000260|0.074712|9.395365|4.785103|14.536393|41.084257|
|3|frontier-rust|0.000431|0.067241|0.044567|4.694368|4.836741|31.267139|
|4|prefix-singular|0.083956|0.181963|8.697297|3.679354|12.769100|38.106971|
|4|prefix-rust|0.082884|0.194502|0.083646|3.774477|4.185473|29.391837|
|4|frontier-singular|0.000231|0.070185|8.735325|5.064904|13.878146|39.310348|
|4|frontier-rust|0.000270|0.055383|0.047652|5.125405|5.245044|30.779692|
|6|prefix-singular|0.087271|0.194302|8.949226|3.999344|13.239697|44.394871|
|6|prefix-rust|0.088332|0.201713|0.109775|3.956419|4.460197|35.260206|
|6|frontier-singular|0.000251|0.061943|8.760773|5.861830|14.632808|45.907673|
|6|frontier-rust|0.000300|0.061332|0.062975|5.766998|5.915191|36.497104|

![Every registered comparison](ratios.svg){width=90%}
