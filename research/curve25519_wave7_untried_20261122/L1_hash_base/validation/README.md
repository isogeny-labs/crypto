# Validator scripts (copied from the session scratchpad after the validator finished)

Independent code written by the validator; it imports nothing from the lead's scripts. sha256 of each file:

    931e41a49294d79514502cd7c685d646a0b5e6331dbc0fed4b19c6501909f2ec  vcompare.py
    1a084339910c5b4f47ecd9c5798278e24e54756ebea1d7bd732633002b9ca438  vdata.py
    878ccbac639b1875c3fc3508372a8dae4500e00115f04426e40ffb45069c233d  vdata_holdout.py
    d3cf0abd762aa4f0a2f9b16d3b83638f3e75c7861a1f680ae9a55958c236d9a3  vfacts.py
    89c0fc3c8f8b832e3117dd158b1dfca6d772fcc0cf3f1f95cc69f9846244f993  vreplay.py
    9d2987b9b1aa8cf7948158862805ab7fa5a9f9ffd4067de5e51ad49b70fa6a66  vreport.py

All but vreport.py match the sha256 values recorded in ../validator-report.json; vreport.py (the report writer) differs from its recorded value. A file named replay.py in the shared scratchpad was overwritten by another validator during the wave; it is not among the scripts named in ../validator-report.json.
