```
scope = 
  { stmts;* }
  { stmts;* return-stmt; }
```

```
function-def = 
  function-signature scope
```

```
stmts = 
  declaration ;
  assignment ;
  expr ; 
  return-stmts ;
```

```
return-stmts = 
  return expr
```

```
expr = 
  expr binary-op expr 
  unary-op expr
  function-call
  int-literal
  ident 
```

```
unary-op = 
  not 
```

```
binary-op = 
  +
  -
  \
  *
```

```
int-literal = 
  -[1-9] number*
  [1-9] number*
```

```
alpha = { 'a'..'z' | 'A'..'Z' }
digit = { '0'..'9' }
positive-int-literal = { '1'..'9' digit* }
negative-int-literal = { -'1'..'9' digit* }
int-literal = { positive-int-literal | negative-int-literal }
ident = (alpha)('_' | alpha | digit)+
  
```
