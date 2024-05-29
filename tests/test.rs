use std::fmt::Display;

use sculpt::{Picker, Sculptor};

include!(concat!(env!("OUT_DIR"), "/tests/test.rs"));

#[test]
fn it_works() {
    let mut callbacks = SheetBuilderCallbacksImpl();
    let sheet = Sheet::build(&mut callbacks);
    println!("{:?}", sheet);
}

#[derive(Debug, Sculptor)]
struct Sheet {
    #[sculptable]
    race: Race,
    class: Class,
}

#[derive(Debug, Picker)]
pub enum Race {
    Dwarf {
        subrace: DwarfSubrace,
        tool_proficiency: ToolProficiency,
    },
    Elf {
        #[sculptable]
        subrace: ElfSubrace,
    },
}

#[derive(Debug, Picker)]
pub enum Class {
    Bard,
    Paladin,
}

#[derive(Debug, Picker)]
pub enum DwarfSubrace {
    HillDwarf,
    MountainDwarf,
}

#[derive(Debug, Picker)]
pub enum ToolProficiency {
    Hammer,
    Saw,
}

#[derive(Debug, Picker)]
pub enum ElfSubrace {
    DarkElf,
    HighElf,
    WoodElf(Cantrip),
}

#[derive(Debug, Picker)]
pub enum Cantrip {
    Prestidigitation,
    Guidance,
}

struct SheetBuilderCallbacksImpl();

impl SheetBuilderCallbacks for SheetBuilderCallbacksImpl {}
