#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    match e {
        Expression::Op { op, left, right, } => {
            let left_val = eval(*left);
            let right_val = eval(*right);
            return match op {
                Operation::Add => left_val + right_val,
                Operation::Sub => left_val - right_val,
                Operation::Mul => left_val * right_val,
                Operation::Div => left_val / right_val,
            };
        }
        Expression::Value(val) => return val
    };
}

#[test]
fn test_value() {
    assert_eq!(
        eval(Expression::Op { 
            op: Operation::Add, 
            left: Box::new(Expression::Value(10)), 
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op { 
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)), 
        right: Box::new(Expression::Value(9)),
    };

    let term2 = Expression::Op { 
        op: Operation::Mul, 
        left: Box::new(Expression::Op { 
            op: Operation::Sub, 
            left: Box::new(Expression::Value(3)), 
            right:  Box::new(Expression::Value(4)),
        }), 
        right:  Box::new(Expression::Value(5)),
    };

    assert_eq!(
        eval(Expression::Op { 
            op: Operation::Add, 
            left: Box::new(term1), 
            right: Box::new(term2), 
        }),
        85
    )
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0)),
        }),
        0
    );

    assert_eq!(
        eval(Expression::Op { 
            op: Operation::Mul, 
            left: Box::new(Expression::Value(0)), 
            right: Box::new(Expression::Value(0)),
        }),
        0
    );

    assert_eq!(
        eval(Expression::Op { 
            op: Operation::Sub, 
            left: Box::new(Expression::Value(0)), 
            right:  Box::new(Expression::Value(0)),
        }),
        0
    );
}

#[test]
fn test_div() {
    assert_eq!(
        eval(Expression::Op { 
            op: Operation::Div, 
            left: Box::new(Expression::Value(10)), 
            right: Box::new(Expression::Value(2)),
        }),
        5
    );
}

fn main() {

}
