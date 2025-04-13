use std::fs::File;
use std::io::Write;
use svg2pdf::ConversionOptions;
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage:\n\tminimal <in-svg> <out-png>");
        println!("{args:?}");
        return Ok(());
    }
    // SVGファイルの読み込み
    let tree = {
        let mut opt = usvg::Options {
            // Get file's absolute directory.
            resources_dir: std::fs::canonicalize(&args[1])
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..usvg::Options::default()
        };
        opt.fontdb_mut().load_system_fonts();

        //let svg_data = std::fs::read(&args[1]).unwrap();

        // SVGファイルをテキストとして読み込み
        let mut svg_text = std::fs::read_to_string(&args[1])?;

        // 現在の日付を取得（例：2025-04-14）
        let current_date = Local::now().format("%Y/%m/%d").to_string();

        // {day}を現在の日付に置換
        svg_text = svg_text.replace("{day}", &current_date);
        let svg_data=svg_text.as_bytes();
        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    // PDF変換オプション
    let conv_opt = ConversionOptions::default();

    // SVGをPDFに変換
    let pdf_data = svg2pdf::to_pdf(&tree, conv_opt, Default::default()).unwrap();

    // PDFをファイルに保存
    let mut file = File::create(&args[2])?;
    file.write_all(&pdf_data)?;

    Ok(())
}