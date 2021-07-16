use std::env;
use std::fs;
use std::vec::Vec;

use json::JsonValue;

fn main() {
    let args: Vec<String> = env::args().collect();
    begin(read_json(&args[1]));
}

fn begin(json: JsonValue) {
    assert_format(&json);
    let expected = get_selling_total(&json);
    let mat_total = calculate_total_material_cost(&json);

    print!("Calculating for: \t{}\n", json["name"]);
    print!("Number of Crafts: \t{}\n", json["num_crafts_wanted"]);
    print!("Produced Per Craft: \t{}\n", json["qty_per_craft"]);
    print!("Labor cost: \t\t{}\n\n", get_total_labor_cost(&json));
    print!("Expected selling total: \t{}\n", f64_to_gsc(&expected));
    print!("Cost of materials in total: \t{}\n", f64_to_gsc(&mat_total));
    print!("Expected Profit: \t\t{}\n", f64_to_gsc(&(expected - mat_total)));
    print!("{}\n", get_individual_material_qty_and_cost_string(&json));
}

fn calculate_total_material_cost(json: &JsonValue) -> f64 {
    let mut total: f64 = 0.0;
    for i in 0..json["materials_per_craft"].len() {
        let qty = json["materials_per_craft"][i]["qty"].as_usize().unwrap();
        let per = json["materials_per_craft"][i]["price_per"].as_f64().unwrap();
        total += per * qty as f64;
    }
    let num_crafts_wanted = json["num_crafts_wanted"].as_usize().unwrap();
    total * num_crafts_wanted as f64
}

fn get_total_labor_cost(json: &JsonValue) -> usize {
    let labor_per_craft: usize = json["cost_of_labor_per_craft"].as_usize().unwrap();
    let crafts: usize = json["num_crafts_wanted"].as_usize().unwrap();
    crafts * labor_per_craft
}

fn get_selling_total(json: &JsonValue) -> f64 {
    let single_price: f64 = json["price_of_single_when_sold"].as_f64().unwrap();
    let qty_per_craft: usize = json["qty_per_craft"].as_usize().unwrap();
    let crafts: usize = json["num_crafts_wanted"].as_usize().unwrap();
    let total_units = qty_per_craft * crafts;
    (single_price * total_units as f64)
}

//Converts a decimal value to gold, silver, copper. Example: 1.0345 = 1g 3s 45c.
fn f64_to_gsc(value: &f64) -> String {
    let str_value = value.to_string();
    let mut gsc = String::new();
    let split: Vec<&str> = str_value.split(".").collect();
    gsc.push_str(&format!("{}g ", split[0]));

    if split.len() == 1 {
        gsc.push_str("0s 0c");
        return gsc;
    }

    match split[1].len() {
        1 => gsc.push_str(&format!("{}0s 0c", &split[1][..])),
        2 => gsc.push_str(&format!("{}s 0c", &split[1][..])),
        3 => gsc.push_str(&format!("{}s {}0c", &split[1][..2], &split[1][2..])),
        _ => {
            gsc.push_str(&format!("{}s ", &split[1][..2]));
            gsc.push_str(&format!("{}c", &split[1][2..4]));
        },
    }
    gsc
}

fn get_individual_material_qty_and_cost_string(json: &JsonValue) -> String {
    let result = String::new();
    let align = longest_name_len(json);
    for i in 0..json["materials_per_craft"].len() {
        let name = json["materials_per_craft"][i]["mat_name"].as_str().unwrap();
        let qty = json["materials_per_craft"][i]["qty"].as_usize().unwrap();
        let price_per =  json["materials_per_craft"][i]["price_per"].as_f64().unwrap();
        let num_crafts =  json["num_crafts_wanted"].as_usize().unwrap();
        let total = price_per * qty as f64 * num_crafts as f64;
        print!("\t{:align$} :: Qty: {:6} :: Price Per Unit: {:10} :: Total: {:30}\n", name, qty, f64_to_gsc(&price_per), f64_to_gsc(&total), align = align);
    }
    result
}

fn longest_name_len(json: &JsonValue) -> usize {
    let mut len = 0;
    for i in 0..json["materials_per_craft"].len() {
        let name = json["materials_per_craft"][i]["mat_name"].as_str().unwrap();
        if name.len() > len {
            len = name.len();
        }
    }
    len
}

fn assert_format(json: &JsonValue) {
    let fields = [
    "name",
    "qty_per_craft",
    "num_crafts_wanted",
    "price_of_single_when_sold",
    "cost_of_labor_per_craft",
    "materials_per_craft"
    ];

    for f in fields {
        assert!(!json[f].is_null(), "Field is missing from json: {}\n", f);
    }

    assert_material_format(json);
}

fn assert_material_format(json: &JsonValue) {
    let mat_fields = [
    "mat_name",
    "qty",
    "price_per"
    ];

    assert!(json["materials_per_craft"].len() > 0, "Empty materials list. Crafting recipes require at least 1 material.\n");

    for i in 0..json["materials_per_craft"].len() {
        for f in mat_fields {
            assert!(!json["materials_per_craft"][i][f].is_null(), "Field is missing from material index {}. Field: {}\n", i, f);
        }
    }
}

fn read_json(path: &str) -> JsonValue {
    json::parse(&fs::read_to_string(path)
     .expect(&format!("File not found. {}", path)))
     .unwrap()
}

static TEST_JSON: &str = r#"
{
    "name": "test",
    "qty_per_craft": 1,
    "num_crafts_wanted": 1,
    "cost_of_labor_per_craft": 5,
    "price_of_single_when_sold": 1.1,
    "materials_per_craft": [
        {"mat_name": "mat1", "qty": 1, "price_per": 1.0},
        {"mat_name": "mat2", "qty": 1, "price_per": 2.0},
        {"mat_name": "mat3", "qty": 1, "price_per": 3.0}
    ]
}
"#;

static TEST_JSON_2: &str = r#"
{
    "name": "test_2",
    "qty_per_craft": 10,
    "num_crafts_wanted": 50,
    "cost_of_labor_per_craft": 20,
    "price_of_single_when_sold": 1.1999,
    "materials_per_craft": [
        {"mat_name": "powder", "qty": 2000, "price_per": 0.04},
        {"mat_name": "ammo", "qty": 20, "price_per": 2.0},
        {"mat_name": "aaaaaaaaaaaaaaaaaaaaalongformatcrap", "qty": 999, "price_per": 0.0012}
    ]
}
"#;

#[test]
fn test_json_parse() {
    json::parse(&TEST_JSON).unwrap();
    json::parse(&TEST_JSON_2).unwrap();
}

#[test]
fn test_f64_to_gsc() {
    let test = 1.0345f64;
    let test = f64_to_gsc(&test);
    assert!(test == "1g 03s 45c", "t1 actual: {}", test);

    let test = -1.0345f64;
    let test = f64_to_gsc(&test);
    assert!(test == "-1g 03s 45c", "t2 actual: {}", test);

    let test = 1.1f64;
    let test = f64_to_gsc(&test);
    assert!(test == "1g 10s 0c", "t3 actual: {}", test);

    let test = -1.1f64;
    let test = f64_to_gsc(&test);
    assert!(test == "-1g 10s 0c", "t4 actual: {}", test);

    let test = 1.03f64;
    let test = f64_to_gsc(&test);
    assert!(test == "1g 03s 0c", "t5 actual: {}", test);

    let test = -1.03f64;
    let test = f64_to_gsc(&test);
    assert!(test == "-1g 03s 0c", "t6 actual: {}", test);

    let test = 1.0305f64;
    let test = f64_to_gsc(&test);
    assert!(test == "1g 03s 05c", "t7 actual: {}", test);

    let test = -1.0305f64;
    let test = f64_to_gsc(&test);
    assert!(test == "-1g 03s 05c", "t8 actual: {}", test);

    let test = 1.035f64;
    let test = f64_to_gsc(&test);
    assert!(test == "1g 03s 50c", "t9 actual: {}", test);

    let test = -1.035f64;
    let test = f64_to_gsc(&test);
    assert!(test == "-1g 03s 50c", "t10 actual: {}", test);
}

#[test]
fn test_caclulate_total_material_cost() {
    let total = calculate_total_material_cost(&json::parse(&TEST_JSON).unwrap());
    assert!(total == 6.0, "actual: {}", total);

    let total = calculate_total_material_cost(&json::parse(&TEST_JSON_2).unwrap());
    assert!(total == 121.1988, "actual: {}", total);
}

#[test]
fn test_total_labor_cost() {
    let labor = get_total_labor_cost(&json::parse(&TEST_JSON).unwrap());
    assert!(labor == 5, "actual: {}", labor);

    let labor = get_total_labor_cost(&json::parse(&TEST_JSON_2).unwrap());
    assert!(labor == 1000, "actual: {}", labor);
}

#[test]
fn test_begin() {
    //begin(json::parse(&TEST_JSON).unwrap());
    begin(json::parse(&TEST_JSON_2).unwrap());
}
