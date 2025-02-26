use std::num::NonZeroU32;
use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

mod editor;
#[cfg(feature = "svg")]
pub mod svg;

const PEAK_METER_DECAY_MS: f64 = 150.0;

pub struct Basic {
    parameters: Arc<BasicParameters>,
    peak_meter: Arc<AtomicF32>,
    peak_meter_decay_weight: f32,
    editor_state: Arc<IcedState>,
}

#[derive(Params)]
pub struct BasicParameters {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Basic {
    fn default() -> Self {
        Self {
            parameters: Arc::new(BasicParameters::default()),
            peak_meter: Arc::new(AtomicF32::new(0.0)),
            peak_meter_decay_weight: 0.0,
            editor_state: editor::default_state(),
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
    const NAME: &'static str = "PhatBass's Basic VST3 Plugin ";
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

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;
    const HARD_REALTIME_ONLY: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> std::sync::Arc<dyn Params> {
        self.parameters.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            let gain = self.parameters.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
                amplitude += *sample;
            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed);
            }
        }

        ProcessStatus::Normal
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        Box::new(|_| ())
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.parameters.clone(),
            self.peak_meter.clone(),
            self.editor_state.clone(),
        )
    }

    fn filter_state(_state: &mut PluginState) {}

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