use bevy::prelude::*;

pub enum ButtonSize {
    Small,
    Medium,
    Large,
    Custom(f32, f32),
}

impl ButtonSize {
    fn width(&self) -> f32 {
        match self {
            ButtonSize::Small => 50.,
            ButtonSize::Medium => 100.,
            ButtonSize::Large => 300.,
            ButtonSize::Custom(width, _) => *width,
        }
    }
    fn height(&self) -> f32 {
        match self {
            ButtonSize::Small => 25.,
            ButtonSize::Medium => 50.,
            ButtonSize::Large => 100.,
            ButtonSize::Custom(_, height) => *height,
        }
    }
    fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Small => 12.,
            ButtonSize::Medium => 16.,
            ButtonSize::Large => 22.,
            ButtonSize::Custom(_, _) => 42.,
        }
    }
}
pub fn custom_button(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    marker: impl Component,
    background_color: impl Into<Color>,
    text_color: impl Into<Color>,
    text: impl Into<String>,
    size: ButtonSize,
) {
    builder
        .spawn((
            Node {
                width: Val::Px(size.width()),
                height: Val::Px(size.height()),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius {
                top_left: Val::Percent(30.),
                top_right: Val::Percent(30.),
                bottom_left: Val::Percent(30.),
                bottom_right: Val::Percent(30.),
            },
            BackgroundColor(background_color.into()),
            Button,
            marker,
        ))
        .with_children(|button_box_text| {
            button_box_text.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("font/pixel-font.ttf"),
                    font_size: size.font_size(),
                    ..default()
                },
                TextColor(text_color.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}
