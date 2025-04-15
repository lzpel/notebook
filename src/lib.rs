use std::collections::HashMap;
use chrono::{Datelike, Duration, Local};
use svg2pdf::ConversionOptions;
use wasm_bindgen::prelude::wasm_bindgen;
use pdf_writer::{Chunk, Content, Finish, Name, Pdf, Rect, Ref, Str};

#[wasm_bindgen]
pub fn output_diary()-> Vec<u8> {
    console_error_panic_hook::set_once();
    let replace: HashMap<String, String> = [
        ("{day}", Local::now().format("%Y/%m/%d"))
    ].iter().map(|(a,b)| (a.to_string(), b.to_string())).collect();
    let svg=include_bytes!("../svg/aday.svg");
    pdf()
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

pub fn pdf_chunk(svg: Vec<u8>, replace: HashMap<String, String>) -> (Chunk, Ref) {
    // SVGファイルの読み込み
    let tree = {
        let mut opt = usvg::Options {
            // Get file's absolute directory.
            resources_dir: None,
            ..usvg::Options::default()
        };
        opt.fontdb_mut().load_system_fonts();

        //let svg_data = std::fs::read(&args[1]).unwrap();

        // SVGファイルをテキストとして読み込み
        let mut svg_text = String::from_utf8(svg).unwrap();

        // {day}を現在の日付に置換
        for (k,v) in replace.iter(){
            svg_text = svg_text.replace(k, v);
        }
        let svg_data=svg_text.as_bytes();
        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    // PDF変換オプション
    let conv_opt = ConversionOptions::default();

    // SVGをPDFに変換
    svg2pdf::to_chunk(&tree, conv_opt).unwrap()
}
pub fn pdf()->Vec<u8>{
    let mut pdf = Pdf::new();

    // 参照IDの初期化
    let catalog_id = Ref::new(1);
    let pages_id = Ref::new(2);
    let mut page_ids = Vec::new();

    // カタログとページツリーの作成
    pdf.catalog(catalog_id).pages(pages_id);

    // ページごとの処理
    for i in 0..3 {
        let page_id = Ref::new(3 + i * 3);
        let contents_id = Ref::new(4 + i * 3);
        let resources_id = Ref::new(5 + i * 3);
        page_ids.push(page_id);

        // SVGデータの作成（例として簡単なSVGを使用）
        let svg_data = include_bytes!("../svg/aday.svg").to_vec();

        let mut replace = HashMap::new();
        replace.insert("{day}".to_string(), format!("2025-04-{}", 16 + i));

        let (chunk, _) = pdf_chunk(svg_data, replace);

        // コンテンツストリームの追加
        pdf.stream(contents_id, chunk.as_bytes());

        // リソースの追加
        pdf.resources(resources_id).extend(chunk.resources);

        // ページの追加
        pdf.page(page_id)
            .parent(pages_id)
            .media_box(Rect::new(0.0, 0.0, 595.0, 842.0))
            .contents(contents_id)
            .resources();
    }

    // ページツリーの設定
    pdf.pages(pages_id).kids(page_ids);
    pdf.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pages() {
        let v= chrono::Local::now().naive_local();
        println!("{:?}", super::pages(v).take(10).collect::<Vec<Page>>())
    }
    #[test]
    fn pdf() {
        let v= super::pdf();
        std::fs::write("output/merged.pdf", v).unwrap();
    }
}