use anyhow::Result;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
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

    println!("{}", Theme::info(format!("Looking up '{}'...", word)));

    let resp = reqwest::get(url).await?;

    if resp.status().is_success() {
        let entries: Vec<DictionaryEntry> = resp.json().await?;
        for entry in entries {
            println!("\n{} {}", Theme::highlight(&entry.word), Theme::dim(entry.phonetic.as_deref().unwrap_or_default()));
            
            for meaning in entry.meanings {
                println!("\n  {}", Theme::header(format!("[{}]", meaning.part_of_speech)));
                
                let mut table = TableFormatter::create_table();
                table.set_header(vec![
                    TableFormatter::header_cell("#"),
                    TableFormatter::header_cell("Definition"),
                    TableFormatter::header_cell("Example"),
                ]);
                
                for (i, def) in meaning.definitions.iter().enumerate() {
                    table.add_row(vec![
                        TableFormatter::value_cell(i + 1),
                        TableFormatter::value_cell(&def.definition),
                        TableFormatter::value_cell(def.example.as_deref().unwrap_or("-")),
                    ]);
                }
                
                println!("{table}");
            }
        }
    } else if resp.status() == 404 {
        println!("{}", Theme::error("No definition found."));
    } else {
        anyhow::bail!("Failed to lookup dictionary: {}", resp.status());
    }

    Ok(())
}
