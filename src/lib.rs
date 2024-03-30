use std::fmt::Display;
use std::io::Write;
use strum_macros::{EnumDiscriminants, VariantArray, Display};

use generated::{CantripPicker, ClassPicker, DwarfSubracePicker, ElfSubracePicker, RacePicker, SheetBuilderCallbacks, ToolProficiencyPicker};

mod generated;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut callbacks = SheetBuilderCallbacksImpl();
        let sheet = Sheet::build(&mut callbacks);
        println!("{:?}", sheet);
    }
}

#[derive(Debug)]
struct Sheet {
    race: Race,
    class: Class
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
pub enum Race {
    Dwarf {
        subrace: DwarfSubrace,
        tool_proficiency: ToolProficiency
    },
    Elf {
        subrace: ElfSubrace,
    }
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
pub enum Class {
    Bard, Paladin
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
pub enum DwarfSubrace {
    HillDwarf, MountainDwarf
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
pub enum ToolProficiency {
    Hammer, Saw
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
pub enum ElfSubrace {
    DarkElf, HighElf, WoodElf(Cantrip)
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Display, VariantArray))]
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
                Ok(n) => match choice.trim().parse::<usize>() {
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