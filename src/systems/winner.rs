use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl <'s> System<'s> for WinnerSystem{
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData){
        for (ball, transform) in (&mut balls, &mut locals).join(){
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius{
                // Right player scored on the left side.
                println!("player 2 scores!");
                true
            }else if ball_x >= ARENA_WIDTH - ball.radius{
                // left player scored on the right side.
                println!("Player 1 scores!");
                true
            }else{
                false
            };
            if did_hit{
                ball.velocity[0] = -ball.velocity[0]; // reverse direction
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);
            }
        }
    }
}