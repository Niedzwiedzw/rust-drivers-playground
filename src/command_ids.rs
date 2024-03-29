#[allow(dead_code)]
// Constants
pub const SETCONTRAST: u8 = 0x81;
pub const DISPLAYALLON_RESUME: u8 = 0xA4;
pub const DISPLAYALLON: u8 = 0xA5;
pub const NORMALDISPLAY: u8 = 0xA6;
pub const INVERTDISPLAY: u8 = 0xA7;
pub const DISPLAYOFF: u8 = 0xAE;
pub const DISPLAYON: u8 = 0xAF;
pub const SETDISPLAYOFFSET: u8 = 0xD3;
pub const SETCOMPINS: u8 = 0xDA;
pub const SETVCOMDETECT: u8 = 0xDB;
pub const SETDISPLAYCLOCKDIV: u8 = 0xD5;
pub const SETPRECHARGE: u8 = 0xD9;
pub const SETMULTIPLEX: u8 = 0xA8;
pub const SETLOWCOLUMN: u8 = 0x00;
pub const SETHIGHCOLUMN: u8 = 0x10;
pub const SETSTARTLINE: u8 = 0x40;
pub const MEMORYMODE: u8 = 0x20;
pub const COLUMNADDR: u8 = 0x21;
pub const PAGEADDR: u8 = 0x22;
pub const COMSCANINC: u8 = 0xC0;
pub const COMSCANDEC: u8 = 0xC8;
pub const SEGREMAP: u8 = 0xA0;
pub const CHARGEPUMP: u8 = 0x8D;
pub const EXTERNALVCC: u8 = 0x1;
pub const SWITCHCAPVCC: u8 = 0x2;

//// Scrolling constants
//SSD1306_ACTIVATE_SCROLL = 0x2F
//SSD1306_DEACTIVATE_SCROLL = 0x2E
//SSD1306_SET_VERTICAL_SCROLL_AREA = 0xA3
//SSD1306_RIGHT_HORIZONTAL_SCROLL = 0x26
//SSD1306_LEFT_HORIZONTAL_SCROLL = 0x27
//SSD1306_VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL = 0x29
//SSD1306_VERTICAL_AND_LEFT_HORIZONTAL_SCROLL = 0x2A