use bevy::{prelude::*, time::Stopwatch};
use std::collections::HashMap;

const CLICK_DURATION_THRESHOLD: f32 = 0.15;
const WORD_DELAY_THRESHOLD: f32 = 0.5;

#[derive(Component)]
struct CharacterDisplay;

// For pushing the current character to the check_morse_char fn
#[derive(Default, Resource)]
struct PushChar(Vec<char>);

#[derive(Resource)]
struct CurrentChar(char);
impl Default for CurrentChar {
    fn default() -> Self {
        // Display nothing at startup
        CurrentChar(' ')
    }
}

#[derive(Resource)]
struct PressTimer {
    stopwatch: Stopwatch,
    is_pressed: bool,
}

impl Default for PressTimer {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            is_pressed: false,
        }
    }
}

#[derive(Resource)]
struct PauseTimer {
    stopwatch: Stopwatch,
    is_activated: bool,
}

impl Default for PauseTimer {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            is_activated: false,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };
    // The letter indicator
    let text = (
        Text::new("Loading..."),
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/Red_Hat_Display/static/RedHatDisplay-Bold.ttf"),
            font_size: 144.0,
            ..default()
        },
        Node {
            bottom: Val::Percent(-50.),
            ..default()
        },
        CharacterDisplay,
    );

    commands.spawn((container, children![text]));
}

fn check_morse_char(chars: &mut ResMut<PushChar>, mut display_char: ResMut<CurrentChar>) {
    let morse_codes = HashMap::from([
        // Letters
        ('A', "•-"),
        ('B', "-•••"),
        ('C', "-•-•"),
        ('D', "-••"),
        ('E', "•"),
        ('F', "••-•"),
        ('G', "--•"),
        ('H', "••••"),
        ('I', "••"),
        ('J', "•---"),
        ('K', "-•-"),
        ('L', "•-••"),
        ('M', "--"),
        ('N', "-•"),
        ('O', "---"),
        ('P', "•--•"),
        ('Q', "--•-"),
        ('R', "•-•"),
        ('S', "•••"),
        ('T', "-"),
        ('U', "••-"),
        ('V', "•••-"),
        ('W', "•--"),
        ('X', "-••-"),
        ('Y', "-•--"),
        ('Z', "--••"),
        // Numbers
        ('0', "-----"),
        ('1', "•----"),
        ('2', "••---"),
        ('3', "•••--"),
        ('4', "••••-"),
        ('5', "•••••"),
        ('6', "-••••"),
        ('7', "--•••"),
        ('8', "---••"),
        ('9', "----•"),
        // Punctuation
        ('.', "•-•-•-"),
        (',', "--••--"),
        ('?', "••--••"),
        ('\'', "•----•"),
        ('/', "-••-•"),
        ('!', "-•-•--"),
        ('(', "-•--•"),
        (')', "-•--•-"),
        ('&', "•-•••"),
        (':', "---•••"),
        (';', "-•-•-•"),
        ('=', "-•••-"),
        ('+', "•-•-•"),
        ('-', "-••••-"),
        ('_', "••--•-"),
        ('\"', "•-••-•"),
        ('$', "•••-••-"),
        ('@', "•--•-•"),
    ]);

    for code in morse_codes {
        if code.1 == chars.0.iter().collect::<String>() {
            info!("{}", code.0);
            display_char.0 = code.0;
        }
    }

    chars.0.clear();
}

fn text_update_system(
    mut query: Query<&mut Text, With<CharacterDisplay>>,
    character: Res<CurrentChar>,
) {
    if character.0 != '□' {
        for mut span in &mut query {
            **span = character.0.to_string();
        }
    }
}

fn register_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut timer: ResMut<PressTimer>,
    mut idle_timer: ResMut<PauseTimer>,
    mut chars: ResMut<PushChar>,
    current_char: ResMut<CurrentChar>,
    time: Res<Time>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        timer.stopwatch.reset();
        timer.is_pressed = true;

        info!("not pressed for {:.2}", idle_timer.stopwatch.elapsed_secs());

        idle_timer.is_activated = false;
    }

    if mouse.pressed(MouseButton::Left) && timer.is_pressed {
        timer.stopwatch.tick(time.delta());
    }

    if mouse.just_released(MouseButton::Left) && timer.is_pressed {
        let press_duration = timer.stopwatch.elapsed_secs();

        if press_duration < CLICK_DURATION_THRESHOLD {
            chars.0.push('•');
        } else if press_duration >= CLICK_DURATION_THRESHOLD {
            chars.0.push('-');
        }

        timer.is_pressed = false;

        idle_timer.stopwatch.reset();
        idle_timer.is_activated = true;
    }

    // Tick the idle timer
    if idle_timer.is_activated {
        idle_timer.stopwatch.tick(time.delta());
        if idle_timer.stopwatch.elapsed_secs() >= WORD_DELAY_THRESHOLD {
            check_morse_char(&mut chars, current_char);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Morse".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<PressTimer>()
        .init_resource::<PauseTimer>()
        .init_resource::<PushChar>()
        .init_resource::<CurrentChar>()
        .add_systems(Startup, setup)
        .add_systems(Update, (register_input, text_update_system))
        .run();
}
