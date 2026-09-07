use bevy::{
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        touch::{TouchInput, TouchPhase},
    },
    prelude::*,
};

use crate::{
    client::ui::RulesScroll,
    game::{board::VertexId, state::GameStatus},
    session::{LocalGameSession, SessionCommand, SessionError},
};

mod board;
mod input;
mod layout;
mod materials;
mod style;
mod sync;
mod ui;
const RULES_SCROLL_LINE: f32 = 28.0;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSystemSet {
    Layout,
    Input,
    Sync,
    Style,
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(board::BOARD_BACKGROUND))
            .insert_resource(SessionResource(LocalGameSession::compact()))
            .init_resource::<UiState>();

        setup(app);

        app.configure_sets(
            Update,
            (
                GameSystemSet::Layout,
                GameSystemSet::Input,
                GameSystemSet::Sync,
                GameSystemSet::Style,
            )
                .chain(),
        );

        add_layout_system(app);
        add_input_system(app);
        add_sync_system(app);
        add_style_system(app);
    }
}

fn setup(app: &mut App) {
    app.add_systems(
        Startup,
        (
            setup_camera,
            materials::setup_stone_materials,
            board::setup_board,
            ui::setup_ui,
        )
            .chain(),
    );
}

fn add_layout_system(app: &mut App) {
    app.add_systems(
        Update,
        (
            layout::layout_control_panel,
            layout::layout_mobile_content,
            layout::fit_board_to_window,
        )
            .in_set(GameSystemSet::Layout),
    );
}

fn add_input_system(app: &mut App) {
    app.add_systems(
        Update,
        (
            input::update_pointer_target,
            input::handle_pointer_place,
            input::handle_keyboard,
            input::handle_buttons,
            scroll_rules,
        )
            .in_set(GameSystemSet::Input),
    );
}

fn add_sync_system(app: &mut App) {
    app.add_systems(
        Update,
        (
            sync::sync_stones,
            sync::sync_preview,
            sync::sync_focus_marker,
            sync::sync_last_move_marker,
            sync::sync_current_player,
            sync::sync_pass_count,
            sync::sync_result,
            sync::sync_feedback,
            sync::sync_modal,
            sync::sync_rules_modal,
        )
            .in_set(GameSystemSet::Sync),
    );
}

fn add_style_system(app: &mut App) {
    app.add_systems(Update, (style::style_buttons).in_set(GameSystemSet::Style));
}

#[derive(Resource)]
struct SessionResource(LocalGameSession);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum FocusTarget {
    #[default]
    Board,
    Pass,
    Resign,
    Restart,
    Rules,
}

impl FocusTarget {
    fn next(self, reverse: bool) -> Self {
        const ORDER: [FocusTarget; 5] = [
            FocusTarget::Board,
            FocusTarget::Pass,
            FocusTarget::Resign,
            FocusTarget::Restart,
            FocusTarget::Rules,
        ];
        let index = ORDER
            .iter()
            .position(|candidate| *candidate == self)
            .unwrap();
        let offset = if reverse { ORDER.len() - 1 } else { 1 };
        ORDER[(index + offset) % ORDER.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModalKind {
    Resign,
    Restart,
    Rules,
}

#[derive(Resource, Default)]
struct UiState {
    hovered: Option<VertexId>,
    focused_vertex: Option<VertexId>,
    focus: FocusTarget,
    modal: Option<ModalKind>,
    feedback: String,
    feedback_is_error: bool,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn open_rules(ui: &mut UiState) {
    ui.modal = Some(ModalKind::Rules);
}

fn request_resign(session: &LocalGameSession, ui: &mut UiState) {
    if session.status() == GameStatus::Playing {
        ui.modal = Some(ModalKind::Resign);
    } else {
        ui.feedback_is_error = true;
        ui.feedback = error_message(SessionError::GameOver).into();
    }
}

fn confirm_modal(session: &mut LocalGameSession, ui: &mut UiState, modal: ModalKind) {
    ui.modal = None;
    match modal {
        ModalKind::Resign => submit_command(session, ui, SessionCommand::Resign),
        ModalKind::Restart => submit_command(session, ui, SessionCommand::Restart),
        ModalKind::Rules => {}
    }
}

fn scroll_rules(
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut touch_input: MessageReader<TouchInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    ui: Res<UiState>,
    mut scroll: Single<(&mut ScrollPosition, &ComputedNode), With<RulesScroll>>,
) {
    if ui.modal != Some(ModalKind::Rules) {
        mouse_wheel.clear();
        touch_input.clear();
        return;
    }

    let wheel_delta = mouse_wheel.read().fold(0.0, |total, event| {
        let scale = match event.unit {
            MouseScrollUnit::Line => RULES_SCROLL_LINE,
            MouseScrollUnit::Pixel => 1.0,
        };
        total - event.y * scale
    });
    let keyboard_delta = if keyboard.just_pressed(KeyCode::ArrowDown) {
        RULES_SCROLL_LINE
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        -RULES_SCROLL_LINE
    } else if keyboard.just_pressed(KeyCode::PageDown) {
        scroll.1.size().y * 0.8
    } else if keyboard.just_pressed(KeyCode::PageUp) {
        -scroll.1.size().y * 0.8
    } else {
        0.0
    };
    let has_touch_move = touch_input
        .read()
        .any(|event| event.phase == TouchPhase::Moved);
    let touch_delta = if has_touch_move {
        touches.iter().map(|touch| -touch.delta().y).sum::<f32>()
    } else {
        0.0
    };
    scroll.0.y = clamped_scroll_position(
        scroll.0.y,
        wheel_delta + keyboard_delta + touch_delta,
        scroll.1.content_size().y,
        scroll.1.size().y,
        scroll.1.inverse_scale_factor,
    );
}

fn clamped_scroll_position(
    current: f32,
    delta: f32,
    content_size: f32,
    visible_size: f32,
    inverse_scale_factor: f32,
) -> f32 {
    let max_offset = (content_size - visible_size).max(0.0) * inverse_scale_factor;
    (current + delta).clamp(0.0, max_offset)
}

fn submit_command(session: &mut LocalGameSession, ui: &mut UiState, command: SessionCommand) {
    match session.submit(command) {
        Ok(()) => {
            ui.feedback_is_error = false;
            ui.feedback = match command {
                SessionCommand::Place(_) => "落子成功".into(),
                SessionCommand::Pass => "已停着".into(),
                SessionCommand::Resign => "对局因认输结束".into(),
                SessionCommand::Restart => "已开始新对局".into(),
            };
        }
        Err(error) => {
            ui.feedback_is_error = true;
            ui.feedback = error_message(error).into();
        }
    }
}

fn error_message(error: SessionError) -> &'static str {
    match error {
        SessionError::InvalidVertex => "该交点不在棋盘上",
        SessionError::Occupied => "该处已有棋子",
        SessionError::Suicide => "禁止自杀落子",
        SessionError::Superko => "该落子违反全局同形规则",
        SessionError::GameOver => "对局已经结束",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upward_touch_motion_scrolls_rules_down_within_bounds() {
        assert_eq!(clamped_scroll_position(0.0, 48.0, 600.0, 300.0, 1.0), 48.0);
        assert_eq!(
            clamped_scroll_position(280.0, 48.0, 600.0, 300.0, 1.0),
            300.0
        );
        assert_eq!(clamped_scroll_position(20.0, -48.0, 600.0, 300.0, 1.0), 0.0);
    }

    #[test]
    fn focus_cycle_is_reversible() {
        assert_eq!(FocusTarget::Board.next(false), FocusTarget::Pass);
        assert_eq!(FocusTarget::Board.next(true), FocusTarget::Rules);
        assert_eq!(FocusTarget::Rules.next(false), FocusTarget::Board);
    }

    #[test]
    fn every_session_error_has_user_feedback() {
        let errors = [
            SessionError::InvalidVertex,
            SessionError::Occupied,
            SessionError::Suicide,
            SessionError::Superko,
            SessionError::GameOver,
        ];

        assert!(
            errors
                .into_iter()
                .all(|error| !error_message(error).is_empty())
        );
    }

    #[test]
    fn resignation_confirmation_is_not_opened_after_game_over() {
        let mut session = LocalGameSession::compact();
        session.submit(SessionCommand::Pass).unwrap();
        session.submit(SessionCommand::Pass).unwrap();
        let mut ui = UiState::default();

        request_resign(&session, &mut ui);

        assert_eq!(ui.modal, None);
        assert!(ui.feedback_is_error);
        assert_eq!(ui.feedback, error_message(SessionError::GameOver));
    }
}
