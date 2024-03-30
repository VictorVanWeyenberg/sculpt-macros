use crate::{Cantrip, CantripDiscriminants, Class, ClassDiscriminants, DwarfSubrace, DwarfSubraceDiscriminants, ElfSubrace, ElfSubraceDiscriminants, Sheet, ToolProficiency, ToolProficiencyDiscriminants};
use crate::generated::generated_builders::SheetBuilder;
use crate::generated::generated_traits::SheetBuilderCallbacks;

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

impl Sheet {
    pub fn build<T: SheetBuilderCallbacks>(t: &mut T) -> Sheet {
        SheetBuilder::<T>::new(t).build()
    }
}