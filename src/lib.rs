#![no_std]
#![no_main]
#![allow(unused)]

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;
mod constants;

pub enum BacklightStatus {
    On,
    Off,
}

pub enum ScreenDirection {
    Normal,
    Reverse,
}

pub enum Bias {
    Bias7_1,
    Bias9_1,
}

#[derive(Debug)]
pub enum DisplayErrors {
    SpiError,
    PinError,
    OutOfBoundsError((usize, usize)),
}

pub struct ST7567<DC, BL, RST, SPI>
where
    DC: OutputPin,
    BL: OutputPin,
    RST: OutputPin,
    SPI: SpiDevice,
{
    dcx: DC,
    blx: BL,
    rst: RST,
    spi_d: SPI,
    buffer: [u8; constants::BUFFER_SIZE],
    screen_direction: ScreenDirection,
    bias: Bias,
}
impl<DC, BL, RST, SPI> ST7567<DC, BL, RST, SPI>
where
    DC: OutputPin,
    BL: OutputPin,
    RST: OutputPin,
    SPI: SpiDevice,
{
    pub fn new(_dcx: DC, _blx: BL, _rst: RST, _display_spi: SPI, _screen_direction: ScreenDirection, _bias: Bias) -> Self {
        Self {
            dcx: _dcx,
            blx: _blx,
            rst: _rst,
            spi_d: _display_spi,
            buffer: [0; constants::BUFFER_SIZE],
            screen_direction: _screen_direction,
            bias: _bias,
        }
    }

    pub fn init(&mut self) -> Result<(), DisplayErrors> {
        self.reset()?;
        let commands: [u8; 10] = [
            match self.bias {
                Bias::Bias7_1 => constants::ST7567_BIAS_1_7,
                Bias::Bias9_1 => constants::ST7567_BIAS_1_9,
            },
            match self.screen_direction {
                 ScreenDirection::Normal => constants::ST7567_SEG_DIR_NORMAL,
                 ScreenDirection::Reverse => constants::ST7567_SEG_DIR_REV 
            },
            match self.screen_direction {
                ScreenDirection::Normal => constants::ST7567_SETCOMREVERSE,
                ScreenDirection::Reverse => constants::ST7567_SETCOMNORMAL
            },
            constants::ST7567_DISPNORMAL,
            constants::ST7567_SETSTARTLINE | 0x00,
            constants::ST7567_POWERCTRL,
            constants::ST7567_REG_RATIO | 4, // Set regulation resistor ratio - lower brightness
            constants::ST7567_DISPON,        // Turn on display
            constants::ST7567_SETCONTRAST,   // Set contrast command
            30,                              // Set contrast value (example: 30)
        ];
        for &command in commands.iter() {
            self.command(&[command])?;
        }
        Ok(())
    }

    pub fn command(&mut self, data: &[u8]) -> Result<(), DisplayErrors> {
        self.dcx.set_low();
        self.spi_d
            .write(data)
            .map_err(|_| DisplayErrors::SpiError)?;
        Ok(())
    }

    pub fn data(&mut self, data: &[u8]) -> Result<(), DisplayErrors> {
        self.dcx.set_high();
        self.spi_d
            .write(data)
            .map_err(|_| DisplayErrors::SpiError)?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), DisplayErrors> {
        self.rst.set_low().map_err(|_| DisplayErrors::PinError)?;
        self.rst.set_high().map_err(|_| DisplayErrors::PinError)?;
        Ok(())
    }

    //rewrite from pimoroni driver
    pub fn backlight(&mut self, status: BacklightStatus) -> Result<(), DisplayErrors> {
        match status {
            BacklightStatus::On => self.blx.set_high().map_err(|_| DisplayErrors::PinError)?,
            _ => self.blx.set_low().map_err(|_| DisplayErrors::PinError)?,
        };
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        self.buffer = [0; constants::BUFFER_SIZE];
        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), DisplayErrors> {
        if x >= constants::SCREEN_WIDTH as usize || y >= constants::SCREEN_HEIGHT as usize {
            return Err((DisplayErrors::OutOfBoundsError((x, y))));
        }

        let offset = ((y / 8) * constants::SCREEN_WIDTH as usize) + x; // byte index
        let bit = y as u8 % 8;
        if value {
            // ON
            self.buffer[offset] = self.buffer[offset] | 1 << bit;
        } else {
            // OFF
            self.buffer[offset] = self.buffer[offset] & !(1 << bit);
        }
        Ok(())
    }

    pub fn show(&mut self) -> Result<(), DisplayErrors> {
        self.command(&[constants::ST7567_ENTER_RMWMODE]);
        for page in 0..8 {
            let offset: usize = page * constants::SCREEN_WIDTH;
            self.command(&[
                constants::ST7567_SETPAGESTART | page as u8,
                match self.screen_direction {
                    ScreenDirection::Normal => constants::ST7567_SETCOLL,
                    ScreenDirection::Reverse => constants::ST7567_SETCOLL | 0x04
                },
                constants::ST7567_SETCOLH,
            ]);
            let start_offset = offset as usize;
            let end_offset = start_offset + constants::SCREEN_WIDTH;
            let mut data = [0u8; constants::SCREEN_WIDTH];
            data.clone_from_slice(&self.buffer[start_offset..end_offset]);
            self.data(&data)?;
        }
        self.command(&[constants::ST7567_EXIT_RMWMODE])?;
        Ok(())
    }
}

use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::*, Pixel};

impl<DC, BL, RST, SPI> DrawTarget for ST7567<DC, BL, RST, SPI>
where
    DC: OutputPin,
    BL: OutputPin,
    RST: OutputPin,
    SPI: SpiDevice,
{
    type Color = BinaryColor;
    type Error = ();
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            self.set_pixel(x as usize, y as usize, color.is_on());
        }
        Ok(())
    }
}

impl<DC, BL, RST, SPI> OriginDimensions for ST7567<DC, BL, RST, SPI>
where
    DC: OutputPin,
    BL: OutputPin,
    RST: OutputPin,
    SPI: SpiDevice,
{
    fn size(&self) -> Size {
        Size::new(
            constants::SCREEN_WIDTH as u32,
            constants::SCREEN_HEIGHT as u32,
        )
    }
}
