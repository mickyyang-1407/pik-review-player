use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqProfile {
    pub name: String,
    pub preamp: f64,
    pub bands: Vec<EqBand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqBand {
    #[serde(rename = "type")]
    pub filter_type: String, // "Peaking", "LowShelf", "HighShelf"
    pub fc: f64,
    pub q: f64,
    pub gain: f64,
}

impl EqProfile {
    pub fn to_mpv_af_string(&self) -> String {
        let mut eq_params = Vec::new();
        for band in &self.bands {
            if band.filter_type == "Peaking" {
                // anequalizer expects width in Hz by default, or 'q' suffix for Q factor
                // c0 = channel 0 (left), c1 = channel 1 (right)
                eq_params.push(format!("c0 f={} w={}q g={}", band.fc, band.q, band.gain));
                eq_params.push(format!("c1 f={} w={}q g={}", band.fc, band.q, band.gain));
            }
            // For now, only handle Peaking or treat everything as Peaking for anequalizer
            // FFmpeg anequalizer doesn't natively support LowShelf/HighShelf in the same way,
            // but we can map them if needed, or just rely on Peaking.
        }
        
        let anequalizer_str = eq_params.join("|");
        // preamp can be handled by volume plugin
        format!("volume=volume={}dB,lavfi=[anequalizer={}]", self.preamp, anequalizer_str)
    }

    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_json(&content)
    }
}
