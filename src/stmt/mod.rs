pub(crate) mod block;
pub(crate) mod expression;
pub mod function;
pub mod r#if;
pub(crate) mod print;
pub(crate) mod var;
pub(crate) mod r#while;

use crate::error::ParseError;
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Expression(expression::Expression),
    Print(print::Print),
    Var(var::Var),
    Block(block::Block),
    If(Box<r#if::If>),
    While(Box<r#while::While>),
    Function(Box<function::Function>),
}

impl Stmt {
    pub fn accept(&self, visitor: &mut Interpreter) -> Result<Option<Object>, ParseError> {
        match self {
            Stmt::Expression(v) => visitor
                .visit_expression_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Print(v) => visitor
                .visit_print_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Var(v) => visitor
                .visit_var_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Block(v) => visitor
                .visit_block_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::If(v) => visitor
                .visit_if_stmt(*v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::While(v) => visitor
                .visit_while_stmt(*v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Function(f) => visitor.visit_function_stmt(*f.clone()).map(|_| Some(Object::Void))
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print(print::Print { expression })
    }
    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression(expression::Expression { expression })
    }

    pub fn var(token: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(var::Var {
            name: token,
            initializer,
        })
    }
    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block(block::Block { statements })
    }
    pub fn r#if(condition: Expr, thenBranch: Stmt, elseBranch: Option<Stmt>) -> Self {
        Stmt::If(Box::new(r#if::If {
            condition,
            thenBranch,
            elseBranch,
        }))
    }
    pub fn r#while(condition: Expr, body: Stmt) -> Self {
        Stmt::While(Box::new(r#while::While { condition, body }))
    }

    pub fn function(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Stmt::Function(Box::new(function::Function { name, params, body }))
    }
}

pub(crate) trait Visitor {
    /// execute expression, ignore result
    fn visit_expression_stmt(&mut self, stmt: expression::Expression) -> Result<(), ParseError>;

    /// print statement
    fn visit_print_stmt(&mut self, stmt: print::Print) -> Result<(), ParseError>;

    /// define var
    fn visit_var_stmt(&mut self, stmt: var::Var) -> Result<(), ParseError>;

    /// execute block
    fn visit_block_stmt(&mut self, stmt: block::Block) -> Result<(), ParseError>;

    /// execute if statement
    fn visit_if_stmt(&mut self, stmt: r#if::If) -> Result<(), ParseError>;

    /// execute while statement
    fn visit_while_stmt(&mut self, stmt: r#while::While) -> Result<(), ParseError>;

    /// define function
    fn visit_function_stmt(&mut self, stmt: function::Function)  -> Result<(), ParseError>;
}
