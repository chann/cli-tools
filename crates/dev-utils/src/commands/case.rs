use anyhow::Result;
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToShoutySnakeCase, ToTrainCase};

pub fn convert(text: &str, target_case: &str) -> Result<()> {
    let result = match target_case.to_lowercase().as_str() {
        "snake" => text.to_snake_case(),
        "camel" => text.to_lower_camel_case(),
        "pascal" => text.to_pascal_case(),
        "kebab" => text.to_kebab_case(),
        "shouty" | "screaming" => text.to_shouty_snake_case(),
        "train" => text.to_train_case(),
        _ => anyhow::bail!("Unsupported case: {}. Use snake, camel, pascal, kebab, shouty, train.", target_case),
    };

    println!("{}", result);
    Ok(())
}
