use std::fmt::Debug;
use anyhow::Error;
use embedded_hal::delay::DelayNs;

#[allow(dead_code)]
pub trait DelaySec {
    fn delay_sec(&mut self, sec: u32);
}

impl<T> DelaySec for T
    where T: DelayNs 
{
    fn delay_sec(&mut self, sec: u32) {
        self.delay_ms(sec * 1000);
    }    
}

#[allow(dead_code)]
pub trait IntoAnyhow<R> {
    fn into_anyhow(self) -> Result<R, Error>;
}

impl<R, E: Debug> IntoAnyhow<R> for Result<R, E> {
    fn into_anyhow(self) -> Result<R, Error> {
        return match self {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow::anyhow!("{:?}", e)),
        }
    }
}