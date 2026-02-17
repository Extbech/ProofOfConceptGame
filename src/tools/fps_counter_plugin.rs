use bevy::{
    app::Plugin,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use test_game::FPS_COUNTER_Z;

/// Plugin for enabling a fps counter in the game.
/// This fps counter can be toggled on/off by pressing the `f12` key.
pub struct FPSCounterPlugin;

impl Plugin for FPSCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_counter)
            .add_systems(
                Update,
                (
                    fps_counter_showhide,
                    fps_text_update_system,
                    fps_color_update_system,
                ),
            );
    }
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ColorText;

fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            FpsRoot,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ZIndex(FPS_COUNTER_Z),
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.8,
            })),
        ))
        .with_children(|child| {
            child
                .spawn((
                    Text::new("FPS: "),
                    TextFont {
                        font: asset_server.load("font/pixel-font.ttf"),
                        font_size: 16.0,
                        ..Default::default()
                    },
                    ColorText,
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font: asset_server.load("font/pixel-font.ttf"),
                        font_size: 16.0,
                        ..Default::default()
                    },
                    FpsText,
                    ColorText,
                ));
        });
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query_span: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query_span {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            **span = format!("{value:>4.0}");
        }
    }
}
fn fps_color_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query_color: Query<&mut TextColor, With<ColorText>>,
) {
    for mut color in &mut query_color {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            color.0 = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::Srgba(Srgba {
                    red: 0.,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                })
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::Srgba(Srgba {
                    red: 1. - (value / 120.0) as f32,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                })
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::Srgba(Srgba {
                    red: 1.,
                    green: ((value - 30.0) / (60.0 - 30.0)) as f32,
                    blue: 0.,
                    alpha: 1.,
                })
            } else {
                // Below 30 FPS, use red color
                Color::Srgba(Srgba {
                    red: 1.,
                    green: 0.,
                    blue: 0.,
                    alpha: 1.,
                })
            }
        }
    }
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Some(mut vis) = q.iter_mut().next() {
            println!("f12 pressed: {:?}", vis);
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}
