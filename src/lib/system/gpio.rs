use crate::error::trace_log_error;
use crate::error::UdmError;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::gpio_types::GpioMetadata;
use crate::UdmResult;
use bon::Builder;
use gpiod::Chip;
use gpiod::Options;

#[derive(Clone, Debug)]
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
impl TryFrom<GpioLineType> for u32 {
    type Error = UdmError;

    fn try_from(value: GpioLineType) -> Result<Self, Self::Error> {
        match value {
            GpioLineType::GpioLine(name) => name
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
                .map_err(|_| {
                    trace_log_error(UdmError::InvalidInput(format!(
                        "No valid numeric u32 found in GPIO line name: {name}"
                    )))
                }),
            GpioLineType::PinNumber(num) => Ok(num as u32),
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
    fn collect_chip(&self) -> Option<u8> {
        self.gpio_chip_path
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u8>()
            .ok()
    }
    pub(crate) fn poll(&mut self) -> UdmResult<()> {
        tracing::debug!("Polling GPIO Line: {:?}", &self.line);
        let request = self.build_chip()?;
        let gpio_lines: u32 = self.line.clone().try_into().map_err(|e| {
            trace_log_error(UdmError::GpioError(format!(
                "Failed to convert GpioLineType to u32: {e}"
            )))
        })?;
        // let opts = Options::input([gpio_lines]).consumer("Udm Gpio Collection");
        // let inputs = request
        //     .request_lines(opts)
        //     .map_err(|e| UdmError::GpioError(e.to_string()))?;
        // let values = inputs.get_values([false; 3])?;
        let line_info = request.line_info(gpio_lines)?;
        // dbg!(values);
        Ok(())
    }

    pub(crate) fn build_chip(&self) -> UdmResult<Chip> {
        if let Some(chip_num) = self.collect_chip() {
            let chip = Chip::new(chip_num).map_err(|e| UdmError::GpioError(e.to_string()))?;
            Ok(chip)
        } else {
            Err(UdmError::GpioError("Invalid Gpio Chip".to_string()))?
        }
    }
}
