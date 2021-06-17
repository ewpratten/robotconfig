use serde::{Serialize, Deserialize};

/// JSON object for a motor
///
/// ```json
/// {
///    "id": 0,
///    "inverted": false // Default: false
/// }
/// ```
///
/// Original source: [here](https://github.com/ewpratten/lib5k/blob/master/lib5k/src/main/java/io/github/frc5024/lib5k/config/types/JSONMotor.java)
#[derive(Serialize, Deserialize)]
pub struct Motor {

    /// Hardware ID of the motor
    pub id: u32,

    /// Weather the outputs are inverted
    pub inverted: Option<bool>
}