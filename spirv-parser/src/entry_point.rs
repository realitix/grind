use std::collections::HashMap;

use enums;
use parse;

use format_from_id;
use is_builtin;
use location_decoration;
use name_from_id;

pub fn get_input_output(doc: &parse::Spirv, instruction: &parse::Instruction) -> (HashMap<String, u32>, HashMap<String, u32>) {
    let (execution, id, ep_name, interface) = match instruction {
        &parse::Instruction::EntryPoint {
            ref execution,
            id,
            ref name,
            ref interface,
            ..
        } => {
            (execution, id, name, interface)
        },
        _ => unreachable!(),
    };

    let ignore_first_array_in = match *execution {
                                    enums::ExecutionModel::ExecutionModelTessellationControl =>
                                        true,
                                    enums::ExecutionModel::ExecutionModelTessellationEvaluation =>
                                        true,
                                    enums::ExecutionModel::ExecutionModelGeometry => true,
                                    _ => false,
                                };

    let ignore_first_array_out = match *execution {
                                    enums::ExecutionModel::ExecutionModelTessellationControl =>
                                        true,
                                    _ => false,
                                };

    let mut input_elements = HashMap::new();
    let mut output_elements = HashMap::new();

    // Filling `input_elements` and `output_elements`.
    for interface in interface.iter() {
        for i in doc.instructions.iter() {
            match i {
                &parse::Instruction::Variable {
                    result_type_id,
                    result_id,
                    ref storage_class,
                    ..
                } if &result_id == interface => {
                    if is_builtin(doc, result_id) {
                        continue;
                    }

                    let (to_insert, ignore_first_array) = match storage_class {
                        &enums::StorageClass::StorageClassInput => (&mut input_elements,
                                                                    ignore_first_array_in),
                        &enums::StorageClass::StorageClassOutput => (&mut output_elements,
                                                                     ignore_first_array_out),
                        _ => continue,
                    };

                    let name = name_from_id(doc, result_id);
                    if name == "__unnamed" {
                        continue;
                    } // FIXME: hack

                    let loc = match location_decoration(doc, result_id) {
                        Some(l) => l,
                        None => panic!("Attribute `{}` (id {}) is missing a location",
                                       name,
                                       result_id),
                    };

                    to_insert.insert(name, loc);
                    
                    // format_from_id(doc, result_type_id, ignore_first_array)
                    // -> return (type, location_length), it can be useful 
                },
                _ => (),
            }
        }
    }

    (input_elements, output_elements)

}