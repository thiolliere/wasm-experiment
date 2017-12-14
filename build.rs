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

    let dest_path = Path::new(&out_dir).join("animations_enum.rs");
    let mut out_anim = File::create(&dest_path).unwrap();

    let dest_path = Path::new(&out_dir).join("make_tileset.sh");
    let mut out_cmd = File::create(&dest_path).unwrap();

    for (format_index, (format, mut files)) in formats.iter_mut().enumerate() {
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
            let mut indices = animations.entry(animation).or_insert(vec!());
            indices.push(i);
        }

        for (name, tiles) in animations {
            out_anim.write_fmt(format_args!("pub const {}: [usize; {}] = [", name.to_uppercase(), tiles.len())).unwrap();
            for tile in tiles {
                out_anim.write_all(b"Tile { ").unwrap();
                // TODO: x, y, width, height
                out_anim.write_fmt(format_args!("tileset: {}, index: {} ", format_index, tile)).unwrap();
                out_anim.write_all(b"}, ").unwrap();
            }
            out_anim.write_all(b"];\n").unwrap();
        }
    }
}
