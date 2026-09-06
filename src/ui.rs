use std::fs;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResizeConstraints, WindowResolution},
};

use crate::{
    board_layout::BoardDefinition,
    game::{
        GameResult,
        board::VertexId,
        player::Player,
        state::{GameStatus, VertexState},
    },
    session::{LocalGameSession, SessionCommand, SessionError},
};

const SIDEBAR_WIDTH: f32 = 300.0;
const BOARD_PADDING: f32 = 72.0;
const HIT_RADIUS: f32 = 0.46;
const BOARD_BACKGROUND: Color = Color::srgb(0.84, 0.69, 0.39);
const PANEL_BACKGROUND: Color = Color::srgb(0.17, 0.13, 0.09);
const LINE_COLOR: Color = Color::srgb(0.22, 0.16, 0.10);
const TEXT_COLOR: Color = Color::srgb(1.0, 0.96, 0.89);
const MUTED_TEXT: Color = Color::srgb(0.82, 0.72, 0.55);
const ACCENT: Color = Color::srgb(0.84, 0.61, 0.23);
const WARNING: Color = Color::srgb(0.78, 0.30, 0.21);
const ERROR: Color = Color::srgb(0.89, 0.38, 0.31);

pub struct HexGoUiPlugin;

impl Plugin for HexGoUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BOARD_BACKGROUND))
            .insert_resource(SessionResource(LocalGameSession::compact()))
            .init_resource::<UiState>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    fit_board_to_window,
                    update_pointer_target,
                    handle_pointer_place,
                    handle_keyboard,
                    handle_buttons,
                    sync_stones,
                    sync_preview,
                    sync_focus_marker,
                    sync_last_move_marker,
                    sync_current_player,
                    sync_pass_count,
                    sync_result,
                    sync_feedback,
                    sync_modal,
                    style_buttons,
                )
                    .chain(),
            );
    }
}

pub fn primary_window() -> Window {
    Window {
        title: "HexGo".into(),
        resolution: WindowResolution::new(1280, 800),
        resize_constraints: WindowResizeConstraints {
            min_width: 1000.0,
            min_height: 700.0,
            ..default()
        },
        resizable: true,
        ..default()
    }
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
}

impl FocusTarget {
    fn next(self, reverse: bool) -> Self {
        const ORDER: [FocusTarget; 4] = [
            FocusTarget::Board,
            FocusTarget::Pass,
            FocusTarget::Resign,
            FocusTarget::Restart,
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

#[derive(Component)]
struct BoardRoot;

#[derive(Component)]
struct Stone(VertexId);

#[derive(Component)]
struct PreviewStone;

#[derive(Component)]
struct FocusMarker;

#[derive(Component)]
struct LastMoveMarker;

#[derive(Component)]
struct CurrentPlayerText;

#[derive(Component)]
struct PassCountText;

#[derive(Component)]
struct FeedbackText;

#[derive(Component)]
struct ResultPanel;

#[derive(Component)]
struct ResultText;

#[derive(Component)]
struct ModalOverlay;

#[derive(Component)]
struct ModalText;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
enum ButtonAction {
    Pass,
    Resign,
    Restart,
    Confirm,
    Cancel,
}

#[derive(Resource)]
struct StoneMaterials {
    black: Handle<ColorMaterial>,
    white: Handle<ColorMaterial>,
    preview_black: Handle<ColorMaterial>,
    preview_white: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fonts: ResMut<Assets<Font>>,
    session: Res<SessionResource>,
) {
    commands.spawn(Camera2d);

    let font = load_cjk_font(&mut fonts);
    let stone_materials = StoneMaterials {
        black: materials.add(Color::srgb(0.055, 0.052, 0.045)),
        white: materials.add(Color::srgb(0.98, 0.95, 0.89)),
        preview_black: materials.add(Color::srgba(0.055, 0.052, 0.045, 0.50)),
        preview_white: materials.add(Color::srgba(0.98, 0.95, 0.89, 0.72)),
    };

    let root = commands
        .spawn((BoardRoot, Transform::default(), Visibility::Visible))
        .id();
    commands.entity(root).with_children(|parent| {
        for &(left, right) in session.0.definition().edges() {
            let left = Vec2::from_array(session.0.definition().position(left).unwrap());
            let right = Vec2::from_array(session.0.definition().position(right).unwrap());
            let delta = right - left;
            parent.spawn((
                Sprite::from_color(LINE_COLOR, Vec2::new(delta.length() + 0.025, 0.055)),
                Transform::from_translation(((left + right) * 0.5).extend(0.0))
                    .with_rotation(Quat::from_rotation_z(delta.y.atan2(delta.x))),
            ));
        }

        let stone_mesh = meshes.add(Circle::new(0.34));
        for vertex in session.0.definition().graph().vertices() {
            let position = Vec2::from_array(session.0.definition().position(vertex).unwrap());
            parent.spawn((
                Stone(vertex),
                Mesh2d(stone_mesh.clone()),
                MeshMaterial2d(stone_materials.black.clone()),
                Transform::from_translation(position.extend(2.0)),
                Visibility::Hidden,
            ));
        }

        parent.spawn((
            FocusMarker,
            Mesh2d(meshes.add(Circle::new(0.43))),
            MeshMaterial2d(materials.add(Color::srgba(0.96, 0.63, 0.22, 0.78))),
            Transform::from_xyz(0.0, 0.0, 1.0),
            Visibility::Hidden,
        ));
        parent.spawn((
            PreviewStone,
            Mesh2d(stone_mesh),
            MeshMaterial2d(stone_materials.preview_black.clone()),
            Transform::from_xyz(0.0, 0.0, 3.0),
            Visibility::Hidden,
        ));
        parent.spawn((
            LastMoveMarker,
            Mesh2d(meshes.add(Circle::new(0.09))),
            MeshMaterial2d(materials.add(WARNING)),
            Transform::from_xyz(0.0, 0.0, 4.0),
            Visibility::Hidden,
        ));
    });

    commands.insert_resource(stone_materials);
    spawn_sidebar(&mut commands, &font);
    spawn_modal(&mut commands, &font);
}

fn load_cjk_font(fonts: &mut Assets<Font>) -> Handle<Font> {
    const CANDIDATES: &[&str] = &[
        "/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/wenquanyi/wqy-zenhei/wqy-zenhei.ttc",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "/System/Library/Fonts/PingFang.ttc",
    ];

    for candidate in CANDIDATES {
        if let Ok(bytes) = fs::read(candidate) {
            info!("Loaded UI font from {}", candidate);
            return fonts.add(Font::from_bytes(bytes));
        }
    }

    warn!("No supported CJK system font was found; using Bevy's default font");
    Handle::default()
}

fn spawn_sidebar(commands: &mut Commands, font: &Handle<Font>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: px(0),
                top: px(0),
                width: px(SIDEBAR_WIDTH),
                height: percent(100),
                padding: UiRect::all(px(28)),
                flex_direction: FlexDirection::Column,
                row_gap: px(16),
                ..default()
            },
            BackgroundColor(PANEL_BACKGROUND),
        ))
        .with_children(|panel| {
            panel.spawn(text_bundle("HEXGO", font, 31.0, TEXT_COLOR));
            panel.spawn(text_bundle("本地双人对局", font, 16.0, MUTED_TEXT));
            panel.spawn((Node {
                height: px(16),
                ..default()
            },));
            panel.spawn((
                CurrentPlayerText,
                text_bundle("当前执子：黑方", font, 23.0, TEXT_COLOR),
            ));
            panel.spawn((
                PassCountText,
                text_bundle("连续停着：0 / 2", font, 17.0, MUTED_TEXT),
            ));
            panel.spawn((
                FeedbackText,
                text_bundle("请选择一个交点落子", font, 16.0, MUTED_TEXT),
                Node {
                    min_height: px(52),
                    margin: UiRect::vertical(px(8)),
                    ..default()
                },
            ));
            panel.spawn(action_button(ButtonAction::Pass, "停着", font));
            panel.spawn(action_button(ButtonAction::Resign, "认输", font));
            panel.spawn(action_button(ButtonAction::Restart, "重新开始", font));
            panel
                .spawn((
                    ResultPanel,
                    Node {
                        display: Display::None,
                        width: percent(100),
                        margin: UiRect::top(px(14)),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(1)),
                        border_radius: BorderRadius::all(px(8)),
                        ..default()
                    },
                    BorderColor::all(ACCENT),
                    BackgroundColor(Color::srgb(0.23, 0.17, 0.11)),
                ))
                .with_children(|result| {
                    result.spawn((
                        ResultText,
                        text_bundle("", font, 16.0, TEXT_COLOR),
                        Node {
                            width: percent(100),
                            ..default()
                        },
                    ));
                });
            panel.spawn((Node {
                flex_grow: 1.0,
                ..default()
            },));
            panel.spawn(text_bundle(
                "Tab 切换区域 · 方向键选择 · Enter 确认",
                font,
                13.0,
                MUTED_TEXT,
            ));
        });
}

fn spawn_modal(commands: &mut Commands, font: &Handle<Font>) {
    commands
        .spawn((
            ModalOverlay,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GlobalZIndex(100),
            BackgroundColor(Color::srgba(0.08, 0.055, 0.03, 0.76)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: px(410),
                        padding: UiRect::all(px(28)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(18),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                ))
                .with_children(|dialog| {
                    dialog.spawn((ModalText, text_bundle("", font, 21.0, TEXT_COLOR)));
                    dialog
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(12),
                            ..default()
                        },))
                        .with_children(|buttons| {
                            buttons.spawn(action_button(ButtonAction::Confirm, "确认", font));
                            buttons.spawn(action_button(ButtonAction::Cancel, "取消", font));
                        });
                });
        });
}

fn text_bundle(text: &str, font: &Handle<Font>, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone().into(),
            font_size: FontSize::Px(size),
            ..default()
        },
        TextLayout::new(Justify::Left, LineBreak::NoWrap),
        TextColor(color),
    )
}

fn action_button(action: ButtonAction, label: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Button,
        action,
        Node {
            width: percent(100),
            height: px(48),
            padding: UiRect::horizontal(px(16)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        },
        BorderColor::all(Color::NONE),
        BackgroundColor(Color::srgb(0.35, 0.25, 0.16)),
        children![text_bundle(label, font, 17.0, TEXT_COLOR)],
    )
}

fn fit_board_to_window(
    window: Single<&Window, With<PrimaryWindow>>,
    session: Res<SessionResource>,
    mut root: Single<&mut Transform, With<BoardRoot>>,
) {
    let board_width = (window.width() - SIDEBAR_WIDTH - BOARD_PADDING * 2.0).max(1.0);
    let board_height = (window.height() - BOARD_PADDING * 2.0).max(1.0);
    let (min, max) = session.0.definition().bounds();
    let extent = Vec2::new(max[0] - min[0], max[1] - min[1]);
    let scale = (board_width / extent.x)
        .min(board_height / extent.y)
        .max(1.0);
    root.scale = Vec3::splat(scale);
    root.translation = Vec3::new(-SIDEBAR_WIDTH * 0.5, 0.0, 0.0);
}

fn update_pointer_target(
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
    if cursor.x >= window.width() - SIDEBAR_WIDTH {
        ui.hovered = None;
        return;
    }
    let Ok(world) = camera.0.viewport_to_world_2d(camera.1, cursor) else {
        ui.hovered = None;
        return;
    };
    let local = (world - root.translation.xy()) / root.scale.x;
    ui.hovered = nearest_vertex(session.0.definition(), local, HIT_RADIUS);
}

fn nearest_vertex(definition: &BoardDefinition, position: Vec2, radius: f32) -> Option<VertexId> {
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

fn handle_pointer_place(
    mouse: Res<ButtonInput<MouseButton>>,
    mut session: ResMut<SessionResource>,
    mut ui: ResMut<UiState>,
) {
    if mouse.just_pressed(MouseButton::Left)
        && ui.modal.is_none()
        && let Some(vertex) = ui.hovered
    {
        ui.focus = FocusTarget::Board;
        ui.focused_vertex = Some(vertex);
        submit_command(&mut session.0, &mut ui, SessionCommand::Place(vertex));
    }
}

fn handle_keyboard(
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
            FocusTarget::Board => {}
        }
    }
}

fn navigate_vertex(
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

fn handle_buttons(
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
            ButtonAction::Confirm => {
                if let Some(modal) = ui.modal {
                    confirm_modal(&mut session.0, &mut ui, modal);
                }
            }
            ButtonAction::Cancel => ui.modal = None,
            _ => {}
        }
    }
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
    }
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

fn sync_stones(
    session: Res<SessionResource>,
    materials: Res<StoneMaterials>,
    mut stones: Query<(&Stone, &mut Visibility, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    for (stone, mut visibility, mut material) in &mut stones {
        match session.0.vertex_state(stone.0) {
            Some(VertexState::Occupied(Player::Black)) => {
                *visibility = Visibility::Visible;
                material.0 = materials.black.clone();
            }
            Some(VertexState::Occupied(Player::White)) => {
                *visibility = Visibility::Visible;
                material.0 = materials.white.clone();
            }
            _ => *visibility = Visibility::Hidden,
        }
    }
}

fn sync_preview(
    session: Res<SessionResource>,
    ui: Res<UiState>,
    materials: Res<StoneMaterials>,
    mut preview: Single<
        (
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial2d<ColorMaterial>,
        ),
        With<PreviewStone>,
    >,
) {
    let preview_vertex = ui.hovered.or((ui.focus == FocusTarget::Board)
        .then_some(ui.focused_vertex)
        .flatten());
    if ui.modal.is_none()
        && session.0.status() == GameStatus::Playing
        && let Some(vertex) = preview_vertex
        && session.0.vertex_state(vertex) == Some(VertexState::Empty)
    {
        let position = Vec2::from_array(session.0.definition().position(vertex).unwrap());
        preview.0.translation = position.extend(3.0);
        *preview.1 = Visibility::Visible;
        preview.2.0 = match session.0.current_player() {
            Player::Black => materials.preview_black.clone(),
            Player::White => materials.preview_white.clone(),
        };
    } else {
        *preview.1 = Visibility::Hidden;
    }
}

fn sync_focus_marker(
    session: Res<SessionResource>,
    ui: Res<UiState>,
    mut focus: Single<(&mut Transform, &mut Visibility), With<FocusMarker>>,
) {
    if ui.focus == FocusTarget::Board
        && ui.modal.is_none()
        && let Some(vertex) = ui.focused_vertex
    {
        let position = Vec2::from_array(session.0.definition().position(vertex).unwrap());
        focus.0.translation = position.extend(1.0);
        *focus.1 = Visibility::Visible;
    } else {
        *focus.1 = Visibility::Hidden;
    }
}

fn sync_last_move_marker(
    session: Res<SessionResource>,
    mut last: Single<(&mut Transform, &mut Visibility), With<LastMoveMarker>>,
) {
    if let Some(vertex) = session.0.last_move() {
        let position = Vec2::from_array(session.0.definition().position(vertex).unwrap());
        last.0.translation = position.extend(4.0);
        *last.1 = Visibility::Visible;
    } else {
        *last.1 = Visibility::Hidden;
    }
}

fn sync_current_player(
    session: Res<SessionResource>,
    mut current_player: Single<&mut Text, With<CurrentPlayerText>>,
) {
    let value = match session.0.status() {
        GameStatus::Playing => format!("当前执子：{}", player_name(session.0.current_player())),
        GameStatus::Finished(_) => "对局已结束".into(),
    };
    if current_player.0 != value {
        current_player.0 = value;
    }
}

fn sync_pass_count(
    session: Res<SessionResource>,
    mut pass_count: Single<&mut Text, With<PassCountText>>,
) {
    let value = format!("连续停着：{} / 2", session.0.consecutive_passes());
    if pass_count.0 != value {
        pass_count.0 = value;
    }
}

fn sync_result(
    session: Res<SessionResource>,
    mut result_panel: Single<&mut Node, With<ResultPanel>>,
    mut result_text: Single<&mut Text, With<ResultText>>,
) {
    if let Some(result) = session.0.result() {
        result_panel.display = Display::Flex;
        let value = result_summary(&session.0, result);
        if result_text.0 != value {
            result_text.0 = value;
        }
    } else {
        result_panel.display = Display::None;
    }
}

fn sync_feedback(
    ui: Res<UiState>,
    mut feedback: Single<(&mut Text, &mut TextColor), With<FeedbackText>>,
) {
    let value = if ui.feedback.is_empty() {
        "请选择一个交点落子".into()
    } else {
        ui.feedback.clone()
    };
    if feedback.0.0 != value {
        feedback.0.0 = value;
    }
    feedback.1.0 = if ui.feedback_is_error {
        ERROR
    } else {
        MUTED_TEXT
    };
}

fn player_name(player: Player) -> &'static str {
    match player {
        Player::Black => "黑方",
        Player::White => "白方",
    }
}

fn result_summary(session: &LocalGameSession, result: GameResult) -> String {
    match result {
        GameResult::WinByResignation { winner } => {
            format!("对局结果\n{}因对方认输获胜", player_name(winner))
        }
        GameResult::WinByScore { winner, margin } => {
            let score = session.score_breakdown();
            format!(
                "对局结果\n{}胜 {:.1} 目\n\n黑方\n棋子：{}\n领地：{}\n总分：{:.1}\n\n白方\n棋子：{}\n领地：{}\n贴目：{:.1}\n总分：{:.1}",
                player_name(winner),
                margin,
                score.black_stones,
                score.black_territory,
                score.black_total,
                score.white_stones,
                score.white_territory,
                score.komi,
                score.white_total,
            )
        }
        GameResult::Draw => {
            let score = session.score_breakdown();
            format!(
                "对局结果\n和棋\n\n黑方总分：{:.1}\n白方总分：{:.1}",
                score.black_total, score.white_total
            )
        }
    }
}

fn sync_modal(
    ui: Res<UiState>,
    mut overlay: Single<&mut Node, With<ModalOverlay>>,
    mut text: Single<&mut Text, With<ModalText>>,
) {
    match ui.modal {
        Some(ModalKind::Resign) => {
            overlay.display = Display::Flex;
            let value = "确认认输？\n当前对局将立即结束。";
            if text.0 != value {
                text.0 = value.into();
            }
        }
        Some(ModalKind::Restart) => {
            overlay.display = Display::Flex;
            let value = "确认重新开始？\n当前棋盘进度将被清空。";
            if text.0 != value {
                text.0 = value.into();
            }
        }
        None => overlay.display = Display::None,
    }
}

fn style_buttons(
    session: Res<SessionResource>,
    ui: Res<UiState>,
    mut buttons: Query<(
        &Interaction,
        &ButtonAction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    for (interaction, action, mut background, mut border) in &mut buttons {
        let disabled = session.0.status() != GameStatus::Playing
            && matches!(action, ButtonAction::Pass | ButtonAction::Resign);
        let focused = match action {
            ButtonAction::Pass => ui.focus == FocusTarget::Pass,
            ButtonAction::Resign => ui.focus == FocusTarget::Resign,
            ButtonAction::Restart => ui.focus == FocusTarget::Restart,
            ButtonAction::Confirm | ButtonAction::Cancel => false,
        };
        background.0 = if disabled {
            Color::srgb(0.23, 0.17, 0.12)
        } else {
            match interaction {
                Interaction::Pressed => Color::srgb(0.61, 0.42, 0.17),
                Interaction::Hovered => Color::srgb(0.46, 0.33, 0.20),
                Interaction::None => Color::srgb(0.35, 0.25, 0.16),
            }
        };
        *border = BorderColor::all(if focused { ACCENT } else { Color::NONE });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn focus_cycle_is_reversible() {
        assert_eq!(FocusTarget::Board.next(false), FocusTarget::Pass);
        assert_eq!(FocusTarget::Board.next(true), FocusTarget::Restart);
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

    #[test]
    fn score_result_lines_fit_the_sidebar_card() {
        let mut session = LocalGameSession::compact();
        session.submit(SessionCommand::Pass).unwrap();
        session.submit(SessionCommand::Pass).unwrap();
        let summary = result_summary(&session, session.result().unwrap());

        assert!(summary.lines().all(|line| line.chars().count() <= 14));
    }
}
