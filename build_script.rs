use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::prelude::*;
use maplit::hashmap;
use std::iter::FromIterator;
use std::path::Path;
use std::process::Command;
use std::{
    env,
    fs::{self, File},
    process,
};
use convert_case::{Case, Casing};
use textwrap::{fill, indent};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Schema {
    #[serde(rename = "$schema")]
    schema: String,
    id: String,
    description: String,
    discriminator: Discriminator,
    one_of: Vec<ClassReference>,
    definitions: HashMap<String, Definition>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClassReference {
    #[serde(rename = "$ref")]
    fhir_ref: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Discriminator {
    property_name: String,
    mapping: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Item {
    Ref(String),
    Enum(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Property {
    Reference {
        description: String,
        #[serde(rename = "$ref")]
        fhir_ref: String,
    },
    Array {
        description: String,
        items: HashMap<String, Item>,
        #[serde(rename = "type")]
        fhir_type: String,
    },
    PatternedTyped {
        description: String,
        pattern: String,
        #[serde(rename = "type")]
        fhir_type: String,
    },
    Typed {
        description: String,
        #[serde(rename = "type")]
        fhir_type: String,
    },
    Enum {
        description: String,
        #[serde(rename = "enum")]
        fhir_enum: Vec<String>,
    },
    Const {
        description: String,
        #[serde(rename = "const")]
        fhir_const: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Definition {
    pattern: Option<String>,
    #[serde(rename = "type")]
    fhir_type: Option<String>,
    description: Option<String>,
    properties: Option<BTreeMap<String, Property>>,
    additional_properties: Option<bool>,
    required: Option<Vec<String>>,
    one_of: Option<Vec<BTreeMap<String, String>>>,
}

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!("OUT_DIR environment variable not defined.");
            process::exit(1)
        }
    };
    fs::create_dir_all(&outdir.to_str().unwrap()).unwrap();

    let builtin_type_to_class_map: HashMap<&str, (&str, Option<&str>)> = hashmap! {
      "int" => ("i64", None),
      "integer" => ("i64", None),
      "integer64" => ("i64", None),
      "number" => ("f64", None), // todo
      "uri" => ("&str", None),
      "url" => ("&str", None),
      "markdown" => ("&str", None),
      "xhtml" => ("&str", None),
      "decimal" => ("f64", None),
      "positiveInt" => ("i64", None),
      "canonical" => ("&str", None),
      "float" => ("f64", None),
      "string" => ("&str", None),
      "code" => ("&str", None),
      "boolean" => ("bool", None),
      "unsignedInt" => ("u64", None),
      "id" => ("&str", None),
      "oid" => ("&str", None),
      "time" => ("&str", None), // todo
      "time" => ("&str", None), // todo
      "instant" => ("&str", None), // todo
      "date" => ("&str", None), // todo
      "base64Binary" => ("&str", None),
      "dateTime" => ("&str", None), // todo
    };

    let property_replacement_map = hashmap! {
      "type" => "fhir_type",
      "r#type" => "fhir_r_type",
      "use" => "fhir_use",
      "<" => "less_than",
      ">" => "greater_than",
      "<=" => "less_than_or_equal",
      ">=" => "greater_than_or_equal",
      "=" => "equal",
      "!=" => "not_equal",
      "0" => "zero",
      "1" => "one",
      "2" => "two",
      "3" => "three",
      "4" => "four",
      "5" => "five",
      "6" => "six",
      "7" => "seven",
      "8" => "eight",
      "9" => "nine",
      "10" => "ten",
      "11" => "eleven",
      "12" => "twelve",
      "r#for" => "fhir_r_for",
      "for" => "fhir_for",
      "r#ref" => "fhir_r_ref",
      "ref" => "fhir_ref",
      "r#abstract" => "fhir_r_abstract",
      "abstract" => "fhir_abstract",
    };


    match fs::read_dir("schema"){
        Ok(schema_files) => {
            for file_entry in schema_files {
                match file_entry {
                    Ok(fp) => {
                        let file_path = fp.path();
                        let file_name = fp.file_name().to_str().unwrap().to_owned();
                        match file_path.extension() {
                            Some(ex) => {
                                if ex == "json" && !file_name.contains("stu3") {
                                    println!("Parsing: {}", &file_name);
                                    let fhir_version = file_name.to_owned().replace(".json", "");
                                    println!("FHIR Version: {}", fhir_version);
                                    fs::remove_dir_all(format!("src/models/{}", fhir_version)).unwrap();
                                    fs::create_dir_all(format!("src/models/{}", fhir_version)).unwrap();
                                    let schema_contents = fs::read_to_string(file_path.to_owned()).expect("Something went wrong reading the file");
                                    let fhir_schema: Schema = serde_json::from_str(&schema_contents).expect("Unable to parse schema.");
                                    let mut reference_to_class_name_map: HashMap<String, String> = HashMap::new();
                                    for (class_name, reference) in &fhir_schema.discriminator.mapping {
                                        println!("ref: {}, class_name: {}", &reference, &class_name);
                                        reference_to_class_name_map.insert(reference.to_string(), class_name.to_string());
                                    }
                                    let mut model_mod_contents = String::new();
                                    model_mod_contents.push_str("#![allow(non_snake_case)]\n\n");
                                    for (definition_name, definition) in &fhir_schema.definitions {
                                        let contents = generate_trait(
                                            &fhir_version,
                                            &definition_name,
                                            &definition,
                                            &reference_to_class_name_map,
                                            &builtin_type_to_class_map,
                                            &property_replacement_map,
                                        );
                                        let path_string = format!("src/models/{}/{}.rs", fhir_version, definition_name);
                                        let wrote_file = write_string_to_file(&contents, &path_string);
                                        if wrote_file {
                                            model_mod_contents.push_str("pub mod ");
                                            model_mod_contents.push_str(&definition_name);
                                            model_mod_contents.push_str(";\n");
                                        }
                                    }
                                    write_string_to_file(&model_mod_contents, format!("src/models/{}/mod.rs", fhir_version).as_str());
                                }
                            }
                            None => {
                                eprintln!("Not a file.");
                            }

                        }
                    }
                    Err(e) => eprintln!("No DirEntry: {}",e),
                }
            }
        },
        Err(e) => {
            eprintln!("Schema folder not found: {}",e);
        }
    };

}

fn write_string_to_file(contents: &str, path_string: &str) -> bool {
    if contents.len() == 0 {
        return false;
    }
    let path = Path::new(&path_string);
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path_string, why.to_string()),
        Ok(file) => file,
    };

    // Write the file contents string to `file`, returns `io::Result<()>`
    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path_string, why.to_string()),
        Ok(_) => println!("successfully wrote to {}", path_string),
    }

    // Run rustfmt on the file if it ends with .rs
    if path_string.ends_with(".rs") {
        Command::new("rustfmt")
            .arg(&path_string)
            .output()
            .expect("failed to execute process");
    }

    return true;
}

fn generate_trait(
    fhir_version: &str,
    name: &str,
    definition: &Definition,
    reference_to_class_name_map: &HashMap<String, String>,
    builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
    property_replacement_map: &HashMap<&str, &str>,
) -> String {
    let mut string = String::new();

    let mut pending_enums: Vec<(String, &Vec<String>)> = Vec::new();
    let mut pending_imports: HashSet<String> = HashSet::new();

    string.push_str("#![allow(unused_imports, non_camel_case_types)]\n\n");

    pending_imports.insert("serde_json::value::Value".to_string());
    pending_imports.insert("std::borrow::Cow".to_string());

    let mut inner_string = String::new();

    let mut validation_string = String::new();

    if let Some(one_of) = &definition.one_of {
        inner_string.push_str("\n#[derive(Debug)]\n");
        inner_string.push_str("pub struct ");
        inner_string.push_str(name);
        inner_string.push_str("<'a> {\n");
        inner_string.push_str("  pub(crate) value: Cow<'a, Value>,\n");
        inner_string.push_str("}\n\n");

        let mut impl_string = String::new();
        impl_string.push_str("impl ");
        impl_string.push_str(name);
        impl_string.push_str("<'_> {\n");
        impl_string.push_str("  pub fn new(value: &Value) -> ");
        impl_string.push_str(name);
        impl_string.push_str(" {\n    ");
        impl_string.push_str(name);
        impl_string.push_str(" { value: Cow::Borrowed(value) }\n  }\n\n");

        impl_string.push_str("  pub fn to_json(&self) -> Value { (*self.value).clone() }\n\n");

        impl_string.push_str("\n  pub fn resource(&self) -> Option<");
        impl_string.push_str(name);
        impl_string.push_str("Enum> {\n");
        impl_string
            .push_str("    let fhir_type = self.value[\"resourceType\"].as_str().unwrap();\n");
        impl_string.push_str("    match fhir_type {\n");

        let mut validation_string = String::new();
        validation_string.push_str("\n  pub fn validate(&self) -> bool {\n");
        validation_string.push_str("    if let Some(resource) = self.resource() {\n");
        validation_string.push_str("      match resource {\n");

        inner_string.push_str("#[derive(Debug)]\n");
        inner_string.push_str("pub enum ");
        inner_string.push_str(name);
        inner_string.push_str("Enum<'a> {\n");
        for hash_map in one_of {
            let fhir_ref = &hash_map["$ref"];
            let original_name = extract_type_from_ref(&fhir_ref);
            let type_definition = type_definition_from_fhir_ref(
                fhir_version,
                &fhir_ref,
                &reference_to_class_name_map,
                builtin_type_to_class_map,
            );
            if let Some(import) = type_definition.import {
                pending_imports.insert(import);
            }
            inner_string.push_str("  Resource");
            inner_string.push_str(&original_name);
            inner_string.push_str("(");
            inner_string.push_str(&type_definition.name);
            inner_string.push_str("<'a>),\n");
            impl_string.push_str("      \"");
            impl_string.push_str(&original_name);
            impl_string.push_str("\" => Some(");
            impl_string.push_str(name);
            impl_string.push_str("Enum::Resource");
            impl_string.push_str(&type_definition.name);
            impl_string.push_str("(");
            impl_string.push_str(&type_definition.name);
            impl_string.push_str(" { value: self.value.clone() })),\n");

            validation_string.push_str("        ");
            validation_string.push_str(name);
            validation_string.push_str("Enum::Resource");
            validation_string.push_str(&type_definition.name);
            validation_string.push_str("(val) => { return val.validate(); },\n");
        }
        validation_string.push_str("      }\n");
        validation_string.push_str("    }\n");
        validation_string.push_str("    return false;\n");
        validation_string.push_str("  }\n");
        impl_string.push_str("      _ => None,\n");
        impl_string.push_str("    }\n");
        impl_string.push_str("  }\n\n");
        impl_string.push_str(&validation_string);
        impl_string.push_str("}\n\n");
        inner_string.push_str("}\n\n");
        inner_string.push_str(&impl_string);
    } else {
        inner_string.push_str("\n\n");

        if let Some(description) = &definition.description {
            inner_string.push_str(&indent(&fill(&description, 82), "/// "));
            inner_string.push_str("\n");
        }
        inner_string.push_str("\n#[derive(Debug)]\n");
        inner_string.push_str("pub struct ");
        inner_string.push_str(name);
        inner_string.push_str("<'a> {\n");
        inner_string.push_str("  pub(crate) value: Cow<'a, Value>,\n}\n\n");

        inner_string.push_str("impl ");
        inner_string.push_str(name);
        inner_string.push_str("<'_> {\n");
        inner_string.push_str("  pub fn new(value: &Value) -> ");
        inner_string.push_str(name);
        inner_string.push_str(" {\n    ");
        inner_string.push_str(name);
        inner_string.push_str(" { value: Cow::Borrowed(value) }\n  }\n\n");

        inner_string.push_str("  pub fn to_json(&self) -> Value { (*self.value).clone() }\n\n");

        validation_string.push_str("  pub fn validate(&self) -> bool {\n");

        let required_property_names: HashSet<String> = match &definition.required {
            Some(strings) => HashSet::from_iter(strings.iter().cloned()),
            None => HashSet::new(),
        };
        let properties = match &definition.properties {
            Some(properties) => properties,
            None => return String::new(),
        };

        let mut builder_constructor_definition = String::new();
        builder_constructor_definition.push_str("\n#[derive(Debug)]\n");
        builder_constructor_definition.push_str("pub struct ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition.push_str("Builder {\n");
        builder_constructor_definition.push_str("  pub(crate) value: Value,\n}\n\n");
        builder_constructor_definition.push_str("impl ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition.push_str("Builder {\n");
        builder_constructor_definition.push_str("  pub fn build(&self) -> ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition.push_str(" {\n");
        builder_constructor_definition.push_str("    ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition
            .push_str(" { value: Cow::Owned(self.value.clone()) }\n  }\n\n");

        builder_constructor_definition.push_str("  pub fn with(existing: ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition.push_str(") -> ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition.push_str("Builder {\n");
        builder_constructor_definition.push_str("    ");
        builder_constructor_definition.push_str(name);
        builder_constructor_definition
            .push_str("Builder { value: (*existing.value).clone() }\n  }\n\n");

        builder_constructor_definition.push_str("  pub fn new(");

        let mut builder_constructor_impl = String::new();
        builder_constructor_impl.push_str(") -> ");
        builder_constructor_impl.push_str(name);
        builder_constructor_impl.push_str("Builder {\n");
        builder_constructor_impl.push_str("    let mut __value: Value = json!({});\n");
        pending_imports.insert("serde_json::json".to_string());

        let mut builder_body = String::new();

        for (property_name, property) in properties {
            let required = required_property_names.contains(&property_name[..]);
            match property {
                Property::Reference {
                    description,
                    fhir_ref,
                } => {
                    let type_definition = type_definition_from_fhir_ref(
                        fhir_version,
                        &fhir_ref,
                        reference_to_class_name_map,
                        builtin_type_to_class_map,
                    );

                    if let Some(import) = &type_definition.import {
                        pending_imports.insert(import.clone());
                    }

                    write_property(
                        &mut inner_string,
                        &mut validation_string,
                        &mut builder_constructor_definition,
                        &mut builder_constructor_impl,
                        &mut builder_body,
                        &property_name,
                        &type_definition,
                        &description,
                        &name,
                        None,
                        required,
                        false,
                        "  ",
                        &property_replacement_map,
                    );
                }
                Property::PatternedTyped {
                    description,
                    pattern,
                    fhir_type,
                } => {
                    let type_definition = type_definition_from_fhir_type(
                        fhir_version,
                        &fhir_type,
                        reference_to_class_name_map,
                        builtin_type_to_class_map,
                    );

                    if let Some(import) = &type_definition.import {
                        pending_imports.insert(import.clone());
                    }

                    write_property(
                        &mut inner_string,
                        &mut validation_string,
                        &mut builder_constructor_definition,
                        &mut builder_constructor_impl,
                        &mut builder_body,
                        &property_name,
                        &type_definition,
                        &description,
                        &name,
                        Some(pattern.to_string()),
                        required,
                        false,
                        "  ",
                        &property_replacement_map,
                    );
                }
                Property::Array {
                    description,
                    items,
                    fhir_type,
                } => {
                    assert_eq!(fhir_type, "array");
                    if let Some(Item::Ref(item_ref)) = items.get("$ref") {
                        let type_definition = type_definition_from_fhir_ref(
                            fhir_version,
                            &item_ref,
                            reference_to_class_name_map,
                            builtin_type_to_class_map,
                        );
                        if let Some(import) = &type_definition.import {
                            pending_imports.insert(import.clone());
                        }
                        write_property(
                            &mut inner_string,
                            &mut validation_string,
                            &mut builder_constructor_definition,
                            &mut builder_constructor_impl,
                            &mut builder_body,
                            &property_name,
                            &type_definition,
                            &description,
                            &name,
                            None,
                            required,
                            true,
                            "  ",
                            &property_replacement_map,
                        );
                    } else if let Some(Item::Enum(item_enum)) = items.get("$ref") {
                        let type_definition = TypeDefinition {
                            name: format!("{}{}", name, property_name.to_case(Case::Pascal)),
                            builtin: false,
                            string_enum: true,
                            import: None,
                        };
                        pending_enums.push((type_definition.name.clone(), item_enum));
                        write_property(
                            &mut inner_string,
                            &mut validation_string,
                            &mut builder_constructor_definition,
                            &mut builder_constructor_impl,
                            &mut builder_body,
                            &property_name,
                            &type_definition,
                            &description,
                            &name,
                            None,
                            required,
                            false,
                            "  ",
                            &property_replacement_map,
                        );
                    }
                }
                Property::Typed {
                    description,
                    fhir_type,
                } => {
                    let type_definition = type_definition_from_fhir_type(
                        fhir_version,
                        &fhir_type,
                        reference_to_class_name_map,
                        builtin_type_to_class_map,
                    );
                    if let Some(import) = &type_definition.import {
                        pending_imports.insert(import.clone());
                    }
                    write_property(
                        &mut inner_string,
                        &mut validation_string,
                        &mut builder_constructor_definition,
                        &mut builder_constructor_impl,
                        &mut builder_body,
                        &property_name,
                        &type_definition,
                        &description,
                        &name,
                        None,
                        required,
                        false,
                        "  ",
                        &property_replacement_map,
                    );
                }
                Property::Enum {
                    description,
                    fhir_enum,
                } => {
                    let type_definition = TypeDefinition {
                        name: format!("{}{}", name, property_name.to_case(Case::Pascal)),
                        builtin: false,
                        string_enum: true,
                        import: None,
                    };
                    pending_enums.push((type_definition.name.clone(), fhir_enum));
                    write_property(
                        &mut inner_string,
                        &mut validation_string,
                        &mut builder_constructor_definition,
                        &mut builder_constructor_impl,
                        &mut builder_body,
                        &property_name,
                        &type_definition,
                        &description,
                        &name,
                        None,
                        required,
                        false,
                        "  ",
                        &property_replacement_map,
                    );
                }
                Property::Const {
                    description: _,
                    fhir_const: _,
                } => {}
            }
        }

        validation_string.push_str("    return true;\n  }\n\n");

        inner_string.push_str(&validation_string);

        inner_string.push_str("}\n\n");

        inner_string.push_str(&builder_constructor_definition);
        inner_string.push_str(&builder_constructor_impl);
        inner_string.push_str("    return ");
        inner_string.push_str(name);
        inner_string.push_str("Builder { value: __value };\n  }\n\n");
        inner_string.push_str(&builder_body);
        inner_string.push_str("\n}\n\n");
    }

    for (enum_name, values) in pending_enums {
        inner_string.push_str("\n#[derive(Debug)]\n");
        inner_string.push_str("pub enum ");
        inner_string.push_str(&enum_name);
        inner_string.push_str(" {\n");
        let mut enum_impl_string = String::new();
        enum_impl_string.push_str("impl ");
        enum_impl_string.push_str(&enum_name);
        enum_impl_string.push_str(" {\n");
        enum_impl_string.push_str("  pub fn from_string(string: &str) -> Option<");
        enum_impl_string.push_str(&enum_name);
        enum_impl_string.push_str("> {\n");
        enum_impl_string.push_str("    match string {\n");

        let mut enum_serialization_string = String::new();
        enum_serialization_string.push_str("  pub fn to_string(&self) -> String {\n");
        enum_serialization_string.push_str("    match self {\n");

        for value in values {
            let sanitized_name = sanitize_name(value, property_replacement_map).to_case(Case::Pascal);
            inner_string.push_str("  ");
            inner_string.push_str(&sanitized_name);
            inner_string.push_str(",\n");

            enum_impl_string.push_str("        \"");
            enum_impl_string.push_str(&value);
            enum_impl_string.push_str("\" => Some(");
            enum_impl_string.push_str(&enum_name);
            enum_impl_string.push_str("::");
            enum_impl_string.push_str(&sanitized_name);
            enum_impl_string.push_str("),\n");

            enum_serialization_string.push_str("        ");
            enum_serialization_string.push_str(&enum_name);
            enum_serialization_string.push_str("::");
            enum_serialization_string.push_str(&sanitized_name);
            enum_serialization_string.push_str(" => \"");
            enum_serialization_string.push_str(&value);
            enum_serialization_string.push_str("\".to_string(),\n");
        }
        enum_serialization_string.push_str("    }\n  }\n");
        inner_string.push_str("}\n\n");
        enum_impl_string.push_str("        _ => None,\n");
        enum_impl_string.push_str("    }\n  }\n\n");
        enum_impl_string.push_str(&enum_serialization_string);
        enum_impl_string.push_str("}\n\n");

        inner_string.push_str(&enum_impl_string);
    }

    for import in pending_imports {
        if !import.ends_with(&format!("::{}", name)) {
            string.push_str(&format!("use {};\n", import));
        }
    }
    string.push_str(&inner_string);

    return string;
}

fn write_property(
    inner_string: &mut String,
    validation_string: &mut String,
    builder_constructor_definition: &mut String,
    builder_constructor_impl: &mut String,
    builder_body: &mut String,
    property_name: &str,
    type_definition: &TypeDefinition,
    description: &str,
    self_name: &str,
    _pattern: Option<String>,
    required: bool,
    array: bool,
    indentation_level: &str,
    property_replacement_map: &HashMap<&str, &str>,
) {
    let sanitized_name = sanitize_property_name(property_name, &property_replacement_map);

    let mut type_name = String::new();
    let mut non_optional_type_name = String::new();
    {
        if array {
            non_optional_type_name.push_str("Vec<")
        }
        non_optional_type_name.push_str(&type_definition.name);
        if array {
            non_optional_type_name.push_str(">")
        }
        if !required {
            type_name.push_str("Option<");
        }
        type_name.push_str(&non_optional_type_name);
        if !required {
            type_name.push_str(">");
        }
    }

    // Builder
    if required {
        // Builder constructor (required props)
        builder_constructor_definition.push_str(&sanitized_name);
        builder_constructor_definition.push_str(": ");
        builder_constructor_definition.push_str(&type_name);
        builder_constructor_definition.push_str(", ");

        builder_constructor_impl.push_str("    __value[\"");
        builder_constructor_impl.push_str(&property_name);
        builder_constructor_impl.push_str("\"] = ");
        if type_definition.builtin {
            builder_constructor_impl.push_str("json!(");
            builder_constructor_impl.push_str(&sanitized_name);
            builder_constructor_impl.push_str(");\n");
        } else if type_definition.string_enum {
            if array {
                builder_constructor_impl.push_str("json!(");
                builder_constructor_impl.push_str(&sanitized_name);
                builder_constructor_impl
                    .push_str(".into_iter().map(|e| e.to_string()).collect::<Vec<_>>());\n")
            } else {
                builder_constructor_impl.push_str("json!(");
                builder_constructor_impl.push_str(&sanitized_name);
                builder_constructor_impl.push_str(".to_string());\n");
            }
        } else {
            if array {
                builder_constructor_impl.push_str("json!(");
                builder_constructor_impl.push_str(&sanitized_name);
                builder_constructor_impl
                    .push_str(".into_iter().map(|e| e.value).collect::<Vec<_>>());\n")
            } else {
                builder_constructor_impl.push_str("json!(");
                builder_constructor_impl.push_str(&sanitized_name);
                builder_constructor_impl.push_str(".value);\n");
            }
        }
    } else {
        // End Builder constructor
        // Builder body begin
        builder_body.push_str("  pub fn ");
        builder_body.push_str(&sanitized_name);
        builder_body.push_str("<'a>(&'a mut self, val: ");
        builder_body.push_str(&non_optional_type_name);
        builder_body.push_str(") -> &'a mut ");
        builder_body.push_str(self_name);
        builder_body.push_str("Builder {\n");
        builder_body.push_str("    self.value[\"");
        builder_body.push_str(&property_name);
        builder_body.push_str("\"] = ");
        if type_definition.builtin {
            builder_body.push_str("json!(val);\n");
        } else if type_definition.string_enum {
            if array {
                builder_body.push_str(
                    "json!(val.into_iter().map(|e| e.to_string()).collect::<Vec<_>>());\n",
                )
            } else {
                builder_body.push_str("json!(val.to_string());\n");
            }
        } else {
            if array {
                builder_body
                    .push_str("json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());\n")
            } else {
                builder_body.push_str("json!(val.value);\n");
            }
        }
        builder_body.push_str("    return self;\n  }\n\n");
    }
    // End Builder

    // Validation
    if required {
        if array {
            if !type_definition.builtin {
                validation_string.push_str("    if !");
                validation_string.push_str("self.");
                validation_string.push_str(&sanitized_name);
                validation_string.push_str("()");
                validation_string.push_str(
                    ".into_iter().map(|e| { e.validate() }).all(|x| x == true) { return false; }\n",
                );
            } else {
                validation_string.push_str("    self.");
                validation_string.push_str(&sanitized_name);
                validation_string.push_str("()");
                validation_string.push_str(".into_iter().for_each(|_e| {});\n");
            }
        } else if !type_definition.builtin && !type_definition.string_enum {
            validation_string.push_str("    if !");
            validation_string.push_str("self.");
            validation_string.push_str(&sanitized_name);
            validation_string.push_str("()");
            validation_string.push_str(".validate() { return false; }\n");
        }
    } else {
        validation_string.push_str("    if let Some(_val) = self.");
        validation_string.push_str(&sanitized_name);
        validation_string.push_str("() {\n");
        if array {
            if !type_definition.builtin {
                validation_string.push_str("      if !_val.into_iter().map(|e| { e.validate() }).all(|x| x == true) { return false; }\n");
            } else {
                validation_string.push_str("      _val.into_iter().for_each(|_e| {});\n");
            }
        } else if !type_definition.builtin && !type_definition.string_enum {
            validation_string.push_str("      if !_val.validate() { return false; }\n");
        }
        validation_string.push_str("    }\n");
    }
    // End Validation

    let mut getter = String::new();
    // getter
    getter.push_str("\n  pub fn ");
    getter.push_str(&sanitized_name);
    getter.push_str("(&self) -> ");
    getter.push_str(&type_name);

    // generated impl
    inner_string.push_str(&indent(
        &fill(&sanitize_description(description), 82),
        &format!("{}{}", indentation_level, "/// "),
    ));
    inner_string.push_str(&getter);
    inner_string.push_str(" {\n");
    if array {
        if type_definition.builtin {
            if required {
                if type_definition.name == "&str" {
                    inner_string.push_str("    self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>()\n");
                } else {
                    inner_string.push_str("    self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string
                        .push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| e.as_");
                    inner_string.push_str(&type_definition.name);
                    inner_string.push_str("().unwrap()).collect::<Vec<_>>()\n");
                }
            } else {
                if type_definition.name == "&str" {
                    inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string
            .push_str("\") {\n      return Some(val.into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n");
                } else {
                    inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| e.as_");
                    inner_string.push_str(&type_definition.name);
                    inner_string
                        .push_str("().unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n");
                }
            }
        } else {
            if required {
                if type_definition.string_enum {
                    inner_string.push_str("    self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| ");
                    inner_string.push_str(&type_definition.name);
                    inner_string.push_str("::from_string(&e).unwrap()).collect::<Vec<_>>()\n");
                } else {
                    inner_string.push_str("    self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| ");
                    inner_string.push_str(&type_definition.name);
                    inner_string.push_str(" { value: Cow::Borrowed(e) }).collect::<Vec<_>>()\n");
                }
            } else {
                if type_definition.string_enum {
                    inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| ");
                    inner_string.push_str(&type_definition.name);
                    inner_string.push_str(
            "::from_string(&e).unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n",
          );
                } else {
                    inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
                    inner_string.push_str(&property_name);
                    inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| ");
                    inner_string.push_str(&type_definition.name);
                    inner_string.push_str(
            " { value: Cow::Borrowed(e) }).collect::<Vec<_>>());\n    }\n    return None;\n",
          );
                }
            }
        }
    // end arrays
    } else if type_definition.builtin {
        if required {
            if type_definition.name == "&str" {
                inner_string.push_str("    self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\").unwrap().as_str().unwrap()\n");
            } else {
                inner_string.push_str("    self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\").unwrap().as_");
                inner_string.push_str(&type_definition.name);
                inner_string.push_str("().unwrap()\n");
            }
        } else {
            if type_definition.name == "&str" {
                inner_string.push_str("    if let Some(Value::String(string)) = self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string
                    .push_str("\") {\n      return Some(string);\n    }\n    return None;\n");
            } else {
                inner_string.push_str("    if let Some(val) = self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\") {\n      return Some(val.as_");
                inner_string.push_str(&type_definition.name);
                inner_string.push_str("().unwrap());\n    }\n    return None;\n");
            }
        }
    } else {
        if required {
            if type_definition.string_enum {
                inner_string.push_str("    ");
                inner_string.push_str(&type_definition.name);
                inner_string.push_str("::from_string(");
                inner_string.push_str("&self.value[\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\"].as_str().unwrap()).unwrap()\n");
            } else {
                inner_string.push_str("    ");
                inner_string.push_str(&type_definition.name);
                inner_string.push_str(" {\n");
                inner_string.push_str("      value: Cow::Borrowed(&self.value[\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\"]),\n    }\n");
            }
        } else {
            if type_definition.string_enum {
                inner_string.push_str("    if let Some(Value::String(val)) = self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\") {\n      return Some(");
                inner_string.push_str(&type_definition.name);
                inner_string.push_str("::from_string(&val).unwrap());\n    }\n    return None;\n");
            } else {
                inner_string.push_str("    if let Some(val) = self.value.get(\"");
                inner_string.push_str(&property_name);
                inner_string.push_str("\") {\n      return Some(");
                inner_string.push_str(&type_definition.name);
                inner_string
                    .push_str(" { value: Cow::Borrowed(val) });\n    }\n    return None;\n");
            }
        }
    }
    // todo arrays!
    inner_string.push_str("  }\n\n");
}

struct TypeDefinition {
    name: String,
    builtin: bool,
    string_enum: bool,
    import: Option<String>,
}

fn type_definition_from_fhir_type(
    fhir_version: &str,
    fhir_type: &str,
    reference_to_class_name_map: &HashMap<String, String>,
    builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> TypeDefinition {
    if let Some(builtin) = builtin_type_to_class_map.get(&fhir_type) {
        return TypeDefinition {
            name: builtin.0.to_string(),
            builtin: true,
            string_enum: false,
            import: match builtin.1 {
                Some(import) => Some(import.to_string()),
                None => None,
            },
        };
    }
    if let None = reference_to_class_name_map.get(fhir_type) {
        // println!("Missing non-builtin class: {}", fhir_type);
    }

    return TypeDefinition {
        name: fhir_type.to_string(),
        builtin: false,
        string_enum: false,
        import: Some(format!("crate::models::{}::{}::{}",fhir_version, fhir_type, fhir_type)),
    };
}

fn type_definition_from_fhir_ref(
    fhir_version: &str,
    fhir_ref: &str,
    reference_to_class_name_map: &HashMap<String, String>,
    builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> TypeDefinition {
    let extracted = extract_type_from_ref(&fhir_ref);
    return type_definition_from_fhir_type(
        fhir_version,
        extracted,
        reference_to_class_name_map,
        builtin_type_to_class_map,
    );
}

fn snake_case_string(string: &str) -> String {
    if string.len() <= 1 {
        return string.to_string();
    }
    // For some reason the inflection library can't snakecase strings
    // with a leading _, so we manually strip it, and snakecase the rest.
    if string.chars().next().unwrap() == '_' {
        let substr = &string[1..];
        return format!("_{}", substr.to_case(Case::Snake));
    }
    return string.to_case(Case::Snake);
}

fn sanitize_name(name: &str, property_replacement_map: &HashMap<&str, &str>) -> String {
    if let Some(replacement) = property_replacement_map.get(name) {
        return replacement.to_string();
    }
    if name.chars().all(is_numeric_char) || name.chars().next().unwrap().is_numeric() {
        return strip_symbols(&format!("FHIR_{}", name));
    }
    return strip_symbols(name);
}

fn strip_symbols(string: &str) -> String {
    string.replace("/", "_").replace(".", "_")
}

fn is_numeric_char(c: char) -> bool {
    c.is_numeric() || c == '.'
}

fn sanitize_property_name(name: &str, property_replacement_map: &HashMap<&str, &str>) -> String {
    return snake_case_string(&sanitize_name(name, property_replacement_map));
}

fn sanitize_description(string: &str) -> String {
    string.replace("\n", "  ").replace("\r", "  ")
}

fn extract_type_from_ref(fhir_ref: &str) -> &str {
    let split: Vec<&str> = fhir_ref.split("/").collect();
    return split[2];
}
