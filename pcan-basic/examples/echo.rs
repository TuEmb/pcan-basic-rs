use embedded_can::Frame;
use pcan_basic;

struct Driver<Can>(Can);

impl<Can> Driver<Can>
where
    Can: embedded_can::blocking::Can,
    Can::Error: core::fmt::Debug,
{
    pub fn echo(&mut self) {
        println!("Enter Echo mode");
        let frame = self.0.try_read().unwrap();
        println!("Receive: {:?}", frame.id());
        self.0.try_write(&frame).unwrap();
    }
}

fn main() -> anyhow::Result<()> {
    let can = pcan_basic::Interface::init(0x011C)?;
    let mut driver = Driver(can);
    driver.echo();

    Ok(())
}
