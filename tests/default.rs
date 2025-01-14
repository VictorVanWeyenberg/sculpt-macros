use sculpt_macros::sculpt;

include!(concat!(env!("OUT_DIR"), "/tests/default.rs"));

#[test]
fn default() {
    let mut callbacks = SheetBuilderCallbacksImpl();
    let sheet = Sheet::build(&mut callbacks);
    println!("{:?}", sheet);
    assert_eq!(sheet.race, Race::Dwarf { subrace: DwarfSubrace::HillDwarf, tool_proficiency: ToolProficiency::Hammer });
    assert_eq!(sheet.class, Class::Bard);
}

#[sculpt]
#[derive(Debug)]
pub struct Sheet {
    race: Race,
    class: Class,
}

#[derive(Debug, PartialEq)]
pub enum Race {
    Dwarf {
        subrace: DwarfSubrace,
        tool_proficiency: ToolProficiency,
    },
    Elf {
        subrace: ElfSubrace,
    },
}

#[derive(Debug, PartialEq)]
pub enum Class {
    Bard,
    Paladin,
}

#[derive(Debug, PartialEq)]
pub enum DwarfSubrace {
    HillDwarf,
    MountainDwarf,
}

#[derive(Debug, PartialEq)]
pub enum ToolProficiency {
    Hammer,
    Saw,
}

#[derive(Debug, PartialEq)]
pub enum ElfSubrace {
    DarkElf,
    HighElf,
    WoodElf(Cantrip),
}

#[derive(Debug, PartialEq)]
pub enum Cantrip {
    Prestidigitation,
    Guidance,
}

struct SheetBuilderCallbacksImpl();

impl SheetBuilderCallbacks for SheetBuilderCallbacksImpl {}
