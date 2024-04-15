.in <raw> (
    10 15
)

.out (
    00
)
;Тест комментария
start:
    IADD R0 R0 R0

main:
    ISMLD R0 #in   1
    ISMLD R1 #in+1 1
    IADD  R2 R0    R1
    ISMST R2 #out  1
    STOP