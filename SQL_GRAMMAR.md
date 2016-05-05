expr → expr '+' term |
       expr '-' term |
       term

term → term '*' factor |
       term '/' factor |
       factor

factor → '-' factor |
         prim

prim → NUMBER |
       NUMBER '^' prim
