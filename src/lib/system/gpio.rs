use crate::error::trace_log_error;
use crate::error::UdmError;
// use crate::rpc_types::gpio_types;
use crate::UdmResult;
use rppal::gpio::Gpio;
use rppal::gpio::Pin;
#[allow(dead_code)]
pub(crate) trait PollSysDevice {
    fn collect(&mut self) -> UdmResult<Self>
    where
        Self: std::marker::Sized;
}

#[allow(dead_code)]
trait Calculation {}

#[allow(dead_code)]
pub(crate) struct PollGpio {
    pub(crate) gpio_pin: u8,
    pub(crate) pin_info: Option<Pin>,
}
impl PollGpio {
    pub fn new(gpio: Gpio, pin: u8) -> UdmResult<Self> {
        Ok(Self {
            gpio_pin: pin,
            pin_info: Some(
                gpio.get(pin)
                    .map_err(|e| trace_log_error(UdmError::GpioError(e.to_string())))?,
            ),
        })
    }
}
