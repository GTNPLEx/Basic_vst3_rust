use atomic_float::AtomicF32;
use nih_plug::editor::ParentWindowHandle;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::*;
use std::sync::Arc;
use std::time::Duration;
use nih_plug_iced::widgets as nih_widgets;

pub mod knob;

// Custom colors
const BACKGROUND_DARK: Color = Color::from_rgb(0.12, 0.12, 0.12);
const BACKGROUND_LIGHTER: Color = Color::from_rgb(0.18, 0.18, 0.18);

use crate::BasicParameters;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(900, 600)
}

#[derive(Default)]
struct Style {
    background: Option<Background>,
}

impl container::StyleSheet for Style {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.background.clone(),
            ..Default::default()
        }
    }
}

pub(crate) fn create(
    params: Arc<BasicParameters>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<BasicEditor>(editor_state, (params, peak_meter))
}

struct HeaderState {
    preset_name: String,
    pick_list_state: pick_list::State<String>,
}

impl HeaderState {
    fn as_string(&self) -> String {
        self.preset_name.clone()
    }
    
    fn new() -> Self {
        Self {
            preset_name: "Default".to_string(),
            pick_list_state: pick_list::State::default(),
        }
    }
}

struct BasicEditor {
    context: Arc<dyn GuiContext>,
    header_state: HeaderState,
    params: Arc<BasicParameters>, 
    peak_meter: Arc<AtomicF32>,
    gain_slider_state: nih_widgets::param_slider::State,
    peak_meter_state: nih_widgets::peak_meter::State,
}

// Define Message enum for handling preset selection
#[derive(Debug, Clone)]
enum Message {
    PresetSelected(String),
    
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for BasicEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<BasicParameters>, Arc<AtomicF32>);

    fn new(
        (params, peak_meter):  Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (
            BasicEditor {
                context,
                header_state: HeaderState::new(),
                params,
                peak_meter,
                gain_slider_state: Default::default(),
                peak_meter_state: Default::default(),
            },
            Command::none(),
        )
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::PresetSelected(preset_name) => {
                // Update the selected preset name
                self.header_state.preset_name = preset_name;
            },

            Message::ParamUpdate(message) => self.handle_param_message(message),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Define preset options
        let preset_options = vec!["Default".to_string(), "Preset-1".to_string(), "Preset-2".to_string()];
        // Get the current gain value
        let gain_value = self.params.gain.value();
         // Get the current peak meter value
        let peak_meter_value = self.peak_meter.load(std::sync::atomic::Ordering::Acquire);
        
        //let knob_svg =Handle::from_path("resources/knob.svg"); // Path to your SVG file
        let knob_svg = knob::load_knob_svg();

        let content = Column::new()
            .padding(10)
            .spacing(15)
            .align_items(Alignment::Start)
            .push(
                Container::new(
                    Column::new()
                        .spacing(20)
                        .width(Length::Fill)
                        .push(
                            Text::new("PhatBass VST")
                                .size(32)
                                .color(Color::from_rgb(0.0, 0.7, 1.0))
                        )
                        .push(
                            Row::new()
                                .spacing(10)
                                .push(Text::new("v1.0.0"))
                                .push(Space::with_width(Length::Fill)) // Add flexible space
                        )


            .push(
                Text::new("Gain")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            .push(
                nih_widgets::PeakMeter::new(
                    &mut self.peak_meter_state,
                    util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
                )
                .hold_time(Duration::from_millis(600)),
            )            

                )
                .padding(10)
                .style(Style {
                    background: Some(Background::Color(BACKGROUND_LIGHTER)),
                })
            )



            .push(Text::new(&format!("PRESET: {}", self.header_state.preset_name)))
            .push(
                PickList::new(
                    &mut self.header_state.pick_list_state,
                    preset_options,
                    Some(self.header_state.preset_name.clone()),
                    Message::PresetSelected
                )
            )


            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(
                            Text::new("Controls")
                                .size(24)
                                .color(Color::from_rgb(0.9, 0.9, 0.9))  // Almost white text
                        )
                )
                .width(Length::Fill)
                .padding(15)
                .style(Style {
                    background: Some(Background::Color(BACKGROUND_LIGHTER)),
                })
            );

            

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Style {
                background: Some(Background::Color(BACKGROUND_DARK)),
            })
            .into()
    }
}