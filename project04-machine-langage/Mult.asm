// R2 = R0 * R1
@i
M=0
@sum
M=0

(LOOP)
  // if i > R1 goto STOP
  @i 
  D=M
  @R1
  D=D-M
  @STOP
  D;JEQ
  // sum = sum + R0
  @R0
  D=M
  @sum
  M=D+M
  // i++
  @i 
  M=M+1
  @LOOP
  0;JMP

(STOP)
  @sum
  D=M
  @R2
  M=D

(END)
  @END
  0;JMP