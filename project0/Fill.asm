// if KBD == 1
// SCREEN is blacken
// else
// SCREEN is cleared
@KBD

// if KBD > 0 goto BLACKEN
@BLACKEN
M;JGT


(BLACKEN)
  @i 
  D=M
  @KBD
  D=D-M
  @END
  D;JEQ

  @SCREEN
  D=M
  @i 
  A=D+M
  M=-1

  @i 
  M=M+1

  0;JMP

(CLEARED)
  @i 
  D=M
  @KBD
  D=D-M
  @END
  D;JEQ

  @SCREEN
  D=M
  @i 
  A=D+M
  M=0

  @i 
  M=M+1

  0;JMP

(END)
  @END
  0;JMP