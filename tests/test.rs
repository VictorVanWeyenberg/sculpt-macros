use std::fmt::Display;
use std::io::Write;
use sculpt::{Picker, Sculptor};

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
    }
}

#[derive(Debug, Picker)]
pub enum Class {
    Bard, Paladin,
}

#[derive(Debug, Picker)]
pub enum DwarfSubrace {
    HillDwarf, MountainDwarf,
}

#[derive(Debug, Picker)]
pub enum ToolProficiency {
    Hammer, Saw,
}

#[derive(Debug, Picker)]
pub enum ElfSubrace {
    DarkElf, HighElf, WoodElf(Cantrip),
}

#[derive(Debug, Picker)]
pub enum Cantrip {
    Prestidigitation, Guidance
}

struct SheetBuilderCallbacksImpl();

impl SheetBuilderCallbacksImpl {
    fn pick<'a, T: Display>(&'a self, prompt: &str, options: &'a Vec<T>) -> &T {
        options.iter().enumerate()
            .for_each(|(i, x)| println!("{}. {}", i + 1, x));
        loop {
            let mut choice = String::new();
            print!("{} [1-{}] > ", prompt, options.len());
            std::io::stdout().flush().expect("Unable to flush stdout.");
            match std::io::stdin().read_line(&mut choice) {
                Ok(_) => match choice.trim().parse::<usize>() {
                    Ok(n) => match options.get(n - 1) {
                        None => println!("Enter a valid number."),
                        Some(v) => {
                            println!();
                            return v
                        }
                    },
                    Err(_) => println!("Enter a number.")
                },
                Err(_) => println!("Could not read input.")
            }
        }
    }
}

impl SheetBuilderCallbacks for SheetBuilderCallbacksImpl {
    // If you uncomment the code below, you can make your own choices.

    /* fn pick_race(&self, picker: &mut impl RacePicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a race", &picker.options()));
    }

    fn pick_class(&self, picker: &mut impl ClassPicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a class", &picker.options()));
    }

    fn pick_dwarf_subrace(&self, picker: &mut impl DwarfSubracePicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a dwarf subrace", &picker.options()));
    }

    fn pick_elf_subrace(&self, picker: &mut impl ElfSubracePicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a elf subrace", &picker.options()));
    }

    fn pick_tool_proficiency(&self, picker: &mut impl ToolProficiencyPicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a tool proficiency", &picker.options()));
    }

    fn pick_cantrip(&self, picker: &mut impl CantripPicker) where Self: Sized {
        picker.fulfill(self.pick("Choose a cantrip", &picker.options()));
    } */
}

// ||||||||||||||||||||||||||||
// || Picker Implementations ||
// ||||||||||||||||||||||||||||

impl<'a, T: SheetBuilderCallbacks> RacePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &RaceDiscriminants) {
        self.race_builder.race = Some(requirement.clone());
        match requirement {
            RaceDiscriminants::Dwarf => self.callbacks.pick_dwarf_subrace(self),
            RaceDiscriminants::Elf => self.callbacks.pick_elf_subrace(self)
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
        self.callbacks.pick_tool_proficiency(self)
    }
}

impl<'a, T: SheetBuilderCallbacks> ElfSubracePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ElfSubraceDiscriminants) {
        self.race_builder.elf_builder.elfsubrace_builder.elfsubrace = Some(requirement.clone());
        match requirement {
            ElfSubraceDiscriminants::DarkElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::HighElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::WoodElf => self.callbacks.pick_cantrip(self)
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
        self.race_builder.elf_builder.elfsubrace_builder.woodelf_builder.cantrip = Some(requirement.clone());
        self.callbacks.pick_class(self)
    }
}

// |||||||||||
// || GENERATED ENUM IMPLS ||
// |||||||||||

impl Into<Class> for ClassDiscriminants {
    fn into(self) -> Class {
        match self {
            ClassDiscriminants::Bard => Class::Bard,
            ClassDiscriminants::Paladin => Class::Paladin,
        }
    }
}

impl Into<DwarfSubrace> for DwarfSubraceDiscriminants {
    fn into(self) -> DwarfSubrace {
        match self {
            DwarfSubraceDiscriminants::HillDwarf => DwarfSubrace::HillDwarf,
            DwarfSubraceDiscriminants::MountainDwarf => DwarfSubrace::MountainDwarf,
        }
    }
}

impl Into<ToolProficiency> for ToolProficiencyDiscriminants {
    fn into(self) -> ToolProficiency {
        match self {
            ToolProficiencyDiscriminants::Hammer => ToolProficiency::Hammer,
            ToolProficiencyDiscriminants::Saw => ToolProficiency::Saw,
        }
    }
}

impl Into<ElfSubrace> for ElfSubraceDiscriminants {
    fn into(self) -> ElfSubrace {
        match self {
            ElfSubraceDiscriminants::DarkElf => ElfSubrace::DarkElf,
            ElfSubraceDiscriminants::HighElf => ElfSubrace::HighElf,
            ElfSubraceDiscriminants::WoodElf => panic!("Cannot turn WoodElf into ElfSubrace without dependencies."),
        }
    }
}

impl Into<Cantrip> for CantripDiscriminants {
    fn into(self) -> Cantrip {
        match self {
            CantripDiscriminants::Prestidigitation => Cantrip::Prestidigitation,
            CantripDiscriminants::Guidance => Cantrip::Guidance,
        }
    }
}

// ||||||||||
// || GENERATED TRAITS ||
// ||||||||||

pub trait SheetBuilderCallbacks {
    fn pick_race(&self, picker: &mut impl RacePicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_class(&self, picker: &mut impl ClassPicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_dwarf_subrace(&self, picker: &mut impl DwarfSubracePicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_elf_subrace(&self, picker: &mut impl ElfSubracePicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_tool_proficiency(&self, picker: &mut impl ToolProficiencyPicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_cantrip(&self, picker: &mut impl CantripPicker) {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }
}


