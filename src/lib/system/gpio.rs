use crate::error::trace_log_error;
use crate::error::UdmError;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::gpio_types::GpioMetadata;
use crate::UdmResult;
use bon::Builder;
use gpiocdev::line::Offset;
use gpiocdev::Request;

#[derive(Clone)]
pub(crate) enum GpioLineType {
    GpioLine(String), // BCM Number or "gpio device line"
    PinNumber(u8),    // GPIO SilkScreen
}
impl TryFrom<FluidRegulator> for GpioLineType {
    type Error = UdmError;

    fn try_from(value: FluidRegulator) -> Result<Self, Self::Error> {
        if let Some(gpio_name) = value.gpio_name {
            Ok(GpioLineType::GpioLine(gpio_name))
        } else if let Some(gpio_pin) = value.gpio_pin {
            Ok(GpioLineType::PinNumber(gpio_pin as u8))
        } else {
            Err(trace_log_error(UdmError::InvalidInput(
                "FluidRegulator must have either gpio_name or gpio_pin defined".to_string(),
            )))
        }
    }
}
impl TryFrom<GpioLineType> for Offset {
    type Error = UdmError;

    fn try_from(value: GpioLineType) -> Result<Self, Self::Error> {
        match value {
            GpioLineType::GpioLine(name) => name.parse::<Offset>().map_err(|e| {
                trace_log_error(UdmError::InvalidInput(format!(
                    "Failed to parse GPIO line name to i32: {e}"
                )))
            }),
            GpioLineType::PinNumber(num) => Ok(num as Offset),
        }
    }
}

#[derive(Builder)]
pub(crate) struct GpioCollection {
    pub(crate) gpio_chip_path: String,
    pub(crate) line: GpioLineType,
    pub(crate) metadata: Option<GpioMetadata>,
}

impl GpioCollection {
    pub(crate) fn new(line: GpioLineType, gpio_chip_path: String) -> Self {
        GpioCollection::builder()
            .line(line)
            .gpio_chip_path(gpio_chip_path)
            .build()
    }
    pub(crate) fn poll(&mut self) -> UdmResult<()> {
        let request = self.build_request()?;
        let gpio_lines: Offset = self.line.clone().try_into().map_err(|e| {
            trace_log_error(UdmError::GpioError(format!(
                "Failed to convert GpioLineType to Offset: {e}"
            )))
        })?;
        let value = request
            .value(gpio_lines)
            .map_err(|e| UdmError::GpioError(e.to_string()))?;
        dbg!(value);
        Ok(())
    }

    pub(crate) fn build_request(&self) -> UdmResult<Request> {
        let gpio_lines: Offset = self.line.clone().try_into().map_err(|e| {
            trace_log_error(UdmError::GpioError(format!(
                "Failed to convert GpioLineType to Offset: {e}"
            )))
        })?;
        Request::builder()
            .on_chip(&self.gpio_chip_path)
            .with_line(gpio_lines)
            .with_consumer("gpio_collection")
            .request()
            .map_err(|e| UdmError::GpioError(e.to_string()))
    }
}
// impl GpioFactory for GpioDispensing {}
// impl GpioFactory for GpioStandBy {}
// #[allow(dead_code)]
// trait Calculation {}

// #[allow(dead_code)]
// pub(crate) struct PollGpio {
//     pub(crate) gpio_pin: u8,
//     pub(crate) pin_info: Option<Pin>,
// }
// impl PollGpio {
//     pub fn new(pin: u8) -> UdmResult<Self> {
//         let gpio = Gpio::new().map_err(|e| UdmError::GpioError(e.to_string()))?;
//         Ok(Self {
//             gpio_pin: pin,
//             pin_info: Some(
//                 gpio.get(pin)
//                     .map_err(|e| trace_log_error(UdmError::GpioError(e.to_string())))?,
//             ),
//         })
//     }
// }
