use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS, Row, Cell};
use goblin::mach::{MachO, segment::Segment, segment::Section, MultiArch, Mach};
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
    // FIXME: For now assume that the first provided argument exists and is the path to a Mach-O
    // binary to examine. Falling back to attempting to examine this utility itself as a helper for
    // testing.
    let args = args().collect::<Vec<_>>();
    let path_arg = 
        if args.len() > 1 {
            &args[1]
        } else if args.len() == 1 {
            &args[0]
        } else {
            panic!("No path to Mach-O binary provided!");
        }
    ;

    let data = std::fs::read(&path_arg).expect("Unable to read Mach-O file");

    // Try to figure out which type of binary was provided
    let mach = match Mach::parse(&data) {
        Ok(mach) => mach,
        Err(error) => panic!("Encountered error while trying to parse binary type: {}", error)
    }; 
    match mach {
        Mach::Fat(multiarch) => {
            for result_item in &multiarch {
                match result_item {
                    Ok(macho) => print_sections(&macho),
                    Err(error) => {
                        panic!("Encountered error while parsing Mach-O from multi-arch binary: {}", error.to_string());
                    }
                }
            }
        },
        Mach::Binary(macho) => print_sections(&macho)
    }
}
