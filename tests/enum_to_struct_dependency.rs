use sculpt_macros::sculpt;

include!(concat!(env!("OUT_DIR"), "/tests/enum_to_struct_dependency.rs"));

#[test]
fn to_struct_dependency() {
    let mut callbacks = SheetBuilderCallbacksImpl();
    let sheet = Sheet::build(&mut callbacks);
    println!("{:?}", sheet);
    assert_eq!(sheet.race, Race { race: BaseRace::Dwarf, subrace: ElfSubrace::WoodElf(Cantrip { cantrip: BaseCantrip::Prestidigitation }) });
    assert_eq!(sheet.class, Class::Bard);
}

#[sculpt]
#[derive(Debug)]
pub struct Sheet {
    race: Race,
    class: Class,
}

#[derive(Debug, PartialEq)]
pub struct Race {
    race: BaseRace,
    subrace: ElfSubrace
}

#[derive(Debug, PartialEq)]
pub enum BaseRace {
    Dwarf,
    Elf
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
pub struct ToolProficiency {
    tool_proficiency: BaseToolProficiency
}

#[derive(Debug, PartialEq)]
pub enum BaseToolProficiency {
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
pub struct Cantrip {
    cantrip: BaseCantrip
}

#[derive(Debug, PartialEq)]
pub enum BaseCantrip {
    Prestidigitation,
    Guidance,
}

struct SheetBuilderCallbacksImpl();

impl SheetBuilderCallbacks for SheetBuilderCallbacksImpl {
    fn pick_elfsubrace(&self, picker: &mut impl ElfSubracePicker) {
        picker.fulfill(&ElfSubraceOptions::WoodElf);
    }
}