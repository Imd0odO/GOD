use std::sync::Arc;
use std::thread::JoinHandle;
use crate::monte_carlo::bounds::Bounds;
use crate::monte_carlo::point::Point;

#[derive(Clone)]
pub struct Function{
    coeficients: Vec<f64>,
}

impl Function {
    fn f(&self, x: &f64) -> f64 {
        let mut f: f64 = 0.0;
        let mut exp: i32 = 0;

        self.coeficients.iter().for_each(|coef| {
            f += coef * x.powi(exp);
            exp += 1;
        });

        return f;
    }

    pub fn new(coeficients: Vec<f64>) -> Function {
        return Function {coeficients};
    }

    fn includes(&self, point: &Point) -> bool {
        let y_0: f64 = self.f(&point.get_x());
        return y_0.abs() > point.get_y().abs() && y_0 * point.get_y() >= 0_f64;
    }

    pub fn approximate(&self, bounds: &Bounds, samples: usize, thread_cnt: usize) -> f64 {
        let mut hits: u64 = 0;
        let mut threads: Vec<JoinHandle<u64>> = vec![];

        let function: Arc<Function> = Arc::new(self.clone());
        let bounds: Arc<Bounds> = Arc::new(*bounds);

        for _ in 0..thread_cnt {
            let function = function.clone();
            let bounds = bounds.clone();

            threads.push(std::thread::spawn(move || {
                let mut hits: u64 = 0;
                for _ in 0..samples/thread_cnt {
                    let point: Point = bounds.get_random_point();
                    if function.includes(&point) {
                        hits += 1;
                    }
                }
                return hits;
            }))
        }

        for _ in 0..samples % thread_cnt {
            let point: Point = bounds.get_random_point();
            if function.includes(&point) {
                hits += 1;
            }
        }

        for thread in threads {
            hits += thread.join().unwrap();
        }

        return (hits as f64 / samples as f64) * bounds.get_area();
    }
}