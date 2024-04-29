#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use iced::{
    border::Radius,
    theme::{
        self,
        palette::{Danger, Extended, Pair, Primary, Secondary, Success},
        Palette,
    },
    widget::{self, button, column, container, row, text, text_input, tooltip::Position, Column},
    window::{self, Icon},
    Application, Background, Border, Color, Command, Length, Settings, Size, Theme,
};

fn main() {
    let mut settings = Settings::default();
    settings.window.min_size = Some(Size {
        width: 900.,
        height: 880.,
    });

    ThemeColors::run(settings).unwrap()
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ResetSelected,
    ResetAll,
    GenerateFromBase,
    SelectWorkingTheme(Theme),
    SelectAppTheme(Theme),
    SelectColor(Select),
    AdjustRed(f32),
    AdjustGreen(f32),
    AdjustBlue(f32),
    AdjustAlpha(f32),
    // ToggleThemeSelection,
    // ToggleLightDarkTheme,
    TryTheme,
}

#[derive(Debug, Clone)]
pub enum Select {
    Palette(usize),
    Extended((usize, usize)),
}

pub struct ThemeColors {
    themes: [Theme; 22],
    app_theme: Theme,
    working_theme: Theme,
    palette: [Color; 5],
    extended: [[Color; 2]; 15],
    selected: Select,
}

impl Application for ThemeColors {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let working_theme = iced::Theme::Dark;
        let extended = Self::populate_extended_array(working_theme.extended_palette());
        let palette = Self::populate_palette_array(working_theme.palette());
        let colorpicker = Self {
            themes: Self::themes_array(),
            app_theme: Theme::Dark,
            working_theme,
            palette,
            extended,
            selected: Select::Palette(0),
        };

        (colorpicker, iced::Command::none())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::None => {}
            Message::ResetSelected => match self.selected {
                Select::Palette(index) => {
                    let palette = Self::populate_palette_array(self.working_theme.palette());
                    self.palette[index] = palette[index];
                }
                Select::Extended((index1, index2)) => {
                    let extended =
                        Self::populate_extended_array(self.working_theme.extended_palette());
                    self.extended[index1][index2] = extended[index1][index2];
                }
            },
            Message::ResetAll => {
                self.palette = Self::populate_palette_array(self.working_theme.palette());
                self.extended = Self::populate_extended_array(self.working_theme.extended_palette())
            }
            Message::GenerateFromBase => self.generate_extended_from_palette(),
            Message::SelectAppTheme(theme) => self.app_theme = theme,
            Message::SelectWorkingTheme(theme) => {
                self.palette = Self::populate_palette_array(theme.palette());
                self.extended = Self::populate_extended_array(theme.extended_palette());
                self.working_theme = theme;
            }
            Message::SelectColor(selected) => self.selected = selected,
            Message::AdjustRed(new_value) => self.adjust_selected_red_color(new_value),
            Message::AdjustGreen(new_value) => self.adjust_selected_green_color(new_value),
            Message::AdjustBlue(new_value) => self.adjust_selected_blue_color(new_value),
            Message::AdjustAlpha(new_value) => self.adjust_selected_alpha_color(new_value),
            // Message::ToggleThemeSelection => self.use_selected_theme = !self.use_selected_theme,
            // Message::ToggleLightDarkTheme => self.light_theme = !self.light_theme,
            Message::TryTheme => {
                self.themes[0] = Theme::custom_with_fn(
                    "Custom".to_string(),
                    self.palette_from_palette_array(),
                    |palette| self.extendedpalette_from_colors_array(palette),
                );
                self.working_theme = self.themes[0].clone();
                self.app_theme = self.themes[0].clone();
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let label = text("App Theme");
        let app_theme_picker =
            widget::pick_list(self.themes.as_slice(), Some(&self.app_theme), |theme| {
                Message::SelectAppTheme(theme)
            });
        let label_and_app_theme_picker = row!(label, app_theme_picker)
            .spacing(5)
            .align_items(iced::Alignment::Center);

        let label = text("Working Theme");
        let working_theme_picker =
            widget::PickList::new(self.themes.as_slice(), Some(&self.working_theme), |theme| {
                Message::SelectWorkingTheme(theme)
            });
        let label_and_working_theme_picker = row!(label, working_theme_picker)
            .spacing(5)
            .align_items(iced::Alignment::Center);

        let top_container = container(
            row!(label_and_app_theme_picker, label_and_working_theme_picker)
                .spacing(20)
                .align_items(iced::Alignment::Center),
        )
        .center_x()
        .width(Length::Fill)
        .padding(10);

        let palette = {
            let label = row!(
                widget::Space::new(50, 1),
                text("Palette")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(Length::Fill)
            .spacing(10);

            let mut background = column!(text("Background"))
                .align_items(iced::Alignment::Center)
                .spacing(10);
            let mut primary = column!(text("Primary"))
                .align_items(iced::Alignment::Center)
                .spacing(10);
            let mut text_color = column!(text("Text"))
                .align_items(iced::Alignment::Center)
                .spacing(10);
            let mut success = column!(text("Success"))
                .align_items(iced::Alignment::Center)
                .spacing(10);
            let mut danger = column!(text("Danger"))
                .align_items(iced::Alignment::Center)
                .spacing(10);

            for (i, color) in self.palette.iter().enumerate() {
                let color_view =
                    container(column!())
                        .width(150)
                        .height(110)
                        .style(container::Appearance {
                            background: Some(Background::Color(*color)),
                            ..container::Appearance::default()
                        });

                let color_selector = button(color_view)
                    .padding(2)
                    .style(theme::Button::Text)
                    .on_press(Message::SelectColor(Select::Palette(i)));

                let mut container = container(color_selector);

                if let Select::Palette(selected) = self.selected {
                    if selected == i {
                        container = container.style(selected_style)
                    }
                }

                match i {
                    0 => background = background.push(container),
                    1 => primary = primary.push(container),
                    2 => text_color = text_color.push(container),
                    3 => success = success.push(container),
                    4 => danger = danger.push(container),
                    _ => unreachable!(),
                }
            }

            let content = column!(
                label,
                row!(
                    widget::Space::new(50, 1),
                    background,
                    primary,
                    text_color,
                    success,
                    danger
                )
                .spacing(10)
            )
            .align_items(iced::Alignment::Center)
            .spacing(5);
            container(content).center_x()
        };

        let extended = {
            let label = row!(
                widget::Space::new(50, 1),
                text("Extended")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .spacing(10)
            .width(Length::Fill);
            let color_strength: Column<'_, Message, Self::Theme, iced::Renderer> = column!(
                widget::Space::new(1, 30),
                container(text("Base")).height(110).width(50).center_y(),
                container(text("Weak")).height(110).width(50).center_y(),
                container(text("Strong")).height(110).width(50).center_y(),
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

            for (i, color) in self.extended.iter().enumerate() {
                let mut text = container(
                    button(text("Text").style(theme::Text::Color(color[1])))
                        .style(theme::Button::Text)
                        .on_press(Message::SelectColor(Select::Extended((i, 1)))),
                );

                if let Select::Extended((index, ii)) = self.selected {
                    if index == i && ii == 1 {
                        text = text.style(selected_style)
                    }
                }

                let color_view = container(text)
                    .center_x()
                    .center_y()
                    .width(150)
                    .height(110)
                    .style(container::Appearance {
                        background: Some(iced::Background::Color(color[0])),
                        ..Default::default()
                    });
                let mut select_color = container(
                    button(color_view)
                        .padding(2)
                        .style(theme::Button::Text)
                        .on_press(Message::SelectColor(Select::Extended((i, 0)))),
                );

                if let Select::Extended((index, ii)) = self.selected {
                    if index == i && ii == 0 {
                        select_color = select_color.style(selected_style)
                    }
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

            let content = column!(label, grid)
                .align_items(iced::Alignment::Center)
                .spacing(5);

            container(content).center_x()
        };

        let rgba_selected = match self.selected {
            Select::Palette(index) => self.palette[index].into_rgba8(),
            Select::Extended((index1, index2)) => self.extended[index1][index2].into_rgba8(),
        };

        let adjust_red = {
            let text = text("Red").width(50);
            let value = match self.selected {
                Select::Palette(index) => self.palette[index].r,
                Select::Extended((index1, index2)) => self.extended[index1][index2].r,
            };
            let slider =
                widget::Slider::new(0.0..=1.0, value, |new_value| Message::AdjustRed(new_value))
                    .step(0.005)
                    .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.3}", value)).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<f32>() {
                        Message::AdjustRed(value)
                    } else {
                        Message::None
                    }
                }),
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
            let value = match self.selected {
                Select::Palette(index) => self.palette[index].g,
                Select::Extended((index1, index2)) => self.extended[index1][index2].g,
            };
            let slider = widget::Slider::new(0.0..=1.0, value, |new_value| {
                Message::AdjustGreen(new_value)
            })
            .step(0.005)
            .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.3}", value)).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<f32>() {
                        Message::AdjustGreen(value)
                    } else {
                        Message::None
                    }
                }),
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
            let value = match self.selected {
                Select::Palette(index) => self.palette[index].b,
                Select::Extended((index1, index2)) => self.extended[index1][index2].b,
            };
            let slider = widget::Slider::new(0.00..=1.00, value, |new_value| {
                Message::AdjustBlue(new_value)
            })
            .step(0.005)
            .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.3}", value)).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<f32>() {
                        Message::AdjustBlue(value)
                    } else {
                        Message::None
                    }
                }),
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
            let value = match self.selected {
                Select::Palette(index) => self.palette[index].a,
                Select::Extended((index1, index2)) => self.extended[index1][index2].a,
            };
            let slider = widget::Slider::new(0.0..=1.0, value, |new_value| {
                Message::AdjustAlpha(new_value)
            })
            .step(0.005)
            .width(300);
            let text_and_slider = row!(text, slider).spacing(5);
            let inputs = row!(
                text_input("", &format!("{:.3}", value)).on_input(|mut input| {
                    if input.is_empty() {
                        input.push('0')
                    }
                    if let Ok(value) = input.parse::<f32>() {
                        Message::AdjustAlpha(value)
                    } else {
                        Message::None
                    }
                }),
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
                button(
                    text("Reset Selected")
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                )
                .on_press(Message::ResetSelected)
                .width(150),
                button(
                    text("Reset All")
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                )
                .on_press(Message::ResetAll)
                .width(150),
                widget::tooltip(
                    button(
                        text("Generate Extended")
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    )
                    .on_press(Message::GenerateFromBase)
                    .width(150),
                    container(
                        text("Generates the extended palette from Palette")
                            .width(Length::Fill)
                            .height(Length::Fill)
                    )
                    .height(60)
                    .width(180)
                    .padding(5)
                    .center_x()
                    .center_y()
                    .style(TooltipContainerStyle::style),
                    Position::Top,
                )
                .gap(10),
                button(
                    text("Try Theme")
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                )
                .on_press(Message::TryTheme)
                .width(150)
            )
            .spacing(30),
        )
        .width(Length::Fill)
        .center_x();

        let content = column!(top_container, palette, extended, sliders, reset)
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
        self.app_theme.clone()
    }
}

impl ThemeColors {
    fn adjust_selected_red_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }

        match self.selected {
            Select::Palette(index) => self.palette[index].r = new_value,
            Select::Extended((index1, index2)) => self.extended[index1][index2].r = new_value,
        }
    }

    fn adjust_selected_green_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }

        match self.selected {
            Select::Palette(index) => self.palette[index].g = new_value,
            Select::Extended((index1, index2)) => self.extended[index1][index2].g = new_value,
        }
    }

    fn adjust_selected_blue_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }

        match self.selected {
            Select::Palette(index) => self.palette[index].b = new_value,
            Select::Extended((index1, index2)) => self.extended[index1][index2].b = new_value,
        }
    }

    fn adjust_selected_alpha_color(&mut self, mut new_value: f32) {
        if new_value > 1.0 {
            new_value = 1.0;
        } else if new_value < 0.0 {
            new_value = 0.0;
        }

        match self.selected {
            Select::Palette(index) => self.palette[index].a = new_value,
            Select::Extended((index1, index2)) => self.extended[index1][index2].a = new_value,
        }
    }

    fn generate_extended_from_palette(&mut self) {
        let palette = self.palette_from_palette_array();

        let extended = Extended::generate(palette);
        self.extended = Self::populate_extended_array(&extended);
    }

    fn palette_from_palette_array(&self) -> Palette {
        Palette {
            background: self.palette[0],
            primary: self.palette[1],
            text: self.palette[2],
            success: self.palette[3],
            danger: self.palette[4],
        }
    }

    fn extendedpalette_from_colors_array(&self, palette: Palette) -> Extended {
        let generated_extended = Extended::generate(palette);
        Extended {
            background: theme::palette::Background {
                base: Pair {
                    color: self.extended[0][0],
                    text: self.extended[0][1],
                },
                weak: Pair {
                    color: self.extended[1][0],
                    text: self.extended[1][1],
                },
                strong: Pair {
                    color: self.extended[2][0],
                    text: self.extended[2][1],
                },
            },
            primary: Primary {
                base: Pair {
                    color: self.extended[3][0],
                    text: self.extended[3][1],
                },
                weak: Pair {
                    color: self.extended[4][0],
                    text: self.extended[4][1],
                },
                strong: Pair {
                    color: self.extended[5][0],
                    text: self.extended[5][1],
                },
            },
            secondary: Secondary {
                base: Pair {
                    color: self.extended[6][0],
                    text: self.extended[6][1],
                },
                weak: Pair {
                    color: self.extended[7][0],
                    text: self.extended[7][1],
                },
                strong: Pair {
                    color: self.extended[8][0],
                    text: self.extended[8][1],
                },
            },
            success: Success {
                base: Pair {
                    color: self.extended[9][0],
                    text: self.extended[9][1],
                },
                weak: Pair {
                    color: self.extended[10][0],
                    text: self.extended[10][1],
                },
                strong: Pair {
                    color: self.extended[11][0],
                    text: self.extended[11][1],
                },
            },
            danger: Danger {
                base: Pair {
                    color: self.extended[12][0],
                    text: self.extended[12][1],
                },
                weak: Pair {
                    color: self.extended[13][0],
                    text: self.extended[13][1],
                },
                strong: Pair {
                    color: self.extended[14][0],
                    text: self.extended[14][1],
                },
            },
            is_dark: generated_extended.is_dark,
        }
    }

    fn populate_palette_array(palette: Palette) -> [Color; 5] {
        [
            palette.background,
            palette.primary,
            palette.text,
            palette.success,
            palette.danger,
        ]
    }

    fn populate_extended_array(palette: &Extended) -> [[Color; 2]; 15] {
        [
            [palette.background.base.color, palette.background.base.text],
            [palette.background.weak.color, palette.background.weak.text],
            [
                palette.background.strong.color,
                palette.background.strong.text,
            ],
            [palette.primary.base.color, palette.primary.base.text],
            [palette.primary.weak.color, palette.primary.weak.text],
            [palette.primary.strong.color, palette.primary.strong.text],
            [palette.secondary.base.color, palette.secondary.base.text],
            [palette.secondary.weak.color, palette.secondary.weak.text],
            [
                palette.secondary.strong.color,
                palette.secondary.strong.text,
            ],
            [palette.success.base.color, palette.success.base.text],
            [palette.success.weak.color, palette.success.weak.text],
            [palette.success.strong.color, palette.success.strong.text],
            [palette.danger.base.color, palette.danger.base.text],
            [palette.danger.weak.color, palette.danger.weak.text],
            [palette.danger.strong.color, palette.danger.strong.text],
        ]
    }

    fn themes_array() -> [Theme; 22] {
        [
            Theme::custom("Custom".to_string(), Theme::Dark.palette()),
            Theme::Light,
            Theme::Dark,
            Theme::Dracula,
            Theme::Nord,
            Theme::SolarizedLight,
            Theme::SolarizedDark,
            Theme::GruvboxLight,
            Theme::GruvboxDark,
            Theme::CatppuccinLatte,
            Theme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato,
            Theme::CatppuccinMocha,
            Theme::TokyoNight,
            Theme::TokyoNightStorm,
            Theme::TokyoNightLight,
            Theme::KanagawaWave,
            Theme::KanagawaDragon,
            Theme::KanagawaLotus,
            Theme::Moonfly,
            Theme::Nightfly,
            Theme::Oxocarbon,
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

struct TooltipContainerStyle;

impl TooltipContainerStyle {
    fn style(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        let mut background = palette.background.base.color;
        background.r += 0.03;
        background.g += 0.03;
        background.b += 0.03;

        container::Appearance {
            background: Some(iced::Background::Color(background)),
            border: Border {
                radius: Radius::from(5.),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
