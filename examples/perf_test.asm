.minus_one <double> (
    1.0
)

pub main:
    ISR R0 100000002   ; Количество итераций
    ISR R1 1
    ITOD R0 R1          ; var pi = 1.0
    ISR R1 2            ; var i = 2
    ISR R2 1            ; var x = 1
    ISR R3 -1            ; x mult 
    

    while_start:
        IJEG R2 R1 while_end       ;

        IMULT R2 R2 R3
        ISR R4 2
        IMULT R4 R1 R4
        ISR R5 1
        ISUB R4 R4 R5


        IINc R2;
        JMP while_start;
    while_end:
    DSMLD R0 #minus_one
    ;LIBCALL print_long
    LIBCALL print_pi
    STOP                ; Остановка программы
    
pub fn:
    ;IOUT R0
    RET