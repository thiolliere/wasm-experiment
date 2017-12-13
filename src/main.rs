#![recursion_limit="500"]
#[macro_use] extern crate stdweb;
extern crate winit;

use stdweb::unstable::TryInto;
use std::os::raw::c_void;
use std::ptr;
use std::cell::RefCell;

mod ffi;
mod animation;

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(ptr::null_mut()));

fn set_main_loop_callback<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { ::ffi::emscripten_set_main_loop(Some(wrapper::<F>), 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

struct Graphics {
    draws: ::stdweb::Value,
}

#[derive(Clone, Copy)]
pub enum Layer {
    Floor,
    Middle,
    Ceil,
}

impl Layer {
    fn size() -> usize {
        Layer::Ceil as usize + 1
    }
}

impl Graphics {
    fn initialize() -> Self {
        let layer_size: ::stdweb::Value = (Layer::size() as u32).into();
        js! {
            tgl.draw = function(camera, dynamic_draws) {
                this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
                for (var i = 0; i < @{layer_size}; i++) {
                    for (var d = 0; d < dynamic_draws[i].length; d++) {
                        var draw = dynamic_draws[i][d];

                        this.context.setTransform(1, 0, 0, 1, 0, 0);
                        this.context.translate(draw[1]-camera[0], draw[2]-camera[1]);
                        this.context.rotate(draw[3]);
                        this.context.scale(camera[2], camera[2]);
                        this.context.drawImage(tileset, 90, 130, 50, 50, -0.5, -0.5, 1, 1);
                    }
                }
            };
        }

        let mut layers = vec![];
        for _ in 0..Layer::size() {
            layers.push(::stdweb::Value::Array(vec![]));
        }

        Graphics {
            draws: layers.into(),
        }
    }

    fn insert_draw(&mut self, id: u32, x: f32, y: f32, rotation: f32, layer: Layer) {
        let draw: Vec<::stdweb::Value> = vec![id.into(), x.into(), y.into(), rotation.into()];
        if let ::stdweb::Value::Array(ref mut draws) = self.draws {
            if let ::stdweb::Value::Array(ref mut draws) = draws[layer as usize] {
                draws.push(draw.into());
            }
        }
    }

    fn draw(&self, camera: &Camera) {
        let camera = ::stdweb::Value::Array(vec![camera.x.into(), camera.y.into(), camera.zoom.into()]);
        js! {
            tgl.draw(@{camera}, @{&self.draws});
        }
    }
}

pub struct Camera {
    zoom: f32,
    x: f32,
    y: f32,
}

pub fn main() {
    stdweb::initialize();
    let mut graphics = Graphics::initialize();
    let mut events_loop = winit::EventsLoop::new();
    set_main_loop_callback(|| {
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Touch(mut touch), ..
                } => {
                    let height: f64 = js! {return tgl.canvas.height}.try_into().unwrap();
                    let width: f64 = js! {return tgl.canvas.width}.try_into().unwrap();

                    touch.location.0 -= width/2.0;
                    touch.location.0 /= width/2.0;
                    touch.location.1 -= height/2.0;
                    touch.location.1 /= height/2.0;
                    touch.location.1 *= -1.;
                },
                _ => (),
            }
        });
        graphics.insert_draw(0, 100.0, 100.0, 0.5, Layer::Ceil);
        graphics.draw(&Camera {
            zoom: 100.0,
            x: 0.0,
            y: 0.0,
        });
    });
}
