
use std::io::prelude::*;
use calamine::{Reader, Xlsx, open_workbook};
use curl::easy::Easy;

fn download() {
    // let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://cloud.nntc.nnov.ru/index.php/s/S5cCa8MWSfyPiqx/download").unwrap();

    if std::path::Path::new("./download.xlsx").exists() {
        return;
    }

    let mut output = std::fs::File::create("download.xlsx").unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        // dst.extend_from_slice(data);
        output.write(data).unwrap();
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

fn main() {
    download();
    let mut excel: Xlsx<_> = open_workbook("download.xlsx").unwrap();
    for i in excel.worksheets() {
        // println!("{}", i.0);

        if let Some(Ok(r)) = excel.worksheet_range(i.0.as_str()) {
            for row in r.rows() {
                // println!("row={:?}, row[0]={:?}", row, row[0]);
                if row[0].to_string().trim() == "3РПУ-20-1" {
                    for a in row {
                        println!("{}", a);
                    }
                }
                if row[0].to_string().trim().contains("Расписание занятий") {
                    println!("{}", row[0]);
                }
            }
        }
    }
}

