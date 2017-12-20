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

pub struct Graphics;

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
            tgl.layer_size = @{layer_size};

            tgl.tiles_to_draw = [];
            tgl.texts_to_draw = [];
            for (var i = 0; i < tgl.layer_size; i++) {
                tgl.tiles_to_draw.push([]);
                tgl.texts_to_draw.push([]);
            }

            tgl.textAlign = ["start", "end", "left", "right", "center"];
            tgl.textBaseline = ["top", "hanging", "middle", "alphabetic", "ideographic", "bottom"];
            tgl.draw = function(camera) {
                // We use 2px and then divide by 2 the camera zoom
                // because there is an issue with 1px
                this.context.font = "2px gameFont";

                var zoom  = camera[2]*Math.min(this.canvas.width, this.canvas.height);
                var zoom2 = zoom/2.0;
                var dx = - camera[0] + this.canvas.width/2.0;
                var dy = - camera[1] + this.canvas.height/2.0;
                this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
                for (var i = 0; i < this.layer_size; i++) {
                    for (var d = 0; d < this.tiles_to_draw[i].length; d++) {
                        var draw = this.tiles_to_draw[i][d];

                        this.context.setTransform(1, 0, 0, 1, 0, 0);
                        this.context.translate(draw[1] + dx, draw[2] + dy);
                        this.context.rotate(draw[3]);
                        this.context.scale(zoom, zoom);
                        var tile = tiles[draw[0]];
                        this.context.drawImage(tilesets[tile[0]], tile[1], tile[2], tile[3], tile[4], -tile[3]/2.0, -tile[4]/2.0, tile[3], tile[4]);
                    }
                    for (var d = 0; d < this.texts_to_draw[i].length; d++) {
                        var draw = this.texts_to_draw[i][d];

                        this.context.setTransform(1, 0, 0, 1, 0, 0);
                        this.context.translate(draw[1] + dx, draw[2] + dy);
                        this.context.rotate(draw[3]);
                        this.context.scale(zoom2*draw[4], zoom2*draw[4]);
                        this.context.textAlign = this.textAlign[draw[5]];
                        this.context.textBaseline = this.textBaseline[draw[6]];
                        var text = draw[0];
                        this.context.fillText(text, 0, 0);
                    }
                    tgl.tiles_to_draw[i] = [];
                    tgl.texts_to_draw[i] = [];
                }
            };
        }

        Graphics
    }

    pub fn insert_tile(&mut self, id: u32, x: f32, y: f32, rotation: f32, layer: Layer) {
        let array: [::stdweb::Value; 4] = [
            id.into(),
            x.into(),
            y.into(),
            rotation.into(),
        ];

        js! {
            var array = @{&array as &[_]};
            tgl.tiles_to_draw[@{layer as u32}].push(array);
        }
    }

    pub fn insert_text(&mut self, text: String, x: f32, y: f32, rotation: f32, scale: f32, align: Align, baseline: Baseline, layer: Layer) {
        let array: [::stdweb::Value; 7] = [
            text.into(),
            x.into(),
            y.into(),
            rotation.into(),
            scale.into(),
            (align as u32).into(),
            (baseline as u32).into(),
        ];

        js! {
            var array = @{&array as &[_]};
            tgl.texts_to_draw[@{layer as u32}].push(array);
        }
    }

    pub fn draw(&mut self, camera: &Camera) {
        let camera: [::stdweb::Value; 3] = [camera.x.into(), camera.y.into(), camera.zoom.into()];
        js! {
            tgl.draw(@{&camera as &[_]});
        }
    }
}
