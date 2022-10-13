use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS, Row};
use goblin::mach::{MachO, segment::Segment};
use std::env::args;

fn main() {
    // For testing we assume that the first argument (the program name) is a path to a Mach-O
    // binary (this is almost true when running this under macOS)
    let path_str = args().next().unwrap();
    let data = std::fs::read(&path_str).expect("Unable to read Mach-O file");
    // TODO: Understand if parsing at offset 0 is really the correct way to do this
    let macho = MachO::parse(&data, 0).expect("Unable to parse Mach-O file");

    let mut segment_names: Vec<String> = vec![];
    // List the segment names
    for segment in &macho.segments {
        // Segment name must exist
        let name = segment.name().unwrap();
        segment_names.push(name.to_owned());
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Segments"])
        .add_rows(
            segment_names.iter()
            .map(|name| vec![name])
        )
    ;
    println!("{table}"); 
    //dbg!(macho);
}
