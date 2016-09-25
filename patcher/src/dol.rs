use std::fmt;
use std::fmt::Debug;
use assembler::Instruction;

pub struct Section {
    pub address: u32,
    pub data: Box<[u8]>,
}

pub struct DolFile {
    pub text_sections: Vec<Section>,
    pub data_sections: Vec<Section>,
    pub bss_address: u32,
    pub bss_size: u32,
    pub entry_point: u32,
}

struct DolHeader {
    text_section_offsets: [u32; 7],
    data_section_offsets: [u32; 11],
    text_section_addresses: [u32; 7],
    data_section_addresses: [u32; 11],
    text_section_sizes: [u32; 7],
    data_section_sizes: [u32; 11],
    bss_address: u32,
    bss_size: u32,
    entry_point: u32,
}


impl Debug for Section {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{:x}", self.address)
    }
}

impl Debug for DolFile {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter,
               r"text_sections: {:#?},
data_sections: {:#?},
bss_address: {:x},
bss_size: {},
entry_point: {:x}",
               self.text_sections,
               self.data_sections,
               self.bss_address,
               self.bss_size,
               self.entry_point)
    }
}

fn read_u32(data: &[u8]) -> u32 {
    (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32)
}

fn write_u32(data: &mut [u8], value: u32) {
    data[0] = (value >> 24) as u8;
    data[1] = (value >> 16) as u8;
    data[2] = (value >> 8) as u8;
    data[3] = value as u8;
}

fn read_sections(data: &[u8],
                 offsets_offset: usize,
                 addresses_offset: usize,
                 lengths_offset: usize,
                 max: usize)
                 -> Vec<Section> {
    let mut sections = Vec::new();
    for i in 0..max {
        let offset = read_u32(&data[4 * i + offsets_offset..]);
        let address = read_u32(&data[4 * i + addresses_offset..]);
        let length = read_u32(&data[4 * i + lengths_offset..]);
        if length == 0 {
            break;
        }
        let section_data = data[offset as usize..(offset + length) as usize]
            .to_vec()
            .into_boxed_slice();
        let section = Section {
            address: address,
            data: section_data,
        };
        sections.push(section);
    }
    sections
}

impl DolFile {
    pub fn new(data: &[u8]) -> Self {
        let text_sections = read_sections(data, 0x0, 0x48, 0x90, 7);
        let data_sections = read_sections(data, 0x1c, 0x64, 0xac, 11);
        let bss_address = read_u32(&data[0xd8..]);
        let bss_size = read_u32(&data[0xdc..]);
        let entry_point = read_u32(&data[0xe0..]);

        DolFile {
            text_sections: text_sections,
            data_sections: data_sections,
            bss_address: bss_address,
            bss_size: bss_size,
            entry_point: entry_point,
        }
    }

    pub fn append(&mut self, other: DolFile) {
        self.text_sections.extend(other.text_sections);
        self.data_sections.extend(other.data_sections);
    }

    pub fn to_bytes(&self) -> Box<[u8]> {
        let mut header = DolHeader::new();
        header.bss_address = self.bss_address;
        header.bss_size = self.bss_size;
        header.entry_point = self.entry_point;

        let mut data = Vec::<u8>::new(); // FIXME
        let mut i = 0;
        let mut offset = 256;

        for section in &self.text_sections {
            header.text_section_offsets[i] = offset as u32;
            header.text_section_addresses[i] = section.address;
            header.text_section_sizes[i] = section.data.len() as u32;

            i += 1;
            offset += section.data.len();
            data.extend(section.data.as_ref());
        }

        i = 0;

        for section in &self.data_sections {
            header.data_section_offsets[i] = offset as u32;
            header.data_section_addresses[i] = section.address;
            header.data_section_sizes[i] = section.data.len() as u32;

            i += 1;
            offset += section.data.len();
            data.extend(section.data.as_ref());
        }

        let mut bytes = header.to_bytes();
        bytes.extend(data);

        bytes.into_boxed_slice()
    }

    pub fn patch(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            let section = self.text_sections
                .iter_mut()
                .chain(self.data_sections.iter_mut())
                .find(|d| {
                    d.address <= instruction.address &&
                    d.address + d.data.len() as u32 > instruction.address
                });

            if let Some(section) = section {
                let index = (instruction.address - section.address) as usize;
                write_u32(&mut section.data[index..], instruction.data);
            } else {
                panic!("Patch couldn't be applied.");
            }
        }
    }
}

impl DolHeader {
    pub fn new() -> Self {
        DolHeader {
            text_section_offsets: [0; 7],
            data_section_offsets: [0; 11],
            text_section_addresses: [0; 7],
            data_section_addresses: [0; 11],
            text_section_sizes: [0; 7],
            data_section_sizes: [0; 11],
            bss_address: 0,
            bss_size: 0,
            entry_point: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0; 256];
        let mut offset = 0;

        for &value in &self.text_section_offsets {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        for &value in &self.data_section_offsets {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        for &value in &self.text_section_addresses {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        for &value in &self.data_section_addresses {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        for &value in &self.text_section_sizes {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        for &value in &self.data_section_sizes {
            write_u32(&mut data[offset..], value);
            offset += 4;
        }

        write_u32(&mut data[offset..], self.bss_address);
        offset += 4;
        write_u32(&mut data[offset..], self.bss_size);
        offset += 4;
        write_u32(&mut data[offset..], self.entry_point);

        data
    }
}
