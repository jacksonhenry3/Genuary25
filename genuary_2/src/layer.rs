use nannou::prelude::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub struct Layer {
    pub upper_bound: [f32; 400],
    pub lower_bound: [f32; 400],
    pub color: Rgb,
}

impl Layer {
    pub fn new(height: f32, lower_bound: [f32; 400]) -> Self {
        let test = srgb(0.1, 0.1, 0.1);
        let earth_tones = [
            srgb(0.4314, 0.2941, 0.2275), // Dark brown (#6E4B3A)
            srgb(0.5451, 0.3529, 0.2549), // Medium brown (#8B5A41)
            srgb(0.6431, 0.4235, 0.3098), // Light brown (#A46C4F)
            srgb(0.6980, 0.5216, 0.4039), // Beige brown (#B28567)
            srgb(0.7922, 0.7294, 0.6314), // Sandstone (#CABAA1)
            srgb(0.7216, 0.6588, 0.5490), // Stone gray (#B8A88C)
            srgb(0.5882, 0.5333, 0.4392), // Earthy taupe (#968970)
            srgb(0.4784, 0.4235, 0.3569), // Warm gray (#7A6C5B)
            srgb(0.3725, 0.3255, 0.2667), // Darker stone gray (#5F5344)
            srgb(0.2667, 0.2275, 0.1843), // Deep earthy brown (#443A2F)
        ];
        let mut rng = thread_rng();
        let color = *earth_tones.choose(&mut rng).unwrap();

        let mut upper_bound = lower_bound.clone();
        let variation = (0..400)
            .map(|_| match rng.gen_range(0..3) {
                0 => 0.0,
                1 => 1.0,
                _ => -1.0,
            })
            .scan(0.0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();

        for (i, upper_pnt) in upper_bound.iter_mut().enumerate() {
            *upper_pnt += variation[i] + height;
        }

        Self {
            upper_bound,
            lower_bound,
            color,
        }
    }

    pub fn update(&mut self,mouse_pos:Point2) {
        let mut rng = thread_rng();
        let variation = (0..400)
            .map(|_| match rng.gen_range(0..3) {
                0 => 0.0,
                1 => 2.0,
                _ => -2.0,
            })
            .collect::<Vec<_>>();


        for (i, upper_pnt) in self.upper_bound.iter_mut().enumerate() {


            
            // println!("mouse_pos: {:?}", mouse_pos);

            *upper_pnt += variation[i];

            // Calculate bias towards mouse position
            let point_pos = pt2(i as f32, *upper_pnt);
            let distance_to_mouse = point_pos.distance(mouse_pos);
            if distance_to_mouse < 1.0 {
                continue;
            }
            if distance_to_mouse > 100.0 {
                continue;
            }
            let bias = (1.0 / (distance_to_mouse + 1.0)) * 10.0; // Adjust the bias factor as needed

            if mouse_pos.y > *upper_pnt {
                *upper_pnt += bias;
            } else {
                *upper_pnt -= bias;
            }
        }

        // Now pull each point towards its neighbors
        for i in 0..400 {
            let diff = self.upper_bound[i] - self.upper_bound[(i + 399) % 400];
            self.upper_bound[i] -= diff / 2.0;

            let diff = self.upper_bound[i] - self.upper_bound[(i + 1) % 400];
            self.upper_bound[i] -= diff / 2.0;

            let diff = self.upper_bound[i] - self.upper_bound[(i + 398) % 400];
            self.upper_bound[i] -= diff / 4.0;
        }
    }
}
