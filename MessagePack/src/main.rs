use core::f64;

use serde::{Deserialize, Serialize};

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
    println!("Hello, world!");
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
