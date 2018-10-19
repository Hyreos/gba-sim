use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::io::prelude::*;
use std::fmt;

struct Nintendo {
    pub texels : [u8; 48]
}

impl fmt::Debug for Nintendo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "texels : [...]")
    }
}

#[derive(Debug)]
pub enum ConType {
    Color = 0x80,
    NColor = 0x81
}

#[derive(Debug)]
pub enum GBSGB_Indicator {
    GB = 0x00,
    SGB = 0x03
}

#[derive(Debug)]
pub enum ROMType {
    ROM_Only = 0x0,
    ROM_MBC1 = 0x1,
    ROM_MBC1_RAM = 0x2,
    ROM_MBC1_RAM_BATT = 0x3,
    ROM_MBC2 = 0x5,
    ROM_MBC2_BATTERY = 0x6,
    ROM_RAM = 0x8,
    ROM_RAM_BATTERY = 0x9,
    ROM_MMM01 = 0xB,
    ROM_MMM01_SRAM = 0xC,
    ROM_MMM01_SRAM_BATT = 0xD,
    ROM_MBC3_TIMER_BATT = 0xF,
    ROM_MBC3_TIMER_RAM_BATT = 0x10,
    ROM_MBC3 = 0x11,
    ROM_MBC3_RAM = 0x12,
    ROM_MBC3_RAM_BATT = 0x13,
    ROM_MBC5 = 0x19,
    ROM_MBC5_RAM = 0x1A,
    ROM_MBC5_RAM_BATT = 0x1B,
    ROM_MBC5_RUMBLE = 0x1C,
    ROM_MBC5_RUMBLE_SRAM = 0x1D,
    ROM_MBC5_RUMBLE_SRAM_BATT = 0x1E,
    Pocket_Camera = 0x1F,
    Bandai_TAMA5 = 0xFD,
    Hudson_HuC_3 = 0xFE,
    Hudson_HuC_1 = 0xFF
}

#[derive(Debug)]
pub enum ROMSize {
    Sz256Kbit = 0x0,
    Sz512Kbit = 0x1,
    Sz1Mbit = 0x2,
    Sz2Mbit = 0x3,
    Sz4Mbit = 0x4,
    Sz8Mbit = 0x5,
    Sz16Mbit = 0x6,
    Sz9Mbit = 0x52,
    Sz10Mbit = 0x53,
    Sz12Mbit = 0x54
}

#[derive(Debug)]
pub enum RAMSize {
    None = 0x0,
    Sz16kBit = 0x1,
    Sz64kBit = 0x2,
    Sz256kBit = 0x3,
    Sz1MBit = 0x4
}

#[derive(Debug)]
pub enum DestinationCode {
    Japanese,
    NonJapanese,
}

#[derive(Debug)]
pub enum LicenseCode {
    Recheck = 0x33,
    Accolade = 0x79,
    Konami = 0xA4
}

#[derive(Debug)]
pub struct Header {
    nintendo : Nintendo,
    pub title : String,
    pub con_type : Option<ConType>,
    pub rom_type : Option<ROMType>,
    pub rom_size : Option<ROMSize>,
    pub ram_size : Option<RAMSize>,
    pub dest_code : Option<DestinationCode>,
    pub lcode : Option<LicenseCode>,
    pub mask_rom_vers_number : u8,
    pub compl_check : u8,
    pub checksum : [u8; 2]
}

impl Header<> {
    fn get_nintendo_texels(&self) -> &[u8; 48] {
        static TEXELS : Nintendo =  Nintendo {
            texels: [
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
            ]
        };

        return &TEXELS.texels;
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        let texels = self.get_nintendo_texels();

        for i in 0..texels.len() {
            if self.nintendo.texels[i] != texels[i] {
                return Err("Invalid ROM header");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub header: Header
}

impl Cartridge<> {
    pub fn new_from_file(path: &str) -> Result<Self, &'static str> {
        println!("Loading ROM {}", path);

        let mut file = match File::open(path) {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                return Err("Error on load ROM");
            }
        };

        let meta = file.metadata().unwrap();

        let mut data = Vec::new();
        data.resize(meta.len() as usize, 0u8);
 
        file.read(&mut data).unwrap();

        Ok(Self {
            header: Header {
                checksum: [ data[0x14E], data[0x14F] ],
                compl_check: data[0x14D],
                con_type: match data[0x143] {
                    0x80 => Some(ConType::Color),
                    _ => None
                },
                dest_code: match data[0x14A] {
                    0 => Some(DestinationCode::Japanese),
                    1 => Some(DestinationCode::NonJapanese),
                    _ => None
                },
                lcode: match data[0x14B] {
                    0x33 => match data[0x144] as u16 | (data[0x145] as u16).wrapping_shl(8) {
                                0x79 => Some(LicenseCode::Accolade),
                                0xA4 => Some(LicenseCode::Konami),
                                _ => None
                            },
                    0x79 => Some(LicenseCode::Accolade),
                    0xA4 => Some(LicenseCode::Konami),
                    _ => None
                },
                mask_rom_vers_number: data[0x14C],
                nintendo: {
                    let mut texel_data = [0; 48];
                    texel_data[..47].clone_from_slice(&data[0x104..0x133]);
                    Nintendo { texels: texel_data }
                },
                ram_size: match data[0x149] {
                    0 => Some(RAMSize::None),
                    1 => Some(RAMSize::Sz16kBit),
                    2 => Some(RAMSize::Sz64kBit),
                    3 => Some(RAMSize::Sz256kBit),
                    4 => Some(RAMSize::Sz1MBit),
                    _ => None
                },
                rom_size: match data[0x148] {
                    0 => Some(ROMSize::Sz256Kbit),
                    1 => Some(ROMSize::Sz512Kbit),
                    2 => Some(ROMSize::Sz1Mbit),
                    3 => Some(ROMSize::Sz2Mbit),
                    4 => Some(ROMSize::Sz4Mbit),
                    5 => Some(ROMSize::Sz8Mbit),
                    6 => Some(ROMSize::Sz16Mbit),
                    0x52 => Some(ROMSize::Sz9Mbit),
                    0x53 => Some(ROMSize::Sz10Mbit),
                    0x54 => Some(ROMSize::Sz12Mbit),
                    _ => None
                },
                title: String::from_utf8(data[0x134..0x142].iter().cloned().collect()).unwrap(),
                rom_type: match data[0x147] {
                    0 => Some(ROMType::ROM_Only),
                    1 => Some(ROMType::ROM_MBC1),
                    2 => Some(ROMType::ROM_MBC1_RAM),
                    3 => Some(ROMType::ROM_MBC1_RAM_BATT),
                    5 => Some(ROMType::ROM_MBC2),
                    6 => Some(ROMType::ROM_MBC2_BATTERY),
                    8 => Some(ROMType::ROM_RAM),
                    9 => Some(ROMType::ROM_RAM_BATTERY),
                    0xB => Some(ROMType::ROM_MMM01),
                    0xC => Some(ROMType::ROM_MMM01_SRAM),
                    0xD => Some(ROMType::ROM_MMM01_SRAM_BATT),
                    0xF => Some(ROMType::ROM_MBC3_TIMER_BATT),
                    0x10 => Some(ROMType::ROM_MBC3_TIMER_RAM_BATT),
                    0x11 => Some(ROMType::ROM_MBC3),
                    0x12 => Some(ROMType::ROM_MBC3_RAM),
                    0x13 => Some(ROMType::ROM_MBC3_RAM_BATT),
                    0x19 => Some(ROMType::ROM_MBC5),
                    0x1A => Some(ROMType::ROM_MBC5_RAM),
                    0x1B => Some(ROMType::ROM_MBC5_RAM_BATT),
                    0x1C => Some(ROMType::ROM_MBC5_RUMBLE),
                    0x1D => Some(ROMType::ROM_MBC5_RUMBLE_SRAM),
                    0x1E => Some(ROMType::ROM_MBC5_RUMBLE_SRAM_BATT),
                    0x1F => Some(ROMType::Pocket_Camera),
                    0xFD => Some(ROMType::Bandai_TAMA5),
                    0xFE => Some(ROMType::Hudson_HuC_3),
                    0xFF => Some(ROMType::Hudson_HuC_1),
                    _ => None
                }
            }
        })
    }
}
