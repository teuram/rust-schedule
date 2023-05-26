
use std::io::BufReader;
use calamine::{
    DataType,
    Reader,
    Rows,
    Xlsx,
    open_workbook_from_rs,
};
use curl::easy::Easy;
use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(value_name = "GROUPS")]
    groups: Vec<String>,
}

static LINK: &str = "https://cloud.nntc.nnov.ru/index.php/s/S5cCa8MWSfyPiqx/download";

fn get_table() -> Vec<u8> {
    let mut easy = Easy::new();
    easy.url(LINK).unwrap();
    let mut dst = Vec::new();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    dst
}

fn raw(row: &[DataType]) {
    eprintln!("warning: table parsing is not possible, emit raw format");
    for a in row {
        println!("{}", a);
    }
    println!("");
}

fn trim(text: String) -> String {
    if let &[194u8, 160u8] = &text[..].as_bytes()[..] {
        "None".to_owned()
    } else {
        text
    }
}

fn form(row: &[DataType]) {
    for a in 0..(row.len() / 3) {
        if !row[a * 3 + 2].is_empty() {
            let (lesson, teather) = {
                let lesson_raw = row[a * 3 + 2].to_string();
                let lesson_raw = lesson_raw.split("/").collect::<Vec<_>>();

                if lesson_raw.len() != 2 {
                    // ("".to_owned(), "".to_owned())
                    continue;
                } else {
                    (lesson_raw[0].trim().to_owned(),
                    lesson_raw[1].trim().to_owned())
                }
            };

            let time = {
                let time_raw = row[a * 3].to_string().replace(" ", "");
                trim(time_raw).trim().to_owned()
            };

            let room = {
                let room_raw = row[a * 3 + 1].to_string();
                trim(room_raw).trim().to_owned()
            };


            println!("[{}] -> {time} -> {room} / {teather}\n\t{lesson}\n", a + 1);
        }
    }
}

fn show_list_groups(rows: Rows<DataType>) {
    let mut groups = std::collections::HashMap::<String, u8>::new();
    for row in rows {
        let s = row[0].to_string();
        if s.chars().count() < 16 {
            if s.chars().count() > 2 {
                if s != "Группа" {
                    groups.insert(row[0].to_string(), 0);
                }
            }
        }
    }
    for g in groups.into_keys() {
        println!("{}", g);
    }
}

fn show_schedule(rows: Rows<DataType>, groups: &Vec<String>) {
    for row in rows {
        if row[0].to_string().trim().contains("Расписание занятий на") {
            println!("\n{}", row[0]);
        }
        for group in groups.iter() {
            if row[0].to_string().trim() == group.as_str() {
                println!("[{}]", row[0]);
                let r = &row[1..];
                if r.len() % 3 != 0 {
                    raw(&row[1..]);
                } else {
                    form(&row[1..]);
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let down = std::io::Cursor::new(get_table());
    let reader = BufReader::new(down);

    let mut excel: Xlsx<_> = open_workbook_from_rs(reader).unwrap();
    for i in excel.worksheets() {
        if let Some(Ok(r)) = excel.worksheet_range(i.0.as_str()) {
            let rows = r.rows();
            // dbg!(rows.len());
            if args.groups.is_empty() {
                show_list_groups(rows);
            } else {
                show_schedule(rows, &args.groups);
            }
        }
    }
}

