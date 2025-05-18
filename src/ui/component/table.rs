use super::super::AppState;
use druid::widget::{Container, Controller, Flex, Label};
use druid::{Color, Env, Event, EventCtx, Widget, WidgetExt};

struct ButtonController {
    on_click: Option<Box<dyn Fn(&mut EventCtx, &mut AppState)>>,
}

impl ButtonController {
    fn new(on_click: Option<Box<dyn Fn(&mut EventCtx, &mut AppState)>>) -> Self {
        ButtonController { on_click }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for ButtonController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if let Some(f) = &self.on_click {
                        f(ctx, data);
                    }
                    ctx.request_paint();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

pub fn make_table(
    columns: Vec<String>,
    data: Vec<Vec<String>>,
    clickable_cols: Vec<(
        usize,
        Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>>,
    )>,
) -> impl Widget<AppState> {
    if columns.is_empty() {
        return Flex::column();
    }

    if !validate_data(columns.len(), &data) {
        return Flex::column();
    }

    let col_widths = calculate_column_widths(&columns, &data);

    let mut table = Flex::column();

    let mut header_row = Flex::row();
    for (i, header) in columns.iter().enumerate() {
        header_row.add_child(bordered_cell(Label::new(header.clone()), col_widths[i]));
    }
    table.add_child(header_row);
    table.add_spacer(8.0);

    for row in data.iter() {
        let mut row_container = Flex::row();
        for (col_idx, cell) in row.iter().enumerate() {
            if let Some((_, handler_fn)) = clickable_cols.iter().find(|(idx, _)| *idx == col_idx) {
                let handler = handler_fn(cell);
                let cell_widget = Label::new(cell.clone());
                let cell_widget = cell_widget.controller(ButtonController::new(Some(handler)));
                row_container.add_child(bordered_cell(cell_widget, col_widths[col_idx]));
            } else {
                row_container
                    .add_child(bordered_cell(Label::new(cell.clone()), col_widths[col_idx]));
            }
        }
        table.add_child(row_container);
        table.add_spacer(4.0);
    }

    table
}

fn bordered_cell<W: Widget<AppState> + 'static>(child: W, width: f64) -> impl Widget<AppState> {
    Container::new(child.padding(4.0))
        .border(Color::grey(0.6), 1.0)
        .fix_width(width)
        .fix_height(30.0)
}

fn calculate_column_widths(column: &[String], data: &[Vec<String>]) -> Vec<f64> {
    let mut widths = vec![0; column.len()];

    for (i, header) in column.iter().enumerate() {
        widths[i] = header.len();
    }

    for row in data {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }

    widths
        .into_iter()
        .map(|len| (len as f64) * 8.0 + 16.0)
        .collect()
}

fn validate_data(col_size: usize, data: &Vec<Vec<String>>) -> bool {
    data.iter().all(|row| row.len() == col_size)
}
