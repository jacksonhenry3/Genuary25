mod layer;
use nannou::{color::white_point::A, prelude::*, state::mouse};
use rand::prelude;



fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    layers: Vec<layer::Layer>,
    mouse_pos: Point2,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    let bottom = [0.0;400];
    // make 10 layers
    let height = 50.0;
    let mut previous_layer_top = [0.0;400];
    let mut layers = vec![];
    for _ in 0..10 {
        let layer = layer::Layer::new(height, previous_layer_top);
        previous_layer_top = layer.upper_bound;
        layers.push(layer);
    }

    Model {
        layers,
        mouse_pos: app.mouse.position(),
    }

    
}


impl Model {
    pub fn update(&mut self,mouse_pos:Point2) {

        self.mouse_pos = mouse_pos;
        

        let mut previous_layer_top = [0.0;400];
        for layer in self.layers.iter_mut() {
           layer.lower_bound = previous_layer_top;
            previous_layer_top = layer.upper_bound;

        }

        for layer in self.layers.iter_mut() {
            layer.update(self.mouse_pos);
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    let win = app.window_rect();

    // before calculating the distance normalize using map_range
    let mouse_pos_x = map_range(app.mouse.position().x, win.left(), win.right(), 0.0, 400.0);
    let mouse_pos_y = map_range(app.mouse.position().y, win.bottom(), win.top(), 0.0, 400.0);
    let mouse_pos = pt2(mouse_pos_x, mouse_pos_y);
    // println!("mouse_pos: {:?}. window {:?}", app.mouse.position(), win);
    model.update(mouse_pos);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win = app.window_rect();
    let step = win.w() / 400 as f32;

    for layer in model.layers.iter() {

        for (i, &y) in layer.lower_bound.iter().enumerate() {
            let x = map_range(i, 0, 400, win.left(), win.right());
            let height = layer.upper_bound[i] - y;
            let y_lower =  map_range(layer.lower_bound[i], 0.0, 400.0, win.bottom(), win.top());
            let y_upper =  map_range(layer.upper_bound[i], 0.0, 400.0, win.bottom(), win.top());
            let step = win.w() / 400 as f32;
            
        
    
        draw.line()
            .start(pt2(x, y_lower-10.0))
            .end(pt2(x, y_upper+10.0))
            .color(layer.color)
            .weight(step);
        }
        
    };
    

    draw.to_frame(app, &frame).unwrap();
}