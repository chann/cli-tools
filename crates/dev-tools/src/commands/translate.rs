use anyhow::Result;
use urlencoding::encode;
use cli_core::ui::Theme;

pub async fn translate(text: &str, target_lang: &str) -> Result<()> {
    // Google Translate mobile API (unofficial but widely used for simple tools)
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
        target_lang,
        encode(text)
    );

    println!("{}", Theme::info(format!("Translating to {}...", target_lang)));

    let resp = reqwest::get(url).await?;

    if resp.status().is_success() {
        let json: serde_json::Value = resp.json().await?;
        if let Some(sentences) = json.get(0).and_then(|v| v.as_array()) {
            let mut translated = String::new();
            for sentence in sentences {
                if let Some(t) = sentence.get(0).and_then(|v| v.as_str()) {
                    translated.push_str(t);
                }
            }
            println!("\n{}", Theme::header("Translated Text"));
            println!("{}", Theme::highlight(&translated));
        }
    } else {
        anyhow::bail!("Failed to translate: {}", resp.status());
    }

    Ok(())
}
