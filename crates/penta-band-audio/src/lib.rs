#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioBackend;

impl AudioBackend {
    #[cfg(feature = "native-audio")]
    pub fn default_host_available() -> bool {
        use cpal::traits::HostTrait;

        cpal::default_host().default_output_device().is_some()
    }

    #[cfg(not(feature = "native-audio"))]
    pub fn default_host_available() -> bool {
        false
    }
}

#[cfg(feature = "native-audio")]
pub fn midi_support_is_available() -> bool {
    midir::MidiOutput::new("Penta Band").is_ok()
}

#[cfg(not(feature = "native-audio"))]
pub fn midi_support_is_available() -> bool {
    false
}
