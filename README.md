# calc_rs
A simple calculator written in rust

## Syntax
```
<statement> ::= <substitute> | <eval1> .
<substitute> ::= "let" <identifier> "=" <eval> .
<identifier> ::= ( 'a' .. 'z' | 'A' .. 'Z' ) { 'a' .. 'z' | 'A' .. 'Z' | '0' .. '9' | '_' } .
<eval1>  ::= <eval2> { ('+' | '-') <eval2> } .
<eval2>  ::= <eval3> { ('*' | '/') <eval3> } .
<eval3>  ::= ( '(' <eval1> ')' ) | <digits> | <identifier> .
<digits> ::= ( '0' .. '9' ) { '0' .. '9' } .
```

## Example
```
> let a = 10
> result -> 10
> let b = 20
> result -> 20
> (a + b) * 10
> result -> 300
```
