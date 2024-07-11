; .in <raw> (
;     10 15
; )

; .out (
;     00
; )
; start:
;     JMP main

; main:
;     ISMLD R0 #in   1
;     ISMLD R1 #in+1 1
;     IADD  R2 R0    R1
;     ISMST R2 #out  1
;     IJEQ  main R0 R1
;     ; JMP   inf_prog
;     STOP

; inf_prog:
;     JMP start




; .n (
;     0C ; 12 в 10-ой
; )

; .result (
;     01 00 00 00  ; 4 байта на результат
; )

; start: 
;     ISMLD R0 #n 1 ; Загрузка n в первый 32-битный регистр
;     ISMLD R1 #result 4 ; Загрузка result во второй 32-битный регистр

;     ISR R2 1 ; Установка в 3 реистр 1 для n > 1
;     while_start:
;         IJEL while_end R0 R2 ; Если R0 <= R2, то переход к while_end
;         IMULT R1 R0 R1 ; result = result * n
;         ISR R3 1 ; Установка в 4 регистр 1
;         ISUB R0 R0 R3 ; n = n - 1
;         JMP while_start ; Переход к while_start
;     while_end:
;     ISMST R1 #result 4 ; Сохранение результата в память    
;     STOP ; Остановка программы





; .n (
;     0C ; 12 в 10-ой
; )

; .result (
;     01 00 00 00  ; 4 байта на результат
; )

; main:
;     jmp start

; calculate:
;     ISR R2 1 ; Установка в 3 реистр 1 для n > 1
;     while_start:
;         IJEL while_end R0 R2 ; Если R0 <= R2, то переход к while_end
;         ;LIBCALL print_IR1
;         IMULT R1 R0 R1 ; result = result * n
;         ISR R3 1 ; Установка в 4 регистр 1
;         ISUB R0 R0 R3 ; n = n - 1
;         JMP while_start ; Переход к while_start
;     while_end:
;     RET

; start: 
;     ISMLD R0 #n 1 ; Загрузка n в первый 32-битный регистр
;     ISMLD R1 #result 4 ; Загрузка result во второй 32-битный регистр
;     CALL calculate
;     ISMST R1 #result 4 ; Сохранение результата в память    
;     ;LIBCALL hello
;     LIBCALL print_IR1
;     STOP ; Остановка программы




; .raw_text (
;     d0 9a d0 b0 d0 ba d0 be d0 b9 2d d1 82 d0 be 20 74 65 78 74 00
; )

; .text <text> (
; Какой-то текст #2
; )

; .multiline_text <text> (
; Line 1\n\a\a\a
; Line 2
; Line 3
; )


; main:
;     LSR R1 #raw_text
;     LIBCALL println
;     LSR R1 #text
;     LIBCALL println
;     LSR R1 #multiline_text
;     LIBCALL println
;     STOP




.counter (
    00 00 00 00 ; 4 байта на счетчик
)

.b <float> (
    15.32
    14.89
)

main:
    ISMLD R0 #counter 4 ; Загрузка счетчика в первый 32-битный регистр
    ISR   R1 100         ; Установка во второй 32-x регистр числа 10
    ITOF  R0 R0         ; Преобразование счетчика во float и сохраниение в первом плавающем регистре
    ITOF  R1 R1         ; Преобразование числа 10 во float и сохраниение во втором плавающем регистре
    FDIV  R0 R0 R1      ; Деление первого плавающего регистра на второй и сохранение в первом
    LIBCALL fcos        ; Вызов функции косинуса для первого плавающего регистра и сохранение в нём
    LIBCALL set_mesh_y  ; Установка значения первого плавающего регистра в меш

    ISR R1 1            ; Установка 1 во второй 32-битный регистр
    IADD R0 R0 R1       ; Увеличение счётчика на 1
    ISMST R0 #counter 4 ; Сохранение счётчика в память
    STOP                ; Остановка программы