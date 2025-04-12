use druid::widget::{Container, Controller, Flex, Label};
use druid::{Color, Env, Event, EventCtx, Widget, WidgetExt};

use crate::ui::AppState;

// Controller to handle click events on a cell
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
    column: Vec<String>,
    data: Vec<Vec<String>>,
    clickable_cols: Vec<(usize, Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>>)>,
) -> impl Widget<AppState> {
    if column.is_empty() {
        println!("Column is empty!");
        return Flex::column();
    }

    if !validate_data(column.len(), &data) {
        println!("Data validation failed: inconsistent row sizes.");
        return Flex::column();
    }

    println!("Column: {:?}", column);
    for row in data.iter() {
        println!("Row: {:?}", row);
    }

    let mut table = Flex::column();

    // Header row with borders
    let mut header_row = Flex::row();
    for header in column.iter() {
        header_row.add_child(bordered_cell(Label::new(header.clone())));
    }
    table.add_child(header_row);
    table.add_spacer(8.0);

    // Data rows with borders
    for row in data.iter() {
        let mut row_container = Flex::row();
        for (col_idx, cell) in row.iter().enumerate() {
            // Check if this column is clickable
            if let Some((_, handler_fn)) = clickable_cols.iter().find(|(idx, _)| *idx == col_idx) {
                // Create a click handler for this cell
                let handler = handler_fn(cell);
                let cell_widget = Label::new(cell.clone());
                let cell_widget = cell_widget.controller(ButtonController::new(Some(handler)));
                row_container.add_child(bordered_cell(cell_widget));
            } else {
                // Non-clickable cell
                row_container.add_child(bordered_cell(Label::new(cell.clone())));
            }
        }
        table.add_child(row_container);
        table.add_spacer(4.0);
    }

    table
}

fn bordered_cell<W: Widget<AppState> + 'static>(child: W) -> impl Widget<AppState> {
    Container::new(child.padding(4.0))
        .border(Color::grey(0.6), 1.0)
        .fix_width(100.0)
        .fix_height(30.0)
}

fn validate_data(col_size: usize, data: &Vec<Vec<String>>) -> bool {
    data.iter().all(|row| row.len() == col_size)
}