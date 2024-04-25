use std::fmt::Display;

use sculpt::{Picker, Sculptor};

include!(concat!(env!("OUT_DIR"), "/tests/sculpt_generated.rs"));

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

// ||||||||||||||||||||||||||||
// || Picker Implementations ||
// ||||||||||||||||||||||||||||

impl<'a, T: SheetBuilderCallbacks> RacePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &RaceDiscriminants) {
        self.race_builder.race = Some(requirement.clone());
        match requirement {
            RaceDiscriminants::Dwarf => self.callbacks.pick_dwarfsubrace(self),
            RaceDiscriminants::Elf => self.callbacks.pick_elfsubrace(self),
        }
    }
}

impl<'a, T: SheetBuilderCallbacks> ClassPicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ClassDiscriminants) {
        self.class = Some(requirement.clone());
    }
}

impl<'a, T: SheetBuilderCallbacks> DwarfSubracePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &DwarfSubraceDiscriminants) {
        self.race_builder.dwarf_builder.subrace = Some(requirement.clone());
        self.callbacks.pick_toolproficiency(self)
    }
}

impl<'a, T: SheetBuilderCallbacks> ElfSubracePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ElfSubraceDiscriminants) {
        self.race_builder.elf_builder.elfsubrace_builder.elfsubrace = Some(requirement.clone());
        match requirement {
            ElfSubraceDiscriminants::DarkElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::HighElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::WoodElf => self.callbacks.pick_cantrip(self),
        }
    }
}

impl<'a, T: SheetBuilderCallbacks> ToolProficiencyPicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ToolProficiencyDiscriminants) {
        self.race_builder.dwarf_builder.tool_proficiency = Some(requirement.clone());
        self.callbacks.pick_class(self)
    }
}

impl<'a, T: SheetBuilderCallbacks> CantripPicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &CantripDiscriminants) {
        self.race_builder
            .elf_builder
            .elfsubrace_builder
            .woodelf_builder
            .cantrip = Some(requirement.clone());
        self.callbacks.pick_class(self)
    }
}
