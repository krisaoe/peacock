use peacock::graphics::{self, Animation, IntRect, Texture, View};
use peacock::window;
use peacock::{Context, ContextBuilder, Result, State};

struct GameState {
    animation: Animation,
}

impl GameState {
    fn new() -> Self {
        let sprite_sheet = Texture::from_file("examples/res/0x72_dungeon_ii.png")
            .expect("Could not load sprite sheet!");

        let animation = Animation::new(
            sprite_sheet,
            vec![
                IntRect::new(128, 76, 15, 20),
                IntRect::new(144, 76, 15, 20),
                IntRect::new(160, 76, 15, 20),
                IntRect::new(176, 76, 15, 20),
            ],
            8,
        );

        Self { animation }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        self.animation.tick();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        let mut view = View::new((0.0, 0.0).into(), (1920.0, 1080.0).into());
        view.zoom(0.25);

        window::set_view(ctx, &view);

        graphics::draw(ctx, &self.animation);

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Animation", 1920, 1080)
        .build()?
        .run(&mut GameState::new())
}
