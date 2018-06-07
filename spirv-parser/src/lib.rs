use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub use parse::ParseError;

mod descriptor_sets;
mod entry_point;
mod enums;
mod parse;
mod spec_consts;
mod structs;

pub struct SpirvReflection {
    pub input: HashMap<String, u32>,
    pub output: HashMap<String, u32>,
    pub uniform: HashMap<String, u32>,
}

pub fn reflect<R>(mut spirv: R) -> SpirvReflection
where
    R: Read,
{
    let mut data = Vec::new();
    spirv.read_to_end(&mut data);

    // now parsing the document
    let doc = parse::parse_spirv(&data).unwrap();

    // get input and output
    let (input, output) = {
        let mut entrypoint = None;
        for instruction in doc.instructions.iter() {
            if let &parse::Instruction::EntryPoint { .. } = instruction {
                entrypoint = Some(instruction.clone());
            }
        }
        match entrypoint {
            Some(i) => entry_point::get_input_output(&doc, &i),
            None => panic!("Can't find entrypoint"),
        }
    };

    SpirvReflection {
        input,
        output,
        uniform: HashMap::new(),
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    ParseError(ParseError),
}

impl From<IoError> for Error {
    #[inline]
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<ParseError> for Error {
    #[inline]
    fn from(err: ParseError) -> Error {
        Error::ParseError(err)
    }
}

/// Returns the vulkano `Format` and number of occupied locations from an id.
///
/// If `ignore_first_array` is true, the function expects the outermost instruction to be
/// `OpTypeArray`. If it's the case, the OpTypeArray will be ignored. If not, the function will
/// panic.
fn format_from_id(doc: &parse::Spirv, searched: u32, ignore_first_array: bool) -> (String, usize) {
    for instruction in doc.instructions.iter() {
        match instruction {
            &parse::Instruction::TypeInt {
                result_id,
                width,
                signedness,
            } if result_id == searched =>
            {
                assert!(!ignore_first_array);
                return (
                    match (width, signedness) {
                        (8, true) => "R8Sint",
                        (8, false) => "R8Uint",
                        (16, true) => "R16Sint",
                        (16, false) => "R16Uint",
                        (32, true) => "R32Sint",
                        (32, false) => "R32Uint",
                        (64, true) => "R64Sint",
                        (64, false) => "R64Uint",
                        _ => panic!(),
                    }.to_owned(),
                    1,
                );
            }
            &parse::Instruction::TypeFloat { result_id, width } if result_id == searched => {
                assert!(!ignore_first_array);
                return (
                    match width {
                        32 => "R32Sfloat",
                        64 => "R64Sfloat",
                        _ => panic!(),
                    }.to_owned(),
                    1,
                );
            }
            &parse::Instruction::TypeVector {
                result_id,
                component_id,
                count,
            } if result_id == searched =>
            {
                assert!(!ignore_first_array);
                let (format, sz) = format_from_id(doc, component_id, false);
                assert!(format.starts_with("R32"));
                assert_eq!(sz, 1);
                let format = if count == 1 {
                    format
                } else if count == 2 {
                    format!("R32G32{}", &format[3..])
                } else if count == 3 {
                    format!("R32G32B32{}", &format[3..])
                } else if count == 4 {
                    format!("R32G32B32A32{}", &format[3..])
                } else {
                    panic!("Found vector type with more than 4 elements")
                };
                return (format, sz);
            }
            &parse::Instruction::TypeMatrix {
                result_id,
                column_type_id,
                column_count,
            } if result_id == searched =>
            {
                assert!(!ignore_first_array);
                let (format, sz) = format_from_id(doc, column_type_id, false);
                return (format, sz * column_count as usize);
            }
            &parse::Instruction::TypeArray {
                result_id,
                type_id,
                length_id,
            } if result_id == searched =>
            {
                if ignore_first_array {
                    return format_from_id(doc, type_id, false);
                }

                let (format, sz) = format_from_id(doc, type_id, false);
                let len = doc.instructions
                    .iter()
                    .filter_map(|e| match e {
                        &parse::Instruction::Constant {
                            result_id,
                            ref data,
                            ..
                        } if result_id == length_id =>
                        {
                            Some(data.clone())
                        }
                        _ => None,
                    })
                    .next()
                    .expect("failed to find array length");
                let len = len.iter().rev().fold(0u64, |a, &b| (a << 32) | b as u64);
                return (format, sz * len as usize);
            }
            &parse::Instruction::TypePointer {
                result_id, type_id, ..
            } if result_id == searched =>
            {
                return format_from_id(doc, type_id, ignore_first_array);
            }
            _ => (),
        }
    }

    panic!("Type #{} not found or invalid", searched)
}

fn name_from_id(doc: &parse::Spirv, searched: u32) -> String {
    doc.instructions
        .iter()
        .filter_map(|i| {
            if let &parse::Instruction::Name {
                target_id,
                ref name,
            } = i
            {
                if target_id == searched {
                    Some(name.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .and_then(|n| if !n.is_empty() { Some(n) } else { None })
        .unwrap_or("__unnamed".to_owned())
}

fn member_name_from_id(doc: &parse::Spirv, searched: u32, searched_member: u32) -> String {
    doc.instructions
        .iter()
        .filter_map(|i| {
            if let &parse::Instruction::MemberName {
                target_id,
                member,
                ref name,
            } = i
            {
                if target_id == searched && member == searched_member {
                    Some(name.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .and_then(|n| if !n.is_empty() { Some(n) } else { None })
        .unwrap_or("__unnamed".to_owned())
}

fn location_decoration(doc: &parse::Spirv, searched: u32) -> Option<u32> {
    doc.instructions
        .iter()
        .filter_map(|i| {
            if let &parse::Instruction::Decorate {
                target_id,
                decoration: enums::Decoration::DecorationLocation,
                ref params,
            } = i
            {
                if target_id == searched {
                    Some(params[0])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
}

/// Returns true if a `BuiltIn` decorator is applied on an id.
fn is_builtin(doc: &parse::Spirv, id: u32) -> bool {
    for instruction in &doc.instructions {
        match *instruction {
            parse::Instruction::Decorate {
                target_id,
                decoration: enums::Decoration::DecorationBuiltIn,
                ..
            } if target_id == id =>
            {
                return true;
            }
            parse::Instruction::MemberDecorate {
                target_id,
                decoration: enums::Decoration::DecorationBuiltIn,
                ..
            } if target_id == id =>
            {
                return true;
            }
            _ => (),
        }
    }

    for instruction in &doc.instructions {
        match *instruction {
            parse::Instruction::Variable {
                result_type_id,
                result_id,
                ..
            } if result_id == id =>
            {
                return is_builtin(doc, result_type_id);
            }
            parse::Instruction::TypeArray {
                result_id, type_id, ..
            } if result_id == id =>
            {
                return is_builtin(doc, type_id);
            }
            parse::Instruction::TypeRuntimeArray { result_id, type_id } if result_id == id => {
                return is_builtin(doc, type_id);
            }
            parse::Instruction::TypeStruct {
                result_id,
                ref member_types,
            } if result_id == id =>
            {
                for &mem in member_types {
                    if is_builtin(doc, mem) {
                        return true;
                    }
                }
            }
            parse::Instruction::TypePointer {
                result_id, type_id, ..
            } if result_id == id =>
            {
                return is_builtin(doc, type_id);
            }
            _ => (),
        }
    }

    false
}

/// Returns the name of the Vulkan something that corresponds to an `OpCapability`.
///
/// Returns `None` if irrelevant.
// TODO: this function is a draft, as the actual names may not be the same
fn capability_name(cap: &enums::Capability) -> Option<&'static str> {
    match *cap {
        enums::Capability::CapabilityMatrix => None, // always supported
        enums::Capability::CapabilityShader => None, // always supported
        enums::Capability::CapabilityGeometry => Some("geometry_shader"),
        enums::Capability::CapabilityTessellation => Some("tessellation_shader"),
        enums::Capability::CapabilityAddresses => panic!(), // not supported
        enums::Capability::CapabilityLinkage => panic!(),   // not supported
        enums::Capability::CapabilityKernel => panic!(),    // not supported
        enums::Capability::CapabilityVector16 => panic!(),  // not supported
        enums::Capability::CapabilityFloat16Buffer => panic!(), // not supported
        enums::Capability::CapabilityFloat16 => panic!(),   // not supported
        enums::Capability::CapabilityFloat64 => Some("shader_f3264"),
        enums::Capability::CapabilityInt64 => Some("shader_int64"),
        enums::Capability::CapabilityInt64Atomics => panic!(), // not supported
        enums::Capability::CapabilityImageBasic => panic!(),   // not supported
        enums::Capability::CapabilityImageReadWrite => panic!(), // not supported
        enums::Capability::CapabilityImageMipmap => panic!(),  // not supported
        enums::Capability::CapabilityPipes => panic!(),        // not supported
        enums::Capability::CapabilityGroups => panic!(),       // not supported
        enums::Capability::CapabilityDeviceEnqueue => panic!(), // not supported
        enums::Capability::CapabilityLiteralSampler => panic!(), // not supported
        enums::Capability::CapabilityAtomicStorage => panic!(), // not supported
        enums::Capability::CapabilityInt16 => Some("shader_int16"),
        enums::Capability::CapabilityTessellationPointSize => {
            Some("shader_tessellation_and_geometry_point_size")
        }
        enums::Capability::CapabilityGeometryPointSize => {
            Some("shader_tessellation_and_geometry_point_size")
        }
        enums::Capability::CapabilityImageGatherExtended => Some("shader_image_gather_extended"),
        enums::Capability::CapabilityStorageImageMultisample => {
            Some("shader_storage_image_multisample")
        }
        enums::Capability::CapabilityUniformBufferArrayDynamicIndexing => {
            Some("shader_uniform_buffer_array_dynamic_indexing")
        }
        enums::Capability::CapabilitySampledImageArrayDynamicIndexing => {
            Some("shader_sampled_image_array_dynamic_indexing")
        }
        enums::Capability::CapabilityStorageBufferArrayDynamicIndexing => {
            Some("shader_storage_buffer_array_dynamic_indexing")
        }
        enums::Capability::CapabilityStorageImageArrayDynamicIndexing => {
            Some("shader_storage_image_array_dynamic_indexing")
        }
        enums::Capability::CapabilityClipDistance => Some("shader_clip_distance"),
        enums::Capability::CapabilityCullDistance => Some("shader_cull_distance"),
        enums::Capability::CapabilityImageCubeArray => Some("image_cube_array"),
        enums::Capability::CapabilitySampleRateShading => Some("sample_rate_shading"),
        enums::Capability::CapabilityImageRect => panic!(), // not supported
        enums::Capability::CapabilitySampledRect => panic!(), // not supported
        enums::Capability::CapabilityGenericPointer => panic!(), // not supported
        enums::Capability::CapabilityInt8 => panic!(),      // not supported
        enums::Capability::CapabilityInputAttachment => None, // always supported
        enums::Capability::CapabilitySparseResidency => Some("shader_resource_residency"),
        enums::Capability::CapabilityMinLod => Some("shader_resource_min_lod"),
        enums::Capability::CapabilitySampled1D => None, // always supported
        enums::Capability::CapabilityImage1D => None,   // always supported
        enums::Capability::CapabilitySampledCubeArray => Some("image_cube_array"),
        enums::Capability::CapabilitySampledBuffer => None, // always supported
        enums::Capability::CapabilityImageBuffer => None,   // always supported
        enums::Capability::CapabilityImageMSArray => Some("shader_storage_image_multisample"),
        enums::Capability::CapabilityStorageImageExtendedFormats => {
            Some("shader_storage_image_extended_formats")
        }
        enums::Capability::CapabilityImageQuery => None, // always supported
        enums::Capability::CapabilityDerivativeControl => None, // always supported
        enums::Capability::CapabilityInterpolationFunction => Some("sample_rate_shading"),
        enums::Capability::CapabilityTransformFeedback => panic!(), // not supported
        enums::Capability::CapabilityGeometryStreams => panic!(),   // not supported
        enums::Capability::CapabilityStorageImageReadWithoutFormat => {
            Some("shader_storage_image_read_without_format")
        }
        enums::Capability::CapabilityStorageImageWriteWithoutFormat => {
            Some("shader_storage_image_write_without_format")
        }
        enums::Capability::CapabilityMultiViewport => Some("multi_viewport"),
    }
}
