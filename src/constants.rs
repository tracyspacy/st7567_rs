//source https://github.com/pimoroni/pimoroni-pico/blob/main/drivers/st7567/st7567.cpp

pub const SCREEN_WIDTH: usize = 128;
pub const SCREEN_HEIGHT: usize = 64;
pub const BUFFER_SIZE: usize = 1024;

//init commands
pub const ST7567_BIAS_1_7: u8 = 0xA3;
pub const ST7567_SEG_DIR_NORMAL: u8 = 0xA0; // fixed from 0xA1 that is reverse
pub const ST7567_SETCOMREVERSE: u8 = 0xC8; // fixed from 0xC0 that is reverse. Set COM output direction, reverse mode
pub const ST7567_DISPNORMAL: u8 = 0xA6;
pub const ST7567_SETSTARTLINE: u8 = 0x40;
pub const ST7567_POWERCTRL: u8 = 0x2F;
pub const ST7567_REG_RATIO: u8 = 0x20; // fixed from 0x22.
pub const ST7567_DISPON: u8 = 0xAF;
pub const ST7567_SETCONTRAST: u8 = 0x81;
//additional commands
pub const ST7567_ENTER_RMWMODE: u8 = 0xE0; // Enter the Read Modify Write mode
pub const ST7567_EXIT_RMWMODE: u8 = 0xEE; // Exit the Read Modify Write mode
pub const ST7567_SETPAGESTART: u8 = 0xB0;
pub const ST7567_PAGESTART_MASK: u8 = 0x07;
pub const ST7567_SETCOLL: u8 = 0x00;
pub const ST7567_COLL_MASK: u8 = 0x0F;
pub const ST7567_SETCOLH: u8 = 0x10;
pub const ST7567_COLH_MASK: u8 = 0x0F;

pub const ST7567_DISPOFF: u8 = 0xAE;
pub const ST7567_STARTLINE_MASK: u8 = 0x3F;
pub const ST7567_SEG_DIR_REV: u8 = 0xA1;
pub const ST7567_DISPINVERSE: u8 = 0xA7; // Inverse disp
pub const ST7567_DISPRAM: u8 = 0xA4; // Resume to RAM content display
pub const ST7567_DISPENTIRE: u8 = 0xA5; // Entire display
pub const ST7567_BIAS_1_9: u8 = 0xA2;
pub const ST7567_EXIT_SOFTRST: u8 = 0xE2;
pub const ST7567_SETCOMNORMAL: u8 = 0xC0; // Set COM output direction, normal mode
pub const ST7567_POWERCTRL_VF: u8 = 0x29; // Control built-in power circuit
pub const ST7567_POWERCTRL_VR: u8 = 0x2A; // Control built-in power circuit
pub const ST7567_POWERCTRL_VB: u8 = 0x2C; // Control built-in power circuit
pub const ST7567_REG_RES_RR0: u8 = 0x21; // Regulation Resistior ratio
pub const ST7567_REG_RES_RR1: u8 = 0x22; // Regulation Resistior ratio
pub const ST7567_REG_RES_RR2: u8 = 0x24; // Regulation Resistior ratio
pub const ST7567_SETBOOSTER: u8 = 0xF8; // Set booster level
pub const ST7567_SETBOOSTER4X: u8 = 0x00; // Set booster level
pub const ST7567_SETBOOSTER5X: u8 = 0x01; // Set booster level
pub const ST7567_NOP: u8 = 0xE3; // NOP Command for no operation
