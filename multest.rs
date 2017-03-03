[#test]

fn is_this_correct(){
assert_eq!(Some(6), eval(&[Token::Operand(2),
                           Token::Operand(3),
                           Token::Operator(Operator::Mul)]));
}