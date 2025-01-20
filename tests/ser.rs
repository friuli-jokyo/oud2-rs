use serde::Serialize;

#[test]
fn simple_struct() {
    #[derive(Serialize)]
    struct RailwayLine {
        name: String,
        station_count: u64,
    }

    let yamanote = RailwayLine {
        name: "Yamanote".to_string(),
        station_count: 30,
    };

    let s = oud2::to_string(&yamanote).unwrap();
    assert_eq!(s, "name=Yamanote\nstation_count=30\n");
}

#[test]
fn nested_struct() {
    #[derive(Serialize)]
    struct City {
        name: String,
    }

    #[derive(Serialize)]
    struct Prefecture {
        name: String,
        prefectual_capital: City,
    }

    let kanagawa = Prefecture {
        name: "Kanagawa".to_string(),
        prefectual_capital: City {
            name: "Yokohama".to_string(),
        },
    };

    let s = oud2::to_string(&kanagawa).unwrap();
    assert_eq!(s, "name=Kanagawa\nprefectual_capital.\nname=Yokohama\n.\n");
}

#[test]
fn vector_primitive() {
    #[derive(Serialize)]
    struct Numbers {
        numbers: Vec<u64>,
    }
    let v = Numbers {
        numbers: vec![1, 2, 3, 4, 5],
    };
    let s = oud2::to_string(&v).unwrap();
    assert_eq!(s, "numbers=1\nnumbers=2\nnumbers=3\nnumbers=4\nnumbers=5\n");
}

#[test]
fn vector_struct() {
    #[derive(Serialize)]
    struct Station {
        name: String,
    }

    #[derive(Serialize)]
    struct RailwayLine {
        name: String,
        stations: Vec<Station>,
    }

    let yamanote = RailwayLine {
        name: "Yamanote".to_string(),
        stations: vec![
            Station {
                name: "Tokyo".to_string(),
            },
            Station {
                name: "Shinagawa".to_string(),
            },
            Station {
                name: "Shibuya".to_string(),
            },
        ],
    };

    let s = oud2::to_string(&yamanote).unwrap();
    assert_eq!(
        s,
        "name=Yamanote\nstations.\nname=Tokyo\n.\nstations.\nname=Shinagawa\n.\nstations.\nname=Shibuya\n.\n"
    );
}
