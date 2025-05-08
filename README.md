# SimpleTokenizer1

简单四则运算的词法分析， 支持加、减、乘、除、幂、小括号、整数

例如:

```shell
1 + 2 ^ 3 - (5 + 1) * 2
```

经过词法分析后应该是这样的，

```rust
[Token::Number(1), Token::Plus, Token::Number(2), Token::Pow, Token::Number(3), Token::Minus, Token::LParen, 
Token::Number(5), Token::Plus, Token::Number(1), Token::RParen, Token::Multiply, Token::Number(2)]
```
