use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "81";

pub const ENCODER_COUNT: usize = 0;

#[derive(Debug, Clone)]
pub enum Kind {
    AKP815,
}

pub const MIRABOX_VID: u16 = 0x5548;
pub const AKP815_PID: u16 = 0x6672;

// Map query to usage page 65440 and usage id 1
pub const AKP815_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, MIRABOX_VID, AKP815_PID);

pub const QUERIES: [DeviceQuery; 1] = [
    AKP815_QUERY,
];

/// Returns correct image format for device kind and key
pub fn get_image_format_for_key(_kind: &Kind, _key: u8) -> ImageFormat {
    ImageFormat {
        mode: ImageMode::JPEG,
        size: (100, 100),
        rotation: ImageRotation::Rot180,
        mirror: ImageMirroring::None,
    }
}

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match (vid, pid) {
            (MIRABOX_VID, AKP815_PID) => Some(Kind::AKP815),
            _ => None,
        }
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        1
    }

    /// Custom name for the device
    pub fn human_name(&self) -> String {
        "Ajazz AKP815".to_string()
    }

    /// ID suffix for device tracking
    pub fn id_suffix(&self) -> String {
        "815".to_string()
    }

    pub fn row_count(&self) -> usize {
        3
    }

    pub fn col_count(&self) -> usize {
        5
    }

    pub fn key_count(&self) -> usize {
        15
    }

    pub fn encoder_count(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
