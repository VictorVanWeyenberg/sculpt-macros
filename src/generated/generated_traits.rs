use strum::VariantArray;
use crate::{CantripDiscriminants, ClassDiscriminants, DwarfSubraceDiscriminants, ElfSubraceDiscriminants, RaceDiscriminants, ToolProficiencyDiscriminants};

pub trait RacePicker {
    fn options(&self) -> Vec<RaceDiscriminants> {
        RaceDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &RaceDiscriminants);
}

pub trait ClassPicker {
    fn options(&self) -> Vec<ClassDiscriminants> {
        ClassDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &ClassDiscriminants);
}

pub trait DwarfSubracePicker {
    fn options(&self) -> Vec<DwarfSubraceDiscriminants> {
        DwarfSubraceDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &DwarfSubraceDiscriminants);
}

pub trait ElfSubracePicker {
    fn options(&self) -> Vec<ElfSubraceDiscriminants> {
        ElfSubraceDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &ElfSubraceDiscriminants);
}

pub trait ToolProficiencyPicker {
    fn options(&self) -> Vec<ToolProficiencyDiscriminants> {
        ToolProficiencyDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &ToolProficiencyDiscriminants);
}

pub trait CantripPicker {
    fn options(&self) -> Vec<CantripDiscriminants> {
        CantripDiscriminants::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &CantripDiscriminants);
}

pub trait SheetBuilderCallbacks {
    fn pick_race(&self, picker: &mut impl RacePicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_class(&self, picker: &mut impl ClassPicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_dwarf_subrace(&self, picker: &mut impl DwarfSubracePicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_elf_subrace(&self, picker: &mut impl ElfSubracePicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_tool_proficiency(&self, picker: &mut impl ToolProficiencyPicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }

    fn pick_cantrip(&self, picker: &mut impl CantripPicker) where Self: Sized {
        let choice = picker.options()[0];
        picker.fulfill(&choice);
    }
}

