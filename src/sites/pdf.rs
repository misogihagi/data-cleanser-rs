use super::interface::WorkFlowTrait;
use crate::utils::Term;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub enum SiteKindPdf {
    Yatsuhashi,
}

pub struct PdfWorkFlow {
    pub kind: SiteKindPdf,
}

impl PdfWorkFlow {
    pub fn new(kind_str: &'static str) -> PdfWorkFlow {
        PdfWorkFlow {
            kind: PdfWorkFlow::my_kind(kind_str).unwrap(),
        }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindPdf> {
        match kind_str {
            "yatsuhashi" => Some(SiteKindPdf::Yatsuhashi),
            _ => None,
        }
    }
}

impl WorkFlowTrait for PdfWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        match PdfWorkFlow::my_kind(kind_str) {
            Some(_) => true,
            None => false,
        }
    }

    async fn get_terms(&self) -> Vec<Term> {
        match &self.kind {
            SiteKindPdf::Yatsuhashi => yatsuhashi().await,
        }
    }
}

pub async fn yatsuhashi() -> Vec<Term> {
    let url = "https://yatsuhashi-sc.com/_p/acre/7432/documents/2025%E5%90%88%E5%AE%BF%E7%94%A8%E5%95%8F%E9%A1%8C%E9%9B%86_%E3%83%81%E3%83%BC%E3%83%A0%E7%94%A8%E8%AA%9E_.pdf";
    let pdf_path = Path::new("/tmp/yatsuhashi.pdf");

    let response = reqwest::get(url).await.unwrap();
    let mut file = File::create(pdf_path).unwrap();
    let body = response.bytes().await.unwrap();
    file.write_all(&body).unwrap();

    let text = match pdf_extract::extract_text(pdf_path) {
        Ok(text) => text,
        Err(e) => {
            println!("Failed to extract text: {:?}", e);
            return vec![];
        }
    };

    let mut terms = Vec::new();
    let lines: Vec<&str> = text
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        let found_title = if line.contains("ゲームマネジメント（3＋1）") {
            Some("ゲームマネジメント（3＋1）")
        } else if line == "食いつく　食いつかせる" {
            Some("食いつく　食いつかせる")
        } else if line == "集める（集）" {
            Some("集める（集）")
        } else if line == "散（さん）・展開" {
            Some("散（さん）・展開")
        } else if line == "絞る（しぼる）閉める（しめる）" {
            Some("絞る（しぼる）閉める（しめる）")
        } else if line == "リーチ" {
            Some("リーチ")
        } else if line == "レーン" {
            Some("レーン")
        } else {
            None
        };

        if let Some(matching_title) = found_title {
            let title = matching_title.to_string();
            let mut body = String::new();

            // If it's the colon case (Game Management)
            if line.contains("：") {
                let parts: Vec<&str> = line.splitn(2, '：').collect();
                if parts.len() > 1 {
                    body.push_str(parts[1]);
                }
            }

            i += 1;
            while i < lines.len() && !is_next_title(lines[i]) {
                let current_line = lines[i];
                // Stop if we hit the diagram labels
                if current_line == "攻" || current_line == "攻撃方向" || current_line == "Mゾーン"
                {
                    break;
                }

                if !body.is_empty() {
                    body.push(' ');
                }
                body.push_str(current_line);
                i += 1;
            }

            terms.push(Term {
                title,
                body: body.trim().to_string(),
                images: vec![],
            });
        } else {
            i += 1;
        }
    }

    if terms.is_empty() {
        println!("No terms found with current parser logic.");
        return terms;
    }

    println!("Parsed {} terms.", terms.len());
    terms
}

fn is_next_title(line: &str) -> bool {
    let titles = [
        "食いつく　食いつかせる",
        "集める（集）",
        "散（さん）・展開",
        "絞る（しぼる）閉める（しめる）",
        "リーチ",
        "レーン",
        "Aゾーン",
    ];
    titles.contains(&line) || line.contains("ゲームマネジメント（3＋1）")
}
