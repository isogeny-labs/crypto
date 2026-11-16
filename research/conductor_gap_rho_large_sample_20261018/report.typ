#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#show table: set text(size: 7.5pt)

= Does Pollard rho see a large prime conductor gap? A large-sample test

Study `research/conductor_gap_rho_large_sample_20261018`, rendered 2026-10-18 14:07 UTC. Unit: S = charged group operations / sqrt(r) as recorded by ecbench (`cost.s`); wall time is not read. Intervals: 95% bootstrap over targets (4000 draws) for the ratio of means; normal 95% for the paired per-target log-ratio. The host was shared (load 24–37 of 32), every run is L0.

== Pooled floor / crater, arm `rho.negation`

#table(columns: 8, [tier / family], [pairs], [n floor], [n crater], [ratio of means], [95% bootstrap], [paired geo-mean], [paired 95%],
[l], [24], [384], [384], [0.917], [0.850, 0.988], [0.905], [0.825, 0.993],
[l+lr], [48], [768], [768], [0.919], [0.871, 0.970], [0.917], [0.858, 0.979],
[l1], [4], [64], [64], [1.092], [0.912, 1.309], [1.079], [0.834, 1.398],
[l2], [4], [64], [64], [0.927], [0.778, 1.100], [0.889], [0.726, 1.087],
[l3], [8], [128], [128], [0.839], [0.741, 0.951], [0.851], [0.725, 0.999],
[l4], [8], [128], [128], [0.910], [0.791, 1.048], [0.890], [0.760, 1.041],
[l5r], [4], [64], [64], [0.933], [0.796, 1.096], [0.956], [0.781, 1.170],
[l6r], [4], [64], [64], [0.822], [0.689, 0.974], [0.812], [0.638, 1.033],
[l7r], [8], [128], [128], [0.966], [0.847, 1.102], [1.004], [0.848, 1.189],
[l8r], [8], [128], [128], [0.926], [0.814, 1.057], [0.904], [0.773, 1.057],
[l:cratertwin/crater], [8], [128], [128], [1.039], [0.919, 1.176], [1.077], [0.934, 1.243],
[l:twin/original], [8], [128], [128], [1.039], [0.919, 1.173], [1.077], [0.934, 1.243],
[lr], [24], [384], [384], [0.920], [0.856, 0.993], [0.928], [0.846, 1.019],
[lr:cratertwin/crater], [8], [128], [128], [0.891], [0.791, 1.005], [0.878], [0.745, 1.035],
[lr:twin/original], [8], [128], [128], [0.891], [0.793, 1.006], [0.878], [0.745, 1.035],
[m], [9], [1152], [1152], [0.982], [0.940, 1.027], [0.994], [0.941, 1.050],
[m1], [3], [384], [384], [1.050], [0.973, 1.131], [1.091], [0.985, 1.208],
[m2], [3], [384], [384], [0.967], [0.895, 1.042], [0.975], [0.890, 1.069],
[m3], [3], [384], [384], [0.933], [0.865, 1.009], [0.924], [0.845, 1.011],
[s], [13], [13312], [13312], [0.998], [0.986, 1.010], [0.999], [0.985, 1.014],
[s1], [12], [12288], [12288], [0.998], [0.985, 1.010], [1.000], [0.985, 1.015],
[s2], [1], [1024], [1024], [1.005], [0.961, 1.051], [0.992], [0.939, 1.048],
[s:cratertwin/crater], [5], [5120], [5120], [1.008], [0.988, 1.028], [1.008], [0.985, 1.032],
[s:floortwin/floor], [5], [5120], [5120], [1.001], [0.981, 1.022], [1.000], [0.976, 1.024],
[s:twin/original], [10], [10240], [10240], [1.004], [0.991, 1.019], [1.004], [0.987, 1.021],
[t], [24], [384], [384], [0.945], [0.878, 1.021], [0.943], [0.861, 1.034],
[t1], [3], [48], [48], [0.974], [0.782, 1.213], [0.957], [0.726, 1.262],
[t2], [3], [48], [48], [0.994], [0.813, 1.230], [1.066], [0.823, 1.380],
[t3], [3], [48], [48], [1.002], [0.809, 1.257], [0.986], [0.778, 1.252],
[t4], [3], [48], [48], [0.935], [0.787, 1.122], [0.938], [0.769, 1.144],
[t5], [3], [48], [48], [0.902], [0.718, 1.117], [0.832], [0.664, 1.041],
[t6], [3], [48], [48], [0.979], [0.788, 1.212], [0.977], [0.697, 1.370],
[t7], [3], [48], [48], [0.828], [0.677, 1.026], [0.846], [0.655, 1.092],
[t8], [3], [48], [48], [0.967], [0.792, 1.183], [0.965], [0.733, 1.271],
[t:cratertwin/crater], [24], [384], [384], [0.967], [0.898, 1.042], [0.982], [0.894, 1.078],
[t:floortwin/cratertwin], [24], [384], [384], [1.040], [0.965, 1.119], [1.022], [0.930, 1.122],
[t:floortwin/floor], [24], [384], [384], [1.064], [0.987, 1.146], [1.063], [0.973, 1.162],
[t:twin/original], [48], [768], [768], [1.014], [0.963, 1.066], [1.022], [0.958, 1.090],
)

#figure(image("figures/pooled_ratios.svg", width: 85%), caption: [Pooled floor/crater by family and tier, arm rho.negation. s: ℓ = 8209 (s1a–d) and 131101 (s2); m: ℓ ≈ 2^20.2 at 45 bits; l: ℓ ≈ 2^25 at 53 bits (l1–l4 frozen, l5r–l8r replication).])

#figure(image("figures/s_distributions.svg", width: 100%), caption: [Recorded S per solve, crater vs floor vs isomorphic twin.])

== Controls

#figure(image("figures/controls.svg", width: 85%), caption: [Positive control rho.plain / rho.negation (expected 1.414) and A/A arm (identical seeds, so exactly 1 per target).])

== Every comparison, arm `rho.negation`

#figure(image("figures/ratios_rho_neg.svg", width: 100%, height: 25cm, fit: "contain"), caption: [Every floor/crater (blue) and twin/original (grey) ratio with its 95% bootstrap interval.])

=== l1-pairs00-03
conductor gap ell~2^25, D=-19, 53-bit, pairs 0..3 of cm_orbits: crater vs floor vs crater twins, 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair0-floor / pair0-crater], [16/16], [1.032], [1.150], [1.114], [0.779, 1.588], [1.112], [0.670, 1.844],
[pair0-crater-twin(u=3) / pair0-crater], [16/16], [1.032], [0.956], [0.926], [0.656, 1.312], [0.990], [0.683, 1.435],
[pair1-floor / pair1-crater], [16/16], [0.967], [1.094], [1.131], [0.762, 1.665], [1.063], [0.567, 1.992],
[pair1-crater-twin(u=3) / pair1-crater], [16/16], [0.967], [0.965], [0.998], [0.733, 1.401], [1.177], [0.709, 1.954],
[pair2-floor / pair2-crater], [16/16], [0.761], [0.965], [1.268], [0.921, 1.735], [1.202], [0.746, 1.935],
[pair2-crater-twin(u=3) / pair2-crater], [16/16], [0.761], [1.210], [1.591], [1.196, 2.180], [1.671], [1.278, 2.184],
[pair3-floor / pair3-crater], [16/16], [1.011], [0.910], [0.900], [0.626, 1.295], [0.956], [0.587, 1.559],
[pair3-crater-twin(u=3) / pair3-crater], [16/16], [1.011], [0.916], [0.906], [0.690, 1.247], [1.062], [0.728, 1.549],
)

=== l2-pairs04-07
conductor gap ell~2^25, D=-19, 53-bit, pairs 4..7 of cm_orbits: crater vs floor vs crater twins, 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair4-floor / pair4-crater], [16/16], [0.847], [0.902], [1.064], [0.714, 1.574], [1.078], [0.709, 1.638],
[pair4-crater-twin(u=3) / pair4-crater], [16/16], [0.847], [1.000], [1.180], [0.824, 1.752], [1.286], [0.874, 1.892],
[pair5-floor / pair5-crater], [16/16], [0.984], [0.718], [0.730], [0.514, 0.985], [0.611], [0.400, 0.933],
[pair5-crater-twin(u=3) / pair5-crater], [16/16], [0.984], [1.054], [1.072], [0.751, 1.496], [0.898], [0.601, 1.342],
[pair6-floor / pair6-crater], [16/16], [0.960], [1.007], [1.049], [0.809, 1.344], [1.012], [0.724, 1.415],
[pair6-crater-twin(u=3) / pair6-crater], [16/16], [0.960], [0.947], [0.987], [0.734, 1.297], [0.919], [0.689, 1.226],
[pair7-floor / pair7-crater], [16/16], [0.939], [0.832], [0.886], [0.588, 1.357], [0.937], [0.624, 1.407],
[pair7-crater-twin(u=3) / pair7-crater], [16/16], [0.939], [0.742], [0.790], [0.529, 1.172], [0.828], [0.483, 1.420],
)

=== l3-pairs08-15
conductor gap ell~2^25, D=-19, 53-bit, pairs 8..15 of cm_orbits: crater vs floor, 16 shared targets. Record outcomes: {'verified': 256}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair8-floor / pair8-crater], [16/16], [0.933], [0.828], [0.888], [0.596, 1.379], [0.989], [0.707, 1.384],
[pair9-floor / pair9-crater], [16/16], [0.963], [0.950], [0.987], [0.691, 1.426], [1.023], [0.735, 1.424],
[pair10-floor / pair10-crater], [16/16], [1.028], [1.039], [1.010], [0.726, 1.421], [1.078], [0.631, 1.842],
[pair11-floor / pair11-crater], [16/16], [1.088], [0.915], [0.841], [0.602, 1.183], [0.876], [0.568, 1.352],
[pair12-floor / pair12-crater], [16/16], [0.996], [0.666], [0.669], [0.459, 0.960], [0.597], [0.335, 1.061],
[pair13-floor / pair13-crater], [16/16], [1.217], [0.859], [0.706], [0.483, 1.021], [0.691], [0.436, 1.096],
[pair14-floor / pair14-crater], [16/16], [0.940], [0.759], [0.807], [0.553, 1.179], [0.855], [0.507, 1.443],
[pair15-floor / pair15-crater], [16/16], [1.099], [0.920], [0.837], [0.599, 1.157], [0.817], [0.547, 1.221],
)

=== l4-pairs16-23
conductor gap ell~2^25, D=-19, 53-bit, pairs 16..23 of cm_orbits: crater vs floor, 16 shared targets. Record outcomes: {'verified': 256}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair16-floor / pair16-crater], [16/16], [1.095], [1.071], [0.978], [0.639, 1.492], [0.944], [0.593, 1.502],
[pair17-floor / pair17-crater], [16/16], [1.148], [0.868], [0.756], [0.520, 1.063], [0.593], [0.351, 1.001],
[pair18-floor / pair18-crater], [16/16], [0.772], [1.010], [1.308], [0.910, 1.915], [1.359], [0.930, 1.985],
[pair19-floor / pair19-crater], [16/16], [0.904], [0.887], [0.982], [0.675, 1.362], [0.867], [0.575, 1.308],
[pair20-floor / pair20-crater], [16/16], [1.031], [0.806], [0.781], [0.508, 1.227], [0.875], [0.563, 1.358],
[pair21-floor / pair21-crater], [16/16], [1.201], [0.935], [0.778], [0.534, 1.157], [0.863], [0.551, 1.352],
[pair22-floor / pair22-crater], [16/16], [1.011], [0.813], [0.804], [0.522, 1.215], [0.759], [0.467, 1.234],
[pair23-floor / pair23-crater], [16/16], [1.059], [1.094], [1.032], [0.760, 1.415], [1.038], [0.716, 1.506],
)

=== l5r-pairs00-03
conductor gap ell~2^25, D=-19, 53-bit, pairs 0..3 of cm_orbits: crater vs floor vs crater twins, 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair0-floor / pair0-crater], [16/16], [1.194], [0.787], [0.659], [0.465, 0.940], [0.667], [0.374, 1.189],
[pair0-crater-twin(u=3) / pair0-crater], [16/16], [1.194], [0.829], [0.695], [0.488, 0.994], [0.706], [0.425, 1.172],
[pair1-floor / pair1-crater], [16/16], [0.847], [0.935], [1.103], [0.778, 1.585], [1.143], [0.767, 1.702],
[pair1-crater-twin(u=3) / pair1-crater], [16/16], [0.847], [0.851], [1.004], [0.653, 1.481], [0.851], [0.449, 1.616],
[pair2-floor / pair2-crater], [16/16], [0.887], [0.981], [1.106], [0.815, 1.503], [1.100], [0.818, 1.479],
[pair2-crater-twin(u=3) / pair2-crater], [16/16], [0.887], [0.884], [0.996], [0.731, 1.377], [0.982], [0.750, 1.286],
[pair3-floor / pair3-crater], [16/16], [0.980], [0.944], [0.963], [0.776, 1.220], [0.997], [0.788, 1.263],
[pair3-crater-twin(u=3) / pair3-crater], [16/16], [0.980], [0.966], [0.986], [0.704, 1.326], [0.819], [0.455, 1.472],
)

=== l6r-pairs04-07
conductor gap ell~2^25, D=-19, 53-bit, pairs 4..7 of cm_orbits: crater vs floor vs crater twins, 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair4-floor / pair4-crater], [16/16], [1.207], [0.820], [0.679], [0.445, 0.995], [0.594], [0.348, 1.013],
[pair4-crater-twin(u=3) / pair4-crater], [16/16], [1.207], [0.934], [0.774], [0.552, 1.078], [0.766], [0.524, 1.120],
[pair5-floor / pair5-crater], [16/16], [1.426], [1.043], [0.732], [0.533, 0.976], [0.663], [0.461, 0.954],
[pair5-crater-twin(u=3) / pair5-crater], [16/16], [1.426], [1.037], [0.727], [0.557, 0.956], [0.711], [0.514, 0.983],
[pair6-floor / pair6-crater], [16/16], [0.893], [1.042], [1.167], [0.849, 1.585], [1.119], [0.719, 1.741],
[pair6-crater-twin(u=3) / pair6-crater], [16/16], [0.893], [1.088], [1.218], [0.934, 1.614], [1.214], [0.829, 1.779],
[pair7-floor / pair7-crater], [16/16], [1.034], [0.843], [0.816], [0.592, 1.197], [0.985], [0.575, 1.687],
[pair7-crater-twin(u=3) / pair7-crater], [16/16], [1.034], [0.956], [0.925], [0.651, 1.356], [1.106], [0.660, 1.854],
)

=== l7r-pairs08-15
conductor gap ell~2^25, D=-19, 53-bit, pairs 8..15 of cm_orbits: crater vs floor, 16 shared targets. Record outcomes: {'verified': 256}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair8-floor / pair8-crater], [16/16], [1.005], [0.991], [0.987], [0.706, 1.386], [1.078], [0.658, 1.766],
[pair9-floor / pair9-crater], [16/16], [1.098], [0.750], [0.683], [0.463, 0.989], [0.690], [0.445, 1.069],
[pair10-floor / pair10-crater], [16/16], [0.778], [0.663], [0.853], [0.597, 1.239], [0.864], [0.590, 1.265],
[pair11-floor / pair11-crater], [16/16], [0.703], [0.806], [1.147], [0.801, 1.662], [1.130], [0.575, 2.219],
[pair12-floor / pair12-crater], [16/16], [0.822], [1.111], [1.350], [0.882, 2.115], [1.444], [0.843, 2.474],
[pair13-floor / pair13-crater], [16/16], [0.897], [1.080], [1.204], [0.845, 1.738], [1.290], [0.838, 1.986],
[pair14-floor / pair14-crater], [16/16], [1.201], [0.809], [0.674], [0.508, 0.891], [0.696], [0.481, 1.009],
[pair15-floor / pair15-crater], [16/16], [0.994], [1.034], [1.041], [0.752, 1.464], [1.100], [0.741, 1.631],
)

=== l8r-pairs16-23
conductor gap ell~2^25, D=-19, 53-bit, pairs 16..23 of cm_orbits: crater vs floor, 16 shared targets. Record outcomes: {'verified': 256}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair16-floor / pair16-crater], [16/16], [1.057], [0.984], [0.931], [0.686, 1.208], [0.794], [0.536, 1.176],
[pair17-floor / pair17-crater], [16/16], [1.192], [1.042], [0.874], [0.604, 1.291], [0.938], [0.501, 1.756],
[pair18-floor / pair18-crater], [16/16], [1.059], [1.373], [1.296], [0.912, 1.839], [1.265], [0.757, 2.112],
[pair19-floor / pair19-crater], [16/16], [0.955], [0.863], [0.904], [0.669, 1.202], [0.868], [0.616, 1.223],
[pair20-floor / pair20-crater], [16/16], [0.943], [0.939], [0.996], [0.696, 1.409], [0.942], [0.626, 1.419],
[pair21-floor / pair21-crater], [16/16], [1.023], [0.767], [0.750], [0.507, 1.108], [0.785], [0.497, 1.239],
[pair22-floor / pair22-crater], [16/16], [1.339], [1.112], [0.831], [0.588, 1.208], [0.914], [0.667, 1.252],
[pair23-floor / pair23-crater], [16/16], [1.143], [0.982], [0.859], [0.599, 1.224], [0.806], [0.506, 1.285],
)

=== m1-ell1211653-q45
conductor gap ell=1211653 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets. Record outcomes: {'verified': 1024}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor(seed1) / crater], [128/128], [0.905], [0.918], [1.015], [0.881, 1.166], [1.059], [0.886, 1.266],
[floor(seed2) / crater], [128/128], [0.905], [0.971], [1.074], [0.938, 1.229], [1.096], [0.918, 1.309],
[floor(seed3) / crater], [128/128], [0.905], [0.961], [1.062], [0.935, 1.207], [1.119], [0.937, 1.335],
)

=== m2-ell1138019-q45
conductor gap ell=1138019 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets. Record outcomes: {'verified': 1024}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor(seed1) / crater], [128/128], [0.967], [0.918], [0.950], [0.829, 1.090], [0.943], [0.802, 1.109],
[floor(seed2) / crater], [128/128], [0.967], [0.943], [0.976], [0.858, 1.116], [0.989], [0.838, 1.166],
[floor(seed3) / crater], [128/128], [0.967], [0.942], [0.974], [0.853, 1.115], [0.995], [0.857, 1.156],
)

=== m3-ell1609903-q45
conductor gap ell=1609903 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets. Record outcomes: {'verified': 1024}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor(seed1) / crater], [128/128], [0.959], [0.913], [0.953], [0.835, 1.091], [0.908], [0.764, 1.079],
[floor(seed2) / crater], [128/128], [0.959], [0.891], [0.929], [0.815, 1.060], [0.936], [0.807, 1.085],
[floor(seed3) / crater], [128/128], [0.959], [0.881], [0.919], [0.810, 1.048], [0.929], [0.804, 1.073],
)

=== s1a-ell8209-q29
conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets. Record outcomes: {'verified': 18432}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [0.978], [0.975], [0.997], [0.955, 1.042], [0.999], [0.948, 1.053],
[floor2 / crater], [1024/1024], [0.978], [0.973], [0.995], [0.952, 1.039], [1.006], [0.955, 1.060],
[floor3 / crater], [1024/1024], [0.978], [0.947], [0.968], [0.925, 1.012], [0.975], [0.925, 1.027],
[crater-twin(u=3) / crater], [1024/1024], [0.978], [0.952], [0.973], [0.930, 1.017], [0.982], [0.931, 1.035],
[floor1-twin(u=3) / crater], [1024/1024], [0.978], [0.979], [1.001], [0.959, 1.045], [1.017], [0.966, 1.069],
)

Arm `rho-plain`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [1.330], [1.328], [0.998], [0.955, 1.047], [0.998], [0.945, 1.053],
[floor2 / crater], [1024/1024], [1.330], [1.300], [0.977], [0.933, 1.025], [0.982], [0.929, 1.039],
[floor3 / crater], [1024/1024], [1.330], [1.328], [0.999], [0.955, 1.047], [1.023], [0.970, 1.079],
[crater-twin(u=3) / crater], [1024/1024], [1.330], [1.339], [1.007], [0.964, 1.053], [1.012], [0.958, 1.069],
[floor1-twin(u=3) / crater], [1024/1024], [1.330], [1.318], [0.991], [0.948, 1.037], [1.003], [0.950, 1.060],
[rho-plain / rho-neg (crater, positive control, expect 1.414)], [1024/1024], [0.978], [1.330], [1.360], [1.299, 1.423], [1.344], [1.273, 1.418],
)

=== s1b-ell8209-q29
conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets. Record outcomes: {'verified': 18432}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [0.963], [0.979], [1.017], [0.972, 1.062], [1.020], [0.968, 1.074],
[floor2 / crater], [1024/1024], [0.963], [0.969], [1.006], [0.962, 1.052], [1.000], [0.949, 1.054],
[floor3 / crater], [1024/1024], [0.963], [0.939], [0.975], [0.933, 1.019], [0.969], [0.920, 1.021],
[crater-twin(u=3) / crater], [1024/1024], [0.963], [0.962], [0.999], [0.955, 1.045], [0.999], [0.948, 1.054],
[floor1-twin(u=3) / crater], [1024/1024], [0.963], [0.993], [1.031], [0.987, 1.077], [1.032], [0.979, 1.087],
)

Arm `rho-plain`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [1.321], [1.326], [1.004], [0.961, 1.050], [0.995], [0.944, 1.047],
[floor2 / crater], [1024/1024], [1.321], [1.357], [1.027], [0.982, 1.073], [1.010], [0.959, 1.063],
[floor3 / crater], [1024/1024], [1.321], [1.332], [1.008], [0.965, 1.053], [0.982], [0.933, 1.035],
[crater-twin(u=3) / crater], [1024/1024], [1.321], [1.351], [1.022], [0.978, 1.067], [1.006], [0.955, 1.060],
[floor1-twin(u=3) / crater], [1024/1024], [1.321], [1.320], [0.999], [0.956, 1.044], [0.991], [0.942, 1.043],
[rho-plain / rho-neg (crater, positive control, expect 1.414)], [1024/1024], [0.963], [1.321], [1.372], [1.314, 1.432], [1.388], [1.319, 1.461],
)

=== s1c-ell8209-q29
conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets. Record outcomes: {'verified': 18432}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [0.967], [0.973], [1.006], [0.963, 1.051], [1.004], [0.953, 1.057],
[floor2 / crater], [1024/1024], [0.967], [0.980], [1.013], [0.968, 1.059], [1.019], [0.970, 1.069],
[floor3 / crater], [1024/1024], [0.967], [0.971], [1.005], [0.963, 1.050], [1.006], [0.955, 1.059],
[crater-twin(u=3) / crater], [1024/1024], [0.967], [0.964], [0.997], [0.955, 1.041], [0.982], [0.933, 1.034],
[floor1-twin(u=3) / crater], [1024/1024], [0.967], [0.936], [0.968], [0.926, 1.013], [0.960], [0.913, 1.010],
)

Arm `rho-plain`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [1.377], [1.318], [0.957], [0.916, 0.999], [0.955], [0.906, 1.007],
[floor2 / crater], [1024/1024], [1.377], [1.324], [0.961], [0.921, 1.005], [0.946], [0.897, 0.998],
[floor3 / crater], [1024/1024], [1.377], [1.369], [0.994], [0.955, 1.037], [0.991], [0.939, 1.045],
[crater-twin(u=3) / crater], [1024/1024], [1.377], [1.303], [0.946], [0.905, 0.989], [0.941], [0.893, 0.990],
[floor1-twin(u=3) / crater], [1024/1024], [1.377], [1.339], [0.972], [0.931, 1.015], [0.964], [0.915, 1.016],
[rho-plain / rho-neg (crater, positive control, expect 1.414)], [1024/1024], [0.967], [1.377], [1.424], [1.363, 1.487], [1.420], [1.348, 1.495],
)

=== s1d-ell8209-q29
conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets. Record outcomes: {'verified': 18432}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [0.969], [0.952], [0.982], [0.939, 1.027], [0.982], [0.931, 1.036],
[floor2 / crater], [1024/1024], [0.969], [0.982], [1.013], [0.970, 1.058], [1.014], [0.963, 1.068],
[floor3 / crater], [1024/1024], [0.969], [0.963], [0.994], [0.951, 1.038], [1.003], [0.952, 1.057],
[crater-twin(u=3) / crater], [1024/1024], [0.969], [1.000], [1.032], [0.989, 1.078], [1.045], [0.992, 1.100],
[floor1-twin(u=3) / crater], [1024/1024], [0.969], [0.972], [1.002], [0.960, 1.047], [1.001], [0.949, 1.056],
)

Arm `rho-plain`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [1.351], [1.314], [0.973], [0.934, 1.017], [0.979], [0.929, 1.033],
[floor2 / crater], [1024/1024], [1.351], [1.325], [0.981], [0.940, 1.024], [1.004], [0.954, 1.057],
[floor3 / crater], [1024/1024], [1.351], [1.306], [0.967], [0.925, 1.009], [0.972], [0.921, 1.025],
[crater-twin(u=3) / crater], [1024/1024], [1.351], [1.326], [0.982], [0.939, 1.025], [0.990], [0.937, 1.045],
[floor1-twin(u=3) / crater], [1024/1024], [1.351], [1.319], [0.976], [0.935, 1.021], [0.989], [0.938, 1.042],
[rho-plain / rho-neg (crater, positive control, expect 1.414)], [1024/1024], [0.969], [1.351], [1.393], [1.334, 1.452], [1.395], [1.324, 1.471],
)

=== s2-ell131101-q37
conductor gap ell=131101, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets. Record outcomes: {'verified': 12288}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [0.914], [0.919], [1.005], [0.961, 1.051], [0.992], [0.939, 1.048],
[crater-twin(u=3) / crater], [1024/1024], [0.914], [0.950], [1.040], [0.994, 1.086], [1.034], [0.979, 1.091],
[floor1-twin(u=3) / crater], [1024/1024], [0.914], [0.922], [1.009], [0.963, 1.059], [0.987], [0.934, 1.043],
)

Arm `rho-plain`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[floor1 / crater], [1024/1024], [1.322], [1.313], [0.994], [0.950, 1.038], [0.997], [0.947, 1.050],
[crater-twin(u=3) / crater], [1024/1024], [1.322], [1.309], [0.990], [0.948, 1.036], [0.981], [0.928, 1.037],
[floor1-twin(u=3) / crater], [1024/1024], [1.322], [1.304], [0.987], [0.944, 1.033], [0.988], [0.935, 1.043],
[rho-plain / rho-neg (crater, positive control, expect 1.414)], [1024/1024], [0.914], [1.322], [1.446], [1.383, 1.510], [1.441], [1.367, 1.520],
)

=== t1-pairs00-02
conductor gap ell~2^25, D=-19, 53-bit, pairs 0..2: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair0-crater-twin(u=5) / pair0-crater], [16/16], [1.005], [0.882], [0.878], [0.629, 1.260], [0.999], [0.629, 1.586],
[pair0-floor / pair0-crater], [16/16], [1.005], [0.943], [0.939], [0.616, 1.428], [0.953], [0.563, 1.612],
[pair0-floor-twin(u=5) / pair0-crater], [16/16], [1.005], [1.078], [1.073], [0.740, 1.580], [1.095], [0.714, 1.680],
[pair1-crater-twin(u=5) / pair1-crater], [16/16], [0.905], [0.874], [0.965], [0.653, 1.416], [0.951], [0.544, 1.660],
[pair1-floor / pair1-crater], [16/16], [0.905], [1.002], [1.107], [0.739, 1.617], [1.073], [0.610, 1.889],
[pair1-floor-twin(u=5) / pair1-crater], [16/16], [0.905], [1.230], [1.358], [1.054, 1.840], [1.582], [1.139, 2.195],
[pair2-crater-twin(u=5) / pair2-crater], [16/16], [0.868], [0.942], [1.086], [0.748, 1.557], [1.048], [0.752, 1.460],
[pair2-floor / pair2-crater], [16/16], [0.868], [0.761], [0.877], [0.637, 1.214], [0.857], [0.607, 1.209],
[pair2-floor-twin(u=5) / pair2-crater], [16/16], [0.868], [1.086], [1.252], [0.907, 1.743], [1.261], [0.809, 1.965],
)

=== t2-pairs03-05
conductor gap ell~2^25, D=-19, 53-bit, pairs 3..5: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair3-crater-twin(u=5) / pair3-crater], [16/16], [0.876], [0.977], [1.115], [0.723, 1.672], [1.102], [0.727, 1.672],
[pair3-floor / pair3-crater], [16/16], [0.876], [0.940], [1.073], [0.760, 1.563], [1.226], [0.754, 1.994],
[pair3-floor-twin(u=5) / pair3-crater], [16/16], [0.876], [0.926], [1.057], [0.715, 1.569], [1.088], [0.684, 1.731],
[pair4-crater-twin(u=5) / pair4-crater], [16/16], [0.992], [1.141], [1.150], [0.823, 1.667], [1.312], [0.865, 1.989],
[pair4-floor / pair4-crater], [16/16], [0.992], [0.689], [0.695], [0.488, 1.008], [0.749], [0.442, 1.267],
[pair4-floor-twin(u=5) / pair4-crater], [16/16], [0.992], [1.149], [1.158], [0.789, 1.732], [1.235], [0.808, 1.886],
[pair5-crater-twin(u=5) / pair5-crater], [16/16], [0.851], [1.030], [1.210], [0.883, 1.692], [1.257], [0.864, 1.830],
[pair5-floor / pair5-crater], [16/16], [0.851], [1.072], [1.260], [0.900, 1.756], [1.319], [1.023, 1.700],
[pair5-floor-twin(u=5) / pair5-crater], [16/16], [0.851], [1.091], [1.281], [0.921, 1.785], [1.279], [0.860, 1.902],
)

=== t3-pairs06-08
conductor gap ell~2^25, D=-19, 53-bit, pairs 6..8: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair6-crater-twin(u=5) / pair6-crater], [16/16], [1.083], [0.980], [0.905], [0.637, 1.273], [0.879], [0.586, 1.321],
[pair6-floor / pair6-crater], [16/16], [1.083], [0.980], [0.905], [0.623, 1.315], [0.847], [0.543, 1.323],
[pair6-floor-twin(u=5) / pair6-crater], [16/16], [1.083], [0.952], [0.879], [0.625, 1.229], [0.835], [0.503, 1.384],
[pair7-crater-twin(u=5) / pair7-crater], [16/16], [0.890], [1.085], [1.219], [0.865, 1.774], [1.337], [0.767, 2.331],
[pair7-floor / pair7-crater], [16/16], [0.890], [0.970], [1.090], [0.755, 1.594], [1.161], [0.734, 1.835],
[pair7-floor-twin(u=5) / pair7-crater], [16/16], [0.890], [0.953], [1.071], [0.752, 1.559], [1.127], [0.608, 2.090],
[pair8-crater-twin(u=5) / pair8-crater], [16/16], [0.792], [0.941], [1.189], [0.810, 1.776], [1.170], [0.735, 1.864],
[pair8-floor / pair8-crater], [16/16], [0.792], [0.821], [1.037], [0.698, 1.527], [0.976], [0.699, 1.363],
[pair8-floor-twin(u=5) / pair8-crater], [16/16], [0.792], [0.847], [1.069], [0.695, 1.592], [0.925], [0.540, 1.583],
)

=== t4-pairs09-11
conductor gap ell~2^25, D=-19, 53-bit, pairs 9..11: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair9-crater-twin(u=5) / pair9-crater], [16/16], [0.950], [0.977], [1.028], [0.737, 1.429], [1.029], [0.720, 1.472],
[pair9-floor / pair9-crater], [16/16], [0.950], [1.070], [1.126], [0.818, 1.544], [1.147], [0.866, 1.521],
[pair9-floor-twin(u=5) / pair9-crater], [16/16], [0.950], [1.029], [1.084], [0.808, 1.470], [1.114], [0.769, 1.613],
[pair10-crater-twin(u=5) / pair10-crater], [16/16], [1.135], [0.900], [0.793], [0.578, 1.087], [0.788], [0.531, 1.168],
[pair10-floor / pair10-crater], [16/16], [1.135], [0.990], [0.872], [0.641, 1.170], [0.875], [0.650, 1.178],
[pair10-floor-twin(u=5) / pair10-crater], [16/16], [1.135], [0.956], [0.842], [0.631, 1.122], [0.866], [0.647, 1.161],
[pair11-crater-twin(u=5) / pair11-crater], [16/16], [1.147], [0.894], [0.779], [0.598, 1.041], [0.811], [0.615, 1.069],
[pair11-floor / pair11-crater], [16/16], [1.147], [0.961], [0.838], [0.613, 1.154], [0.823], [0.534, 1.268],
[pair11-floor-twin(u=5) / pair11-crater], [16/16], [1.147], [0.796], [0.694], [0.455, 1.013], [0.586], [0.360, 0.954],
)

=== t5-pairs12-14
conductor gap ell~2^25, D=-19, 53-bit, pairs 12..14: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair12-crater-twin(u=5) / pair12-crater], [16/16], [0.762], [1.090], [1.431], [0.977, 2.067], [1.386], [0.869, 2.211],
[pair12-floor / pair12-crater], [16/16], [0.762], [0.774], [1.016], [0.692, 1.447], [0.979], [0.682, 1.404],
[pair12-floor-twin(u=5) / pair12-crater], [16/16], [0.762], [0.781], [1.025], [0.664, 1.509], [0.910], [0.593, 1.397],
[pair13-crater-twin(u=5) / pair13-crater], [16/16], [1.057], [0.847], [0.801], [0.571, 1.120], [0.801], [0.545, 1.179],
[pair13-floor / pair13-crater], [16/16], [1.057], [0.748], [0.707], [0.462, 1.056], [0.680], [0.497, 0.930],
[pair13-floor-twin(u=5) / pair13-crater], [16/16], [1.057], [0.861], [0.814], [0.558, 1.175], [0.785], [0.523, 1.179],
[pair14-crater-twin(u=5) / pair14-crater], [16/16], [1.003], [0.935], [0.932], [0.682, 1.229], [0.779], [0.461, 1.316],
[pair14-floor / pair14-crater], [16/16], [1.003], [1.025], [1.022], [0.705, 1.399], [0.866], [0.537, 1.395],
[pair14-floor-twin(u=5) / pair14-crater], [16/16], [1.003], [0.764], [0.762], [0.536, 1.059], [0.671], [0.471, 0.954],
)

=== t6-pairs15-17
conductor gap ell~2^25, D=-19, 53-bit, pairs 15..17: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair15-crater-twin(u=5) / pair15-crater], [16/16], [0.878], [0.954], [1.087], [0.733, 1.614], [1.373], [0.635, 2.970],
[pair15-floor / pair15-crater], [16/16], [0.878], [1.146], [1.306], [0.901, 1.947], [1.663], [0.757, 3.652],
[pair15-floor-twin(u=5) / pair15-crater], [16/16], [0.878], [1.080], [1.230], [0.853, 1.812], [1.611], [0.744, 3.488],
[pair16-crater-twin(u=5) / pair16-crater], [16/16], [1.062], [0.887], [0.835], [0.554, 1.197], [0.726], [0.477, 1.103],
[pair16-floor / pair16-crater], [16/16], [1.062], [0.761], [0.716], [0.487, 1.023], [0.662], [0.463, 0.947],
[pair16-floor-twin(u=5) / pair16-crater], [16/16], [1.062], [1.023], [0.963], [0.699, 1.294], [0.923], [0.668, 1.275],
[pair17-crater-twin(u=5) / pair17-crater], [16/16], [0.895], [1.050], [1.173], [0.845, 1.607], [1.122], [0.749, 1.679],
[pair17-floor / pair17-crater], [16/16], [0.895], [0.868], [0.969], [0.679, 1.358], [0.847], [0.534, 1.344],
[pair17-floor-twin(u=5) / pair17-crater], [16/16], [0.895], [1.026], [1.146], [0.829, 1.572], [1.114], [0.786, 1.579],
)

=== t7-pairs18-20
conductor gap ell~2^25, D=-19, 53-bit, pairs 18..20: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair18-crater-twin(u=5) / pair18-crater], [16/16], [1.076], [0.977], [0.908], [0.635, 1.284], [0.842], [0.592, 1.197],
[pair18-floor / pair18-crater], [16/16], [1.076], [0.854], [0.794], [0.562, 1.064], [0.729], [0.472, 1.125],
[pair18-floor-twin(u=5) / pair18-crater], [16/16], [1.076], [1.181], [1.097], [0.798, 1.497], [0.997], [0.664, 1.497],
[pair19-crater-twin(u=5) / pair19-crater], [16/16], [0.930], [0.963], [1.035], [0.715, 1.561], [1.260], [0.863, 1.841],
[pair19-floor / pair19-crater], [16/16], [0.930], [0.991], [1.065], [0.702, 1.669], [1.218], [0.760, 1.953],
[pair19-floor-twin(u=5) / pair19-crater], [16/16], [0.930], [0.925], [0.994], [0.658, 1.540], [1.112], [0.770, 1.606],
[pair20-crater-twin(u=5) / pair20-crater], [16/16], [1.258], [0.851], [0.676], [0.469, 0.991], [0.663], [0.431, 1.020],
[pair20-floor / pair20-crater], [16/16], [1.258], [0.858], [0.682], [0.487, 0.958], [0.682], [0.462, 1.006],
[pair20-floor-twin(u=5) / pair20-crater], [16/16], [1.258], [1.129], [0.897], [0.637, 1.249], [0.881], [0.525, 1.479],
)

=== t8-pairs21-23
conductor gap ell~2^25, D=-19, 53-bit, pairs 21..23: crater, crater twin, floor, floor twin (u=5), 16 shared targets. Record outcomes: {'verified': 192}.

Arm `rho-neg`:
#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],
[pair21-crater-twin(u=5) / pair21-crater], [16/16], [1.188], [0.798], [0.672], [0.469, 0.929], [0.632], [0.377, 1.059],
[pair21-floor / pair21-crater], [16/16], [1.188], [1.133], [0.954], [0.666, 1.339], [0.873], [0.565, 1.348],
[pair21-floor-twin(u=5) / pair21-crater], [16/16], [1.188], [1.211], [1.019], [0.754, 1.375], [1.038], [0.759, 1.419],
[pair22-crater-twin(u=5) / pair22-crater], [16/16], [1.095], [1.004], [0.917], [0.659, 1.308], [1.058], [0.636, 1.758],
[pair22-floor / pair22-crater], [16/16], [1.095], [1.093], [0.999], [0.707, 1.439], [1.065], [0.552, 2.052],
[pair22-floor-twin(u=5) / pair22-crater], [16/16], [1.095], [0.981], [0.896], [0.619, 1.330], [0.942], [0.509, 1.744],
[pair23-crater-twin(u=5) / pair23-crater], [16/16], [1.128], [1.070], [0.948], [0.666, 1.328], [0.859], [0.481, 1.531],
[pair23-floor / pair23-crater], [16/16], [1.128], [1.073], [0.951], [0.696, 1.325], [0.968], [0.719, 1.303],
[pair23-floor-twin(u=5) / pair23-crater], [16/16], [1.128], [0.919], [0.815], [0.565, 1.157], [0.769], [0.470, 1.258],
)

== Audits
#table(columns: 4, [session], [ok], [replays], [receipt sha256],
[l1-pairs00-03], [True (0 problems, 192 verified)], [24/24 identical], [eeee9d305271a6ac…],
[l2-pairs04-07], [True (0 problems, 192 verified)], [24/24 identical], [63df9c5b03b3fa3b…],
[l3-pairs08-15], [True (0 problems, 256 verified)], [24/24 identical], [ae924832bb99daef…],
[l4-pairs16-23], [True (0 problems, 256 verified)], [24/24 identical], [38b964cd30fecb31…],
[l5r-pairs00-03], [True (0 problems, 192 verified)], [24/24 identical], [3c9088a925c3dd28…],
[l6r-pairs04-07], [True (0 problems, 192 verified)], [24/24 identical], [255c2f82dc580467…],
[l7r-pairs08-15], [True (0 problems, 256 verified)], [24/24 identical], [1a585de883f2edef…],
[l8r-pairs16-23], [True (0 problems, 256 verified)], [24/24 identical], [42041d1f7764ce88…],
[m1-ell1211653-q45], [True (0 problems, 1024 verified)], [1024/1024 identical], [29da8815787c1a0e…],
[m2-ell1138019-q45], [True (0 problems, 1024 verified)], [1024/1024 identical], [f12ca5f9177d4f04…],
[m3-ell1609903-q45], [True (0 problems, 1024 verified)], [1024/1024 identical], [f3d51c7d19ce198a…],
[s1a-ell8209-q29], [True (0 problems, 18432 verified)], [18432/18432 identical], [3d269081dcb8522a…],
[s1b-ell8209-q29], [True (0 problems, 18432 verified)], [18432/18432 identical], [aee3f976a7de24a6…],
[s1c-ell8209-q29], [True (0 problems, 18432 verified)], [18432/18432 identical], [609df87ca96578b1…],
[s1d-ell8209-q29], [True (0 problems, 18432 verified)], [18432/18432 identical], [d066ffda5361fba6…],
[s2-ell131101-q37], [True (0 problems, 12288 verified)], [12288/12288 identical], [0448f5176a4034f3…],
[t1-pairs00-02], [True (0 problems, 192 verified)], [24/24 identical], [21224a23cb7a4f51…],
[t2-pairs03-05], [True (0 problems, 192 verified)], [24/24 identical], [625ebcf7f75ad712…],
[t3-pairs06-08], [True (0 problems, 192 verified)], [24/24 identical], [02d7c366a6180ffc…],
[t4-pairs09-11], [True (0 problems, 192 verified)], [24/24 identical], [8db8f383a18d1424…],
[t5-pairs12-14], [True (0 problems, 192 verified)], [24/24 identical], [da5b91f7016278a7…],
[t6-pairs15-17], [True (0 problems, 192 verified)], [24/24 identical], [b4225928c95c9260…],
[t7-pairs18-20], [True (0 problems, 192 verified)], [24/24 identical], [5fbe19c56bcd715c…],
[t8-pairs21-23], [True (0 problems, 192 verified)], [24/24 identical], [b31b6289abd0b074…],
)

== Formal companion
See `lean/RepresentationHashInvariance.lean` and `lean/README.md`: transport of an r-adding walk along a group isomorphism preserves the ρ-length and is a bijection of the walk space (axioms: propext, Quot.sound).
