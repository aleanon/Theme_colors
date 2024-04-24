use iced::{
    theme::{self, palette},
    widget::{
        self, button, column, container, row, shader::wgpu::naga::back, text, text_input, Column,
        Container,
    },
    Application, Border, Color, Command, Element, Length, Padding, Settings, Size, Theme,
};

fn main() {
    let mut settings = Settings::default();
    settings.window.min_size = Some(Size {
        width: 900.,
        height: 780.,
    });

    ThemeColors::run(settings).unwrap()
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ResetSelected,
    ResetAll,
    SelectTheme(Theme),
    SelectColor(usize),
    AdjustRed(f32),
    AdjustGreen(f32),
    AdjustBlue(f32),
    AdjustAlpha(f32),
}

pub struct ThemeColors {
    theme: Theme,
    colors: [Color; 15],
    selected_color: usize,
}

impl Application for ThemeColors {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::Dark;
        let colors = Self::populate_colors_array(&theme);
        let colorpicker = Self {
            theme,
            colors,
            selected_color: 0,
        };

        (colorpicker, iced::Command::none())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::None => {}
            Message::ResetSelected => {
                let colors = Self::populate_colors_array(&self.theme);
                self.colors[self.selected_color] = colors[self.selected_color];
            }
            Message::ResetAll => self.colors = Self::populate_colors_array(&self.theme),
            Message::SelectTheme(theme) => {
                self.colors = Self::populate_colors_array(&theme);
                self.theme = theme;
            }
            Message::SelectColor(selected) => self.selected_color = selected,
            Message::AdjustRed(new_value) => self.adjust_selected_red_color(new_value),
            Message::AdjustGreen(new_value) => self.adjust_selected_green_color(new_value),
            Message::AdjustBlue(new_value) => self.adjust_selected_blue_color(new_value),
            Message::AdjustAlpha(new_value) => self.adjust_selected_alpha_color(new_value),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let theme_picker = widget::PickList::new(iced::Theme::ALL, Some(&self.theme), |theme| {
            Message::SelectTheme(theme)
        });

        let top_container = container(theme_picker)
            .center_x()
            .width(Length::Fill)
            .padding(10);

        let color_strength: Column<'_, Message, Self::Theme, iced::Renderer> = column!(
            widget::Space::new(1, 30),
            container(text("Base")).height(150).width(50).center_y(),
            container(text("Weak")).height(150).width(50).center_y(),
            container(text("Strong")).height(150).width(50).center_y(),
        )
        .spacing(10);

        let mut background = column!(text("Background").height(30))
            .spacing(10)
            .align_items(iced::Alignment::Center);
        let mut primary = column!(text("Primary").height(30))
            .spacing(10)
            .align_items(iced::Alignment::Center);
        let mut secondary = column!(text("Secondary").height(30))
            .spacing(10)
            .align_items(iced::Alignment::Center);
        let mut success = column!(text("Success").height(30))
            .spacing(10)
            .align_items(iced::Alignment::Center);
        let mut danger = column!(text("Danger").height(30))
            .spacing(10)
            .align_items(iced::Alignment::Center);

        for (i, color) in self.colors.iter().enumerate() {
            let color_view =
                container(column!())
                    .width(150)
                    .height(150)
                    .style(container::Appearance {
                        background: Some(iced::Background::Color(*color)),
                        ..Default::default()
                    });
            let mut select_color = container(
                button(color_view)
                    .padding(2)
                    .style(theme::Button::Text)
                    .on_press(Message::SelectColor(i)),
            );

            if i == self.selected_color {
                select_color = select_color.style(selected_style)
            }

            match i {
                0..=2 => background = background.push(select_color),
                3..=5 => primary = primary.push(select_color),
                6..=8 => secondary = secondary.push(select_color),
                9..=11 => success = success.push(select_color),
                12..=14 => danger = danger.push(select_color),
                _ => unreachable!(),
            }
        }

        let grid = row!(
            color_strength,
            background,
            primary,
            secondary,
            success,
            danger
        )
        .spacing(10);

        let rgba_selected = self.colors[self.selected_color].into_rgba8();

        let adjust_red = {
            let text = text("Red").width(50);
            let slider =
                widget::Slider::new(0.0..=1.0, self.colors[self.selected_color].r, |new_value| {
                    Message::AdjustRed(new_value)
                })
                .step(0.01)
                .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.2}", &self.colors[self.selected_color].r)).on_input(
                    |mut input| {
                        if input.is_empty() {
                            input.push('0')
                        }
                        if let Ok(value) = input.parse::<f32>() {
                            Message::AdjustRed(value)
                        } else {
                            Message::None
                        }
                    }
                ),
                text_input("", &rgba_selected[0].to_string()).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<u8>() {
                        Message::AdjustRed(value as f32 / 255.0)
                    } else {
                        Message::None
                    }
                })
            )
            .width(350);
            column!(text_and_slider, inputs).spacing(5)
        };

        let adjust_green = {
            let text = text("Green").width(50);
            let slider =
                widget::Slider::new(0.0..=1.0, self.colors[self.selected_color].g, |new_value| {
                    Message::AdjustGreen(new_value)
                })
                .step(0.01)
                .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.2}", &self.colors[self.selected_color].g)).on_input(
                    |mut input| {
                        if input.is_empty() {
                            input.push('0')
                        }
                        if let Ok(value) = input.parse::<f32>() {
                            Message::AdjustGreen(value)
                        } else {
                            Message::None
                        }
                    }
                ),
                text_input("", &rgba_selected[1].to_string()).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<u8>() {
                        Message::AdjustGreen(value as f32 / 255.0)
                    } else {
                        Message::None
                    }
                })
            )
            .width(350);
            column!(text_and_slider, inputs).spacing(5)
        };

        let adjust_blue = {
            let text = text("Blue").width(50);
            let slider = widget::Slider::new(
                0.00..=1.00,
                self.colors[self.selected_color].b,
                |new_value| Message::AdjustBlue(new_value),
            )
            .step(0.01)
            .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.2}", &self.colors[self.selected_color].b)).on_input(
                    |mut input| {
                        if input.is_empty() {
                            input.push('0')
                        }
                        if let Ok(value) = input.parse::<f32>() {
                            Message::AdjustBlue(value)
                        } else {
                            Message::None
                        }
                    }
                ),
                text_input("", &rgba_selected[2].to_string()).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<u8>() {
                        Message::AdjustBlue(value as f32 / 255.0)
                    } else {
                        Message::None
                    }
                })
            )
            .width(350);
            column!(text_and_slider, inputs).spacing(5)
        };

        let adjust_alpha = {
            let text = text("Alpha").width(50);
            let slider =
                widget::Slider::new(0.0..=1.0, self.colors[self.selected_color].a, |new_value| {
                    Message::AdjustAlpha(new_value)
                })
                .step(0.01)
                .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.2}", &self.colors[self.selected_color].a)).on_input(
                    |mut input| {
                        if input.is_empty() {
                            input.push('0')
                        }
                        if let Ok(value) = input.parse::<f32>() {
                            Message::AdjustAlpha(value)
                        } else {
                            Message::None
                        }
                    }
                ),
                text_input("", &rgba_selected[3].to_string()).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<u8>() {
                        Message::AdjustAlpha(value as f32 / 255.0)
                    } else {
                        Message::None
                    }
                })
            )
            .width(350);
            column!(text_and_slider, inputs).spacing(5)
        };

        let red_green = row!(adjust_red, adjust_green).spacing(10);
        let blue_alpha = row!(adjust_blue, adjust_alpha).spacing(10);
        let sliders = container(column!(red_green, blue_alpha).spacing(10))
            .width(Length::Fill)
            .center_x();
        let reset = container(
            row!(
                button(text("Reset Selected"))
                    .style(theme::Button::Primary)
                    .on_press(Message::ResetSelected)
                    .width(120),
                button(text("Reset All"))
                    .style(theme::Button::Primary)
                    .on_press(Message::ResetAll)
                    .width(120)
            )
            .spacing(30),
        )
        .width(Length::Fill)
        .center_x();

        let content = column!(top_container, grid, sliders, reset)
            .align_items(iced::Alignment::Center)
            .spacing(15);

        widget::container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .into()
    }

    fn title(&self) -> String {
        "Theme Colors".to_string()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl ThemeColors {
    fn adjust_selected_red_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }

        self.colors[self.selected_color].r = new_value;
    }

    fn adjust_selected_green_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }
        self.colors[self.selected_color].g = new_value;
    }

    fn adjust_selected_blue_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }
        self.colors[self.selected_color].b = new_value;
    }

    fn adjust_selected_alpha_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }
        self.colors[self.selected_color].a = new_value;
    }

    fn populate_colors_array(theme: &Theme) -> [Color; 15] {
        let palette = theme.extended_palette();
        [
            palette.background.base.color,
            palette.background.weak.color,
            palette.background.strong.color,
            palette.primary.base.color,
            palette.primary.weak.color,
            palette.primary.strong.color,
            palette.secondary.base.color,
            palette.secondary.weak.color,
            palette.secondary.strong.color,
            palette.success.base.color,
            palette.success.weak.color,
            palette.success.strong.color,
            palette.danger.base.color,
            palette.danger.weak.color,
            palette.danger.strong.color,
        ]
    }
}

fn selected_style(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        border: Border {
            color: palette.background.base.color.inverse(),
            width: 1.,
            ..Default::default()
        },
        ..Default::default()
    }
}
