use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::rc::Rc;

#[derive(Debug)]
enum Value {
    X,
    Y,
}

impl Distribution<Value> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        match rng.gen_range(0..=1) {
            0 => Value::X,
            _ => Value::Y,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Substitute,
    Mult,
    Average,
    Sin,
    Cos,
    ScaledSigmoid,
    Sqrt,
}

impl Distribution<Opcode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Opcode {
        match rng.gen_range(0..=6) {
            0 => Opcode::Substitute,
            1 => Opcode::Mult,
            2 => Opcode::Average,
            3 => Opcode::Sin,
            4 => Opcode::Cos,
            5 => Opcode::ScaledSigmoid,
            _ => Opcode::Sqrt
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    operation: Opcode,
    left: Option<Rc<Expression>>,
    right: Option<Rc<Expression>>,
    value: Option<Value>,
}

impl Expression {
    pub fn new(depth: usize) -> Self {
        if depth == 0 {
            // Take any terminal
            Expression {
                operation: Opcode::Substitute,
                left: None,
                right: None,
                value: Some(rand::random()),
            }
        } else {
            // Take any operation
            let op: Opcode = rand::random();
            match op {
                Opcode::Substitute => Expression {
                    operation: op,
                    left: None,
                    right: None,
                    value: Some(rand::random()),
                },
                Opcode::Sin | Opcode::Cos | Opcode::ScaledSigmoid | Opcode::Sqrt => Expression {
                    operation: op,
                    left: Some(Rc::new(Expression::new(depth - 1))),
                    right: None,
                    value: None,
                },
                _ => Expression {
                    operation: op,
                    left: Some(Rc::new(Expression::new(depth - 1))),
                    right: Some(Rc::new(Expression::new(depth - 1))),
                    value: None,
                },
            }
        }
    }

    pub fn eval(&self, x: f32, y: f32) -> f32 {
        match self.operation {
            Opcode::Substitute => match self.value.as_ref().unwrap() {
                Value::X => x,
                Value::Y => y,
            },
            Opcode::Mult => {
                self.left.as_ref().unwrap().as_ref().eval(x, y)
                    * self.right.as_ref().unwrap().as_ref().eval(x, y)
            },
            Opcode::Sin => {
                (self.left.as_ref().unwrap().as_ref().eval(x, y) * std::f32::consts::PI).sin()
            },
            Opcode::Cos => {
                (self.left.as_ref().unwrap().as_ref().eval(x, y) * std::f32::consts::PI).cos()
            },
            Opcode::ScaledSigmoid => {
                2. / (1. + (-self.left.as_ref().unwrap().as_ref().eval(x, y)).exp()) - 1.0
            },
            Opcode::Sqrt => {
                // Note: here we take an absolute value of the nested expression as square root
                // isn't defined for negative arguments
                self.left.as_ref().unwrap().as_ref().eval(x, y).abs().sqrt()
            }
            Opcode::Average => {
                (self.left.as_ref().unwrap().as_ref().eval(x, y)
                    + self.right.as_ref().unwrap().as_ref().eval(x, y))
                    / 2.0
            },
        }
    }
}
