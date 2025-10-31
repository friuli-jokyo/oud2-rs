use serde::{Deserialize, Serialize};

#[test]
fn simple_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct RailwayLine {
        name: String,
        station_count: u64,
    }

    let input = "name=Yamanote\nstation_count=30\n";
    let result: RailwayLine = oud2::from_str(input).unwrap();

    assert_eq!(result.name, "Yamanote");
    assert_eq!(result.station_count, 30);
}

#[test]
fn nested_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct City {
        name: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Prefecture {
        name: String,
        prefectual_capital: City,
    }

    let input = "name=Kanagawa\nprefectual_capital.\nname=Yokohama\n.\n";
    let result: Prefecture = oud2::from_str(input).unwrap();

    assert_eq!(result.name, "Kanagawa");
    assert_eq!(result.prefectual_capital.name, "Yokohama");
}

#[test]
fn vector_primitive() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Numbers {
        numbers: Vec<u64>,
    }

    let input = "numbers=1\nnumbers=2\nnumbers=3\nnumbers=4\nnumbers=5\n";
    let result: Numbers = oud2::from_str(input).unwrap();

    assert_eq!(result.numbers, vec![1, 2, 3, 4, 5]);
}

#[test]
fn vector_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Station {
        name: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct RailwayLine {
        name: String,
        stations: Vec<Station>,
    }

    let input = "name=Yamanote\nstations.\nname=Tokyo\n.\nstations.\nname=Shinagawa\n.\nstations.\nname=Shibuya\n.\n";
    let result: RailwayLine = oud2::from_str(input).unwrap();

    assert_eq!(result.name, "Yamanote");
    assert_eq!(result.stations.len(), 3);
    assert_eq!(result.stations[0].name, "Tokyo");
    assert_eq!(result.stations[1].name, "Shinagawa");
    assert_eq!(result.stations[2].name, "Shibuya");
}

#[test]
fn round_trip_simple() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let original = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Person = oud2::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn round_trip_nested() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Address {
        street: String,
        city: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        address: Address,
    }

    let original = Person {
        name: "Bob".to_string(),
        address: Address {
            street: "Main St".to_string(),
            city: "Tokyo".to_string(),
        },
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Person = oud2::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn round_trip_vector() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Data {
        values: Vec<i32>,
    }

    let original = Data {
        values: vec![1, -2, 3, -4, 5],
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Data = oud2::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn various_numeric_types() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Numbers {
        u8_val: u8,
        u16_val: u16,
        u32_val: u32,
        u64_val: u64,
        i8_val: i8,
        i16_val: i16,
        i32_val: i32,
        i64_val: i64,
        f32_val: f32,
        f64_val: f64,
    }

    let original = Numbers {
        u8_val: 255,
        u16_val: 65535,
        u32_val: 4294967295,
        u64_val: 18446744073709551615,
        i8_val: -128,
        i16_val: -32768,
        i32_val: -2147483648,
        i64_val: -9223372036854775808,
        f32_val: 3.14,
        f64_val: 2.71828,
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Numbers = oud2::from_str(&serialized).unwrap();

    assert_eq!(deserialized.u8_val, original.u8_val);
    assert_eq!(deserialized.u16_val, original.u16_val);
    assert_eq!(deserialized.u32_val, original.u32_val);
    assert_eq!(deserialized.u64_val, original.u64_val);
    assert_eq!(deserialized.i8_val, original.i8_val);
    assert_eq!(deserialized.i16_val, original.i16_val);
    assert_eq!(deserialized.i32_val, original.i32_val);
    assert_eq!(deserialized.i64_val, original.i64_val);
    assert!((deserialized.f32_val - original.f32_val).abs() < 0.0001);
    assert!((deserialized.f64_val - original.f64_val).abs() < 0.0001);
}

#[test]
fn bool_values() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Flags {
        enabled: bool,
        disabled: bool,
    }

    let original = Flags {
        enabled: true,
        disabled: false,
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Flags = oud2::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn escaped_strings() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Text {
        content: String,
    }

    let original = Text {
        content: "Hello\\World\nNew Line".to_string(),
    };

    let serialized = oud2::to_string(&original).unwrap();
    let deserialized: Text = oud2::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}
