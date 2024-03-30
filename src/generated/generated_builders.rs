use crate::{CantripDiscriminants, ClassDiscriminants, DwarfSubraceDiscriminants, ElfSubrace, ElfSubraceDiscriminants, Race, RaceDiscriminants, Sheet, ToolProficiencyDiscriminants};
use crate::generated::generated_traits::{CantripPicker, ClassPicker, DwarfSubracePicker, ElfSubracePicker, RacePicker, SheetBuilderCallbacks, ToolProficiencyPicker};

pub struct SheetBuilder<'a, T: SheetBuilderCallbacks> {
    race: Option<RaceDiscriminants>,
    class: Option<ClassDiscriminants>,
    dwarf_builder: DwarfBuilder,
    elf_builder: ElfBuilder,

    callbacks: &'a T
}

impl<'a, T: SheetBuilderCallbacks> SheetBuilder<'a, T> {
    pub fn new(t: &'a mut T) -> SheetBuilder<T> {
        SheetBuilder {
            race: None,
            class: None,
            dwarf_builder: Default::default(),
            elf_builder: Default::default(),
            callbacks: t,
        }
    }

    pub fn build(mut self) -> Sheet {
        self.callbacks.pick_race(&mut self);
        let race = match self.race.unwrap() {
            RaceDiscriminants::Dwarf => self.dwarf_builder.build(),
            RaceDiscriminants::Elf => self.elf_builder.build()
        };
        let class = self.class.unwrap().into();
        Sheet { race, class }
    }
}

#[derive(Default)]
struct DwarfBuilder {
    subrace: Option<DwarfSubraceDiscriminants>,
    tool_proficiency: Option<ToolProficiencyDiscriminants>,
}

impl DwarfBuilder {
    pub fn build(self) -> Race {
        let subrace = self.subrace.unwrap().into();
        let tool_proficiency = self.tool_proficiency.unwrap().into();
        Race::Dwarf { subrace, tool_proficiency }
    }
}

#[derive(Default)]
struct ElfBuilder {
    subrace: Option<ElfSubraceDiscriminants>,
    wood_elf_builder: WoodElfBuilder
}

impl ElfBuilder {
    pub fn build(self) -> Race {
        let subrace = match self.subrace.unwrap() {
            ElfSubraceDiscriminants::WoodElf => self.wood_elf_builder.build(),
            v => v.into()
        };
        Race::Elf { subrace }
    }
}

#[derive(Default)]
struct WoodElfBuilder {
    cantrip: Option<CantripDiscriminants>
}

impl WoodElfBuilder {
    pub fn build(self) -> ElfSubrace {
        let cantrip = self.cantrip.unwrap().into();
        ElfSubrace::WoodElf(cantrip)
    }
}

// Picker Implementations

impl<'a, T: SheetBuilderCallbacks> RacePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &RaceDiscriminants) {
        self.race = Some(requirement.clone());
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
        self.dwarf_builder.subrace = Some(requirement.clone());
        self.callbacks.pick_tool_proficiency(self)
    }
}

impl<'a, T: SheetBuilderCallbacks> ElfSubracePicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ElfSubraceDiscriminants) {
        self.elf_builder.subrace = Some(requirement.clone());
        match requirement {
            ElfSubraceDiscriminants::DarkElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::HighElf => self.callbacks.pick_class(self),
            ElfSubraceDiscriminants::WoodElf => self.callbacks.pick_cantrip(self)
        }
    }
}

impl<'a, T: SheetBuilderCallbacks> ToolProficiencyPicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &ToolProficiencyDiscriminants) {
        self.dwarf_builder.tool_proficiency = Some(requirement.clone());
        self.callbacks.pick_class(self)
    }
}

impl<'a, T: SheetBuilderCallbacks> CantripPicker for SheetBuilder<'a, T> {
    fn fulfill(&mut self, requirement: &CantripDiscriminants) {
        self.elf_builder.wood_elf_builder.cantrip = Some(requirement.clone());
        self.callbacks.pick_class(self)
    }
}
