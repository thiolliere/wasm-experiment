use std::collections::HashMap;
use std::process::Command;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut formats = HashMap::new();
    for file in fs::read_dir("assets/animations").unwrap().map(|f| f.unwrap().path()) {
        let command = Command::new("identify")
            .args(&["-format", "%[fx:w]x%[fx:h]", file.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(command.status.success());
        let stdout = String::from_utf8(command.stdout).unwrap();
        let format: Vec<_> = stdout.split("x").collect();
        let format = [
            usize::from_str_radix(format[0], 10).unwrap(),
            usize::from_str_radix(format[1], 10).unwrap(),
        ];
        let mut files = formats.entry(format).or_insert(vec!());
        files.push(file);
    }
    let mut formats: Vec<_> = formats.drain().collect();
    formats.sort();

    // Write animation_enum and make_tileset
    let dest_path = Path::new(&out_dir).join("animations.rs");
    let mut out_anim = File::create(&dest_path).unwrap();

    let dest_path = Path::new(&out_dir).join("make_tileset.sh");
    let mut out_cmd = File::create(&dest_path).unwrap();

    let mut tiles = vec!();
    for (format_index, &mut (format, ref mut files)) in formats.iter_mut().enumerate() {
        files.sort();

        let width_len = (files.len() as f64).sqrt() as usize;
        let height_len = width_len + 1;
        for x in 0..width_len {
            out_cmd.write_all(b"convert ").unwrap();
            for y in 0..height_len {
                if let Some(file) = files.get(y*width_len + x) {
                    out_cmd.write_fmt(format_args!("{} ", file.to_str().unwrap())).unwrap();
                }
            }
            out_cmd.write_fmt(format_args!("+append TEMPORAR_FILE_TO_GENERATE_TILESET_{}.png\n", x)).unwrap();
        }

        out_cmd.write_all(b"convert ").unwrap();
        for x in 0..width_len {
            out_cmd.write_fmt(format_args!("TEMPORAR_FILE_TO_GENERATE_TILESET_{}.png ", x)).unwrap();
        }
        out_cmd.write_fmt(format_args!("-append $TARGET_DIR/tileset{}.png\n", format_index)).unwrap();
        for x in 0..width_len {
            out_cmd.write_fmt(format_args!("rm TEMPORAR_FILE_TO_GENERATE_TILESET_{}.png\n", x)).unwrap();
        }

        let mut animations = HashMap::new();
        for (i, file) in files.iter().enumerate() {
            let mut animation = String::from(file.file_name().unwrap().to_str().unwrap());
            let animation_len = animation.len();
            animation.truncate(animation_len - 8);
            let mut tile_ids = animations.entry(animation).or_insert(vec!());
            tile_ids.push(tiles.len());
            let x = i%width_len;
            let y = i/width_len;
            tiles.push([format_index, x*format[0], y*format[1], format[0], format[1]]);
        }

        for (name, tile_ids) in animations {
            out_anim.write_fmt(format_args!("pub const {}: [u32; {}] = [", name.to_uppercase(), tile_ids.len())).unwrap();
            for id in tile_ids {
                out_anim.write_fmt(format_args!("{}, ", id)).unwrap();
            }
            out_anim.write_all(b"];\n").unwrap();
        }
    }

    // Write index.html
    let names = formats.iter()
        .enumerate()
        .map(|(i, _)| format!("tileset{}.png", i))
        .collect::<Vec<_>>();
    let tileset_pattern = format!("s/tileset_names = \\[\\]; \\/\\/ Filled by build.rs/tileset_names = {:?};/", names);
    let tiles_pattern = format!("s/tiles = \\[\\]; \\/\\/ Filled by build.rs/tiles = {:?};/", tiles);
    let sed_pattern = format!("{};{}", tileset_pattern, tiles_pattern);

    let template = String::from("src/index.html");
    let dest_path = Path::new(&out_dir).join("index.html");
    let out_html = File::create(&dest_path).unwrap();

    assert!(Command::new("sed")
        .args(&[sed_pattern, template])
        .stdout(out_html)
        .status()
        .unwrap()
        .success());
}
