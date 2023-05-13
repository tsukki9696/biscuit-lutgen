use clap::ValueEnum;
use exoquant::Color;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, ValueEnum)]
pub enum Palette {
    CatppuccinMocha,
    CatppuccinMacchiato,
    CatppuccinFrappe,
    CatppuccinLatte,
}

impl Palette {
    pub fn get(&self) -> &'static [Color] {
        match self {
            Palette::CatppuccinMocha => &catppuccin::MOCHA,
            Palette::CatppuccinMacchiato => &catppuccin::MACCHIATO,
            Palette::CatppuccinFrappe => &catppuccin::FRAPPE,
            Palette::CatppuccinLatte => &catppuccin::LATTE,
        }
    }
}

pub mod catppuccin {
    use exoquant::Color;

    #[allow(dead_code)]
    pub const ALL: [(&str, &[Color; 26]); 4] = [
        ("mocha", &MOCHA),
        ("macchiato", &MACCHIATO),
        ("frappe", &FRAPPE),
        ("latte", &LATTE),
    ];

    pub const MOCHA: [Color; 26] = [
        Color {
            r: 245,
            g: 224,
            b: 220,
            a: 255,
        },
        Color {
            r: 242,
            g: 205,
            b: 205,
            a: 255,
        },
        Color {
            r: 245,
            g: 194,
            b: 231,
            a: 255,
        },
        Color {
            r: 203,
            g: 166,
            b: 247,
            a: 255,
        },
        Color {
            r: 243,
            g: 139,
            b: 168,
            a: 255,
        },
        Color {
            r: 235,
            g: 160,
            b: 172,
            a: 255,
        },
        Color {
            r: 250,
            g: 179,
            b: 135,
            a: 255,
        },
        Color {
            r: 249,
            g: 226,
            b: 175,
            a: 255,
        },
        Color {
            r: 166,
            g: 227,
            b: 161,
            a: 255,
        },
        Color {
            r: 148,
            g: 226,
            b: 213,
            a: 255,
        },
        Color {
            r: 137,
            g: 220,
            b: 235,
            a: 255,
        },
        Color {
            r: 116,
            g: 199,
            b: 236,
            a: 255,
        },
        Color {
            r: 137,
            g: 180,
            b: 250,
            a: 255,
        },
        Color {
            r: 180,
            g: 190,
            b: 254,
            a: 255,
        },
        Color {
            r: 205,
            g: 214,
            b: 244,
            a: 255,
        },
        Color {
            r: 186,
            g: 194,
            b: 222,
            a: 255,
        },
        Color {
            r: 166,
            g: 173,
            b: 200,
            a: 255,
        },
        Color {
            r: 147,
            g: 153,
            b: 178,
            a: 255,
        },
        Color {
            r: 127,
            g: 132,
            b: 156,
            a: 255,
        },
        Color {
            r: 108,
            g: 112,
            b: 134,
            a: 255,
        },
        Color {
            r: 88,
            g: 91,
            b: 112,
            a: 255,
        },
        Color {
            r: 69,
            g: 71,
            b: 90,
            a: 255,
        },
        Color {
            r: 49,
            g: 50,
            b: 68,
            a: 255,
        },
        Color {
            r: 30,
            g: 30,
            b: 46,
            a: 255,
        },
        Color {
            r: 24,
            g: 24,
            b: 37,
            a: 255,
        },
        Color {
            r: 17,
            g: 17,
            b: 27,
            a: 255,
        },
    ];

    pub const MACCHIATO: [Color; 26] = [
        Color {
            r: 244,
            g: 219,
            b: 214,
            a: 255,
        },
        Color {
            r: 240,
            g: 198,
            b: 198,
            a: 255,
        },
        Color {
            r: 245,
            g: 189,
            b: 230,
            a: 255,
        },
        Color {
            r: 198,
            g: 160,
            b: 246,
            a: 255,
        },
        Color {
            r: 237,
            g: 135,
            b: 150,
            a: 255,
        },
        Color {
            r: 238,
            g: 153,
            b: 160,
            a: 255,
        },
        Color {
            r: 245,
            g: 169,
            b: 127,
            a: 255,
        },
        Color {
            r: 238,
            g: 212,
            b: 159,
            a: 255,
        },
        Color {
            r: 166,
            g: 218,
            b: 149,
            a: 255,
        },
        Color {
            r: 139,
            g: 213,
            b: 202,
            a: 255,
        },
        Color {
            r: 145,
            g: 215,
            b: 227,
            a: 255,
        },
        Color {
            r: 125,
            g: 196,
            b: 228,
            a: 255,
        },
        Color {
            r: 138,
            g: 173,
            b: 244,
            a: 255,
        },
        Color {
            r: 183,
            g: 189,
            b: 248,
            a: 255,
        },
        Color {
            r: 202,
            g: 211,
            b: 245,
            a: 255,
        },
        Color {
            r: 184,
            g: 192,
            b: 224,
            a: 255,
        },
        Color {
            r: 165,
            g: 173,
            b: 203,
            a: 255,
        },
        Color {
            r: 147,
            g: 154,
            b: 183,
            a: 255,
        },
        Color {
            r: 128,
            g: 135,
            b: 162,
            a: 255,
        },
        Color {
            r: 110,
            g: 115,
            b: 141,
            a: 255,
        },
        Color {
            r: 91,
            g: 96,
            b: 120,
            a: 255,
        },
        Color {
            r: 73,
            g: 77,
            b: 100,
            a: 255,
        },
        Color {
            r: 54,
            g: 58,
            b: 79,
            a: 255,
        },
        Color {
            r: 36,
            g: 39,
            b: 58,
            a: 255,
        },
        Color {
            r: 30,
            g: 32,
            b: 48,
            a: 255,
        },
        Color {
            r: 24,
            g: 25,
            b: 38,
            a: 255,
        },
    ];

    pub const FRAPPE: [Color; 26] = [
        Color {
            r: 242,
            g: 213,
            b: 207,
            a: 255,
        },
        Color {
            r: 238,
            g: 190,
            b: 190,
            a: 255,
        },
        Color {
            r: 244,
            g: 184,
            b: 228,
            a: 255,
        },
        Color {
            r: 202,
            g: 158,
            b: 230,
            a: 255,
        },
        Color {
            r: 231,
            g: 130,
            b: 132,
            a: 255,
        },
        Color {
            r: 234,
            g: 153,
            b: 156,
            a: 255,
        },
        Color {
            r: 239,
            g: 159,
            b: 118,
            a: 255,
        },
        Color {
            r: 229,
            g: 200,
            b: 144,
            a: 255,
        },
        Color {
            r: 166,
            g: 209,
            b: 137,
            a: 255,
        },
        Color {
            r: 129,
            g: 200,
            b: 190,
            a: 255,
        },
        Color {
            r: 153,
            g: 209,
            b: 219,
            a: 255,
        },
        Color {
            r: 133,
            g: 193,
            b: 220,
            a: 255,
        },
        Color {
            r: 140,
            g: 170,
            b: 238,
            a: 255,
        },
        Color {
            r: 186,
            g: 187,
            b: 241,
            a: 255,
        },
        Color {
            r: 198,
            g: 208,
            b: 245,
            a: 255,
        },
        Color {
            r: 181,
            g: 191,
            b: 226,
            a: 255,
        },
        Color {
            r: 165,
            g: 173,
            b: 206,
            a: 255,
        },
        Color {
            r: 148,
            g: 156,
            b: 187,
            a: 255,
        },
        Color {
            r: 131,
            g: 139,
            b: 167,
            a: 255,
        },
        Color {
            r: 115,
            g: 121,
            b: 148,
            a: 255,
        },
        Color {
            r: 98,
            g: 104,
            b: 128,
            a: 255,
        },
        Color {
            r: 81,
            g: 87,
            b: 109,
            a: 255,
        },
        Color {
            r: 65,
            g: 69,
            b: 89,
            a: 255,
        },
        Color {
            r: 48,
            g: 52,
            b: 70,
            a: 255,
        },
        Color {
            r: 41,
            g: 44,
            b: 60,
            a: 255,
        },
        Color {
            r: 35,
            g: 38,
            b: 52,
            a: 255,
        },
    ];

    pub const LATTE: [Color; 26] = [
        Color {
            r: 220,
            g: 138,
            b: 120,
            a: 255,
        },
        Color {
            r: 221,
            g: 120,
            b: 120,
            a: 255,
        },
        Color {
            r: 234,
            g: 118,
            b: 203,
            a: 255,
        },
        Color {
            r: 136,
            g: 57,
            b: 239,
            a: 255,
        },
        Color {
            r: 210,
            g: 15,
            b: 57,
            a: 255,
        },
        Color {
            r: 230,
            g: 69,
            b: 83,
            a: 255,
        },
        Color {
            r: 254,
            g: 100,
            b: 11,
            a: 255,
        },
        Color {
            r: 223,
            g: 142,
            b: 29,
            a: 255,
        },
        Color {
            r: 64,
            g: 160,
            b: 43,
            a: 255,
        },
        Color {
            r: 23,
            g: 146,
            b: 153,
            a: 255,
        },
        Color {
            r: 4,
            g: 165,
            b: 229,
            a: 255,
        },
        Color {
            r: 32,
            g: 159,
            b: 181,
            a: 255,
        },
        Color {
            r: 30,
            g: 102,
            b: 245,
            a: 255,
        },
        Color {
            r: 114,
            g: 135,
            b: 253,
            a: 255,
        },
        Color {
            r: 76,
            g: 79,
            b: 105,
            a: 255,
        },
        Color {
            r: 92,
            g: 95,
            b: 119,
            a: 255,
        },
        Color {
            r: 108,
            g: 111,
            b: 133,
            a: 255,
        },
        Color {
            r: 124,
            g: 127,
            b: 147,
            a: 255,
        },
        Color {
            r: 140,
            g: 143,
            b: 161,
            a: 255,
        },
        Color {
            r: 156,
            g: 160,
            b: 176,
            a: 255,
        },
        Color {
            r: 172,
            g: 176,
            b: 190,
            a: 255,
        },
        Color {
            r: 188,
            g: 192,
            b: 204,
            a: 255,
        },
        Color {
            r: 204,
            g: 208,
            b: 218,
            a: 255,
        },
        Color {
            r: 220,
            g: 224,
            b: 232,
            a: 255,
        },
        Color {
            r: 230,
            g: 233,
            b: 239,
            a: 255,
        },
        Color {
            r: 239,
            g: 241,
            b: 245,
            a: 255,
        },
    ];
}