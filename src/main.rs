use rust_hdl::prelude::*;

const SIZE: usize = 16;

#[derive(LogicBlock, Default)]
struct ALU {
    pub a: Signal<In, Bits<SIZE>>,
    pub b: Signal<In, Bits<SIZE>>,
    pub o: Signal<Out, Bits<SIZE>>,
}

impl Logic for ALU {
    #[hdl_gen]
    fn update(&mut self) {
        self.o.next = self.a.val() + self.b.val();
    } 
}

fn main() {
    let mut uut = ALU::default();
    uut.connect_all();
    let verilog = generate_verilog(&uut);
    println!("{verilog}");
}
