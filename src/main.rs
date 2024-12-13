mod graph;
mod node;
mod renderer;
mod update;

use graph::Graph;
use macroquad::math::Vec2;
use update::AppState;

use std::time::{Duration, Instant};

const FPS_LIMIT: u64 = 240;
const FRAME_TIME_TARGET: Duration = Duration::from_nanos(1_000_000_000 / FPS_LIMIT);

#[macroquad::main("graph_rs")]
async fn main() {
    let mut graph: Graph = Graph::new(false);
    graph.add_node(10, Vec2::new(100.0, 300.0));
    graph.add_node(20, Vec2::new(200.0, 200.0));
    graph.add_node(30, Vec2::new(300.0, 200.0));
    graph.add_node(40, Vec2::new(400.0, 200.0));
    graph.add_node(50, Vec2::new(200.0, 400.0));
    graph.add_node(60, Vec2::new(300.0, 400.0));
    graph.add_node(70, Vec2::new(400.0, 400.0));

    graph.add_edge(10, 20);
    graph.add_edge(20, 30);
    graph.add_edge(30, 40);
    graph.add_edge(10, 50);
    graph.add_edge(50, 60);
    graph.add_edge(60, 70);

    let mut cur_state = AppState::new(graph);

    loop {
        let start_time = Instant::now();

        renderer::render(&cur_state);
        cur_state = update::update(cur_state);

        spin_sleep::sleep(FRAME_TIME_TARGET - start_time.elapsed());
        macroquad::prelude::next_frame().await
    }
}
