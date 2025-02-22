use std::num::NonZeroU32;

use nih_plug::prelude::*;
use std::sync::Arc;

mod editor;

pub struct Basic {
    parameters: Arc<BasicParameters>,
}

#[derive(Params)]
struct BasicParameters {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Basic {
    fn default() -> Self {
        Self {
            parameters: Arc::new(BasicParameters::default()),
        }
    }
}

impl Default for BasicParameters {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for Basic {
    const NAME: &'static str = "PhatBass's Basic Plugin ";
    const VENDOR: &'static str = "PhatBass";
    const URL: &'static str = "https://phatbass.com";
    const EMAIL: &'static str = "info@phatbass.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [nih_plug::prelude::AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        }
    ];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> std::sync::Arc<dyn nih_plug::prelude::Params> {
        return self.parameters.clone()
    }

    fn process(
        &mut self,
        buffer: &mut nih_plug::prelude::Buffer,
        _aux: &mut nih_plug::prelude::AuxiliaryBuffers,
        _context: &mut impl nih_plug::prelude::ProcessContext<Self>,
    ) -> nih_plug::prelude::ProcessStatus {

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                *sample = 0.0;
            }
        }
        
        ProcessStatus::Normal
    }

    const MIDI_INPUT: nih_plug::prelude::MidiConfig = nih_plug::prelude::MidiConfig::None;

    const MIDI_OUTPUT: nih_plug::prelude::MidiConfig = nih_plug::prelude::MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    const HARD_REALTIME_ONLY: bool = false;

    fn task_executor(&mut self) -> nih_plug::prelude::TaskExecutor<Self> {
        // In the default implementation we can simply ignore the value
        Box::new(|_| ())
    }

    fn editor(&mut self, _async_executor: nih_plug::prelude::AsyncExecutor<Self>) -> Option<Box<dyn nih_plug::prelude::Editor>> {
        editor::create()
    }

    fn filter_state(_state: &mut nih_plug::prelude::PluginState) {}

    fn initialize(
        &mut self,
        _audio_io_layout: &nih_plug::prelude::AudioIOLayout,
        _buffer_config: &nih_plug::prelude::BufferConfig,
        _context: &mut impl nih_plug::prelude::InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Basic {
    const VST3_CLASS_ID: [u8; 16] = *b"PhatbassGain0001";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx, Vst3SubCategory::Tools
    ];
}

nih_export_vst3!(Basic);
