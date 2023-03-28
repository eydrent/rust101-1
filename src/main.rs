use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "./src/assets/history.csv";
const FIRST_TAG: &str = "INICIO";

// TYPE, TAG, TEXT, LIVE
// Data structure for the history.csv file
#[derive(Debug)]
struct HistoryS {
    data_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<HistoryS>,
}

// generator function
impl HistoryS {
    fn new(row: StringRecord) -> HistoryS {
        let life: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);

        return HistoryS {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
            options: Vec::new(),
        };
    }
}

fn main() {
    let mut life: i32 = 100;
    let mut actual_tag = FIRST_TAG;
    let mut last_record = "".to_string();

    let mut history_data: HashMap<String, HistoryS> = HashMap::new();

    let contents = fs::read_to_string(FILENAME).unwrap();
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(contents.as_bytes());

    for result in reader.records() {
        let result = result.unwrap();
        let data = HistoryS::new(result);

        if data.data_type == "SITUACION" {
            let record_tag = data.tag.clone();
            history_data.insert(record_tag.clone(), data);
            last_record = record_tag;
        } else if data.data_type == "OPCION" {
            if let Some(data) = history_data.get_mut(&last_record) {
                (*data).options.push(data);
            }
        }
    }

    loop {
        println!("Tienes {} puntos de vida", life);

        if let Some(data) = history_data.get(actual_tag) {
            println!("{}", data.text);

            for (index, option) in data.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            selection:i32 = selection.trim().parse().unwrap_or(0);

            if let Some(selected_option) = &data.options.get(selection) {
                actual_tag = &selected_option.tag;
            } else {
                println!("Opción no válida");
            }

            life += data.life;
            println!("");
        } else {
            println!("No se ha encontrado el tag {}", actual_tag);
            break;
        }

        // if life is 0 or less, the game ends
        if life <= 0 {
            println!("Has muerto");
            break;
        }
    }

    println!("{:?}", history_data["DERECHA"])
}
