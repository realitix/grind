use std::collections::HashMap;
use std::mem;
use std::vec::Vec;

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct TranspilationResult {
    pub text: String,
    // attributes: name: location
    pub attributes: HashMap<String, u32>,
    // uniforms: name: set/binding
    pub uniforms: HashMap<String, [u32; 2]>,
    // varyings: name: location
    pub varyings: HashMap<String, u32>,
}

fn get_version(lines: &Vec<&str>) -> u32 {
    let mut version = 0;

    // First, try with #version
    let first_line: Vec<&str> = lines[0].split(" ").collect();
    if first_line[0] == "#version" {
        version = first_line[1].parse::<u32>().unwrap();
    }

    // then with keyword
    if version == 0 {
        for line in lines.iter() {
            if line.find("attribute").is_some() || line.find("uniform").is_some()
                || line.find("varying").is_some()
                || line.find("gl_FragColor").is_some()
            {
                version = 120;
                break;
            }
        }
    }

    version
}

fn next_attribute_location(attributes: &HashMap<String, u32>) -> u32 {
    let mut current_location = None;
    let mut max_location = 0;

    // TODO: Take into account the location length
    // example: a matrix 4 consumes 4 locations
    for (key, val) in attributes.iter() {
        if *val >= max_location {
            current_location = Some(*val);
            max_location = *val;
        }
    }

    match current_location {
        Some(l) => l + 1,
        None => 0,
    }
}

fn transpile120(lines: &Vec<&str>, shader_type: ShaderType) -> TranspilationResult {
    let mut result = String::from("#version 450\n");
    result.push_str("#extension GL_ARB_separate_shader_objects :enable\n");

    match shader_type {
        ShaderType::Vertex => result.push_str("out gl_PerVertex {vec4 gl_Position;};\n"),
        ShaderType::Fragment => result.push_str("layout(location = 0) out vec4 out_color;\n"),
        _ => panic!("Unknow shader type"),
    };

    let mut attributes = HashMap::new();

    for line in lines.iter() {
        if line.find("#version").is_some() {
            continue;
        } else if line.find("attribute").is_some() {
            let location = next_attribute_location(&attributes);
            let tokens: Vec<&str> = line.split(" ").collect();
            attributes.insert(tokens[2].to_string(), location);
            let s = format!("layout(location={}) in", location);
            result.push_str(&line.replace("attribute", &s));
        } else if line.find("varying").is_some() {
            // TODO location autogenerate
            match shader_type {
                ShaderType::Vertex => {
                    let location = 0;
                    let s = format!("layout(location={}) out", location);
                    result.push_str(&line.replace("varying", &s));
                }
                ShaderType::Fragment => {
                    let location = 0;
                    let s = format!("layout(location={}) in", location);
                    result.push_str(&line.replace("varying", &s));
                }
            }
        } else if line.find("gl_FragColor").is_some() {
            result.push_str(&line.replace("gl_FragColor", "out_color"));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    TranspilationResult {
        text: result,
        uniforms: HashMap::new(),
        attributes: HashMap::new(),
        varyings: HashMap::new(),
    }
}

fn transpile100(lines: &Vec<&str>, shader_type: ShaderType) -> TranspilationResult {
    //TODO: remove precision qualifier
    transpile120(lines, shader_type)
}

pub fn transpile(code: &str, shader_type: ShaderType) -> TranspilationResult {
    let lines = code.lines().collect();
    let version = get_version(&lines);

    match version {
        100 => transpile100(&lines, shader_type),
        120 => transpile120(&lines, shader_type),
        _ => panic!("Can't determine shader version"),
    }
}
