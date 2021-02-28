use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;
use elf;

#[derive(Debug,Clone)]
struct ProcMapEntry {
    start: usize,
    end: usize,
    perm: String,
    offset: usize,
    name: Option<String>,
}

impl ProcMapEntry {
    fn parse(entry: &str) -> Result<Self, String> {
	let mut t = entry.split(" ");
	if t.clone().count() < 6 {
	    return Err("parse error: unkown format".into());
	}
	let range: Vec<usize> = t.next()
	    .unwrap()
	    .split("-")
	    .map(|v|
		 usize::from_str_radix(v, 16).unwrap())
	    .take(2)
	    .collect();
	let perm = t.next().unwrap();
	let offset = usize::from_str_radix(t.next().unwrap(), 16).unwrap();
	let fname = match t.last().unwrap() {
	    "" => None,
	    v => Some(v.into()),
	};
	Ok(
	    Self{
		start: range[0],
		end: range[1],
		perm: perm.into(),
		offset: offset,
		name: fname,
	})
    }
}

#[derive(Debug,Clone)]
struct ElfMapEntry {
    start: usize,
    end: usize,
    perm: String,
    offset: usize,
    elf_sections: Vec<elf::types::SectionHeader>,
}
impl ElfMapEntry {
    fn new(pme: &ProcMapEntry) -> Self{
	Self {
	    start: pme.start,
	    end: pme.end,
	    perm: pme.perm.clone(),
	    offset: pme.offset,
	    elf_sections: Vec::new(),
	}
    }
}

#[derive(Debug,Clone)]
struct ElfMaps {
    module: HashMap<String, Vec<ElfMapEntry>>
}

impl ElfMaps {
    fn new(proc_map_entries: &[ProcMapEntry]) -> Result<Self, String> {
	// create module map: module.name -> [ElfMapEntry1,...]
	let mut module = HashMap::new();
	for pme in proc_map_entries.iter()
	    .filter(|v| v.name.is_some() && v.name.as_ref().unwrap().starts_with("/")) {
		if !module.contains_key(pme.name.as_ref().unwrap()) {
		    module.insert(pme.name.as_ref().unwrap().clone(), Vec::new());
		}
		module.get_mut(pme.name.as_ref().unwrap())
		    .unwrap().push(ElfMapEntry::new(pme));
	}

	// add elf infos
	let mut ret = HashMap::new();
	for module_name in module.keys() {
	    let elf_file = match elf::File::open_path(module_name) {
		Ok(v) => v,
		Err(_) => continue,
	    };
	    ret.insert(module_name.to_string(), vec![]);
	    let pmes = module.get(module_name).unwrap();
	    let mut pme_iter = pmes.iter();
	    let mut pme_current = pme_iter.next().unwrap();
	    let mut pme_new = pme_current.clone(); 
	    for pme_next in pme_iter {
		for elf_section in &elf_file.sections {
		    if pme_current.offset as u64 <= elf_section.shdr.offset &&
			pme_next.offset as u64 > elf_section.shdr.offset {
			    pme_new.elf_sections.push(elf_section.shdr.clone());
			}
		}
		ret.get_mut(module_name).unwrap().push(pme_new);
		pme_current = pme_next;
		pme_new = pme_current.clone();
	    }
	}
	
	Ok(Self {
		module: ret,
	    })
    }
}

impl fmt::Display for ElfMaps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	for (module_name, pmes) in &self.module {
	    for pme in pmes {
		for sec in &pme.elf_sections {
		    write!(f, "{:x}-{:x} {} {:24} {:08x} {:08x} {:08x} {}\n",
			pme.start + sec.offset as usize,
			pme.start + sec.offset as usize + sec.size as usize,
			pme.perm,
			sec.name,
			pme.offset,
			sec.offset,
                        sec.size,
			module_name,
		    )?;
		}
	    }
	}
	Ok(())
    }
}

fn usage(file_name: &str){
    println!("Usage: {} <pid>", file_name);
    println!("Output format:\nstart-end permission section_name maps_offset elf_offset section_size filename\n");
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>>{

    // args parsing
    let mut args = std::env::args();
    let fname = args.next().unwrap();
    if args.len() != 1 {
	usage(&fname);
	return Ok(());
    }
    let pid: u32 = match args.next().unwrap().parse() {
	Ok(v) => v,
	Err(_) => {
	    usage(&fname);
	    return Ok(());
	},
    }; 
    let proc_dir = PathBuf::from("/proc").join(pid.to_string());

    // read /proc/<pid>/maps
    let mut maps = "".to_string();
    let mut f = File::open(proc_dir.join("maps"))?;
    f.read_to_string(&mut maps)?;

    let map_entries: Vec<ProcMapEntry> = maps
	.lines()
	.map(|l| ProcMapEntry::parse(l).unwrap())
	.collect();

    // take parse map entries and add elf section header infos
    let elf_maps = ElfMaps::new(&map_entries)?;
    println!("{}", elf_maps);

    Ok(())
}
