use crate::client::layout;
use crate::{
    board_layout::BoardDefinition,
    client::{
        FocusTarget, ModalKind, SessionResource, UiState, board::BoardRoot, confirm_modal,
        open_rules, request_resign, submit_command,
    },
    game::board::VertexId,
    session::SessionCommand,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub(super) const HIT_RADIUS: f32 = 0.46;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ButtonAction {
    Pass,
    Resign,
    Restart,
    Rules,
    Confirm,
    Cancel,
    CloseRules,
}

pub fn vertex_at_screen_position(
    position: Vec2,
    window: &Window,
    camera: (&Camera, &GlobalTransform),
    root: &Transform,
    definition: &BoardDefinition,
) -> Option<VertexId> {
    let window_size = Vec2::new(window.width(), window.height());
    if !layout::screen_position_is_on_board(window_size, position) {
        return None;
    }
    let world = camera.0.viewport_to_world_2d(camera.1, position).ok()?;
    let local = (world - root.translation.xy()) / root.scale.x;
    nearest_vertex(definition, local, HIT_RADIUS)
}

pub fn nearest_vertex(
    definition: &BoardDefinition,
    position: Vec2,
    radius: f32,
) -> Option<VertexId> {
    definition
        .positions()
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            let distance = Vec2::from_array(*point).distance_squared(position);
            (distance <= radius * radius).then_some((distance, VertexId::new(index)))
        })
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .map(|(_, vertex)| vertex)
}

pub(super) fn update_pointer_target(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    root: Single<&Transform, With<BoardRoot>>,
    session: Res<SessionResource>,
    mut ui: ResMut<UiState>,
) {
    if ui.modal.is_some() {
        ui.hovered = None;
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        ui.hovered = None;
        return;
    };
    ui.hovered = vertex_at_screen_position(cursor, &window, *camera, &root, session.0.definition());
}

pub(super) fn handle_pointer_place(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    root: Single<&Transform, With<BoardRoot>>,
    mut session: ResMut<SessionResource>,
    mut ui: ResMut<UiState>,
) {
    if ui.modal.is_some() {
        return;
    }

    let touched_vertex = touches.iter_just_pressed().find_map(|touch| {
        vertex_at_screen_position(
            touch.position(),
            &window,
            *camera,
            &root,
            session.0.definition(),
        )
    });
    let vertex = touched_vertex.or_else(|| {
        (mouse.just_pressed(MouseButton::Left) && !touches.any_just_pressed())
            .then_some(ui.hovered)
            .flatten()
    });

    if let Some(vertex) = vertex {
        ui.focus = FocusTarget::Board;
        ui.focused_vertex = Some(vertex);
        submit_command(&mut session.0, &mut ui, SessionCommand::Place(vertex));
    }
}

pub(super) fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<SessionResource>,
    mut ui: ResMut<UiState>,
) {
    if let Some(modal) = ui.modal {
        if keyboard.just_pressed(KeyCode::Escape) {
            ui.modal = None;
        } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
            confirm_modal(&mut session.0, &mut ui, modal);
        }
        return;
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        ui.focus = ui
            .focus
            .next(keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));
        if ui.focus == FocusTarget::Board && ui.focused_vertex.is_none() {
            ui.focused_vertex = Some(VertexId::new(0));
        }
    }

    if ui.focus == FocusTarget::Board {
        let direction = if keyboard.just_pressed(KeyCode::ArrowLeft) {
            Some(Vec2::NEG_X)
        } else if keyboard.just_pressed(KeyCode::ArrowRight) {
            Some(Vec2::X)
        } else if keyboard.just_pressed(KeyCode::ArrowUp) {
            Some(Vec2::Y)
        } else if keyboard.just_pressed(KeyCode::ArrowDown) {
            Some(Vec2::NEG_Y)
        } else {
            None
        };

        if let Some(direction) = direction {
            let current = ui.focused_vertex.unwrap_or(VertexId::new(0));
            ui.focused_vertex = navigate_vertex(session.0.definition(), current, direction);
        }

        if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
            let vertex = ui.focused_vertex.unwrap_or(VertexId::new(0));
            ui.focused_vertex = Some(vertex);
            submit_command(&mut session.0, &mut ui, SessionCommand::Place(vertex));
        }
    } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        match ui.focus {
            FocusTarget::Pass => submit_command(&mut session.0, &mut ui, SessionCommand::Pass),
            FocusTarget::Resign => request_resign(&session.0, &mut ui),
            FocusTarget::Restart => ui.modal = Some(ModalKind::Restart),
            FocusTarget::Rules => open_rules(&mut ui),
            FocusTarget::Board => {}
        }
    }
}
pub(super) fn navigate_vertex(
    definition: &BoardDefinition,
    current: VertexId,
    direction: Vec2,
) -> Option<VertexId> {
    let origin = Vec2::from_array(definition.position(current)?);
    definition
        .positions()
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            let candidate = VertexId::new(index);
            if candidate == current {
                return None;
            }
            let delta = Vec2::from_array(*point) - origin;
            let forward = delta.dot(direction);
            if forward <= 0.01 {
                return None;
            }
            let sideways = delta.perp_dot(direction).abs();
            let score = sideways / forward * 100.0 + delta.length();
            Some((score, candidate))
        })
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .map(|(_, vertex)| vertex)
        .or(Some(current))
}

pub(super) fn handle_buttons(
    interactions: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    mut session: ResMut<SessionResource>,
    mut ui: ResMut<UiState>,
) {
    for (interaction, action) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            ButtonAction::Pass if ui.modal.is_none() => {
                ui.focus = FocusTarget::Pass;
                submit_command(&mut session.0, &mut ui, SessionCommand::Pass);
            }
            ButtonAction::Resign if ui.modal.is_none() => {
                ui.focus = FocusTarget::Resign;
                request_resign(&session.0, &mut ui);
            }
            ButtonAction::Restart if ui.modal.is_none() => {
                ui.focus = FocusTarget::Restart;
                ui.modal = Some(ModalKind::Restart);
            }
            ButtonAction::Rules if ui.modal.is_none() => {
                ui.focus = FocusTarget::Rules;
                open_rules(&mut ui);
            }
            ButtonAction::Confirm => {
                if let Some(modal) = ui.modal {
                    confirm_modal(&mut session.0, &mut ui, modal);
                }
            }
            ButtonAction::Cancel => ui.modal = None,
            ButtonAction::CloseRules => ui.modal = None,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::{
        board_layout::BoardDefinition,
        client::input::{HIT_RADIUS, navigate_vertex, nearest_vertex},
        game::board::VertexId,
    };

    #[test]
    fn pointer_hit_testing_uses_the_nearest_vertex() {
        let definition = BoardDefinition::compact();
        let target = VertexId::new(12);
        let position = Vec2::from_array(definition.position(target).unwrap());

        assert_eq!(
            nearest_vertex(&definition, position, HIT_RADIUS),
            Some(target)
        );
        assert_eq!(
            nearest_vertex(&definition, Vec2::splat(100.0), HIT_RADIUS),
            None
        );
    }

    #[test]
    fn directional_navigation_moves_and_stops_at_the_boundary() {
        let definition = BoardDefinition::compact();
        let start = VertexId::new(0);
        let right = navigate_vertex(&definition, start, Vec2::X).unwrap();

        assert_ne!(right, start);
        assert!(definition.position(right).unwrap()[0] > definition.position(start).unwrap()[0]);

        let left = navigate_vertex(&definition, start, Vec2::NEG_X).unwrap();
        assert_eq!(left, start);
    }
}
