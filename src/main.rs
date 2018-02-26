mod util;

extern crate gio;
extern crate gtk;
extern crate cairo;
#[macro_use] extern crate scan_rules;

use gio::prelude::*;
use gtk::prelude::*;
use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

use util::world::World;
use util::point3::Point3;
use util::polygon3::Polygon3;
use util::color::Color;

use cairo::enums::{FontSlant, FontWeight};

fn main() {
    let filename = "miku.obj";
//    let filename = "box.obj";
    let polygons = obj_to_polygons(filename);
    let width = 640.0f64;
    let height = 480.0f64;
    let scale = 50000.0f64;
    let world = Rc::new(RefCell::new(World::new(&polygons, width, height, scale)));

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("rust3d");
    window.set_default_size(640, 480);

    let drawing_area = gtk::DrawingArea::new();
    window.add(&drawing_area);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let rotate = Rc::new(Cell::new(220i32));
    drawing_area.connect_draw(move |_: &gtk::DrawingArea, cr: &cairo::Context| {
        let r = Rc::clone(&rotate);
        let w = Rc::clone(&world);
        {
            let _r = r.get();
            let _w = w.borrow();
            _w.draw(_r, cr);
        }
        {
            let _r = r.get();
            r.set((_r + 3) % 360);
        }
        Inhibit(false)
    });

    timeout_add(100, move || {
        drawing_area.queue_draw_area(0, 0, width as i32, height as i32);
        gtk::Continue(true)
    });

    window.show_all();
    gtk::main();
}

fn obj_to_polygons(filename: &str) -> Vec<(Polygon3, Color)> {
    // 初期データ(objファイル)読み込み
    let file = BufReader::new(File::open(filename).unwrap());
    let mut polygons: Vec<Polygon3> = Vec::new();
    let mut vs: Vec<Point3> = Vec::new();

    for line_iter in file.lines() {
        match line_iter  {
            Ok(line) => {
                // 頂点データ
                scan!(&line; ("v ", let x: f64, let y: f64, let z: f64) => {
                    let p = Point3::new(x, y, z, 1.0f64);
//                    println!("{:?}", p);
                    vs.push(p);
                }).or_else(|n| {
                    // ポリゴン(4点)
                    scan!(&line; ("f ", let a: usize, "/", let _: usize, let b: usize, "/", let _: usize, let c: usize, "/", let _: usize, let d: usize, "/", let _: usize) => {
                        let p1 = Polygon3::new(&vs[a - 1], &vs[b - 1], &vs[c - 1]);
                        let p2 = Polygon3::new(&vs[a - 1], &vs[c - 1], &vs[d - 1]);
//                        println!("{:?}", p1);
//                        println!("{:?}", p2);
                        polygons.push(p1);
                        polygons.push(p2);
                    })
                }).or_else(|n| {
                    // ポリゴン(3点)
                    scan!(&line; ("f ", let a: usize, "/", let _: usize, let b: usize, "/", let _: usize, let c: usize, "/", let _: usize) => {
                        let p = Polygon3::new(&vs[a - 1], &vs[b - 1], &vs[c - 1]);
//                        println!("{:?}", p);
                        polygons.push(p);
                    })
                });
                // TODO: 中央に寄せる
            }
            Err(e) => println!("{}", e)
        }
    }
    polygons.into_iter()
        .map(|p| (p, Color::new(255, 230, 230)))
        .collect()
}
