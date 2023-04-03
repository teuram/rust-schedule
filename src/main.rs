
use std::io::prelude::*;
use calamine::{
    DataType,
    Reader,
    Xlsx,
    open_workbook
};
use curl::easy::Easy;

static LINK: &str = "https://cloud.nntc.nnov.ru/index.php/s/S5cCa8MWSfyPiqx/download";

fn download() {
    let mut easy = Easy::new();
    easy.url(LINK).unwrap();

    if std::path::Path::new("./download.xlsx").exists() {
        return;
    }

    let mut output = std::fs::File::create("download.xlsx").unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        output.write(data).unwrap();
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

fn raw(row: &[DataType]) {
    eprintln!("RAW");
    for a in row {
        println!("{}", a);
    }
    println!("");
}

fn form(row: &[DataType]) {
    for a in 0..5 {
        if !row[a * 3 + 2].is_empty() {
            let t = row[a * 3].to_string().replace(" ", "");
            let time = if let &[194u8, 160u8] = &t[..].as_bytes()[..] {
                "None".to_owned()
            } else {
                t
            };
            let r = row[a * 3 + 1].to_string();
            let room = if let &[194u8, 160u8] = &r[..].as_bytes()[..] {
                "None".to_owned()
            } else {
                r
            };
            let a = row[a * 3 + 2].to_string();
            let sp = a.split("/").collect::<Vec<_>>();

            // println!("{:?}", time.bytes());
            // println!("{:?}", room.bytes());
            if sp.len() == 2 {
                println!("{} -> {} / {}\n\t{}\n", time.trim(), room.trim(), sp[1].trim(), sp[0].trim());
            }
        }
    }
}

fn main() {
    download();

    let show_groups = ["3РПУ-20-1", "23-1МРПс"];

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();

    println!("from: {LINK}");
    println!("dumped: {time:?}");

    // https://doc.rust-lang.org/std/time/index.html
    // add speed

    let mut excel: Xlsx<_> = open_workbook("download.xlsx").unwrap();
    for i in excel.worksheets() {

        if let Some(Ok(r)) = excel.worksheet_range(i.0.as_str()) {
            let rows = r.rows();
            // dbg!(rows.len());
            for row in rows {
                if row[0].to_string().trim().contains("Расписание занятий на ") {
                    println!("\n{}", row[0]);
                }
                for group in show_groups {
                    if row[0].to_string().trim() == group {
                        println!("{}", group);
                        let r = &row[1..];
                        // dbg!(row.len());
                        if r.len() != 18 {
                            raw(&row[1..]);
                        } else {
                            form(&row[1..]);
                        }
                    }
                }
            }
        }
    }
}

