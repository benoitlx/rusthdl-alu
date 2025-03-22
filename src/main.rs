use rust_hdl::prelude::*;

const SIZE: usize = 16;

#[derive(Copy, Clone, PartialEq, Debug, LogicState)]
enum Operation {
    Addition,
    Substraction,
    And,
    Or,
}

#[derive(LogicBlock, Default)]
struct ALU {
    pub a: Signal<In, Bits<SIZE>>,
    pub b: Signal<In, Bits<SIZE>>,
    
    // RustHDL will automatically adapt the number of wire needed to represents this signal
    pub op: Signal<In, Operation>,

    pub o: Signal<Out, Bits<SIZE>>,
}

impl Logic for ALU {
    #[hdl_gen]
    fn update(&mut self) {
        match self.op.val() {
            Operation::Addition => self.o.next = self.a.val() + self.b.val(),
            Operation::Substraction => self.o.next = self.a.val() - self.b.val(),
            Operation::And => self.o.next = self.a.val() & self.b.val(),
            Operation::Or => self.o.next = self.a.val() | self.b.val(),
        } 
    } 
}

fn main() {
    let mut uut = ALU::default();
    uut.connect_all();
    let verilog = generate_verilog(&uut);
    println!("{verilog}");
}
