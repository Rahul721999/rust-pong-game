use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadExpect, System, SystemData, Write, WriteStorage}, ui::UiText,
};

use crate::pong::{Ball, ScoreBorad, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl <'s> System<'s> for WinnerSystem{
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBorad>,
        ReadExpect<'s, ScoreText>
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text
    ): Self::SystemData){
        for (ball, transform) in (&mut balls, &mut locals).join(){
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius{
                // Right player scored on the left side.
                println!("player 2 scores!");
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p2_score){
                    text.text = scores.score_right.to_string();
                }
                true
            }else if ball_x >= ARENA_WIDTH - ball.radius{
                // left player scored on the right side.
                println!("Player 1 scores!");
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score){
                    text.text = scores.score_left.to_string();
                }
                true
            }else{
                false
            };
            if did_hit{
                ball.velocity[0] = -ball.velocity[0]; // reverse direction
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right 
                )
            }
        }
    }
}