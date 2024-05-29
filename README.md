# sculpt

Work in progress type safe builder derive.

So far this is only a proof of concept. The macros have yet to be written.

# How it's going to work

## Sheet

```rust
#[derive(Sculptor)]
struct Sheet {
    race: Race,
    #[sculptable]
    class: Class
}
```

Sheet is the struct we want to build. Sheet has two fields: race and class. Sheet will derive Sculptor. Sculptor will 
generate the following code:

- The SheetBuilder struct that accepts an instance of SheetBuilderCallbacks as generic parameter.
- The implementation of SheetBuilder with a constructor method and a build method.

```rust
pub struct SheetBuilder<'a, T: SheetBuilderCallbacks> {
    race_builder: RaceBuilder,
    class: Option<ClassOptions>,

    callbacks: &'a T
}

impl<'a, T: SheetBuilderCallbacks> SheetBuilder<'a, T> {
    pub fn new(t: &'a mut T) -> SheetBuilder<T> {
        SheetBuilder {
            race_builder: RaceBuilder::default(),
            class: None,
            callbacks: t,
        }
    }

    pub fn build(mut self) -> Sheet {
        self.callbacks.pick_race(&mut self);
        let race = self.race_builder.build();
        let class = self.class.unwrap().into();
        Sheet { race, class }
    }
}

impl Sheet {
    pub fn build<T: SheetBuilderCallbacks>(t: &mut T) -> Sheet {
        SheetBuilder::<T>::new(t).build()
    }
}
```

Because `Race` and `Class` are both enum (with dependencies), and the user (or program) has to pick one of the options, 
they have to be annotated as such. `Class` is a leaf dependency which means the user (or program) can just pick one of 
the options. `Class` is annotated with `pick`. `pick` is the default creation method for a struct field, so it may also 
be omitted. `Race` has dependencies of its own so when the user picks one of the `Race` options, it has to traverse 
down the `Race` dependency tree in order to build the `Race`. `Race` is annotated with `sculpt`.

Because of these annotations, we can generate the following: 

- `SheetBuilder` needs a `RaceBuilder` and an `Option<ClassOptions>`.
- The `SheetBuilder` constructor method can be filled in.
- The `SheetBuilder` build method can be filled in.

### Class

```rust
#[derive(Picker)]
pub enum Class {
    Bard, Paladin
}
```

`Class` is a leaf dependency and an enum. The user (or program) simply has to pick on of the options, so we only have 
to derive the Picker for it. We'll cover how to use the picker traits below.

Deriving the Picker trait will generate the following code:

- ClassOptions so a Class can be chosen without first havinf to provide the dependencies.
- An Into<Class> implementation for ClassOptions.
- The ClassPicker trait which provides all choices to the builder and allows it to fulfill the dependency. We'll talk 
  about how to use these trait later on.

```rust
enum ClassOptions {
    Bard, Paladin    
}

impl Into<Class> for ClassOptions {
    fn into(self) -> Class {
        match self {
            ClassOptions::Bard => Class::Bard,
            ClassOptions::Paladin => Class::Paladin,
        }
    }
}

pub trait ClassPicker {
    fn options(&self) -> Vec<ClassOptions> {
        ClassOptions::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &ClassOptions);
}
```

### Race

```rust
#[derive(Picker)]
pub enum Race {
    #[sculptable]
    Dwarf {
        subrace: DwarfSubrace,
        tool_proficiency: ToolProficiency
    },
    #[sculptable]
    Elf {
        subrace: ElfSubrace,
    }
}
```

Each of the `Race` variants have their own dependencies. The user (or program) should pick one of the variants but once 
this choice is locked in, sculpt should continue to traverse the dependency tree of the chosen variant. For example:
when we choose a Dwarf, sculpt should offer to pick a dwarf subrace and then a tool proficiency. When we pick an Elf,
sculpt should offer to pick an elf subrace. After we've picked the tool proficiency or the elf subrace, sculpt should
offer to pick a `Class` (in the Sheet struct), but we'll look at how to do this later.

Because the user (or program) needs to pick a choice, the `Race` enum should derive the Picker trait.
Because each of the variants need to be built, they are each annotated as `sculpt`.

The will generate the following code:

- The `Race` picker.
- The `Race` builder with the implementation of its build method.
- The `DwarfBuilder` with the implementation of its build method. This is alright because the `Dwarf`'s dependencies 
  don't have any dependencies themselves.
- The `ElfBuilder` works a bit different. Because its dependency has dependencies of its own, the dependency also needs 
  to be annotated with `sculpt`. That way its builder can also be generated correctly.

```rust
pub trait RacePicker {
    fn options(&self) -> Vec<RaceOptions> {
        RaceOptions::VARIANTS.to_vec()
    }
    fn fulfill(&mut self, requirement: &RaceOptions);
}

#[derive(Default)]
struct RaceBuilder {
    race: Option<RaceOptions>,
    dwarf_builder: DwarfBuilder,
    elf_builder: ElfBuilder,
}

impl RaceBuilder {
    fn build(self) -> Race {
        match self.race.unwrap() {
            RaceOptions::Dwarf => self.dwarf_builder.build(),
            RaceOptions::Elf => self.elf_builder.build()
        }
    }
}

#[derive(Default)]
struct DwarfBuilder {
    subrace: Option<DwarfSubraceOptions>,
    tool_proficiency: Option<ToolProficiencyOptions>,
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
  elf_subrace_builder: ElfSubraceBuilder
}

impl ElfBuilder {
  pub fn build(self) -> Race {
    let subrace = self.elf_subrace_builder.build();
    Race::Elf { subrace }
  }
}
```

#### Elf Subrace

```rust
#[derive(Picker)]
pub enum ElfSubrace {
  DarkElf, 
  HighElf, 
  #[sculptable]
  WoodElf(Cantrip)
}
```

bla bla bla

## Last but not least

Don't forget to implement the Picker traits for the other enums.

Sadly the picker implementations for the SheetBuilder still have to be written manually.
Maybe we can help these get generated through a build script.

