use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DictionaryEntry {
    word: String,
    phonetic: Option<String>,
    meanings: Vec<Meaning>,
}

#[derive(Deserialize, Debug)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
struct Definition {
    definition: String,
    example: Option<String>,
}

pub async fn lookup(word: &str) -> Result<()> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    println!("{}", format!("Looking up '{}'...", word).dimmed());

    let resp = reqwest::get(url).await?;

    if resp.status().is_success() {
        let entries: Vec<DictionaryEntry> = resp.json().await?;
        for entry in entries {
            println!("\n{} {}", entry.word.bold().cyan(), entry.phonetic.unwrap_or_default().dimmed());
            for meaning in entry.meanings {
                println!("\n  [{}]", meaning.part_of_speech.italic().yellow());
                for (i, def) in meaning.definitions.iter().enumerate() {
                    println!("  {}. {}", i + 1, def.definition);
                    if let Some(example) = &def.example {
                        println!("     {}", format!("\"{}\"", example).dimmed());
                    }
                }
            }
        }
    } else if resp.status() == 404 {
        println!("{}", "No definition found.".red());
    } else {
        anyhow::bail!("Failed to lookup dictionary: {}", resp.status());
    }

    Ok(())
}
