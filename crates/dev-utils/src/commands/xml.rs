use anyhow::{Result, anyhow};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Cursor;

pub fn format(xml: &str, indent: bool) -> Result<()> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut writer = if indent {
        Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2)
    } else {
        Writer::new(Cursor::new(Vec::new()))
    };

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(event) => {
                writer.write_event(event)
                    .map_err(|e| anyhow!("Failed to write XML event: {}", e))?;
            }
            Err(e) => return Err(anyhow!("Failed to read XML event: {}", e)),
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    println!("{}", String::from_utf8(result)?);
    Ok(())
}
