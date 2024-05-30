use sculpt_macros::{Picker, Sculptor};

include!(concat!(env!("OUT_DIR"), "/tests/test.rs"));

#[test]
fn it_works() {
    let mut callbacks = SheetBuilderCallbacksImpl();
    let sheet = Sheet::build(&mut callbacks);
    println!("{:?}", sheet);
    assert_eq!(sheet.race, Race::Dwarf { subrace: DwarfSubrace::HillDwarf, tool_proficiency: ToolProficiency::Hammer });
    assert_eq!(sheet.class, Class::Bard);
}

#[derive(Debug, Sculptor)]
struct Sheet {
    #[sculptable]
    race: Race,
    class: Class,
}

#[derive(Debug, Picker, PartialEq)]
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

#[derive(Debug, Picker, PartialEq)]
pub enum Class {
    Bard,
    Paladin,
}

#[derive(Debug, Picker, PartialEq)]
pub enum DwarfSubrace {
    HillDwarf,
    MountainDwarf,
}

#[derive(Debug, Picker, PartialEq)]
pub enum ToolProficiency {
    Hammer,
    Saw,
}

#[derive(Debug, Picker, PartialEq)]
pub enum ElfSubrace {
    DarkElf,
    HighElf,
    WoodElf(Cantrip),
}

#[derive(Debug, Picker, PartialEq)]
pub enum Cantrip {
    Prestidigitation,
    Guidance,
}

struct SheetBuilderCallbacksImpl();

impl SheetBuilderCallbacks for SheetBuilderCallbacksImpl {}
