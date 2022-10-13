use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS, Row, Cell};
use goblin::mach::{MachO, segment::Segment, segment::Section};
use std::env::args;

/// Convenience extension to [MachO].
trait MachOExt
{
    /// Gets an iterator to all the [Section]s present in the binary.
    fn iter_sections<'a>(&'a self) -> Box<dyn Iterator<Item = Section> + 'a>;
}

impl MachOExt for MachO<'_>
{
    fn iter_sections<'a>(&'a self) -> Box<dyn Iterator<Item = Section> + 'a>
    {
        Box::new(
            self.segments.sections()
            .flatten()
            .map(|item| {
                let (section, _) = item.unwrap();
                section
            })
        )
    }
}

fn print_segments(macho: &MachO) {
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
}

fn print_sections(macho: &MachO) {
    let mut rows: Vec<Row> = vec![];

    for segment in &macho.segments {
        let segment_name = segment.name().expect("Malformed section missing name");
        rows.push(Row::from(vec![
                segment_name, &format!("Contains {} sections", segment.nsects)
        ]));
        let sections = segment.sections().expect("Malformed segment missing sections");
        for (section, _) in sections {
            let section_name = section.name().expect("Malformed section missing name");
            rows.push(Row::from(vec!["", section_name]));
        }
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Segment", "Section"])
        .add_rows(rows)
    ;
    println!("{table}"); 
}

fn main() {
    // For testing we assume that the first argument (the program name) is a path to a Mach-O
    // binary (this is almost true when running this under macOS)
    let path_str = args().next().unwrap();
    let data = std::fs::read(&path_str).expect("Unable to read Mach-O file");
    // TODO: Understand if parsing at offset 0 is really the correct way to do this
    let macho = MachO::parse(&data, 0).expect("Unable to parse Mach-O file");

    print_sections(&macho); 
    //dbg!(macho);
}
