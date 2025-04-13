use std::collections::HashMap;
use chrono::Local;
use svg2pdf::ConversionOptions;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn output_diary()-> Vec<u8> {
    console_error_panic_hook::set_once();
    let replace: HashMap<String, String> = [
        ("{day}", Local::now().format("%Y/%m/%d"))
    ].iter().map(|(a,b)| (a.to_string(), b.to_string())).collect();
    let svg=include_bytes!("../svg/aday.svg");
    pdf(svg.to_vec(), replace)
}

pub fn pdf(svg: Vec<u8>, replace: HashMap<String, String>)->Vec<u8>{
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
    let pdf_data = svg2pdf::to_pdf(&tree, conv_opt, Default::default()).unwrap();
    pdf_data
}