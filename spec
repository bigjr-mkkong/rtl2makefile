(let ((reg d1) (wire w1 w2)) (circ (conn w1 (+ d1 1)) (conn d1 (w1))))

circuit always start with "let"

then goes into variable binding list

variable binding list consist of name with types
only reg and wire are support rn

after variable binding finished, we gose into circuit definition expressions

circuit expression start with circ, then followed by a lot of "connections"

each connection(conn [var1] ([con-expr])) means take var1 as output,
con-expr as the result of evaluation
It also represent this circuit:

con-expr ==> var1

con-expr and var1 can be either wire or reg(reg to reg connection is not support rn)

run_list.py is the script we used to boost simulation for one "cycle". This toy example for now can only simulate some simple combination logics and counter.

Compiler will generate a fsig/ directory, copy run_list.py into this directory, write some initial value to circuit input, then run the script. It will print all the circuit value in each "cycle"
