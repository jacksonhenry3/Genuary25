use nannou::{color::white_point::A, prelude::*, state::mouse};
use rand::prelude;
struct boid {position: Point2,velocity: Vector2}
impl boid {
    pub fn update(&mut self, others: &Vec<boid>) {
        let mut acceleration = Vec2::new(0.0, 0.0);
        let mut count = 0;
        for other in others.iter() {
            let distance = self.position.distance(other.position);
            if distance < 50.0 {
                acceleration += other.position - self.position;
                count += 1;}}
        if count > 0 {
            acceleration /= count as f32;
            self.velocity += acceleration;}}}
impl Clone for boid {
    fn clone(&self) -> Self {
        boid {
            position: self.position,
            velocity: self.velocity,}}}
fn main() {nannou::app(model).update(update).run();}
struct Model {boids: Vec<boid>}
fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    let mut boids = Vec::new();
    for _ in 0..100 {
        boids.push(boid {
            position: pt2(random_range(-400.0, 400.0), random_range(-400.0, 400.0)),
            velocity: vec2(random_range(-10.0, 10.0), random_range(-1.0, 1.0)),});}
        
    Model {boids}}
        
fn update(app: &App, model: &mut Model, _update: Update) {let old_boids = model.boids.clone(); for boid in model.boids.iter_mut() {boid.update(&old_boids);}}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for boid in model.boids.iter() {
        draw.ellipse().xy(boid.position).radius(5.0).color(BLACK);}
    draw.to_frame(app, &frame).unwrap();
}