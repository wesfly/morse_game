use bevy::{prelude::*, time::Stopwatch};
use std::collections::HashMap;

// The following lines control the difficulty. Feel free to tweak around.

// Dot vs. dash duration
const CLICK_DURATION_THRESHOLD: f32 = 0.15;
// The time it takes between two inputs to count as a new character, 0.2 is for the advanced
const NEW_CHARACTER_DELAY: f32 = 0.2;

// Tone frequency
const FREQUENCY: f32 = 600.;

#[derive(Component)]
struct CharacterDisplay;

#[derive(Component)]
struct MorseDisplay;

#[derive(Component)]
struct PlainHistory;

// For pushing the current character to the check_morse_char fn
#[derive(Default, Resource)]
struct PushChar(Vec<char>);

#[derive(Resource, Default)]
struct CharHistory(String);

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

#[derive(Default, Resource)]
struct CurrentAudio(Option<Entity>);

impl Default for PauseTimer {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            is_activated: false,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut volume: ResMut<GlobalVolume>) {
    commands.spawn(Camera2d);
    volume.volume = bevy::audio::Volume::Linear(0.5);

    // Create container
    let container_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();

    // Create character display text
    let character_text = commands
        .spawn((
            Text::new(""),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
                font: asset_server.load("fonts/Red_Hat_Display/static/RedHatDisplay-Bold.ttf"),
                font_size: 144.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            CharacterDisplay,
        ))
        .id();

    // Create morse display text
    let morse_text = commands
        .spawn((
            Text::new("Click to start"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
                font: asset_server.load("fonts/Red_Hat_Display/static/RedHatDisplay-Regular.ttf"),
                font_size: 28.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            MorseDisplay,
        ))
        .id();

    let plain_history = commands
        .spawn((
            Text::new("History"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
                font: asset_server.load("fonts/Red_Hat_Display/static/RedHatDisplay-Regular.ttf"),
                font_size: 28.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            PlainHistory,
        ))
        .id();

    commands
        .entity(container_entity)
        .add_children(&[character_text, morse_text, plain_history]);
}

fn check_morse_char(
    is_final: bool,
    chars: &mut ResMut<PushChar>,
    display_char: &mut ResMut<CurrentChar>,
    char_history: &mut ResMut<CharHistory>,
) {
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
            display_char.0 = code.0;
            if is_final {
                char_history.0.push_str(code.0.to_string().as_str());
            }
        }
    }
    if is_final {
        chars.0.clear();
    }
}

fn text_update_system(
    mut query: Query<&mut Text, With<CharacterDisplay>>,
    character: Res<CurrentChar>,
) {
    if character.0 != '□' {
        for mut span in &mut query {
            **span = character.0.to_string();
        }
    } else {
        for mut span in &mut query {
            **span = ' '.to_string();
        }
    }
}

fn morse_display_update_system(
    mut query: Query<&mut Text, With<MorseDisplay>>,
    chars: Res<PushChar>,
) {
    for mut text in &mut query {
        if chars.0.is_empty() == false {
            **text = chars.0.iter().collect::<String>();
        }
    }
}

fn history_display_update_system(
    mut query: Query<&mut Text, With<PlainHistory>>,
    history: Res<CharHistory>,
) {
    for mut text in &mut query {
        **text = history.0.clone();
    }
}

fn register_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<PressTimer>,
    mut idle_timer: ResMut<PauseTimer>,
    mut chars: ResMut<PushChar>,
    mut commands: Commands,
    mut pitch_assets: ResMut<Assets<Pitch>>,
    mut current_audio: ResMut<CurrentAudio>,
    mut current_char: ResMut<CurrentChar>,
    mut char_history: ResMut<CharHistory>,
    time: Res<Time>,
) {
    if mouse.just_pressed(MouseButton::Left) || keyboard.just_pressed(KeyCode::Enter) {
        timer.stopwatch.reset();
        timer.is_pressed = true;

        #[cfg(debug_assertions)]
        info!("not pressed for {:.2}", idle_timer.stopwatch.elapsed_secs());

        idle_timer.is_activated = false;

        // Start continuous audio playback
        let audio_entity = commands
            .spawn(AudioPlayer(
                pitch_assets.add(Pitch::new(FREQUENCY, std::time::Duration::from_secs(60))), // Long duration
            ))
            .id();
        current_audio.0 = Some(audio_entity);
    }

    if (mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Enter)) && timer.is_pressed {
        timer.stopwatch.tick(time.delta());
    }

    if (mouse.just_released(MouseButton::Left) || keyboard.just_released(KeyCode::Enter))
        && timer.is_pressed
    {
        let press_duration = timer.stopwatch.elapsed_secs();

        if press_duration < CLICK_DURATION_THRESHOLD {
            chars.0.push('•');
        } else if press_duration >= CLICK_DURATION_THRESHOLD {
            chars.0.push('-');
        }

        timer.is_pressed = false;

        // Stop the audio
        if let Some(audio_entity) = current_audio.0 {
            commands.entity(audio_entity).despawn();
            current_audio.0 = None;
        }

        idle_timer.stopwatch.reset();
        idle_timer.is_activated = true;
        check_morse_char(false, &mut chars, &mut current_char, &mut char_history);
    }

    if mouse.just_pressed(MouseButton::Right) || keyboard.just_pressed(KeyCode::Space) {
        char_history.0.clear();
    }

    // Tick the idle timer
    if idle_timer.is_activated {
        idle_timer.stopwatch.tick(time.delta());
        if idle_timer.stopwatch.elapsed_secs() >= NEW_CHARACTER_DELAY {
            check_morse_char(true, &mut chars, &mut current_char, &mut char_history);
        }
    }

    if keyboard.just_pressed(KeyCode::Backspace) {
        char_history.0.pop();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.11)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Morse Game".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<PressTimer>()
        .init_resource::<PauseTimer>()
        .init_resource::<PushChar>()
        .init_resource::<CurrentChar>()
        .init_resource::<CurrentAudio>()
        .init_resource::<CharHistory>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                register_input,
                text_update_system,
                morse_display_update_system,
                history_display_update_system,
            ),
        )
        .run();
}
