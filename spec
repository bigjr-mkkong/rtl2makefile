(let ((reg d1) (wire w1 w2)) (circ (conn w1 (+ d1 1)) (conn d1 (w1))))

circuit always start with "let"
then goes into variable binding list

variable binding list consist of name with types
only reg and wire are support rn

after variable binding finished, we gose into circuit definition expression

circuit expression start with circ, then followed by a lot of "connections"

each connection(conn [var1] ([con-expr])) means take var1 as output, con-expr as
the result of evaluation

variables in con-expr are read by circuit, variables in var1 are written

wire type stays same when it's read from or written to
reg type would becomes [name]_q and [name]_d. Only [name]_d are allow to written to,
also, only [name]_q are allow to read from
