.in <raw> (
    10 15
)

.out (
    00
)

start:
    JMP main

main:
    ISMLD R0 #in   1
    ISMLD R1 #in+1 1
    IADD  R2 R0    R1
    ISMST R2 #out  1
    IJEQ  main R0 R1
    JMP   inf_prog
    STOP

inf_prog:
    JMP start