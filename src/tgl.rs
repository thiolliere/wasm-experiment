pub struct Camera {
    pub zoom: f32,
    pub x: f32,
    pub y: f32,
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

pub struct Graphics {
    tiles_to_draw: ::stdweb::Value,
    texts_to_draw: ::stdweb::Value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Align {
    Start,
    End,
    Left,
    Right,
    Center,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Baseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

impl Graphics {
    pub fn initialize() -> Self {
        let layer_size: ::stdweb::Value = (Layer::size() as u32).into();
        js! {
            tgl.textAlign = ["start", "end", "left", "right", "center"];
            tgl.textBaseline = ["top", "hanging", "middle", "alphabetic", "ideographic", "bottom"];
            tgl.layer_size = @{layer_size};
            // We use 2px and then divide by 2 the camera zoom
            // because there is an issue with 1px
            tgl.context.font = "2px gameFont";
            tgl.draw = function(camera, tiles_to_draw, texts_to_draw) {
                this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
                for (var i = 0; i < this.layer_size; i++) {
                    for (var d = 0; d < tiles_to_draw[i].length; d++) {
                        var draw = tiles_to_draw[i][d];

                        this.context.setTransform(1, 0, 0, 1, 0, 0);
                        this.context.translate(draw[1]-camera[0], draw[2]-camera[1]);
                        this.context.rotate(draw[3]);
                        this.context.scale(camera[2], camera[2]);
                        var tile = tiles[draw[0]];
                        this.context.drawImage(tilesets[tile[0]], tile[1], tile[2], tile[3], tile[4], -0.5, -0.5, 1, 1);
                    }
                    for (var d = 0; d < texts_to_draw[i].length; d++) {
                        var draw = texts_to_draw[i][d];

                        this.context.setTransform(1, 0, 0, 1, 0, 0);
                        this.context.translate(draw[1]-camera[0], draw[2]-camera[1]);
                        this.context.rotate(draw[3]);
                        this.context.scale(camera[2]/2, camera[2]/2);
                        this.context.textAlign = this.textAlign[draw[4]];
                        this.context.textBaseline = this.textBaseline[draw[5]];
                        var text = draw[0];
                        this.context.fillText(text, 0, 0);
                    }
                }
            };
        }

        let mut layers = vec![];
        for _ in 0..Layer::size() {
            layers.push(::stdweb::Value::Array(vec![]));
        }

        Graphics {
            tiles_to_draw: layers.clone().into(),
            texts_to_draw: layers.into(),
        }
    }

    pub fn insert_tile(&mut self, id: u32, x: f32, y: f32, rotation: f32, layer: Layer) {
        let draw: Vec<::stdweb::Value> = vec![id.into(), x.into(), y.into(), rotation.into()];
        if let ::stdweb::Value::Array(ref mut draws) = self.tiles_to_draw {
            if let ::stdweb::Value::Array(ref mut draws) = draws[layer as usize] {
                draws.push(draw.into());
            }
        }
    }

    pub fn insert_text(&mut self, text: String, x: f32, y: f32, rotation: f32, align: Align, baseline: Baseline, layer: Layer) {
        let draw: Vec<::stdweb::Value> = vec![text.into(), x.into(), y.into(), rotation.into(), (align as u32).into(), (baseline as u32).into()];
        if let ::stdweb::Value::Array(ref mut draws) = self.texts_to_draw {
            if let ::stdweb::Value::Array(ref mut draws) = draws[layer as usize] {
                draws.push(draw.into());
            }
        }
    }

    pub fn draw(&mut self, camera: &Camera) {
        let camera = ::stdweb::Value::Array(vec![camera.x.into(), camera.y.into(), camera.zoom.into()]);
        js! {
            tgl.draw(@{camera}, @{&self.tiles_to_draw}, @{&self.texts_to_draw});
        }

        // Clear to draw
        if let ::stdweb::Value::Array(ref mut draws) = self.tiles_to_draw {
            for i in 0..Layer::size() {
                if let ::stdweb::Value::Array(ref mut draws) = draws[i] {
                    draws.clear();
                }
            }
        }
        if let ::stdweb::Value::Array(ref mut draws) = self.texts_to_draw {
            for i in 0..Layer::size() {
                if let ::stdweb::Value::Array(ref mut draws) = draws[i] {
                    draws.clear();
                }
            }
        }
    }
}
