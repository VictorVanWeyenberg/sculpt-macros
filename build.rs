use std::fmt::Debug;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use proc_macro2::TokenTree;
use quote::{format_ident, quote, ToTokens};
use rust_format::{Formatter, RustFmt};
use syn::{ItemEnum, Type};

const OPTIONS: &str = "Discriminants";

// =================================================================================================
// Dependency Tree Domain
// =================================================================================================

#[derive(Debug)]
struct DependencyTree {
    nodes: Vec<DependencyNode>,
}

impl DependencyTree {
    fn new(nodes: Vec<DependencyNode>) -> DependencyTree {
        DependencyTree { nodes }
    }

    fn find_node(&self, d_type: &syn::Type) -> &DependencyNode {
        match d_type {
            syn::Type::Path(type_path) => self
                .nodes
                .iter()
                .find(|dn| dn.name == *type_path.path.get_ident().unwrap())
                .unwrap(),
            _ => panic!("Finding node with type that's not a type path."),
        }
    }

    fn find_root(&self) -> Option<&DependencyNode> {
        let mut possible_roots = self.nodes.iter()
            .filter(|node| node.d_type.is_root());
        let number_of_roots = possible_roots.clone().count();
        if number_of_roots == 0 {
            None
        } else if number_of_roots > 1 {
            panic!("Only one root Sculptor allowed per file.");
        } else {
            possible_roots.next()
        }
    }
}

#[derive(Debug)]
struct DependencyNode {
    name: syn::Ident,
    d_type: DependencyType,
    formatter: IdentFormatter,
}

impl DependencyNode {
    fn new(name: syn::Ident, d_type: DependencyType) -> DependencyNode {
        let formatter = IdentFormatter(name.clone());
        DependencyNode {
            name,
            d_type,
            formatter,
        }
    }

    fn generate_picker_method(&self) -> Option<proc_macro2::TokenStream> {
        if !self.d_type.is_pickable() {
            return None;
        }
        let pick_method = self.formatter.pick_method();
        let picker_trait = self.formatter.picker_trait();
        Some(quote! {
            fn #pick_method(&self, picker: &mut impl #picker_trait) {
                let choice = picker.options()[0];
                picker.fulfill(&choice);
            }
        })
    }
}

#[derive(Debug)]
struct IdentFormatter(syn::Ident);

impl IdentFormatter {
    fn ident_builder_callbacks(&self) -> syn::Ident {
        format_ident!("{}BuilderCallbacks", self.0)
    }

    fn pick_method(&self) -> syn::Ident {
        format_ident!("pick_{}", self.0.to_string().to_lowercase())
    }

    fn picker_trait(&self) -> syn::Ident {
        format_ident!("{}Picker", self.0)
    }

    fn ident_builder(&self) -> syn::Ident {
        format_ident!("{}Builder", self.0)
    }

    fn options_enum(&self) -> syn::Ident {
        format_ident!("{}{}", self.0, OPTIONS)
    }
}

#[derive(Debug)]
enum DependencyType {
    Struct {
        fields: Vec<StructField>,
        is_root: bool,
    },
    Tuple {
        types: Vec<syn::Type>,
    },
    Enum {
        variants: Vec<EnumVariantType>,
        is_pickable: bool,
    },
}

impl DependencyType {
    fn new_struct(fields: Vec<StructField>, is_root: bool) -> DependencyType {
        DependencyType::Struct { fields, is_root }
    }

    fn new_tuple(types: Vec<syn::Type>) -> DependencyType {
        DependencyType::Tuple { types }
    }

    fn new_enum(variants: Vec<EnumVariantType>, is_pickable: bool) -> DependencyType {
        DependencyType::Enum {
            variants,
            is_pickable
        }
    }

    fn is_root(&self) -> bool {
        match self {
            DependencyType::Struct { is_root, .. } => *is_root,
            DependencyType::Enum { .. } => false,
            DependencyType::Tuple { .. } => false
        }
    }

    fn is_pickable(&self) -> bool {
        match self {
            DependencyType::Struct { .. } => false,
            DependencyType::Enum { is_pickable, .. } => *is_pickable,
            DependencyType::Tuple { .. } => false
        }
    }
}

#[derive(Debug)]
struct StructField {
    name: syn::Ident,
    s_type: Rc<syn::Type>,
    is_sculptable: bool,
}

impl StructField {
    fn new(name: syn::Ident, s_type: syn::Type, is_sculptable: bool) -> StructField {
        StructField {
            name,
            s_type: Rc::new(s_type),
            is_sculptable
        }
    }
}

#[derive(Debug)]
enum EnumVariantType {
    Struct {
        name: syn::Ident,
        fields: Vec<StructField>,
    },
    Tuple {
        name: syn::Ident,
        types: Vec<syn::Type>,
    },
    Raw {
        name: syn::Ident,
    },
}

impl EnumVariantType {
    fn new_struct(name: syn::Ident, fields: Vec<StructField>) -> EnumVariantType {
        EnumVariantType::Struct { name, fields }
    }

    fn new_tuple(name: syn::Ident, types: Vec<syn::Type>) -> EnumVariantType {
        EnumVariantType::Tuple { name, types }
    }

    fn new_raw(name: syn::Ident) -> EnumVariantType {
        EnumVariantType::Raw { name }
    }

    fn name(&self) -> &syn::Ident {
        match self {
            EnumVariantType::Struct { name, .. } => name,
            EnumVariantType::Tuple { name, .. } => name,
            EnumVariantType::Raw { name } => name,
        }
    }
}

// =================================================================================================
// Find files
// =================================================================================================

fn main() {
    let project_root_dir = env!("CARGO_MANIFEST_DIR");
    let sources = Path::new(project_root_dir).join("src").to_path_buf();
    let tests = Path::new(project_root_dir).join("tests").to_path_buf();
    let out_dir = &std::env::var("OUT_DIR").expect("Cannot find out_dir.");
    let out_sources = Path::new(out_dir)
        .join("src")
        .join("sculpt_generated.rs")
        .to_path_buf();
    let out_tests = Path::new(out_dir)
        .join("tests")
        .join("sculpt_generated.rs")
        .to_path_buf();
    read_dir(sources)
        .into_iter()
        .map(to_ast)
        .map(to_dependency_tree)
        .for_each(|dt| dt.generate(&out_sources));
    read_dir(tests)
        .into_iter()
        .map(to_ast)
        .map(to_dependency_tree)
        .for_each(|dt| dt.generate(&out_tests));
}

fn read_dir(dir: PathBuf) -> Vec<PathBuf> {
    let mut rust_files = vec![];
    match fs::read_dir(dir) {
        Ok(read_dir) => rust_files.extend(read_dir_entries(read_dir)),
        Err(_) => {}
    }
    rust_files
}

fn read_dir_entries(entries: ReadDir) -> Vec<PathBuf> {
    let mut rust_files = vec![];
    entries
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .for_each(|p| rust_files.extend(read_path(p)));
    rust_files
}

fn read_path(path: PathBuf) -> Vec<PathBuf> {
    let mut rust_files = vec![];
    if path.is_dir() {
        rust_files.extend(read_dir(path))
    } else if path.is_file() {
        match path.extension() {
            None => {}
            Some(extension) => {
                if "rs" == extension {
                    rust_files.push(path)
                }
            }
        }
    }
    rust_files
}

// =================================================================================================
// Conversion to Dependency Tree
// =================================================================================================

fn to_ast(path: PathBuf) -> syn::File {
    let mut file = fs::File::open(&path).expect(&format!("Cannot open file. {:?}", path));
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect(&format!("Cannot read contents. {:?}", path));
    syn::parse_file(&content).expect(&format!("Cannot parse file. {:?}", path))
}

fn to_dependency_tree(ast: syn::File) -> DependencyTree {
    let nodes: Vec<DependencyNode> = ast
        .items
        .into_iter()
        .map(to_dependency_node)
        .filter(|dn| dn.is_some())
        .map(|dn| dn.unwrap())
        .collect();
    DependencyTree::new(nodes)
}

fn to_dependency_node(item: syn::Item) -> Option<DependencyNode> {
    match item {
        syn::Item::Enum(item_enum) => Some(to_enum_dependency_node(item_enum)),
        syn::Item::Struct(item_struct) => Some(to_struct_dependency_node(item_struct)),
        _ => None,
    }
}

fn is_item_enum_picker(item_enum: &syn::ItemEnum) -> bool {
    for attr in &item_enum.attrs {
        if attr.path().is_ident("derive") {
            if let Some(meta_list) = attr.meta.require_list().ok() {
                if meta_list.tokens.clone().into_iter().any(|tree| {
                    if let TokenTree::Ident(ident) = tree {
                        ident == "Picker"
                    } else {
                        false
                    }
                }) {
                    return true;
                }
            }
        }
    }
    false
}

fn to_enum_dependency_node(item_enum: syn::ItemEnum) -> DependencyNode {
    let is_pickable = is_item_enum_picker(&item_enum);
    let variants: Vec<EnumVariantType> = item_enum
        .variants
        .into_iter()
        .map(to_enum_variant_type)
        .collect();
    let d_type = DependencyType::new_enum(variants, is_pickable);
    DependencyNode::new(item_enum.ident, d_type)
}

fn to_enum_variant_type(variant: syn::Variant) -> EnumVariantType {
    match variant.fields {
        syn::Fields::Named(named_fields) => {
            let fields: Vec<StructField> = named_fields
                .named
                .into_iter()
                .map(to_struct_field)
                .collect();
            EnumVariantType::new_struct(variant.ident, fields)
        }
        syn::Fields::Unnamed(unnamed_fields) => {
            let types: Vec<syn::Type> = unnamed_fields.unnamed.into_iter().map(|f| f.ty).collect();
            EnumVariantType::new_tuple(variant.ident, types)
        }
        syn::Fields::Unit => EnumVariantType::new_raw(variant.ident),
    }
}

fn to_struct_field(field: syn::Field) -> StructField {
    let is_sculptable = field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("sculptable"));
    StructField::new(field.ident.unwrap(), field.ty, is_sculptable)
}

fn is_item_struct_root(item_struct: &syn::ItemStruct) -> bool {
    for attr in &item_struct.attrs {
        if attr.path().is_ident("derive") {
            if let Some(meta_list) = attr.meta.require_list().ok() {
                if meta_list.tokens.clone().into_iter().any(|tree| {
                    if let TokenTree::Ident(ident) = tree {
                        ident == "Sculptor"
                    } else {
                        false
                    }
                }) {
                    return true;
                }
            }
        }
    }
    false
}

fn to_struct_dependency_node(item_struct: syn::ItemStruct) -> DependencyNode {
    let is_root = is_item_struct_root(&item_struct);
    let d_type = match item_struct.fields {
        syn::Fields::Named(named_fields) => {
            let fields: Vec<StructField> = named_fields
                .named
                .into_iter()
                .map(to_struct_field)
                .collect();
            DependencyType::new_struct(fields, is_root)
        }
        syn::Fields::Unnamed(unnamed_fields) => {
            let types: Vec<syn::Type> = unnamed_fields
                .unnamed
                .into_iter()
                .map(|field| field.ty)
                .collect();
            DependencyType::new_tuple(types)
        }
        syn::Fields::Unit => panic!("Struct field turns out to be unit!"),
    };
    DependencyNode::new(item_struct.ident, d_type)
}

// =================================================================================================
// Generation
// =================================================================================================

impl DependencyTree {
    fn generate(self, out_file: &PathBuf) {
        if let Some(root_node) = self.find_root() {
            let callbacks_trait = self.generate_callbacks_trait();
            // let picker_implementations = self.generate_picker_implementations(root_node);
            let gen = quote! {
                #callbacks_trait
                // #(#picker_implementations)*
            };
            let code = format!("{}", gen);
            let code = RustFmt::default().format_str(code).unwrap();
            let parent = out_file.parent().unwrap();
            fs::create_dir_all(parent).unwrap();
            match File::create(out_file) {
                Ok(mut file) => file.write_all(code.as_bytes()).unwrap(),
                Err(error) => println!("{}", error),
            }
        }
    }

    fn generate_callbacks_trait(&self) -> proc_macro2::TokenStream {
        let builder_callbacks_trait_name = self
            .find_root()
            .unwrap()
            .formatter
            .ident_builder_callbacks();
        let pick_methods = self.generate_pick_methods();
        quote! {
            pub trait #builder_callbacks_trait_name {
                #(#pick_methods)*
            }
        }
    }

    fn generate_pick_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.nodes
            .iter()
            .map(|node| node.generate_picker_method())
            .filter_map(|node| node)
            .collect()
    }

    fn generate_picker_implementations(&self, root_node: &DependencyNode) -> Vec<proc_macro2::TokenStream> {
        self.nodes.iter().filter(|node| node.d_type.is_pickable())
            .map(|node| self.generate_picker_implementation(root_node, node))
            .collect()
    }

    fn generate_picker_implementation(&self, root_node: &DependencyNode, node: &DependencyNode) -> proc_macro2::TokenStream {
        let root_callbacks_trait = root_node.formatter.ident_builder_callbacks();
        let node_picker = node.formatter.picker_trait();
        let root_builder = root_node.formatter.ident_builder();
        let node_options_enum = node.formatter.options_enum();
        let option_pick_next_calls: Vec<proc_macro2::TokenStream> =
            if let DependencyType::Enum { is_pickable: true, variants, .. } = &node.d_type {
                variants.iter()
                    .map(|variant| self.variant_to_pick_next_call(root_node, node, variant))
                    .collect()
            } else {
                panic!("Trying to generate picker implementation of a dependency type that's not an enum.");
            };
        quote! {
            impl<'a, T: #root_callbacks_trait> #node_picker for #root_builder<'a, T> {
                fn fulfill(&mut self, requirement: &#node_options_enum) {
                    self.race_builder.race = Some(requirement.clone()); // <-- Can you see the error? :p
                    match requirement {
                        #(#option_pick_next_calls,)*
                    }
                }
            }
        }
    }

    fn variant_to_pick_next_call(&self, root_node: &DependencyNode, node: &DependencyNode, variant: &EnumVariantType) -> proc_macro2::TokenStream {
        let node_options_enum = node.formatter.options_enum();
        let variant_name = variant.name();
        let next_pick_call = self.generate_next_pick_call(root_node, node, variant);
        quote! {
             #node_options_enum::#variant_name => self.callbacks.#next_pick_call(self)
        }
    }

    fn generate_next_pick_call(&self, root_node: &DependencyNode, node: &DependencyNode, variant: &EnumVariantType) -> proc_macro2::TokenStream {
        match variant {
            EnumVariantType::Struct { fields, .. } => {
                let first_type = &fields.first().unwrap().s_type;
                let next_node = self.find_node(first_type);
                let next_pick_method = next_node.formatter.pick_method();
                quote!(#next_pick_method)
            }
            EnumVariantType::Tuple { types, .. } => {
                let first_type = &types.first().unwrap();
                let next_node = self.find_node(first_type);
                let next_pick_method = next_node.formatter.pick_method();
                quote!(#next_pick_method)
            }
            EnumVariantType::Raw { name } => {
                quote!(pick_race)
            }
        }
    }
}

//           Baz     Quux1
// Foo -> Bar -> Grault
//           `-> Quux -> Corge -^
//            Qux    `-> Garply -^
//                   Quux2