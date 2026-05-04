use anyhow::Result;
use regex::Regex;

pub fn process(text: &str, kind: &str) -> Result<()> {
    let mut masked_text = text.to_string();

    match kind {
        "email" => {
            masked_text = mask_emails(&masked_text);
        }
        "phone" => {
            masked_text = mask_phones(&masked_text);
        }
        "card" => {
            masked_text = mask_cards(&masked_text);
        }
        "ip" => {
            masked_text = mask_ips(&masked_text);
        }
        "all" => {
            masked_text = mask_emails(&masked_text);
            masked_text = mask_phones(&masked_text);
            masked_text = mask_cards(&masked_text);
            masked_text = mask_ips(&masked_text);
        }
        _ => anyhow::bail!("Unsupported mask kind: {}", kind),
    }

    println!("{}", masked_text);
    Ok(())
}

fn mask_emails(text: &str) -> String {
    let re = Regex::new(r"([a-zA-Z0-9_.+-]+)@([a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let user = &caps[1];
        let domain = &caps[2];
        if user.len() <= 2 {
            format!("*@{}", domain)
        } else {
            format!("{}***@{}", &user[..2], domain)
        }
    }).to_string()
}

fn mask_phones(text: &str) -> String {
    // Basic phone pattern: matches formats like 010-1234-5678, 02-123-4567, etc.
    let re = Regex::new(r"(\d{2,3})[-.\s]?(\d{3,4})[-.\s]?(\d{4})").unwrap();
    re.replace_all(text, "$1-****-$3").to_string()
}

fn mask_cards(text: &str) -> String {
    // Basic card pattern: 16 digits often separated by - or space
    let re = Regex::new(r"(\d{4})[-.\s]?(\d{4})[-.\s]?(\d{4})[-.\s]?(\d{4})").unwrap();
    re.replace_all(text, "$1-****-****-$4").to_string()
}

fn mask_ips(text: &str) -> String {
    let re = Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})").unwrap();
    re.replace_all(text, "$1.$2.***.***").to_string()
}
