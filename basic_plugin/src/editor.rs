use nih_plug::editor::ParentWindowHandle;
use nih_plug::prelude::Editor;
use nih_plug::context::gui::GuiContext;
use nih_plug_iced::*;
use std::sync::Arc;

pub(crate) fn create() -> Option<Box<dyn Editor>> {
    create_iced_editor::<BasicEditor>(IcedState::from_size(300, 400), ())
}

struct BasicEditor {
    context: Arc<dyn GuiContext>,
}

impl IcedEditor for BasicEditor {
    type Executor = executor::Default;
    type Message = ();
    type InitializationFlags = ();

    fn new(
        _initialization_flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (BasicEditor{
            context: context,
        }, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .padding(20)
            .spacing(15)
            .align_items(Alignment::Center)
            .push(
                Container::new(
                    Text::new("PhatBass VST")
                        .size(32)
                        .color(Color::from_rgb(0.0, 0.7, 1.0))
                )
                .padding(10)
            )
            .push(
                Container::new(
                    Text::new("Audio Plugin")
                        .size(20)
                        .color(Color::from_rgb(0.8, 0.8, 0.8))
                )
                .padding(5)
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(
                            Text::new("Controls")
                                .size(24)
                                .color(Color::from_rgb(1.0, 1.0, 1.0))
                        )
                )
                .width(Length::Fill)
                .padding(15)
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
