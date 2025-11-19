#![no_std]

// use chrono::{DateTime, Datelike, Timelike as _, Utc};
use defmt::Format;
use serde::{Deserialize, Serialize};
use uom::si::{
    f32::{Frequency, Pressure, VolumeRate},
    frequency::hertz,
    pressure::millibar,
};

pub const REPORT_BYTES: usize = core::mem::size_of::<Report>();
pub const SETPOINT_BYTES: usize = core::mem::size_of::<Setpoint>();
pub const BAUDRATE: u32 = 115200;

pub const SYSTOLE_RATIO_DEFAULT: f32 = 3.0 / 7.0;

pub fn serialize_report(report: Report, buf: &mut [u8]) -> postcard::Result<&mut [u8]> {
    postcard::to_slice_cobs(&report, buf)
}

pub fn deserialize_report(buf: &mut [u8]) -> postcard::Result<Report> {
    postcard::from_bytes_cobs(buf)
}

pub fn serialize_setpoint(setpoint: Setpoint, buf: &mut [u8]) -> postcard::Result<&mut [u8]> {
    postcard::to_slice_cobs(&setpoint, buf)
}

pub fn deserialize_setpoint(buf: &mut [u8]) -> postcard::Result<Setpoint> {
    postcard::from_bytes_cobs(buf)
}

#[derive(Deserialize, Serialize, Clone, Format, Debug)]
pub struct Report {
    pub setpoint: Setpoint,
    pub app_state: AppState,
    pub measurements: Measurements,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Setpoint {
    // pub current_time: DateTimeWrapper,
    pub mockloop_setpoint: Option<MockloopSetpoint>,
    pub heart_controller_setpoint: Option<HeartControllerSetpoint>,
}

/// Setpoint for the mockloop hemodynamics controller
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MockloopSetpoint {
    pub systemic_resistance: f32,
    pub pulmonary_resistance: f32,
    pub systemic_afterload_compliance: f32,
    pub pulmonary_afterload_compliance: f32,
}

/// Setpoint for the pneumatic heart prototype controller
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeartControllerSetpoint {
    /// Desired heart rate
    pub heart_rate: Frequency,
    /// Desired regulator pressure
    pub pressure: Pressure,
    /// Ratio of systole duration to total cardiac phase duration
    /// NOTE: usually 3/7
    pub systole_ratio: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Measurements {
    /// Milliseconds since boot of mcu
    pub timestamp: u64,
    pub regulator_actual_pressure: Pressure,
    pub systemic_flow: VolumeRate,
    pub pulmonary_flow: VolumeRate,
    pub systemic_preload_pressure: Pressure,
    pub systemic_afterload_pressure: Pressure,
    pub pulmonary_preload_pressure: Pressure,
    pub pulmonary_afterload_pressure: Pressure,
}

#[derive(PartialEq, Clone, Copy, Deserialize, Serialize, Format, Default, Debug)]
pub enum AppState {
    #[default]
    StandBy,
    Running,
    Fault,
}

impl AppState {
    pub fn next(self) -> Self {
        match self {
            AppState::StandBy => AppState::Running,
            AppState::Running => AppState::Fault,
            AppState::Fault => AppState::StandBy,
        }
    }
}

// Format impls from here
impl Format for Measurements {
    fn format(&self, fmt: defmt::Formatter) {
        use uom::si::pressure::millimeter_of_mercury;
        use uom::si::volume_rate::liter_per_minute;

        defmt::write!(
            fmt,
            "Measurement(time: {} ms - reg: {} mmHg, sf: {} lpm, pf: {} lpm, spp: {} mmHg, sap: {} mmHg, ppp: {} mmHg, pap: {} mmHg)",
            self.timestamp,
            self.regulator_actual_pressure
                .get::<millimeter_of_mercury>(),
            self.systemic_flow.get::<liter_per_minute>(),
            self.systemic_flow.get::<liter_per_minute>(),
            self.systemic_preload_pressure
                .get::<millimeter_of_mercury>(),
            self.systemic_afterload_pressure
                .get::<millimeter_of_mercury>(),
            self.pulmonary_preload_pressure
                .get::<millimeter_of_mercury>(),
            self.pulmonary_afterload_pressure
                .get::<millimeter_of_mercury>(),
        );
    }
}

impl Format for Setpoint {
    fn format(&self, fmt: defmt::Formatter) {
        use defmt::write;

        write!(fmt, "Setpoint( Heart: ");
        match &self.heart_controller_setpoint {
            Some(sp) => write!(
                fmt,
                "(rate: hr: {}hz,  pressure: {}mbar, systole_ratio: {})",
                sp.heart_rate.get::<hertz>(),
                sp.pressure.get::<millibar>(),
                sp.systole_ratio,
            ),
            None => write!(fmt, "DISABLED"),
        };
        write!(fmt, " - Loop: ");
        match &self.mockloop_setpoint {
            Some(sp) => write!(
                fmt,
                "(resistance sys/pul: {}/{}, compliance sys/pul {}/{})",
                sp.systemic_resistance,
                sp.pulmonary_resistance,
                sp.systemic_afterload_compliance,
                sp.pulmonary_afterload_compliance,
            ),
            None => write!(fmt, "DISABLED"),
        };

        write!(fmt, " )");
    }
}
