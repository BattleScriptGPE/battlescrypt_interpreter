pub trait AST: std::fmt::Debug {
    fn new(&self);
    fn to_string(&self);
    fn evaluate(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct PrintStatement;

#[derive(Debug, Clone, Copy)]
pub struct EOFStatement;

#[derive(Debug, Clone, Copy)]
pub struct VarStatement;

impl AST for PrintStatement {
    fn new(&self) {
        println!("new Print");
    }

    fn to_string(&self) {
        println!("to_string Print");
    }

    fn evaluate(&self) {
        println!("evaluate Print");
    }
}

impl AST for EOFStatement {
    fn new(&self) {
        println!("new EOF");
    }

    fn to_string(&self) {
        println!("to_string EOF");
    }

    fn evaluate(&self) {
        println!("evaluate EOF");
    }
}

impl AST for VarStatement {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }
}

/* struct Cat;

impl Animal for Cat {
    fn tell(&self) { println!("Meow"); }
    fn pet(&mut self) { println!("purr");
    fn feed(&mut self, food: Food) { println!("lick"); }
} */
