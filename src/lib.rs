use std::collections::HashMap;
use chrono::{Datelike, Duration, Local};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn output_diary()-> Vec<u8> {
    console_error_panic_hook::set_once();
    Default::default()
}

#[derive(Debug)]
pub enum Page{
    Week(chrono::NaiveDate),
    Day(chrono::NaiveDate),
}

pub fn pages(dt: chrono::NaiveDateTime)-> impl Iterator<Item=Page>{
    let date = dt.date();

    // 週の月曜日の日付を取得
    let weekday = date.weekday();
    let days_from_monday = weekday.num_days_from_monday() as i64;
    let monday = date - Duration::days(days_from_monday);

    // 月曜〜日曜の正午のNaiveDateTimeを生成
    (0..)
        .map(move |i| {
            let monday = monday + Duration::weeks(i);
            let v:[Page; 8]=core::array::from_fn(|v| if v==0{
                Page::Week(monday)
            }else{
                Page::Day(monday+Duration::weeks(i-1))
            });
            v.into_iter()
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_render;
    use typst;
    #[test]
    fn pages() {
        let v= chrono::Local::now().naive_local();
        println!("{:?}", super::pages(v).take(10).collect::<Vec<Page>>())
    }
    #[test]
    fn pdf() {
        // コード参考
        // https://github.com/sciguy16/oas2pdf/blob/main/src/typst_world.rs
        // Typstソースをメモリ上に用意
        let source = r#"
    #let name = "世界"
    Hello, *#name*!
    "#;

        // 仮想的なWorldを用意（ファイルではなくインメモリ）
        let world = typst::World::new(
            std::path::PathBuf::from("."),
            Bytes::from_static(source.as_bytes()),
        );

        // コンパイル
        let result = compile(&world);

        if let Ok(document) = result {
            // PDFとして書き出し
            let pdf_bytes = render_pdf(&document, None);
            fs::write("output.pdf", pdf_bytes).unwrap();
        } else {
            eprintln!("Compile error: {:?}", result.err());
        }
    }
}