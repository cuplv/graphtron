// Default parameters
// ------------------

// icedit -x <Num> / --width <Num>
// Width in pixels
const DEFAULT_WIDTH: u32 = 800;

// icedit -y <Num> / --height <Num>
// Height in pixels
const DEFAULT_HEIGHT: u32 = 800;

// icedit -r / --reference
// uses spec implementation without optimisations

// icedit test
//see main() for full testing options
//const TEST_WIDTH: u32 = 800;
//const TEST_HEIGHT: u32 = 800;
//const DEFAULT_RND_START: u32 = 100;
//const DEFAULT_RND_CMDS: u32 = 10;
//const DEFAULT_OUTFILE: Option<&'static str> = Some("testout.csv");
const OPEN_GL: OpenGL = OpenGL::V3_2;

extern crate time;
extern crate rand;
#[macro_use]
extern crate clap;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
extern crate adapton;

//use std::env::current_exe;
//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use time::Duration;
use glutin_window::GlutinWindow;
//use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
//use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{Events, EventLoop};
//use piston::input::{Button, Event, Input, Key};
use piston::input::{Event};
use piston::window::WindowSettings;

// fn render(c: graphics::context::Context, g: &mut GlGraphics, f: &mut GlyphCache, t: &List<String>, time: Duration, s: &Inputstatus) {
//   graphics::clear([0.0, 0.0, 0.0, 1.0], g);

//   //main text
//   let size = 22.0;
//   let mut text = graphics::Text::new(22);
//   text.color = [1.0, 1.0, 1.0, 1.0];
//   let mut loc = size;

//   for st in t.iter() {
//     text.draw(
//       st, f, &c.draw_state,
//       c.trans(10.0, loc).transform,
//       g); 
//     loc += size;
//     if loc > 800.0 {break}
//   }

//   //info section
//   let hud_bcolor = [0.02, 0.02, 0.02, 0.8];
//   let hud_back = [590.0, 5.0, 200.0, 90.0];
//   graphics::rectangle(hud_bcolor, hud_back, c.transform, g); // shaded background
//   let size = 16.0;
//   let mut text = graphics::Text::new(16);
//   text.color = [0.5, 1.0, 0.5, 1.0];
//   let (px,py) = (600.0, size*1.5);
//   let clock = "Time(ms): ".to_string() + &time.num_milliseconds().to_string();
//   text.draw(
//     &clock, f, &c.draw_state,
//     c.trans(px, py).transform,
//     g);
//   let (stat, cur) = match *s {
//     Inputstatus::Insert(ref d,ref show) => {
//       let stat = "I ";
//       let dir = match d { &Dir::L => "<- (c-arrows)", &Dir::R => "-> (c-arrows)" };
//       let cur = if *show { "Visible (c-s)"} else { "Invisible (c-s)"};
//       (stat.to_string() + &dir, cur.to_string())
//     }
//     Inputstatus::Overwrite(ref d,ref show) => {
//       let stat = "O ";
//       let dir = match d { &Dir::L => "<- (c-arrows)", &Dir::R => "-> (c-arrows)" };
//       let cur = if *show { "Visible (c-s)"} else { "Invisible (c-s)"};
//       (stat.to_string() + &dir, cur.to_string())
//     }
//     _ => ("".to_string(), "".to_string()) // should not happen
//   };
//   text.draw(
//     &stat, f, &c.draw_state,
//     c.trans(px, py+(size*1.5)).transform,
//     g); 
//   text.draw(
//     &cur, f, &c.draw_state,
//     c.trans(px, py+(size*3.0)).transform,
//     g); 

// }

// fn render_cursor(c: graphics::context::Context, g: &mut GlGraphics, f: &mut GlyphCache, cc:CCs, t: &String) {
//   graphics::clear([0.0, 0.0, 0.0, 1.0], g);

//   //choose title
//   let size = 48.0;
//   let mut text = graphics::Text::new(48);
//   text.color = [1.0, 1.0, 1.0, 1.0];
//   let (px,py) = (200.0,250.0);
//   let prompt = match cc {
//     CCs::Mk => {"Create cursor: "}
//     CCs::Switch => {"Switch to cursor: "}
//     CCs::Jmp => {"Jump to cursor: "}
//     CCs::Join => {"Join with cursor: "}
//   }.to_string();

//   //render screen
//   text.draw(
//     &prompt, f, &c.draw_state,
//     c.trans(px, py).transform,
//     g); 
//   text.draw(
//     t, f, &c.draw_state,
//     c.trans(px + size, py + size*1.5).transform,
//     g); 
// }

// Returns a result containing a GlutinWindow or an error if the window
// settings are not supported
fn try_create_window(x: u32, y: u32) -> Result<GlutinWindow, String> {
  WindowSettings::new("ICEdit", [x, y])
    .exit_on_esc(true)
    .opengl(OPEN_GL)
    .build()
}

fn main() {

  // //command-line
  // let args = clap::App::new("IC_Edit")
  //   .version("0.2")
  //   .author("Kyle Headley <kyle.headley@colorado.edu>")
  //   .about("Incremental Text Editor")
  //   .args_from_usage("\
  //     -x --width=[width]              'initial editor width in pixels'
  //     -y --height=[height]            'initial editor height in pixels'
  //    ")
  //   .get_matches();
  // let test;
  // let windowless;
  // let test_args =
  //   if let Some(matches) = args.subcommand_matches("test") {
  //     test = true; windowless = false; matches
  //   } else if let Some(matches) = args.subcommand_matches("windowless") {
  //     test = true; windowless = true; matches
  //   } else {
  //     test = false; windowless = false; &args
  //   };
  // let x = value_t!(test_args.value_of("width"), u32).unwrap_or(if test {TEST_WIDTH} else {DEFAULT_WIDTH});
  // let y = value_t!(test_args.value_of("height"), u32).unwrap_or(if test {TEST_HEIGHT} else {DEFAULT_HEIGHT});
  // let rnd_start = value_t!(test_args.value_of("rnd_start"), u32).unwrap_or(if test {DEFAULT_RND_START} else {0});
  // let rnd_adds = value_t!(test_args.value_of("rnd_cmds"), u32).unwrap_or(if test {DEFAULT_RND_CMDS} else {0});
  // let keep_open = if test {test_args.is_present("keep_open")} else {true};
  // let show_curs = !test_args.is_present("hide_curs");
  // let no_cursors = test_args.is_present("no_cursors");
  // let use_adapton = !test_args.is_present("spec_only");
  // let use_spec = !test_args.is_present("fast_only");
  // let outfile = match test_args.value_of("outfile") {
  //   None => if test {DEFAULT_OUTFILE} else {None},
  //   Some(f) => Some(f)
  // };
  // let mut outfile = outfile.map(|f| {
  //   OpenOptions::new()
  //   .create(true)
  //   .write(true)
  //   .append(true)
  //   .open(f)
  //   .unwrap()
  // });
  // //TODO: the clap library supports this in param parsing
  // //assert_eq!(use_adapton || use_spec, true);

  let x = DEFAULT_WIDTH;
  let y = DEFAULT_HEIGHT;
  
  // graphics
  let window = try_create_window(x, y).unwrap();
  let mut gl = GlGraphics::new(OPEN_GL);
  //let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
  //let mut font = GlyphCache::new(&exe_directory.join("../../FiraMono-Bold.ttf")).unwrap();
    
  // input
//  let mut command_key_down = false;
  //let mut status = Inputstatus::Insert(Dir::R, show_curs);

  for e in window.events().max_fps(60).ups(50) {
    match e {
      Event::Update(_) => { },
      Event::Idle(_) => { },
      Event::AfterRender(_) => { },
      Event::Render(args) => {
        gl.draw(args.viewport(), |c, g|{
          let node_color  = [1.0,0.0,1.0,1.0];  
          let node_geom   = [100.0,100.0,10.0,10.0];
          graphics::clear([0.05, 0.05, 0.05, 1.0], g);
          graphics::rectangle(node_color, node_geom, c.transform, g);

          let node_geom   = [110.0,110.0,10.0,10.0];
          graphics::rectangle(node_color, node_geom, c.transform, g);
          
          let node_geom   = [90.0,110.0,10.0,10.0];
          graphics::rectangle(node_color, node_geom, c.transform, g);
        });  
      },
      _ => println!("{:?}", e),      
    }
  }
}
