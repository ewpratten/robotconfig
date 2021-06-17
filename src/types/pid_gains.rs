use serde::{Serialize, Deserialize};

/// JSON object for PID gains
///
/// ```json
/// {
///    "Kp": 0.0,
///    "Ki": 0.0,
///    "Kd": 0.0,
///    "FF": 0.0, // Default: 0.0
///    "settlingTime": 0.0 // Seconds. Default: 0.0
/// }
/// ```
///
/// Original source: [here](https://github.com/ewpratten/lib5k/blob/master/lib5k/src/main/java/io/github/frc5024/lib5k/config/types/JSONPIDGains.java)
#[derive(Serialize, Deserialize)]
pub struct PidGains {

    /// **P** gain constant
    #[serde(rename = "Kp")]
    pub p_gain: f32,

    /// **I** gain constant
    #[serde(rename = "Ki")]
    pub i_gain: f32,

    /// **D** gain constant
    #[serde(rename = "Kd")]
    pub d_gain: f32,

    /// Feed-Forward constant
    #[serde(rename = "FF")]
    pub feed_forward: Option<f32>,

    /// Number of seconds to let the controller settle for
    #[serde(rename = "settlingTime")]
    pub settling_time: Option<f32>,
    
}