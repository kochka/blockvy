//! SFX + background music playback for gameplay events.
//!
//! **SFX**: the plugin preloads a handle per sample at startup, then converts
//! gameplay events into a single [`PlaySfx`] message. Any system in the
//! app can request a sound by writing a [`PlaySfx`] — [`play_sfx_events`]
//! spawns a one-shot `AudioPlayer` for each, with `PlaybackSettings::DESPAWN`
//! so the entity cleans itself up when the sample ends.
//!
//! **Music**: a single looping [`BackgroundMusic`] entity is spawned when
//! `GameState::Playing` is entered and torn down when leaving to `Home` or
//! `GameOver`. Pausing the game pauses the sink in place, so resuming picks
//! the track back up where it left off; a restart from the pause menu
//! despawns first so the new run starts from the top.
//!
//! Missing files on disk surface as Bevy asset warnings, never crashes.
//! The corresponding sound simply doesn't play.

use bevy::audio::{
    AudioPlayer, AudioSink, AudioSinkPlayback, AudioSource, PlaybackSettings, Volume,
};
use bevy::prelude::*;

use crate::board::PieceLocked;
use crate::game::{GameState, ResumeFromPause, Score};

pub struct AudioPlugin;

/// Discrete gameplay sounds the SFX layer knows how to play. Systems that
/// trigger sounds write to `MessageWriter<PlaySfx>`; the audio module is
/// the only place that resolves the enum into an actual asset handle.
#[derive(Message, Debug, Clone, Copy)]
pub enum PlaySfx {
    Rotate,
    HardDrop,
    Lock,
    LineClear,
    Tetris,
    LevelUp,
    GameOver,
}

#[derive(Resource, Default)]
struct SfxHandles {
    rotate: Handle<AudioSource>,
    hard_drop: Handle<AudioSource>,
    lock: Handle<AudioSource>,
    line_clear: Handle<AudioSource>,
    tetris: Handle<AudioSource>,
    level_up: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct MusicHandles {
    game_theme: Handle<AudioSource>,
}

/// Marks the currently-playing background music entity so state-change
/// systems can find it — to pause the sink, unpause it, or despawn.
#[derive(Component)]
struct BackgroundMusic;

/// Music sits under SFX in the mix so line-clear samples still cut through.
const MUSIC_VOLUME: Volume = Volume::Linear(0.4);

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlaySfx>()
            .init_resource::<SfxHandles>()
            .init_resource::<MusicHandles>()
            .add_systems(Startup, preload_audio)
            .add_systems(Update, (emit_lock_sfx, emit_level_up_sfx, play_sfx_events))
            // Music lifecycle wired to the state machine. `OnExit(Paused)`
            // decides between unpause and despawn based on `ResumeFromPause`
            // *before* `OnEnter(Playing)` runs, so the spawn-if-absent
            // system there never races with `start_run` clearing the flag.
            .add_systems(OnEnter(GameState::Playing), spawn_music_if_absent)
            .add_systems(OnEnter(GameState::Paused), pause_music)
            .add_systems(OnExit(GameState::Paused), resume_or_stop_music)
            .add_systems(OnEnter(GameState::Home), stop_music)
            .add_systems(OnEnter(GameState::GameOver), stop_music);
    }
}

fn preload_audio(
    mut sfx: ResMut<SfxHandles>,
    mut music: ResMut<MusicHandles>,
    asset_server: Res<AssetServer>,
) {
    sfx.rotate = asset_server.load("sfx/rotate.ogg");
    sfx.hard_drop = asset_server.load("sfx/hard_drop.ogg");
    sfx.lock = asset_server.load("sfx/lock.ogg");
    sfx.line_clear = asset_server.load("sfx/line_clear.ogg");
    sfx.tetris = asset_server.load("sfx/tetris_4_lines.ogg");
    sfx.level_up = asset_server.load("sfx/level_up.ogg");
    sfx.game_over = asset_server.load("sfx/game_over.ogg");
    music.game_theme = asset_server.load("music/game_theme_a.ogg");
}

/// Turns each `PieceLocked` into exactly one sound: top-out wins over any
/// clear, a 4-line clear (Tetris) beats a 1-3 line clear, which beats a
/// plain lock. Guarantees we never stack two lock-adjacent samples on the
/// same frame.
fn emit_lock_sfx(mut locks: MessageReader<PieceLocked>, mut sfx: MessageWriter<PlaySfx>) {
    for lock in locks.read() {
        let event = if lock.outcome.topped_out {
            PlaySfx::GameOver
        } else {
            match lock.outcome.lines_cleared {
                0 => PlaySfx::Lock,
                1..=3 => PlaySfx::LineClear,
                _ => PlaySfx::Tetris,
            }
        };
        sfx.write(event);
    }
}

/// Detects a level bump via `Score` change detection. The `Local` remembers
/// the previously-seen level so we react only to transitions; it also
/// swallows the frame-1 pulse (previous starts at 0, score.level at 1) and
/// resets silently when the score is wiped for a fresh run.
fn emit_level_up_sfx(score: Res<Score>, mut previous: Local<u32>, mut sfx: MessageWriter<PlaySfx>) {
    if score.level == *previous {
        return;
    }
    if score.level > *previous && *previous != 0 {
        sfx.write(PlaySfx::LevelUp);
    }
    *previous = score.level;
}

fn play_sfx_events(
    mut events: MessageReader<PlaySfx>,
    handles: Res<SfxHandles>,
    mut commands: Commands,
) {
    for event in events.read() {
        let handle = match event {
            PlaySfx::Rotate => &handles.rotate,
            PlaySfx::HardDrop => &handles.hard_drop,
            PlaySfx::Lock => &handles.lock,
            PlaySfx::LineClear => &handles.line_clear,
            PlaySfx::Tetris => &handles.tetris,
            PlaySfx::LevelUp => &handles.level_up,
            PlaySfx::GameOver => &handles.game_over,
        };
        commands.spawn((AudioPlayer::new(handle.clone()), PlaybackSettings::DESPAWN));
    }
}

/// Spawns the looping music entity when entering Playing, unless one is
/// already live (which happens when we're resuming from Paused —
/// `resume_or_stop_music` has already unpaused the existing sink).
fn spawn_music_if_absent(
    mut commands: Commands,
    handles: Res<MusicHandles>,
    existing: Query<(), With<BackgroundMusic>>,
) {
    if !existing.is_empty() {
        return;
    }
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(handles.game_theme.clone()),
        PlaybackSettings::LOOP.with_volume(MUSIC_VOLUME),
    ));
}

fn pause_music(sinks: Query<&AudioSink, With<BackgroundMusic>>) {
    for sink in &sinks {
        sink.pause();
    }
}

/// On leaving Paused, either resume the existing sink (user hit Resume)
/// or despawn it so the next `OnEnter(Playing)` starts a fresh track
/// (user hit Restart, or navigated home). Bevy's state ordering runs
/// `OnExit(Paused)` before `OnEnter(Playing)`, so this doesn't race with
/// [`spawn_music_if_absent`] or `start_run`'s clearing of the resume flag.
fn resume_or_stop_music(
    mut commands: Commands,
    resume: Res<ResumeFromPause>,
    music: Query<(Entity, &AudioSink), With<BackgroundMusic>>,
) {
    for (entity, sink) in &music {
        if resume.0 {
            sink.play();
        } else {
            commands.entity(entity).despawn();
        }
    }
}

fn stop_music(mut commands: Commands, music: Query<Entity, With<BackgroundMusic>>) {
    for entity in &music {
        commands.entity(entity).despawn();
    }
}
