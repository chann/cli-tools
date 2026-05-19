use anyhow::Result;
use cli_core::ui::Theme;
use fake::faker::lorem::en::*;
use fake::Fake;

pub fn generate(count: usize, kind: &str) -> Result<()> {
    println!("{}", Theme::header(format!("Generating {} {}", count, kind)));
    println!();

    match kind.to_lowercase().as_str() {
        "words" | "word" => {
            let words: Vec<String> = Words(count..count + 1).fake();
            println!("{}", words.join(" "));
        }
        "sentences" | "sentence" => {
            let sentences: Vec<String> = Sentences(count..count + 1).fake();
            for s in sentences {
                println!("{}", s);
            }
        }
        "paragraphs" | "paragraph" | "p" => {
            let paragraphs: Vec<String> = Paragraphs(count..count + 1).fake();
            for (i, p) in paragraphs.iter().enumerate() {
                println!("{}", p);
                if i < paragraphs.len() - 1 {
                    println!();
                }
            }
        }
        _ => anyhow::bail!("Unsupported lorem kind: {}. Supported: words, sentences, paragraphs", kind),
    }

    Ok(())
}
