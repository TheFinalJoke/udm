// use crate::error::trace_log_error;
// use crate::error::UdmError;
// // use crate::rpc_types::gpio_types;
// use crate::UdmResult;

// #[async_trait::async_trait]
// pub(crate) trait SysDevice {
//     async fn collect(&mut self) -> Option<Pin>
//     where
//         Self: std::marker::Sized;
// }

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
