#[derive(Debug)]
enum Operation{Add, Sub, Mul, Div}

#[derive(Debug)]
enum Expression{Op{op: Operation, left: Box<Expression>, right: Box<Expression> }, Value(i64),}

fn eval(expr: &Expression) -> i64{
    match expr{
        Expression::Value(n) => *n,
        Expression::Op{op, left, right} => {
            let l = eval(left);
            let r = eval(right);
            match op{
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r,
            }
        }
    }
}

fn main(){
    use Operation::*;
    use Expression::*;

    let expr = Op{
        op: Add,
        left: Box::new(Value(1)),
        right: Box::new(Value(1)),
    };

    println!("1 + 1 = {}", eval(&expr));
}
