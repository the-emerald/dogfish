use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::square::ParseError::BitBoardNotUnit;
use once_cell::sync::Lazy;
use crate::board_representation::square::Square;
use std::convert::TryFrom;
use crate::board_representation::bitboard::files_ranks::{FILE_A_BITBOARD, FILE_H_BITBOARD, RANK_8_BITBOARD, RANK_1_BITBOARD};
use crate::piece::piecetype::PieceType;
use crate::board_representation::bitboard::shift::Direction::{North, East, South, West};

pub static SLIDING_ROOK: Lazy<SlidingRook> = Lazy::new(|| {
    let mut table = [BitBoard::new(0); 0x19000];
    let mut magics = [Magic::default(); 64];

    let attack_directions = [North, East, South, West];
    let mut prev_offset = 0;
    let mut size = 0;

    // Populate the magic fields
    for (square, magic) in magics.iter_mut().enumerate() {
        let sq = Square::try_from(square as u64).unwrap();
        let mut occupancy = [BitBoard::new(0); 4096];
        let mut reference = [BitBoard::new(0); 4096];

        let mut edges = ((RANK_1_BITBOARD | RANK_8_BITBOARD) & !BitBoard::bitboard_of_rank(sq)) |
            ((FILE_A_BITBOARD | FILE_H_BITBOARD) & !BitBoard::bitboard_of_file(sq));

        magic.mask = PieceType::sliding_attack(attack_directions, sq, 0.into()) & !edges;
        magic.magic = MAGIC_NUMBERS_ROOK[square];
        magic.shift = 64_u64 - (u64::from(magic.mask).count_ones() as u64);

        magic.table = prev_offset + size;
        prev_offset = magic.table;

        // Carry-Rippler, courtesy of Stockfish
        let mut b: BitBoard = 0.into();
        size = 0;
        loop {
            occupancy[size] = b;
            reference[size] = PieceType::sliding_attack(attack_directions, sq, b);

            // println!("Size: {}", size);
            // println!("Occp: {:?}", b);
            // println!("Mask: {:?}", magic.mask);
            // println!("Magic: {:?}", magic.magic);
            // println!("Shift: {:?}", magic.shift);

            let idx: usize = u64::from(((b & magic.mask) * (magic.magic.into())) >> (magic.shift.into())) as usize;
            table[magic.table + idx] = reference[size];

            // println!("{} set to: {:?}", magic.table + idx, reference[size]);

            size += 1;
            b = (b - magic.mask) & magic.mask;
            // println!("Ok.\n-----");
            if !(b == 0.into()) {
                break;
            }
        }
    }
    SlidingRook::new(table, magics)
});

pub struct SlidingRook {
    pub table: [BitBoard; 0x19000],
    pub magic: [Magic; 64],
}

impl SlidingRook {
    pub fn new(table: [BitBoard; 0x19000], magic: [Magic; 64]) -> Self {
        Self {
            table,
            magic
        }
    }
}

// TODO: Copy from magic rook
pub static MAGIC_BISHOP: Lazy<[Magic; 64]> = Lazy::new(|| {
    unimplemented!()
});

pub struct SlidingBishop {
    pub table: [BitBoard; 0x1480],
    pub magic: [Magic; 64],
}

impl SlidingBishop {
    pub fn new(table: [BitBoard; 0x1480], magic: [Magic; 64]) -> Self {
        Self {
            table,
            magic
        }
    }
}

#[derive(Copy, Clone)]
pub struct Magic {
    table: usize,
    mask: BitBoard,
    magic: u64,
    shift: u64,
}

impl Magic {
    pub fn new(table: usize, mask: BitBoard, magic: u64, shift: u64) -> Self {
        Self {
            table,
            mask,
            magic,
            shift,
        }
    }

    pub fn table(self) -> usize {
        self.table
    }

    pub fn mask(self) -> BitBoard {
        self.mask
    }

    pub fn magic(self) -> u64 {
        self.magic
    }

    pub fn shift(self) -> u64 {
        self.shift
    }
}

impl Default for Magic {
    fn default() -> Self {
        Self {
            table: 0,
            mask: 0.into(),
            magic: 0,
            shift: 0,
        }
    }
}


static MAGIC_NUMBERS_BISHOP: [u64; 64] =
    [306397059236266368, 6638343277122827280, 10377420549504106496, 9193021019258913, 2306408226914042898, 10379110636817760276, 27167319028441088, 7566153073497751552,
        1513227076520969216, 301917653126479936, 72075465430409232, 2343002121441460228, 36033212782477344, 9223373154083475456, 6935629192638251008, 72621648200664064,
        2310506081245267984, 2533291987569153, 146934404644733024, 1838417834950912, 579856052833622016, 1729946448243595776, 705208029025040, 2886877732040869888,
        10092575566416331020, 5635409948247040, 738739924278198804, 4648849515743289408, 9233786889293807616, 1155253577929753088, 435164712050360592, 3026700562025580641,
        4612284839965491969, 10448650511900137472, 571823356120080, 40569782189687936, 148620986995048708, 4901113822871308288, 4612077461748908288, 10204585674276944,
        2534512027246592, 5766297627561820676, 13809969191200768, 1153062656578422784, 9318235838682899712, 11533824475839595776, 433770548762247233, 92326036501692936,
        9227053213059129360, 577024872779350852, 108087561569959936, 582151826703646856, 81404176367767, 316415319130374273, 9113856212762624, 145453328103440392,
        441392350330618400, 1126492748710916, 2309220790581891072, 3026423624667006980, 18019391702696464, 4516931289817600, 1450317422841301124, 9246488805123342592];

static MAGIC_NUMBERS_ROOK: [u64; 64] =
    [36028867955671040, 2395917338224361536, 936757656041832464, 648535942831284356, 36037595259731970, 13943151043426386048, 432349966580056576, 4683745813775001856,
        1191624314978336800, 4611756662317916160, 4625338105090543616, 140806208356480, 1688987371057664, 9288708641522688, 153403870897537280, 281550411726850,
        2401883155071024, 1206964838111645696, 166705754384925184, 36039792408011264, 10376580514281768960, 9148486532465664, 578787319189340418, 398007816633254020,
        2341872150903791616, 2314850762536009728, 297238127310798880, 2251868801728768, 2594082183614301184, 820222482337235456, 37717655469424904, 577596144088011012,
        1152991874030502016, 3171026856472219648, 20415869351890944, 4611844348286345472, 2455605323386324224, 140754676613632, 1740713828645089416, 58361257132164,
        70370893791232, 9227880322828615684, 72092778695295040, 577023839834341392, 4723150143565660416, 563087661073408, 651083773116450, 72128789630550047,
        153192758223054976, 869194865525653568, 4972009250306933248, 1031325449119138048, 1297041090863464576, 580401419157405824, 1657992643584, 306245066729521664,
        15206439601351819394, 14143290885479661953, 1688988407201810, 18065251325837538, 1152927311403745429, 162411078742050817, 334255838724676, 27323018585852550];
