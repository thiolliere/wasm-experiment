#![recursion_limit="500"]
#[macro_use] extern crate stdweb;
extern crate winit;
#[macro_use] extern crate lazy_static;

use stdweb::unstable::TryInto;
use std::os::raw::c_void;
use std::ptr;
use std::cell::RefCell;

mod ffi;
mod animation;
mod audio;
mod tgl;

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

pub fn main() {
    stdweb::initialize();
    let mut graphics = tgl::Graphics::initialize();
    let mut events_loop = winit::EventsLoop::new();
    let mut hero = animation::Animated::new(animation::Entity::Character, animation::State::Walking);
    let dt = 1.0/60.0;
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
        hero.update(dt);
        graphics.insert_tile(hero.tile(), 100.0, 100.0, 0.5, tgl::Layer::Ceil);
        graphics.draw(&tgl::Camera {
            zoom: 100.0,
            x: 0.0,
            y: 0.0,
        });
    });
}
