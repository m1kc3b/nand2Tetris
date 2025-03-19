pub const ADD: &str = r#"// add
@SP
A=M-1
D=M
@SP
M=M-1
@SP
A=M-1
D=D+M
M=D

"#;

pub const SUB: &str = r#"// sub
@SP
A=M
A=A-1
A=A-1
D=M
A=A+1
D=D-M
@SP
M=M-1
M=M-1
A=M
M=D
@SP
M=M+1

"#;

pub const POP_Y: &str = r#"
@SP
M=M-1
A=M
D=M"#;

pub const GET_X: &str = r#"
@SP
A=M-1"#;

pub const PUSH_X: &str = r#"
@SP
A=M
M=D
@SP
M=M+1"#;

pub const NEG: &str = r#"// neg
@SP
A=M-1
M=-M

"#;

pub const AND: &str = r#"// and
@SP
A=M-1
D=M
@SP
M=M-1
A=M-1
D=D&M
M=D

"#;

pub const OR: &str = r#"// or
@SP
A=M-1
D=M
M=0
@SP
M=M-1
A=M-1
M=D|M

"#;

pub const NOT: &str = r#"// not
@SP
A=M-1
M=!M

"#;