use anyhow::Result;
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToShoutySnakeCase, ToTrainCase};
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn convert(text: &str, target_case: Option<&str>) -> Result<()> {
    if let Some(target) = target_case {
        let result = match target.to_lowercase().as_str() {
            "snake" => text.to_snake_case(),
            "camel" => text.to_lower_camel_case(),
            "pascal" => text.to_pascal_case(),
            "kebab" => text.to_kebab_case(),
            "shouty" | "screaming" => text.to_shouty_snake_case(),
            "train" => text.to_train_case(),
            _ => anyhow::bail!("Unsupported case: {}. Use snake, camel, pascal, kebab, shouty, train.", target),
        };
        println!("{}", result);
    } else {
        println!("{}", Theme::header(format!("Case Conversions for: \"{}\"", text)));
        
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Format"),
            TableFormatter::header_cell("Result"),
        ]);
        
        table.add_row(vec![
            TableFormatter::value_cell("Snake Case"),
            TableFormatter::highlight_cell(text.to_snake_case()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Camel Case"),
            TableFormatter::highlight_cell(text.to_lower_camel_case()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Pascal Case"),
            TableFormatter::highlight_cell(text.to_pascal_case()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Kebab Case"),
            TableFormatter::highlight_cell(text.to_kebab_case()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Shouty Snake"),
            TableFormatter::highlight_cell(text.to_shouty_snake_case()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Train Case"),
            TableFormatter::highlight_cell(text.to_train_case()),
        ]);
        
        println!("{}", table);
    }
    
    Ok(())
}
