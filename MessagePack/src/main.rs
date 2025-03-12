use core::f64;

use serde::{Deserialize, Serialize};
use serde_reflection::{ContainerFormat, Format, Registry, Samples, Tracer, TracerConfig};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Test {
    a: i32,
    b: i32,
    c: String,
    time: i64,
    distance: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeSmallPackage {
    val: f64,
    valid: bool,
    time: i32,
    dist: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeBigVecs {
    times: Vec<i32>,
    dists: Vec<i32>,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeSmallArray {
    times: [i32; 10],
    dists: [i32; 10],
    data: [u8; 10],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeFloats {
    floats: [f64; 32],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeFloatVecs {
    floats: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SomeSingleFloat {
    float: f64,
    float2: f32,
}

fn main() {
    let mut tracer = Tracer::new(TracerConfig::default());
    tracer.trace_type::<Test>(&Samples::default()).unwrap();
    let registry = tracer.registry().unwrap();
    let str = serde_json::to_string(&registry).unwrap();
    println!("\n###### SERDE_REFLECTION: \n");
    println!("{}", str);
    let proto = generate_protobuf(&registry);
    println!("\n###### PROTOBUF: \n");
    println!("{}", proto);
    let ros2_msg = generate_ros2_msg(&registry);
    println!("\n###### ROS2 MSG: \n");
    println!("{}", ros2_msg);
    println!("\n###### THRIFT: \n");
    let thrift = generate_thrift(&registry);
    println!("{}", thrift);
    println!("\n###### FLATBUFFERS: \n");
    let flatbuffers = generate_flatbuffers(&registry);
    println!("{}", flatbuffers);
}

fn generate_thrift(registry: &Registry) -> String {
    let mut thrift_def = String::new();

    for (name, container) in registry {
        if let ContainerFormat::Struct(struct_format) = container {
            thrift_def.push_str(&format!("struct {} {{\n", name));

            let mut field_index = 1;
            for t in struct_format {
                let field_name = t.name.clone();
                let field_type_str = match t.value {
                    Format::U8 | Format::U16 | Format::U32 => "i32",
                    Format::I8 | Format::I16 | Format::I32 => "i32",
                    Format::I64 => "i64",
                    Format::U64 => "i64", // Thrift does not have a uint64, using i64
                    Format::F32 => "float",
                    Format::F64 => "double",
                    Format::Bool => "bool",
                    Format::Str => "string",
                    _ => "binary", // Default to binary for unknown/complex types
                };

                thrift_def.push_str(&format!(
                    "  {} {} = {};\n",
                    field_type_str, field_name, field_index
                ));
                field_index += 1;
            }
            thrift_def.push_str("}\n\n");
        }
    }

    thrift_def
}

fn generate_flatbuffers(registry: &Registry) -> String {
    let mut flatbuffers_def = String::new();

    for (name, container) in registry {
        if let ContainerFormat::Struct(struct_format) = container {
            flatbuffers_def.push_str(&format!("table {} {{\n", name));

            for t in struct_format {
                let field_name = t.name.clone();
                let field_type_str = match t.value {
                    Format::U8 | Format::U16 | Format::U32 => "int",
                    Format::I8 | Format::I16 | Format::I32 => "int",
                    Format::I64 => "long",
                    Format::U64 => "ulong", // FlatBuffers uses unsigned long for 64-bit unsigned
                    Format::F32 => "float",
                    Format::F64 => "double",
                    Format::Bool => "bool",
                    Format::Str => "string",
                    _ => "ubyte", // Default to ubyte for unknown/complex types
                };

                flatbuffers_def.push_str(&format!("  {}: {};\n", field_name, field_type_str));
            }

            flatbuffers_def.push_str("}\n\n");
            flatbuffers_def.push_str(&("\nroot_type ".to_owned() + name + ";\n"));
        }
    }

    flatbuffers_def
}

fn generate_protobuf(registry: &Registry) -> String {
    let mut proto_def = String::new();
    proto_def.push_str("syntax = \"proto3\";\n\n");

    for (name, container) in registry {
        if let ContainerFormat::Struct(struct_format) = container {
            proto_def.push_str(&format!("message {} {{\n", name));

            let mut field_index = 1;
            for t in struct_format {
                let field_name = t.name.clone();
                let field_type_str = match t.value {
                    serde_reflection::Format::U8
                    | serde_reflection::Format::U16
                    | serde_reflection::Format::U32 => "uint32",
                    serde_reflection::Format::I8
                    | serde_reflection::Format::I16
                    | serde_reflection::Format::I32 => "int32",
                    serde_reflection::Format::I64 => "int64",
                    serde_reflection::Format::U64 => "uint64",
                    serde_reflection::Format::F32 => "float",
                    serde_reflection::Format::F64 => "double",
                    serde_reflection::Format::Bool => "bool",
                    serde_reflection::Format::Str => "string",
                    _ => "bytes", // Default to bytes for unknown/complex types
                };

                proto_def.push_str(&format!(
                    "  {} {} = {};\n",
                    field_type_str, field_name, field_index
                ));
                field_index += 1;
            }
            proto_def.push_str("}\n\n");
        }
    }
    proto_def
}
fn rust_to_ros_type(format: &Format) -> String {
    match format {
        Format::Bool => "bool".to_string(),
        Format::I8 => "int8".to_string(),
        Format::I16 => "int16".to_string(),
        Format::I32 => "int32".to_string(),
        Format::I64 => "int64".to_string(),
        Format::U8 => "uint8".to_string(),
        Format::U16 => "uint16".to_string(),
        Format::U32 => "uint32".to_string(),
        Format::U64 => "uint64".to_string(),
        Format::F32 => "float32".to_string(),
        Format::F64 => "float64".to_string(),
        Format::Str => "string".to_string(),
        Format::Seq(inner) => format!("{}[]", rust_to_ros_type(inner)), // Convert Vec<T> to T[]
        _ => "UNKNOWN_TYPE".to_string(),
    }
}

fn generate_ros2_msg(registry: &Registry) -> String {
    let mut msg_def = String::new();

    for (_name, container) in registry {
        if let ContainerFormat::Struct(struct_format) = container {
            for t in struct_format {
                let field_name = t.name.clone();
                let field_type = &t.value;
                let ros_type = rust_to_ros_type(field_type);
                msg_def.push_str(&format!("{} {}\n", ros_type, field_name));
            }
        }
    }

    msg_def
}
#[cfg(test)]
mod test {
    use std::f32;

    use super::*;

    #[test]
    fn test() {
        let test = Test {
            a: 10,
            b: 20,
            c: "Hello".to_string(),
            time: 1000,
            distance: 100.0,
        };
        let gg = rmp_serde::to_vec(&test).unwrap();
        let test2: Test = rmp_serde::from_slice(&gg).unwrap();

        assert_eq!(test, test2);
    }

    #[test]
    fn test_small_package() {
        let test = SomeSmallPackage {
            val: 10.0,   // 8 bytes
            valid: true, // 1 byte
            time: 1000,  // 4 bytes
            dist: 100,   // 4 bytes
                         // 8+1+4+4 = 17 bytes
        };
        let bytes = rmp_serde::to_vec(&test).unwrap();
        let length = bytes.len();
        let expected = 15; // actually only 15 bytes, pretty cool right? Optimize on integers.

        assert_eq!(length, expected);
    }

    #[test]
    fn test_arrays() {
        let arrysstruct = SomeSmallArray {
            times: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], // 10 * 4 = 40 bytes
            dists: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100], // 10 * 4 = 40 bytes
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],  // 10 * 1 = 10 bytes
                                                    // 40 + 40 + 10 = 90 bytes - Should be.
        };
        let bytes = rmp_serde::to_vec(&arrysstruct).unwrap();
        let length = bytes.len();
        let expected = 34;
        // Dayum, that's pretty good.
        // If we have small intgers, we can save a lot of space, but when we have larger integers
        // we need to use the variable length integers which will take up more space.

        assert_eq!(length, expected);
    }

    #[test]
    fn test_large_vecs() {
        // Make vecs of 10000 random elements
        let vecsstruct = SomeBigVecs {
            times: (0..10000).collect(), // 10000 * 4 = 40000 bytes
            dists: (0..10000).collect(), // 10000 * 4 = 40000 bytes
            data: (0..10000).map(|x| x as u8).collect(), // 10000 * 1 = 10000 bytes
                                         // 40000 + 40000 + 10000 = 90000 bytes - Should be.
        };
        let bytes = rmp_serde::to_vec(&vecsstruct).unwrap();
        let length = bytes.len();
        let expected = 74234; // Somewhat okay, but still pretty damn good.

        assert_eq!(length, expected);
    }

    #[test]
    fn test_floats() {
        let floats = SomeFloats {
            floats: [
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0,
                30.0, 31.0, 32.0,
            ], // 32 * 8 = 256 bytes
        };
        let bytes = rmp_serde::to_vec(&floats).unwrap();
        let length = bytes.len();
        let expected = 292; // Pretty bad ? why...

        assert_eq!(length, expected);
    }

    #[test]
    fn test_floatsvec() {
        // Serialize arrays are treated as vectors such that it needs some extra stuff like length
        // and other things probably.
        let floats = SomeFloatVecs {
            floats: (0..32).map(|x| x as f64).collect(),
        };
        let bytes = rmp_serde::to_vec(&floats).unwrap();
        let length = bytes.len();
        let expected = 292;

        assert_eq!(length, expected);
    }

    #[test]
    fn test_fl() {
        let floats = SomeSingleFloat {
            float: f64::MAX, // 8 bytes
            float2: f32::MIN, // 4 bytes
                             // 8 + 4 = 12 bytes
        };
        let bytes = rmp_serde::to_vec(&floats).unwrap();
        let length = bytes.len();
        let expected = 15; // Not bad. seems to only have 3 bytes overhead.

        assert_eq!(length, expected);
    }
}
