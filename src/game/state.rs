//! Top-level game state machine and the transitions between its variants.
//!
//! ```text
//! Home     --Start-->    Playing
//! Home     --Options-->  Options    --Back-->  Home
//! Home     --Quit-->     AppExit
//! Playing  --Esc-->      Paused     --Esc-->   Playing  (resume)
//!                                   --R-->     Playing  (restart)
//!                                   --H-->     Home
//! Playing  --GameOver--> GameOver   --R-->     Playing
//!                                   --Esc-->   Home
//! ```
//!
//! Keyboard shortcuts use layout-independent keys where possible (Escape,
//! Enter, Space) so AZERTY/QWERTY users see the same behavior. Letter
//! shortcuts (R) only stick around when their physical position matches
//! across layouts.
//!
//! `Home` is the default — gameplay only begins once the player presses
//! Start. The "fresh run" reset (board, bag, score, gravity, active piece)
//! lives in [`start_run`] and fires on every entry into Playing, so
//! restarts and home-relaunches share one path.

use bevy::prelude::*;

use crate::board::{ActivePiece, Board, GravityTimer, PendingLineClear, PieceLocked};
use crate::pieces::SevenBag;

use super::score::Score;

/// Set when the Paused overlay's Resume action transitions to Playing, so
/// `start_run` knows to keep the current run instead of starting fresh.
/// Cleared by `start_run` after every entry into Playing, so the next
/// non-Resume transition (Home Start, GameOver R, Paused Restart) gets the
/// default reset behavior.
#[derive(Resource, Default)]
pub struct ResumeFromPause(pub bool);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Home,
    Options,
    Playing,
    Paused,
    GameOver,
}

/// Escape while Playing enters Paused. Lives on the Playing-gated side so
/// holding Escape across other states does nothing surprising.
pub fn pause_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

/// Reads [`PieceLocked`] messages and transitions to [`GameState::GameOver`]
/// as soon as a lock signals a top-out. Drains pending messages while not
/// playing so they don't pile up across restarts.
pub fn check_game_over(
    mut events: MessageReader<PieceLocked>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() != GameState::Playing {
        events.clear();
        return;
    }
    for event in events.read() {
        if event.outcome.topped_out {
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

/// Fresh-run setup. Runs every time the state machine enters Playing —
/// whether from Home (first run), GameOver (R restart) or a future
/// pause-restart. Resets every gameplay-owned resource so the run starts
/// from a known clean slate.
pub fn start_run(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut bag: ResMut<SevenBag>,
    mut score: ResMut<Score>,
    mut gravity: ResMut<GravityTimer>,
    mut resume: ResMut<ResumeFromPause>,
) {
    if resume.0 {
        resume.0 = false;
        return;
    }
    board.reset();
    *bag = SevenBag::new();
    *score = Score::default();
    *gravity = GravityTimer::default();
    // A restart mid-clear must not leave the row-clear timer hanging: it
    // would fire on the fresh board and spawn a bonus piece.
    commands.remove_resource::<PendingLineClear>();
    let kind = bag.next();
    commands.insert_resource(ActivePiece::new(kind, kind.spawn_origin()));
}

/// Removes the active piece on entering GameOver. Gravity and input
/// early-return on a missing piece, so this is enough to freeze gameplay
/// without gating their systems on the state. A pending line-clear from
/// the final lock is also dropped — its overlay is a visual artefact and
/// the game-over screen should take over the board immediately.
pub fn on_game_over_enter(mut commands: Commands) {
    commands.remove_resource::<ActivePiece>();
    commands.remove_resource::<PendingLineClear>();
}

/// Keyboard shortcuts available while the GameOver overlay is up.
/// **R** restarts in place (R sits in the same physical position on
/// QWERTY and AZERTY, so the binding is portable), **Escape** returns
/// to the home screen. The actual run reset for R lives in
/// [`start_run`] via `OnEnter(Playing)`.
pub fn game_over_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Home);
    }
}
