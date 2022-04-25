# PL-0_Lexer_Parser
A simple PL/0 language Lexer and Parser.

## EBNF

```
<程序>::=<分程序>
<分程序>::[<常量说明部分>][<变量说明部分>][<过程说明部分>]<语句>
<常量说明部分>::=const<常量定义>{,<常量定义>}; 
<常量定义>::=<标识符>=<无符号整数> 
<变量说明部分>::=var<标识符>{,<标识符>}; 
<过程说明部分>::=<过程首部><分程序>{;<过程说明部分>}; 
<过程首部>::=procedure<标识符>;
<语句>::=<赋值语句>|<条件语句>|<当型循环语句>|<过程调用语句>|<读语句>|<写语句>|<复合语句>|<空语句>
<赋值语句>::=<标识符>:=<表达式>
<复合语句>::=begin<语句>{;<语句>}end
<条件>::=<表达式><关系运算符><表达式>|odd<表达式> 
<表达式>::=[+|-]<项>{<加法运算符><项>} 
<项>::=<因子>{<乘法运算符><因子>}
<因子>::=<标识符>|<无符号整数>|'('<表达式>')'
<加法运算符>::=+|- 
<乘法运算符>::=*|/
<关系运算法>::==|#|<|<=|>|>=
<条件语句>::=if<条件>then<语句>
<过程调用语句>::=call<标识符>
<当型循环语句>::=while<条件>DO<语句>
<读语句>::=read'('<标识符>{,<标识符>}')' 
<写语句>::=write'('<表达式>{,<表达式>}')' 
<字母>::=a|b|...|X|Y|Z 
<数字>::=0|1|2|...|8|9 
<标识符>::=<字母>{<字母>|<数字>}
<无符号整数>::=<数字>{<数字>}
```

```
<programs>::=<>
<>::=[<const_declaration>][<var_declaration>][<procedure_declaration>]<>
<const_declaration>::=const<const_definition>{,<const_definition>};
<const_definition>::=<identifier>=<unsigned_integer>
<var_declaration>::=var<identifier>{,<identifier>}
<procedure_declaration>::=<procedure_header><>{;<procedure_declaration>};
<procedure_header>::=procedure<identifier>;
<>::=<


<identifier>::=
<unsigned_integer>::=
```



## 关键字

const/var/procedure/begin/end/odd/if/then/call/while/do/read/write

## 运算符

"+"  "-"  "*"  "/"  "<"  "<="  ">"  ">="  "#"  "="  ":="

## 界符

"("  ")"  ","  ";"  "."

