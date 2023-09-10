use snafu::Snafu;
#[cfg(feature = "defmt")]
use defmt::Format;

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    IncorrectDeviceId,
    ReadingDeviceAddressOverI2C,
    WriteConfig,
    WriteControlMeas,
    WriteRegister,
    ReadRegister,
    NormalModeNeedsMeasStandbyTime,
}
