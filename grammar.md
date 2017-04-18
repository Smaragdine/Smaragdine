# Smaragdine Grammar
This is a work in progress.   
The grammar is described using a modified Wirth syntax notation.

Table of symbols:

| Usage | Notation |
| ----- | -------- |
| definition | `=` |
| alternation | `|` |
| zero or one | `[ ... ]`|
| zero or more | `{ ... }` |
| grouping | `( ... )` |
| comment | `(* ... *)` |
| terminal string | `' ... '` |
| terminal string | `" ... "` |

```ebnf
(* basics *)

digit              = '0'...'9'
char               = (* Any UTF-8 code-point *)
char_ascii         = 'a'...'z'
                   | 'A'...'Z'

identifier         = char_ascii { char_ascii | digit | '_' | '!' | '?' }

(* literals *)
literal_integer    = [ '0' ( 'x' | 'b' ) ] { digit }
literal_float      = ( { digit } '.' digit { digit } )
                   | ( '.' digit { digit } )
literal_char       = "'" char "'"
literal_string     = '"' [ { char } ] '"'
literal_raw_string = 'r' literal_string
literal            = literal_char
                   | literal_string
                   | literal_raw_string
                   | literal_integer

(* operators *)

op_unary           = '+'
                   | '-'
                   | '~'
                   | '!'
op_assignment      = '='
                   | '&='
                   | '|='
                   | '^='
                   | '+='
                   | '-='
                   | '*='
                   | '/='
                   | '%='
                   | '<<='
                   | '>>='

(* expressions *)

ex_primary         = identifier | literal | expression
ex_postfix         = ex_primary
                   | ( ex_postfix '[' expression ']' )
                   | ( ex_postfix { ex_assignment } )
                   | ( ex_postfix '.' identifier )
ex_unary           = ex_postfix
                   | ( op_unary ex_additive )
ex_multiplicative  = ex_unary
                   | ( ex_multiplicative '*' ex_unary )
                   | ( ex_multiplicative '/' ex_unary )
                   | ( ex_multiplicative '%' ex_unary )
ex_additive        = ex_multiplicative
                   | ( ex_additive '+' ex_multiplicative )
                   | ( ex_additive '-' ex_multiplicative )
ex_shift           = ex_additive
                   | ( ex_shift '<<' ex_additive )
                   | ( ex_shift '>>' ex_additive )
ex_relational      = ex_shift
                   | ( ex_relational '<' ex_shift )
                   | ( ex_relational '>' ex_shift )
                   | ( ex_relational '<=' ex_shift )
                   | ( ex_relational '>=' ex_shift )
ex_equality        = ex_relational
                   | ( ex_equality '==' ex_relational )
                   | ( ex_equality '!=' ex_relational )
ex_bitwise_and     = ex_equality
                   | ( ex_bitwise_and '&'  ex_equality    )
ex_bitwise_xor     = ex_bitwise_and
                   | ( ex_bitwise_xor '^'  ex_bitwise_and )
ex_bitwise_or      = ex_bitwise_xor
                   | ( ex_bitwise_or  '|'  ex_bitwise_xor )
ex_logical_and     = ex_bitwise_or 
                   | ( ex_logical_and '&&' ex_bitwise_or  )
ex_logical_or      = ex_logical_and
                   | ( ex_logical_or  '||' ex_logical_and )
ex_conditional     = ex_logical_or
ex_constant        = ex_conditional
ex_assignment      = ex_conditional
                   | ex_unary op_assignment ex_assignment
expression         = ex_assignment

(* statements *)
(* TODO *)
(* NOTE: Statements are expressions too *)
```
