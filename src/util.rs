/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 31-03-2022
* *[filename] util.rs
* *[summary] Utilities regarding technology (.tlef) parsing functionalities.
* ***********************************
*/
// crates.io
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

/** TYPE
* *[name]   TLefDecimal 
* *[description] Internal decimal type for all the decimal-valued data
*/
pub type TLefDecimal = rust_decimal::Decimal;

/** TYPE
* *[name]   V5P4/8 
* *[description] Static short-hands for common LEF Decimal
* *              values indicating the specfication version of the LEF file.
*/
pub(crate) static V5P8: Lazy<TLefDecimal> = Lazy::new(|| TLefDecimal::from_str("5.8").unwrap());
pub(crate) static V5P4: Lazy<TLefDecimal> = Lazy::new(|| TLefDecimal::from_str("5.4").unwrap());

/**
* *[name] <object name>
* *[description] <brief description>
* ![deprecated] <deprecation description> (if deprecated)
* [variables]
* @par <param name> (type) : <first var. description>
*/
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LefDbuPerMicron(pub u32);
impl LefDbuPerMicron {
    /// Create a new [TLefDbuPerMicron], checking internally required conditions
    pub fn try_new(x: LefDecimal) -> LefResult<Self> {
        // Check that this is an integer, no fractional part
        if !x.fract().is_zero() {
            return Err("DBU per Micron must be an integer".into());
        }
        // Check the for allowed values
        if ![100, 200, 400, 800, 1000, 2000, 4000, 8000, 10_000, 20_000].contains(&x.mantissa()) {
            return Err("Invalid DBU per Micron value".into());
        }
        // Convert to u32. Note the `unwrap` here is safe,
        // as we have already verified `mantissa` is in the list above,
        // all of which fit in a u32.
        let val = u32::try_from(x.mantissa()).unwrap();
        Ok(Self(val))
    }
    /// Return `self`'s value as an integer.
    pub fn value(&self) -> u32 {
        self.0
    }
}
/// # Lef Physical-Dimension Units
///
/// Conversion factors for a variety of physical quantities.  
/// Only the distance-measurement `database_microns` is supported.
///
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LefUnits {
    /// Database Distance Units per Micron
    /// Defaults to 100, i.e. 1 DBU = 10nm
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_microns: Option<LefDbuPerMicron>,

    // Unsupported Fields
    #[serde(default, skip_serializing)]
    pub time_ns: Unsupported,
    #[serde(default, skip_serializing)]
    pub capacitance_pf: Unsupported,
    #[serde(default, skip_serializing)]
    pub resistance_ohms: Unsupported,
    #[serde(default, skip_serializing)]
    pub power_mw: Unsupported,
    #[serde(default, skip_serializing)]
    pub current_ma: Unsupported,
    #[serde(default, skip_serializing)]
    pub voltage_volts: Unsupported,
    #[serde(default, skip_serializing)]
    pub frequency_mhz: Unsupported,
}