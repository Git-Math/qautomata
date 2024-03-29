use core::universe::types::{Configuration, Coordinates, Universe};
use lazy_static::lazy_static;
use nannou::{draw::mesh::vertex::Color, glam::Vec2, prelude::*, state::mouse::ButtonPosition};
use nannou_egui::{self, egui, Egui};
use std::sync::Mutex;

lazy_static! {
    static ref STATE_FILE: Mutex<String> = Mutex::new(String::new());
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub enum State {
    Drawing,
    Running,
    Paused,
}

pub enum DrawState {
    Creating,
    Deleting,
    NoDraw,
    NoClick,
}

pub struct Model {
    pub state: State,
    pub draw_state: DrawState,
    pub egui: Egui,
    pub win_w: f32,
    pub win_h: f32,
    pub block_size: f32,
    pub block_stroke: f32,
    pub cols: i32,
    pub rows: i32,
    pub auto_measure: bool,
    pub show_rules_squares: bool,
    pub show_numbers: bool,
    pub universe_file: Option<String>,
    pub universe_measure_max: usize,
    pub universe: Universe,
    pub selected_configuration: Option<usize>,
    pub configurations_max: usize,
}

pub fn run(state_file: Option<String>) {
    *STATE_FILE.lock().unwrap() = match state_file {
        Some(sf) => sf,
        None => "".to_string(),
    };
    nannou::app(model).update(update).view(view).run();
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();

    egui::Window::new("controls")
        .resizable(false)
        .show(&ctx, |ui| {
            ui.horizontal(|ui| match model.state {
                State::Drawing => {
                    if ui.button("Start").clicked() {
                        model.state = State::Paused;
                    }
                }
                State::Running => {
                    if ui.button("Pause").clicked() {
                        model.state = State::Paused;
                    }
                }
                State::Paused => {
                    if ui.button("Reset").clicked() {
                        model.selected_configuration = None;
                        model.universe = match &model.universe_file {
                            Some(universe_file) => {
                                Universe::new_from_files(universe_file.as_str()).unwrap()
                            }
                            None => {
                                model.state = State::Drawing;
                                Universe::new()
                            }
                        }
                    }
                    if ui.button("Run").clicked() {
                        model.state = State::Running;
                    }
                    if ui.button("Step").clicked() {
                        model.universe.step();

                        if model.auto_measure
                            && model.universe.state.len() > model.universe_measure_max
                        {
                            model.universe.measure();
                            model.selected_configuration = None;
                        }
                    }
                    if ui.button("Measure").clicked() {
                        model.universe.measure();
                        model.selected_configuration = None;
                    }
                }
            });
            ui.separator();
            ui.checkbox(&mut model.auto_measure, "Auto measure");
            if model.auto_measure {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut model.universe_measure_max)
                            .clamp_range(2..=65536)
                            .speed(0.1),
                    );
                    ui.label("Max superposed configurations before measure");
                });
            }
            ui.separator();
            ui.label(format!("Step: {}", model.universe.step_count));
            ui.label(format!("Is even step: {}", model.universe.is_even_step));
            ui.checkbox(&mut model.show_rules_squares, "Show rules squares");
            ui.separator();
            ui.label(format!(
                "Configurations count: {}",
                model.universe.state.len()
            ));
            ui.add_space(4.0);
            ui.checkbox(&mut model.show_numbers, "Show numbers");
            let row_height = 10.;
            let num_rows = if model.universe.state.len() <= model.configurations_max {
                model.universe.state.len()
            } else {
                model.configurations_max
            };
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    ui.selectable_value(&mut model.selected_configuration, None, "Combined state");
                    for row in row_range {
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut model.selected_configuration,
                                Some(row),
                                format!(
                                    "Configuration: {}, amplitude: {:.4}, probability: {:.2}%",
                                    row + 1,
                                    model.universe.state[row].amplitude,
                                    model.universe.state[row].amplitude.norm_sqr() * 100.
                                ),
                            );
                        });
                    }
                    if model.universe.state.len() > model.configurations_max {
                        ui.label(format!(
                            "can't show more than {} configurations",
                            model.configurations_max
                        ));
                    }
                });
        });
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title("nannou web test")
        .size(WIDTH, HEIGHT)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let egui_window_ref = app.window(main_window).unwrap();
    let egui = Egui::from_window(&egui_window_ref);

    // If a state file is provided, we use it to create the universe
    // Else we create an empty universe in which we can draw cells
    let state_file = STATE_FILE.lock().unwrap();
    let (universe, universe_file, state) = match state_file.as_str() {
        "" => (Universe::new(), None, State::Drawing),
        sf => (
            Universe::new_from_files(sf).unwrap(),
            Some(sf.to_string()),
            State::Running,
        ),
    };

    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();
    let block_size = 32.;
    let block_stroke = 0.2;
    let cols = (win_w / block_size).ceil() as i32;
    let rows = (win_h / block_size).ceil() as i32;

    Model {
        state,
        draw_state: DrawState::NoClick,
        egui,
        win_w,
        win_h,
        block_size,
        block_stroke,
        cols,
        rows,
        auto_measure: true,
        show_rules_squares: false,
        show_numbers: false,
        universe_file,
        universe_measure_max: 128,
        selected_configuration: None,
        universe,
        configurations_max: 1024,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match model.state {
        State::Drawing => update_drawing(app, model),
        State::Running => {
            let frame_to_skip = 10;
            // Since we are unable to set the frame rate of the nannou app
            // we use this trick to skip some frames in order to slow down
            // the simulation. You can find out more in this comment: https://github.com/nannou-org/nannou/issues/708#issuecomment-1047032678
            if app.elapsed_frames() % frame_to_skip != 0 {
                return;
            } else {
                model.universe.step();

                if model.auto_measure && model.universe.state.len() > model.universe_measure_max {
                    model.universe.measure();
                    model.selected_configuration = None;
                }
            }
        }
        State::Paused => (),
    }

    update_ui(model);
}

fn update_drawing(app: &App, model: &mut Model) {
    match app.mouse.buttons.left() {
        ButtonPosition::Up => {
            if !matches!(model.draw_state, DrawState::NoClick) {
                model.draw_state = DrawState::NoClick;
            }
        }
        ButtonPosition::Down(click_pos) => {
            if matches!(model.draw_state, DrawState::NoClick) {
                model.draw_state = match get_cell_coordinates(click_pos, model) {
                    Some(cell_coordinates) => {
                        match model.universe.state[0].living_cells.get(&cell_coordinates) {
                            Some(_) => DrawState::Deleting,
                            None => DrawState::Creating,
                        }
                    }
                    None => DrawState::NoDraw,
                };
            }

            if matches!(model.draw_state, DrawState::Creating | DrawState::Deleting) {
                let mouse_pos = app.mouse.position();

                if let Some(cell_coordinates) = get_cell_coordinates(&mouse_pos, model) {
                    match model.universe.state[0].living_cells.get(&cell_coordinates) {
                        Some(_) => {
                            if matches!(model.draw_state, DrawState::Deleting) {
                                model.universe.state[0]
                                    .living_cells
                                    .remove(&cell_coordinates);
                            }
                        }
                        None => {
                            if matches!(model.draw_state, DrawState::Creating) {
                                model.universe.state[0]
                                    .living_cells
                                    .insert(cell_coordinates, false);
                            }
                        }
                    };
                    model.universe.compute_combined_state();
                };
            }
        }
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let universe = &model.universe;
    let draw = app.draw();
    let m = &model;

    let gray = Color::new(22. / 255., 27. / 255., 34. / 255., 1.);
    draw.background().color(gray);

    let gdraw = draw.x_y(
        m.block_size / 2. - m.win_w / 2.0,
        (m.block_size / 2. - m.win_h / 2.0) * -1.0,
    );

    for i in 0..m.cols {
        for j in 0..m.rows {
            match m.selected_configuration {
                None => {
                    draw_combined_state(i, j, universe, &gdraw, m);
                }
                Some(index) => {
                    if index >= universe.state.len() {
                        continue;
                    }
                    draw_configuration(i, j, &universe.state[index], &gdraw, m);
                }
            }
        }
    }

    if m.show_rules_squares {
        draw_rules_squares(&draw, m);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
    ui_view(app, model, frame);
}

fn draw_combined_state(i: i32, j: i32, universe: &Universe, gdraw: &Draw, m: &Model) {
    match universe.combined_state.get(&Coordinates { x: i, y: j }) {
        Some(probability) => {
            //This required because of nannou's coordinate system
            //where the origin is the center of the window
            //and the y axis is inverted, so we need to have a negative
            //y coodinate
            let j = -j;
            //draw living cells
            let green = Color::new(0.0, 1., 0.0, *probability as f32);
            gdraw
                .rect()
                .stroke(GRAY)
                .stroke_weight(m.block_stroke)
                .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                .w_h(m.block_size, m.block_size)
                .color(green);

            if m.show_numbers {
                gdraw
                    .text(format!("{:.2}", *probability as f32).as_str())
                    .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                    .font_size(12)
                    .color(BLACK);
            }
        }
        None => {
            //This required because of nannou's coordinate system
            //where the origin is the center of the window
            //and the y axis is inverted, so we need to have a negative
            //y coodinate
            let j = -j;
            gdraw
                .rect()
                .no_fill()
                .stroke(GRAY)
                .stroke_weight(m.block_stroke)
                .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                .w_h(m.block_size, m.block_size);
        }
    }
}

fn draw_configuration(i: i32, j: i32, configuration: &Configuration, gdraw: &Draw, m: &Model) {
    match configuration.living_cells.get(&Coordinates { x: i, y: j }) {
        Some(_) => {
            //This required because of nannou's coordinate system
            //where the origin is the center of the window
            //and the y axis is inverted, so we need to have a negative
            //y coodinate
            let j = -j;
            //draw living cells
            let green = Color::new(0.0, 1., 0.0, 1.);
            gdraw
                .rect()
                .stroke(GRAY)
                .stroke_weight(m.block_stroke)
                .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                .w_h(m.block_size, m.block_size)
                .color(green);
        }
        None => {
            //This required because of nannou's coordinate system
            //where the origin is the center of the window
            //and the y axis is inverted, so we need to have a negative
            //y coodinate
            let j = -j;
            gdraw
                .rect()
                .no_fill()
                .stroke(GRAY)
                .stroke_weight(m.block_stroke)
                .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                .w_h(m.block_size, m.block_size);
        }
    }
}

fn draw_rules_squares(draw: &Draw, m: &Model) {
    let black = Color::new(0., 0., 0., 1.);
    // Set the starting index depending on the step parity
    // On even steps, we will draw lines on indexes 0/2/4/...
    // On odd steps, we will draw lines on indexes 1/3/5/...
    let s = if m.universe.is_even_step { 0 } else { 1 };

    // Draw the vertical lines of the rules squares
    let start_y = (m.rows as f32 / 2.) * m.block_size;
    let end_y = -start_y;
    for i in (s..(m.cols + 1)).step_by(2) {
        let x = (i as f32 - (m.cols as f32 / 2.)) * m.block_size;
        draw.line()
            .start(pt2(x, start_y))
            .end(pt2(x, end_y))
            .weight(m.block_stroke * 3.)
            .color(black);
    }

    // Draw the horizontal lines of the rules squares
    let start_x = (m.cols as f32 / 2.) * m.block_size;
    let end_x = -start_x;
    for j in (s..(m.rows + 1)).step_by(2) {
        let y = (j as f32 - (m.rows as f32 / 2.)) * m.block_size;
        draw.line()
            .start(pt2(start_x, y))
            .end(pt2(end_x, y))
            .weight(m.block_stroke * 3.)
            .color(black);
    }
}

fn get_cell_coordinates(pos: &Vec2, m: &Model) -> Option<Coordinates> {
    let x = ((pos.x + (m.win_w / 2.)) / m.block_size) as i32;
    let y = (((pos.y * -1.) + (m.win_h / 2.)) / m.block_size) as i32;

    if x >= 0 && x < m.cols && y >= 0 && y < m.rows {
        Some(Coordinates { x, y })
    } else {
        None
    }
}
