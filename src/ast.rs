pub trait AST: std::fmt::Debug {
    fn new(&self);
    fn to_string(&self);
    fn evaluate(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct PrintStatement;

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

/* struct Cat;

impl Animal for Cat {
    fn tell(&self) { println!("Meow"); }
    fn pet(&mut self) { println!("purr");
    fn feed(&mut self, food: Food) { println!("lick"); }
} */
