import Tangent

/-! Kernel-checked arithmetic certificate for all frozen auxiliary-x-collision pairs. -/

namespace XedniCollisionTangent

set_option maxRecDepth 20000
set_option maxHeartbeats 0

def sourceTable : List Coords := [
  ⟨2, 16, 2, 6⟩, -- source 13
  ⟨2, 16, 2, -6⟩, -- source 14
  ⟨2, 23, 2, 4⟩, -- source 19
  ⟨2, 23, 2, -4⟩, -- source 20
  ⟨2, -20, 2, 14⟩, -- source 25
  ⟨2, -20, 2, -14⟩, -- source 26
  ⟨2, -12, 2, 16⟩, -- source 33
  ⟨2, -12, 2, -16⟩, -- source 34
  ⟨2, 16, -2, 6⟩, -- source 54
  ⟨2, 16, -2, -6⟩, -- source 55
  ⟨2, 23, -2, 4⟩, -- source 60
  ⟨2, 23, -2, -4⟩, -- source 61
  ⟨2, -20, -2, 14⟩, -- source 66
  ⟨2, -20, -2, -14⟩, -- source 67
  ⟨2, -12, -2, 16⟩, -- source 74
  ⟨2, -12, -2, -16⟩, -- source 75
  ⟨5, 16, 20, 6⟩, -- source 95
  ⟨5, 16, 20, -6⟩, -- source 96
  ⟨5, -23, 20, 11⟩, -- source 103
  ⟨5, -23, 20, -11⟩, -- source 104
  ⟨5, -17, 20, 15⟩, -- source 111
  ⟨5, -17, 20, -15⟩, -- source 112
  ⟨5, -16, 20, 17⟩, -- source 113
  ⟨5, -16, 20, -17⟩, -- source 114
  ⟨5, 16, -20, 6⟩, -- source 136
  ⟨5, 16, -20, -6⟩, -- source 137
  ⟨5, -23, -20, 11⟩, -- source 144
  ⟨5, -23, -20, -11⟩, -- source 145
  ⟨5, -17, -20, 15⟩, -- source 152
  ⟨5, -17, -20, -15⟩, -- source 153
  ⟨5, -16, -20, 17⟩, -- source 154
  ⟨5, -16, -20, -17⟩, -- source 155
  ⟨8, 15, 19, 16⟩, -- source 175
  ⟨8, 15, 19, -16⟩, -- source 176
  ⟨8, 22, 19, 7⟩, -- source 181
  ⟨8, 22, 19, -7⟩, -- source 182
  ⟨8, -20, 19, 14⟩, -- source 189
  ⟨8, -20, 19, -14⟩, -- source 190
  ⟨8, -3, 19, 16⟩, -- source 203
  ⟨8, -3, 19, -16⟩, -- source 204
  ⟨8, 15, -19, 16⟩, -- source 216
  ⟨8, 15, -19, -16⟩, -- source 217
  ⟨8, 22, -19, 7⟩, -- source 222
  ⟨8, 22, -19, -7⟩, -- source 223
  ⟨8, -20, -19, 14⟩, -- source 230
  ⟨8, -20, -19, -14⟩, -- source 231
  ⟨8, -3, -19, 16⟩, -- source 244
  ⟨8, -3, -19, -16⟩, -- source 245
  ⟨10, 17, 1, 10⟩, -- source 261
  ⟨10, 17, 1, -10⟩, -- source 262
  ⟨10, -23, 1, 11⟩, -- source 267
  ⟨10, -23, 1, -11⟩, -- source 268
  ⟨10, -18, 1, 23⟩, -- source 273
  ⟨10, -18, 1, -23⟩, -- source 274
  ⟨10, -12, 1, 16⟩, -- source 279
  ⟨10, -12, 1, -16⟩, -- source 280
  ⟨10, -4, 1, 19⟩, -- source 283
  ⟨10, -4, 1, -19⟩, -- source 284
  ⟨10, 17, -1, 10⟩, -- source 302
  ⟨10, 17, -1, -10⟩, -- source 303
  ⟨10, -23, -1, 11⟩, -- source 308
  ⟨10, -23, -1, -11⟩, -- source 309
  ⟨10, -18, -1, 23⟩, -- source 314
  ⟨10, -18, -1, -23⟩, -- source 315
  ⟨10, -12, -1, 16⟩, -- source 320
  ⟨10, -12, -1, -16⟩, -- source 321
  ⟨10, -4, -1, 19⟩, -- source 324
  ⟨10, -4, -1, -19⟩, -- source 325
  ⟨11, 22, 7, 7⟩, -- source 345
  ⟨11, 22, 7, -7⟩, -- source 346
  ⟨11, -17, 7, 15⟩, -- source 357
  ⟨11, -17, 7, -15⟩, -- source 358
  ⟨11, -10, 7, 18⟩, -- source 363
  ⟨11, -10, 7, -18⟩, -- source 364
  ⟨11, -3, 7, 16⟩, -- source 367
  ⟨11, -3, 7, -16⟩, -- source 368
  ⟨11, 22, -7, 7⟩, -- source 386
  ⟨11, 22, -7, -7⟩, -- source 387
  ⟨11, -17, -7, 15⟩, -- source 398
  ⟨11, -17, -7, -15⟩, -- source 399
  ⟨11, -10, -7, 18⟩, -- source 404
  ⟨11, -10, -7, -18⟩, -- source 405
  ⟨11, -3, -7, 16⟩, -- source 408
  ⟨11, -3, -7, -16⟩, -- source 409
  ⟨14, -21, 7, 8⟩, -- source 433
  ⟨14, -21, 7, -8⟩, -- source 434
  ⟨14, -21, -7, 8⟩, -- source 474
  ⟨14, -21, -7, -8⟩, -- source 475
  ⟨15, 8, 16, 19⟩, -- source 496
  ⟨15, 8, 16, -19⟩, -- source 497
  ⟨15, 22, 16, 7⟩, -- source 509
  ⟨15, 22, 16, -7⟩, -- source 510
  ⟨15, -20, 16, 14⟩, -- source 517
  ⟨15, -20, 16, -14⟩, -- source 518
  ⟨15, -18, 16, 23⟩, -- source 519
  ⟨15, -18, 16, -23⟩, -- source 520
  ⟨15, 8, -16, 19⟩, -- source 537
  ⟨15, 8, -16, -19⟩, -- source 538
  ⟨15, 22, -16, 7⟩, -- source 550
  ⟨15, 22, -16, -7⟩, -- source 551
  ⟨15, -20, -16, 14⟩, -- source 558
  ⟨15, -20, -16, -14⟩, -- source 559
  ⟨15, -18, -16, 23⟩, -- source 560
  ⟨15, -18, -16, -23⟩, -- source 561
  ⟨16, 2, 6, 2⟩, -- source 574
  ⟨16, 2, 6, -2⟩, -- source 575
  ⟨16, 5, 6, 20⟩, -- source 576
  ⟨16, 5, 6, -20⟩, -- source 577
  ⟨16, 23, 6, 4⟩, -- source 593
  ⟨16, 23, 6, -4⟩, -- source 594
  ⟨16, -17, 6, 15⟩, -- source 603
  ⟨16, -17, 6, -15⟩, -- source 604
  ⟨16, -12, 6, 16⟩, -- source 607
  ⟨16, -12, 6, -16⟩, -- source 608
  ⟨16, 2, -6, 2⟩, -- source 615
  ⟨16, 2, -6, -2⟩, -- source 616
  ⟨16, 5, -6, 20⟩, -- source 617
  ⟨16, 5, -6, -20⟩, -- source 618
  ⟨16, 23, -6, 4⟩, -- source 634
  ⟨16, 23, -6, -4⟩, -- source 635
  ⟨16, -17, -6, 15⟩, -- source 644
  ⟨16, -17, -6, -15⟩, -- source 645
  ⟨16, -12, -6, 16⟩, -- source 648
  ⟨16, -12, -6, -16⟩, -- source 649
  ⟨17, 10, 10, 1⟩, -- source 662
  ⟨17, 10, 10, -1⟩, -- source 663
  ⟨17, -18, 10, 23⟩, -- source 683
  ⟨17, -18, 10, -23⟩, -- source 684
  ⟨17, -16, 10, 17⟩, -- source 687
  ⟨17, -16, 10, -17⟩, -- source 688
  ⟨17, -4, 10, 19⟩, -- source 693
  ⟨17, -4, 10, -19⟩, -- source 694
  ⟨17, 10, -10, 1⟩, -- source 703
  ⟨17, 10, -10, -1⟩, -- source 704
  ⟨17, -18, -10, 23⟩, -- source 724
  ⟨17, -18, -10, -23⟩, -- source 725
  ⟨17, -16, -10, 17⟩, -- source 728
  ⟨17, -16, -10, -17⟩, -- source 729
  ⟨17, -4, -10, 19⟩, -- source 734
  ⟨17, -4, -10, -19⟩, -- source 735
  ⟨22, 8, 7, 19⟩, -- source 742
  ⟨22, 8, 7, -19⟩, -- source 743
  ⟨22, 11, 7, 7⟩, -- source 746
  ⟨22, 11, 7, -7⟩, -- source 747
  ⟨22, 15, 7, 16⟩, -- source 750
  ⟨22, 15, 7, -16⟩, -- source 751
  ⟨22, -20, 7, 14⟩, -- source 763
  ⟨22, -20, 7, -14⟩, -- source 764
  ⟨22, 8, -7, 19⟩, -- source 783
  ⟨22, 8, -7, -19⟩, -- source 784
  ⟨22, 11, -7, 7⟩, -- source 787
  ⟨22, 11, -7, -7⟩, -- source 788
  ⟨22, 15, -7, 16⟩, -- source 791
  ⟨22, 15, -7, -16⟩, -- source 792
  ⟨22, -20, -7, 14⟩, -- source 804
  ⟨22, -20, -7, -14⟩, -- source 805
  ⟨23, 2, 4, 2⟩, -- source 820
  ⟨23, 2, 4, -2⟩, -- source 821
  ⟨23, 16, 4, 6⟩, -- source 834
  ⟨23, 16, 4, -6⟩, -- source 835
  ⟨23, -21, 4, 8⟩, -- source 843
  ⟨23, -21, 4, -8⟩, -- source 844
  ⟨23, -12, 4, 16⟩, -- source 853
  ⟨23, -12, 4, -16⟩, -- source 854
  ⟨23, -10, 4, 18⟩, -- source 855
  ⟨23, -10, 4, -18⟩, -- source 856
  ⟨23, 2, -4, 2⟩, -- source 861
  ⟨23, 2, -4, -2⟩, -- source 862
  ⟨23, 16, -4, 6⟩, -- source 875
  ⟨23, 16, -4, -6⟩, -- source 876
  ⟨23, -21, -4, 8⟩, -- source 884
  ⟨23, -21, -4, -8⟩, -- source 885
  ⟨23, -12, -4, 16⟩, -- source 894
  ⟨23, -12, -4, -16⟩, -- source 895
  ⟨23, -10, -4, 18⟩, -- source 896
  ⟨23, -10, -4, -18⟩, -- source 897
  ⟨-23, 5, 11, 20⟩, -- source 904
  ⟨-23, 5, 11, -20⟩, -- source 905
  ⟨-23, 10, 11, 1⟩, -- source 908
  ⟨-23, 10, 11, -1⟩, -- source 909
  ⟨-23, -16, 11, 17⟩, -- source 933
  ⟨-23, -16, 11, -17⟩, -- source 934
  ⟨-23, -12, 11, 16⟩, -- source 935
  ⟨-23, -12, 11, -16⟩, -- source 936
  ⟨-23, 5, -11, 20⟩, -- source 945
  ⟨-23, 5, -11, -20⟩, -- source 946
  ⟨-23, 10, -11, 1⟩, -- source 949
  ⟨-23, 10, -11, -1⟩, -- source 950
  ⟨-23, -16, -11, 17⟩, -- source 974
  ⟨-23, -16, -11, -17⟩, -- source 975
  ⟨-23, -12, -11, 16⟩, -- source 976
  ⟨-23, -12, -11, -16⟩, -- source 977
  ⟨-21, 14, 8, 7⟩, -- source 994
  ⟨-21, 14, 8, -7⟩, -- source 995
  ⟨-21, 23, 8, 4⟩, -- source 1004
  ⟨-21, 23, 8, -4⟩, -- source 1005
  ⟨-21, -10, 8, 18⟩, -- source 1019
  ⟨-21, -10, 8, -18⟩, -- source 1020
  ⟨-21, 14, -8, 7⟩, -- source 1035
  ⟨-21, 14, -8, -7⟩, -- source 1036
  ⟨-21, 23, -8, 4⟩, -- source 1045
  ⟨-21, 23, -8, -4⟩, -- source 1046
  ⟨-21, -10, -8, 18⟩, -- source 1060
  ⟨-21, -10, -8, -18⟩, -- source 1061
  ⟨-20, 2, 14, 2⟩, -- source 1066
  ⟨-20, 2, 14, -2⟩, -- source 1067
  ⟨-20, 8, 14, 19⟩, -- source 1070
  ⟨-20, 8, 14, -19⟩, -- source 1071
  ⟨-20, 15, 14, 16⟩, -- source 1078
  ⟨-20, 15, 14, -16⟩, -- source 1079
  ⟨-20, 22, 14, 7⟩, -- source 1084
  ⟨-20, 22, 14, -7⟩, -- source 1085
  ⟨-20, 2, -14, 2⟩, -- source 1107
  ⟨-20, 2, -14, -2⟩, -- source 1108
  ⟨-20, 8, -14, 19⟩, -- source 1111
  ⟨-20, 8, -14, -19⟩, -- source 1112
  ⟨-20, 15, -14, 16⟩, -- source 1119
  ⟨-20, 15, -14, -16⟩, -- source 1120
  ⟨-20, 22, -14, 7⟩, -- source 1125
  ⟨-20, 22, -14, -7⟩, -- source 1126
  ⟨-18, 10, 23, 1⟩, -- source 1154
  ⟨-18, 10, 23, -1⟩, -- source 1155
  ⟨-18, 15, 23, 16⟩, -- source 1160
  ⟨-18, 15, 23, -16⟩, -- source 1161
  ⟨-18, 17, 23, 10⟩, -- source 1164
  ⟨-18, 17, 23, -10⟩, -- source 1165
  ⟨-18, -4, 23, 19⟩, -- source 1185
  ⟨-18, -4, 23, -19⟩, -- source 1186
  ⟨-18, 10, -23, 1⟩, -- source 1195
  ⟨-18, 10, -23, -1⟩, -- source 1196
  ⟨-18, 15, -23, 16⟩, -- source 1201
  ⟨-18, 15, -23, -16⟩, -- source 1202
  ⟨-18, 17, -23, 10⟩, -- source 1205
  ⟨-18, 17, -23, -10⟩, -- source 1206
  ⟨-18, -4, -23, 19⟩, -- source 1226
  ⟨-18, -4, -23, -19⟩, -- source 1227
  ⟨-17, 5, 15, 20⟩, -- source 1232
  ⟨-17, 5, 15, -20⟩, -- source 1233
  ⟨-17, 11, 15, 7⟩, -- source 1238
  ⟨-17, 11, 15, -7⟩, -- source 1239
  ⟨-17, 16, 15, 6⟩, -- source 1244
  ⟨-17, 16, 15, -6⟩, -- source 1245
  ⟨-17, -10, 15, 18⟩, -- source 1265
  ⟨-17, -10, 15, -18⟩, -- source 1266
  ⟨-17, -3, 15, 16⟩, -- source 1269
  ⟨-17, -3, 15, -16⟩, -- source 1270
  ⟨-17, 5, -15, 20⟩, -- source 1273
  ⟨-17, 5, -15, -20⟩, -- source 1274
  ⟨-17, 11, -15, 7⟩, -- source 1279
  ⟨-17, 11, -15, -7⟩, -- source 1280
  ⟨-17, 16, -15, 6⟩, -- source 1285
  ⟨-17, 16, -15, -6⟩, -- source 1286
  ⟨-17, -10, -15, 18⟩, -- source 1306
  ⟨-17, -10, -15, -18⟩, -- source 1307
  ⟨-17, -3, -15, 16⟩, -- source 1310
  ⟨-17, -3, -15, -16⟩, -- source 1311
  ⟨-16, 5, 17, 20⟩, -- source 1314
  ⟨-16, 5, 17, -20⟩, -- source 1315
  ⟨-16, 17, 17, 10⟩, -- source 1328
  ⟨-16, 17, 17, -10⟩, -- source 1329
  ⟨-16, -23, 17, 11⟩, -- source 1334
  ⟨-16, -23, 17, -11⟩, -- source 1335
  ⟨-16, 5, -17, 20⟩, -- source 1355
  ⟨-16, 5, -17, -20⟩, -- source 1356
  ⟨-16, 17, -17, 10⟩, -- source 1369
  ⟨-16, 17, -17, -10⟩, -- source 1370
  ⟨-16, -23, -17, 11⟩, -- source 1375
  ⟨-16, -23, -17, -11⟩, -- source 1376
  ⟨-12, 2, 16, 2⟩, -- source 1394
  ⟨-12, 2, 16, -2⟩, -- source 1395
  ⟨-12, 10, 16, 1⟩, -- source 1400
  ⟨-12, 10, 16, -1⟩, -- source 1401
  ⟨-12, 16, 16, 6⟩, -- source 1408
  ⟨-12, 16, 16, -6⟩, -- source 1409
  ⟨-12, 23, 16, 4⟩, -- source 1414
  ⟨-12, 23, 16, -4⟩, -- source 1415
  ⟨-12, -23, 16, 11⟩, -- source 1416
  ⟨-12, -23, 16, -11⟩, -- source 1417
  ⟨-12, 2, -16, 2⟩, -- source 1435
  ⟨-12, 2, -16, -2⟩, -- source 1436
  ⟨-12, 10, -16, 1⟩, -- source 1441
  ⟨-12, 10, -16, -1⟩, -- source 1442
  ⟨-12, 16, -16, 6⟩, -- source 1449
  ⟨-12, 16, -16, -6⟩, -- source 1450
  ⟨-12, 23, -16, 4⟩, -- source 1455
  ⟨-12, 23, -16, -4⟩, -- source 1456
  ⟨-12, -23, -16, 11⟩, -- source 1457
  ⟨-12, -23, -16, -11⟩, -- source 1458
  ⟨-10, 11, 18, 7⟩, -- source 1484
  ⟨-10, 11, 18, -7⟩, -- source 1485
  ⟨-10, 23, 18, 4⟩, -- source 1496
  ⟨-10, 23, 18, -4⟩, -- source 1497
  ⟨-10, -21, 18, 8⟩, -- source 1500
  ⟨-10, -21, 18, -8⟩, -- source 1501
  ⟨-10, -17, 18, 15⟩, -- source 1506
  ⟨-10, -17, 18, -15⟩, -- source 1507
  ⟨-10, -3, 18, 16⟩, -- source 1515
  ⟨-10, -3, 18, -16⟩, -- source 1516
  ⟨-10, 11, -18, 7⟩, -- source 1525
  ⟨-10, 11, -18, -7⟩, -- source 1526
  ⟨-10, 23, -18, 4⟩, -- source 1537
  ⟨-10, 23, -18, -4⟩, -- source 1538
  ⟨-10, -21, -18, 8⟩, -- source 1541
  ⟨-10, -21, -18, -8⟩, -- source 1542
  ⟨-10, -17, -18, 15⟩, -- source 1547
  ⟨-10, -17, -18, -15⟩, -- source 1548
  ⟨-10, -3, -18, 16⟩, -- source 1556
  ⟨-10, -3, -18, -16⟩, -- source 1557
  ⟨-4, 10, 19, 1⟩, -- source 1564
  ⟨-4, 10, 19, -1⟩, -- source 1565
  ⟨-4, 17, 19, 10⟩, -- source 1574
  ⟨-4, 17, 19, -10⟩, -- source 1575
  ⟨-4, -18, 19, 23⟩, -- source 1586
  ⟨-4, -18, 19, -23⟩, -- source 1587
  ⟨-4, 10, -19, 1⟩, -- source 1605
  ⟨-4, 10, -19, -1⟩, -- source 1606
  ⟨-4, 17, -19, 10⟩, -- source 1615
  ⟨-4, 17, -19, -10⟩, -- source 1616
  ⟨-4, -18, -19, 23⟩, -- source 1627
  ⟨-4, -18, -19, -23⟩, -- source 1628
  ⟨-3, 8, 16, 19⟩, -- source 1644
  ⟨-3, 8, 16, -19⟩, -- source 1645
  ⟨-3, 11, 16, 7⟩, -- source 1648
  ⟨-3, 11, 16, -7⟩, -- source 1649
  ⟨-3, -17, 16, 15⟩, -- source 1670
  ⟨-3, -17, 16, -15⟩, -- source 1671
  ⟨-3, -10, 16, 18⟩, -- source 1676
  ⟨-3, -10, 16, -18⟩, -- source 1677
  ⟨-3, 8, -16, 19⟩, -- source 1685
  ⟨-3, 8, -16, -19⟩, -- source 1686
  ⟨-3, 11, -16, 7⟩, -- source 1689
  ⟨-3, 11, -16, -7⟩, -- source 1690
  ⟨-3, -17, -16, 15⟩, -- source 1711
  ⟨-3, -17, -16, -15⟩, -- source 1712
  ⟨-3, -10, -16, 18⟩, -- source 1717
  ⟨-3, -10, -16, -18⟩, -- source 1718
]

theorem sourceTable_length : sourceTable.length = 336 := by decide

def sourceCase (i : Fin 336) : Coords :=
  sourceTable.get ⟨i.val, by rw [sourceTable_length]; exact i.isLt⟩

def arithmeticBlocked (c : Coords) : Prop :=
  (c.y1-c.y2)^2 % (c.x1-c.x2) ≠ 0 ∨
    ((c.x1-c.x2)^2*(c.x1+2*c.x2)-(c.y1-c.y2)^2) % 47 ≠ 0

theorem all_source_cases_blocked : ∀ i : Fin 336, arithmeticBlocked (sourceCase i) := by
  dsimp only [arithmeticBlocked]
  decide

theorem no_relation_for_all_collision_cases (i : Fin 336)
    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] (k1 k2 : ℤ)
    (ha1 : W.a₁ = ((47*k1 : ℤ) : ℚ))
    (ha2 : W.a₂ = ((47*k2 : ℤ) : ℚ))
    (hP : W.Nonsingular ((sourceCase i).x1 : ℚ) ((sourceCase i).y1 : ℚ))
    (hQ : W.Nonsingular ((sourceCase i).x2 : ℚ) ((sourceCase i).y2 : ℚ)) :
    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x1 : ℚ)
      ((sourceCase i).y1 : ℚ) hP) +
    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)
      ((sourceCase i).y2 : ℚ) hQ) +
    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)
      ((sourceCase i).y2 : ℚ) hQ) ≠ 0 := by
  rcases all_source_cases_blocked i with hrem | hmod
  · have hdiv : ¬ ((sourceCase i).x1-(sourceCase i).x2) ∣
        ((sourceCase i).y1-(sourceCase i).y2)^2 := by
      intro hdvd
      exact hrem (Int.emod_eq_zero_of_dvd hdvd)
    exact no_relation_of_nondvd W (47*k1) (47*k2) _ _ _ _ ha1 ha2 hP hQ hdiv
  · exact no_relation_of_mod47 W k1 k2 _ _ _ _ ha1 ha2 hP hQ hmod

#print axioms all_source_cases_blocked
#print axioms no_relation_for_all_collision_cases

end XedniCollisionTangent
