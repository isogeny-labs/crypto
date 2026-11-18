# C145: repaired complete early census and exact singular-cell guards

The source defect was a coordinate-type confusion:basisreturns(image_vector,selector_witness),but C144cused the witness as a coefficient image. A witness must first pass through the original early linear map. The new variant preserves every old source/failed attempt and explicitly verifies E*witness=image for all24basis entries before constructing planes. This is a changed failed assumption,not an identical retry.

Actual evidence:

{
  "diagnostic": {
    "out": {
      "mapped_states": 64,
      "coefficient_checks": 20160,
      "row_checks": 384,
      "old_mapping_counterexample": {
        "state": 1,
        "selector_witness_hex": "0x11c7818",
        "correct_coefficient_image_hex": "0x140",
        "actual_prefix_hex": "0xc1523fb15c",
        "old_misencoded_prefix_hex": "0xc15323c804"
      },
      "all_registered_contexts_complete": true,
      "original_complete": false,
      "total_seconds": 1.3339855799858924,
      "post_exec_memory": {
        "start": {
          "VmSize": "36232 kB",
          "VmHWM": "9752 kB",
          "VmRSS": "9752 kB"
        },
        "finish": {
          "VmSize": "37860 kB",
          "VmHWM": "11100 kB",
          "VmRSS": "11100 kB"
        }
      }
    },
    "error": null
  },
  "census": {
    "out": {
      "rank_oracles": 320,
      "actual_selector_readbacks": 32,
      "complete_early_census": true,
      "conditional_parameter_upper_bound": 213920982300098560,
      "global_original_root_count": null,
      "original_complete": false,
      "all_registered_contexts_complete": true,
      "early_rank": 24,
      "early_states": 16777216,
      "rank_distribution": {
        "0": 0,
        "1": 0,
        "2": 74,
        "3": 10154,
        "4": 569404,
        "5": 6346064,
        "6": 9851520
      },
      "development_target_met": true,
      "independent_rank_masks_identical": true,
      "plane_sha256": [
        [
          "4bda3a28f4ffe603c0ec1258c0034d65a1a0d35ab7bd523a834608adabf03cc5",
          "97217c78271c9ac81d9e72272cc01dc36db269536ea25e5ae7043d76e2a76500",
          "7e2d644f90b629b1238b6530b094bfe5f9638ef23b27ee66f184a22f808d5052",
          "5647f05ec18958947d32874eeb788fa396a05d0bab7c1b71f112ceb7e9b31eee",
          "41e132122a094c6d92da076b842c56985576b723aadb33288ce8a63059332679",
          "9cda7d092dcd33818140ee70f8837a870d6aeffd7fd33894894cac04908c8698",
          "7e2d644f90b629b1238b6530b094bfe5f9638ef23b27ee66f184a22f808d5052"
        ],
        [
          "f34a28407070b3404cdc8e66b1f3d119645e25420a0883c0d74fcb951358cfc4",
          "d5d4001fcedb499388342a04076604f1c896f483ae54e40e92b569159709a730",
          "1fd9bb543158759e02a5c8326d798aa884e0e97b5067d80ed9f3993923d4ab5b",
          "6ddd502ce9622ebb70cc1d05467f7ac7fe0c5396f88853cba52ceb132f8a5493",
          "1213249b3e6732a3e134aa19e450ab3470348c0ba09fd1124fb169f232c6f6b5",
          "029d612238dd3611ddda83ab6eebc81520bafeb88cfe040c39b9d7925d4cebb2",
          "f1d4cf100035d3d0478d57b85fd1823bd0585d21447ac076263b4ddd1008a95e"
        ],
        [
          "ba1fa65f4f4100f1427095ed3d5785125ac2e83502f8ad14fcff540a96c74f54",
          "f0c639e8c82af7db1bee441c2cec6df954d723457f7e907bc4117bc78d651b99",
          "c0c0ef7434f3667dd6b6749612fa65679fb722f5bf0496992d5c04dffaa9191a",
          "d94e2efd164763124d921904ec6a0ea892f95a151df084f2cd9657510451293e",
          "2135ce5ef728a2c69333a23276ef7a0b28a2941e30661b58178a0855e77278da",
          "274d3f5877e6e791cf2960b9b758bfb6b06fec6a923ec25706eb3b1c3c3ec895",
          "c870c0824a276c88eb17af813844df4fdd086a493928f6b5cff69d2fa48efaf7"
        ],
        [
          "5035eba4242614931c934fc1f859a60f289d064584d2fbf7f60d8799e5203dc8",
          "048962ae95d5006c5ee8065d7ec54deb5a3aeb69df5839b90c2b162aace11a71",
          "9477f723460bf107c24c6a738e2d6a6e7899ec099febb1ae935320e2224113fe",
          "20a0e7f0daf9ff836c077d54ec19200855e364b2f42e33409d29c56d96fd840f",
          "c7c416763c1f6e710221b1fac1df53a48a67120757980b912cf347ae96a0180e",
          "e3c286917b3326ab699daad0a811d432cdb972a9a119ddd5e00fcfd371e4c51d",
          "a69ddbf26eca807b206334187fc1dba54e3a2fa23711718b85dcf5718f00ebc5"
        ],
        [
          "8d565ee9746e4b8e8fcacb4781daefd8d60d8d02a32cc7d50f21a8f806814412",
          "bca15de05abec263ccc8465073f498f8dac99d2a519a945e5f43c38bcb268d26",
          "d4fee218c37ee262f034268e855be14b06d566dcb803671f5f1c8c30dc3de16f",
          "6b063fc0fe2fd2ede25005f056f46a4be90c0a35389d5e51ebe2a667b623afce",
          "4c0d392fb7234ba5550656ba5c20336943767a8ac425ca785224c85e3aabec27",
          "b5a60a833987bf48c6233fdfc7b9ccca6744f2f39e7c1f0cf6673a89118cd8f0",
          "777adae546e1f63f380ae61dd8512f2359aa99637aff53af07d1ec33d5d0f423"
        ],
        [
          "8e969e807df61b7d881fdf2b3b2196cc47a88fe20138c7cc0da5f7c8e3456c86",
          "08ec45d62c5e934acf33c50768703e8baeaff6cb198e8a3850661cd81df27e6b",
          "5dfdf3f654b907e101aa34cbd27238b22944572bf46ae26116781cb4c0d333c4",
          "49b5fd8cb960ddd0d8043de5c6b1d0aac232798590cabc8329d3a9d1b15d0b92",
          "28ef728bc8e680f1b5672959b1b20d406bf5f2118b966fe8272d3a8025964b43",
          "e4c8be84cfc7b5841c8b81f9bb5ce84e4622743ac9d0912b175c43e34c2ed068",
          "d305aee55cb8589ee55329682d058e94d947516f3567c1db0c31b0595a2dbb90"
        ]
      ],
      "intermediate_planes_retained": false,
      "total_seconds": 3.639010894985404,
      "post_exec_memory": {
        "start": {
          "VmSize": "36308 kB",
          "VmHWM": "10092 kB",
          "VmRSS": "10092 kB"
        },
        "finish": {
          "VmSize": "309136 kB",
          "VmHWM": "288704 kB",
          "VmRSS": "282432 kB"
        }
      }
    },
    "error": null
  },
  "guards": {
    "out": {
      "bdd_oracles": 96,
      "selector_checks": 128,
      "complete_sparse_guards": true,
      "all_registered_contexts_complete": true,
      "original_complete": false,
      "early_coordinate_selector_masks_hex": [
        "0x141b4a8ea5000000",
        "0x4000000000000000",
        "0x1866dc638a200000",
        "0x36f4c87205880000",
        "0x2f693e69f1500000",
        "0x3343477e70682000",
        "0x61354474dab40000",
        "0x7e7b961942a10000",
        "0xedd4cc2bef43000",
        "0x1fd04864889a0000",
        "0x41ee5a4c75004000",
        "0x21b2bb4271478000",
        "0x370c720d9703f400",
        "0x444d4e1c45f56800",
        "0x53994ee8d88aa020",
        "0x554ad358cbbbda00",
        "0x5986f4097de60300",
        "0x169ccc9a5d296740",
        "0x2720c64991795880",
        "0x32904d47d39c53c1",
        "0x7ce1a6430f658aa8",
        "0x6c1674918a9596cc",
        "0x205a1e866c88c88a",
        "0x40a7fd50a0b27a10"
      ],
      "total_seconds": 2.9543825969740283,
      "post_exec_memory": {
        "start": {
          "VmSize": "36308 kB",
          "VmHWM": "10100 kB",
          "VmRSS": "10100 kB"
        },
        "finish": {
          "VmSize": "243000 kB",
          "VmHWM": "288744 kB",
          "VmRSS": "216880 kB"
        }
      },
      "strata": [
        {
          "rank": 2,
          "states": 74,
          "root": 949,
          "nonterminal_nodes": 948,
          "truth_sha256": "6e61457ed3061571c44b791a6ad60faecb89fabfd7e4a6953c64bc00316b3919",
          "parameter_upper_bound": 10170482556928
        },
        {
          "rank": 3,
          "states": 10154,
          "root": 18832,
          "nonterminal_nodes": 18831,
          "truth_sha256": "34937feaf56c6af27d268948d9449d8fe9fe3928284609b2175fcc505d15421b",
          "parameter_upper_bound": 697777566777344
        }
      ]
    },
    "error": null
  },
  "accepted_census": true,
  "supervisor": {
    "diagnostic": {
      "status": "ok",
      "wall_seconds": 1.400841519003734,
      "peak_process_rss_kib": 942880
    },
    "census": {
      "status": "ok",
      "wall_seconds": 3.703381819999777,
      "peak_process_rss_kib": 937824
    },
    "guards": {
      "status": "ok",
      "wall_seconds": 3.0318451980128884,
      "peak_process_rss_kib": 288744
    }
  },
  "workers": 3,
  "qualified_speedup": null,
  "goal_complete": false
}

The rank6/all64-state diagnostic compares actual original-selector evaluations with typed coefficient images,then every primal and tangent coefficient with the independent full scalar jet:20160coefficient checks and384matrix-row checks. A concrete old-map counterexample is retained:state1has selector witness0x11c7818but coefficient image0x140. The old misencoded family was not the actual normalized early family. Both the unchanged pointwise recurrence and the original C144short-kernel result remain distinct from that input-encoding defect.

The repaired census evaluates all16777216reachable early cells of the fixed conditional k3,n83/d18norm chart. Entire rank masks agree between two independent algorithms:masked row Gaussian elimination and descending-column basis insertion. Tiny complete2x3matrices/random6x7controls and32actual-selector readbacks precede acceptance. For rank r,each early cell contains at most2^(39-r)parameters satisfying six necessary corrections;therefore U=sum count_r*2^(39-r)is a rigorous upper bound for this conditional63parameter chart. It is not an exact original root count,not a bound across all source/U/ghost/other-leading/outer branches,and not a speed ratio.

The complete distribution reveals74rank2and10154rank3cells that the256sample had missed. Never impose rank>=4globally. The fresh third experiment retains these strata as exact reduced ordered binary decision diagrams. Each guard is expanded back to the full2^24truth mask and matches bit-for-bit. Original63selectors map to the24early-coordinate bits through exported exact linear masks,checked on128points. All96small truth-table controls compare every point. Full node lists,roots,coordinate masks and truth fingerprints are in raw evidence. BDDnode counts are construction/storage observations,not a runtime gain;rank3's18831nodes may add material circuit cost.

Three90s4GiBCPU0workers ran once:the two prerequisites through normal root resume/run-ready and the new guard hypothesis in append-only childledger/c145after the census passed. Source/input hashes,frozen manifests,failed C144raw and all earlier attempts remain unchanged. Whole-child lifetimeRSSand post-exec VmHWM/VmRSSare kept separately. Auxiliary census/bound/count fields are explicitly demoted on late/error serialization paths;supervisor full-child status/deadline additionally gates acceptance. Full root and child validation receipts and portable source/raw replay are retained.

Novelty review:the basis(image,witness)contract is established exact linear algebra;bit-slicing,truth tables,Gaussian elimination and reduced ordered BDDsare established techniques. Prior pinned FLINT/M4RIprimary-source receipts remain applicable;Bryant's1986Graph-Based Algorithms for Boolean Function Manipulation(DOI10.1109/TC.1986.1676819)is the canonical BDDreference,not a new lookup or novel-primitive claim. The contribution tested here is exact application to this conditional finite-field family. Complete generalized-rootfinding novelty and the qualified matching gate remain unestablished.

Next: C146-rank-stratified-K16-compiler. Compile a normal rank>=4decoder and every rank2/rank3exceptional branch with exact free coordinates,left-null consistency and original selector/q/source recovery. Compare direct symbolic Gaussian guards with the BDDguards and charge the full construction/extraction/verification cost. A59inputnormal branch cannot replace the exceptional branches;rank2needs up to5delayed free bits. Require independent complete tiny original-root sets and a bounded full-graph cost profile before any changed native diagnostic. Reflected prime frontend/general additive quotients remain queued.

C144's42.993xratio is only a tangent-kernel development comparison against existing seven full jets with internal checks. This cycle has no new matched speed ratio. The20hard preregistered completed genuineF4/F5/F6gate,independent complete original roots,fastestbaseline>=1s,threecold repeats,A/A<=10%,qualified isolation,requested field/curve/scales and2xfloor/50xstretch remain active and unpassed.
