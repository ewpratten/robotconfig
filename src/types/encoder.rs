use serde::{Serialize, Deserialize};

/// JSON object for an encoder
///
/// ```json
/// {
///    "cpr": 0,
///    "inverted": false // Default: false
/// }
/// ```
///
/// Original source: [here](https://github.com/ewpratten/lib5k/blob/master/lib5k/src/main/java/io/github/frc5024/lib5k/config/types/JSONEncoder.java)
#[derive(Serialize, Deserialize)]
pub struct Encoder {

    /// Number of encoder counts (or pulses) per full rotation of the sensor
    pub cpr: i32,

    /// Weather the inputs are inverted
    pub inverted: Option<bool>
}