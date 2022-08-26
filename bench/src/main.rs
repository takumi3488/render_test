use indicatif::ProgressBar;
use reqwest;
use std::{fs::File, io::Write};
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    // Railsが起動しているかチェック
    loop {
        sleep(Duration::from_micros(1)).await;
        if is_index("").await {
            break;
        }
    }

    // 計測パラメータの設定
    let times: usize = 5000;
    let mut sym_nanos = vec![];
    let mut str_nanos = vec![];

    // 計測
    println!("計測を開始");
    let bar = ProgressBar::new(times as u64 * 2);
    for _ in 1..=times {
        let sym_nano = bench("render_test/sym").await;
        sym_nanos.push(sym_nano);
        bar.inc(1);
        let str_nano = bench("render_test/str").await;
        str_nanos.push(str_nano);
        bar.inc(1);
    }
    bar.finish();

    // 結果の整形
    sym_nanos.sort_unstable();
    str_nanos.sort_unstable();
    let sym_micros: Vec<f64> = sym_nanos.iter().map(|&x| x as f64 / 1e3).collect();
    let str_micros: Vec<f64> = str_nanos.iter().map(|&x| x as f64 / 1e3).collect();
    let sym_avg = sym_micros.iter().sum::<f64>() / times as f64;
    let str_avg = str_micros.iter().sum::<f64>() / times as f64;

    // 結果の出力
    let mut res = String::from("");
    res += &format!(",シンボル[μs],文字列[μs],比(シンボルを100とする)\n");
    res += &format!(
        "誤差込み平均値,{:.2}±{:.2},{:.2}±{:.2},{:.2}\n",
        sym_avg,
        sd(&sym_micros, sym_avg),
        str_avg,
        sd(&str_micros, str_avg),
        100.0 * str_avg / sym_avg
    );
    res += &format!(
        "中央値,{:.2},{:.2},{:.2}\n",
        &sym_micros[times / 2],
        &str_micros[times / 2],
        100.0 * &sym_micros[times / 2] / &str_micros[times / 2]
    );
    let mut file = File::create("result.csv").unwrap();
    write!(file, "{}", res).unwrap();
}

fn sd(v: &Vec<f64>, avg: f64) -> f64 {
    let mut s = 0.0;
    for &v in v {
        s += (avg - v).powf(2.0);
    }
    (s / v.len() as f64).sqrt()
}

async fn bench(path: &str) -> usize {
    let start = Instant::now();
    let res = is_index(path).await;
    if !res {
        panic!("renderに失敗しました")
    }
    let end = start.elapsed();
    return end.as_nanos() as usize;
}

fn url(path: &str) -> String {
    let base_url = "http://localhost:3000/foos";
    return format!("{}/{}", base_url, path);
}

async fn is_index(path: &str) -> bool {
    let res = reqwest::get(url(path)).await;
    if res.is_err() {
        return false;
    }
    let text = res.unwrap().text().await;
    if text.is_err() {
        return false;
    }
    text.unwrap().contains("Foos#index")
}
