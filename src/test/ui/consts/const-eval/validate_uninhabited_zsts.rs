// stderr-per-bitwidth

const fn foo() -> ! {
    unsafe { std::mem::transmute(()) }
    //~^ ERROR evaluation of constant value failed
    //~| WARN the type `!` does not permit zero-initialization [invalid_value]
}

#[derive(Clone, Copy)]
enum Empty { }

#[warn(const_err)]
const FOO: [Empty; 3] = [foo(); 3];

#[warn(const_err)]
const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
//~^ ERROR evaluation of constant value failed
//~| WARN the type `Empty` does not permit zero-initialization

fn main() {
    FOO;
    BAR;
}
