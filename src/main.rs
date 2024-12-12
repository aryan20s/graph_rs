mod graph;
mod node;
mod renderer;

use graph::{Graph, GraphTraversal};
use macroquad::{input::is_key_pressed, math::Vec2};

use std::time::{Duration, Instant};

const FPS_LIMIT: u64 = 240;
const FRAME_TIME_TARGET: Duration = Duration::from_nanos(1_000_000_000 / FPS_LIMIT);

#[macroquad::main("graph_rs")]
async fn main() {
    let mut graph: Graph = Graph::new(false);
    graph.add_node(10, Vec2::new(100.0, 100.0));
    graph.add_node(20, Vec2::new(200.0, 300.0));
    graph.add_node(30, Vec2::new(100.0, 500.0));
    graph.add_node(40, Vec2::new(400.0, 300.0));
    graph.add_node(50, Vec2::new(50.0, 300.0));

    graph.add_edge(10, 20);
    graph.add_edge(20, 30);
    graph.add_edge(20, 40);
    graph.add_edge(10, 50);

    let mut bfs = Some(graph.start_iterate(10));

    loop {
        let start_time = Instant::now();
        renderer::render(&mut graph, &bfs);

        if is_key_pressed(macroquad::input::KeyCode::A) && bfs.is_some() {
            bfs = graph.iterate_bfs(bfs.unwrap());
        }

        spin_sleep::sleep(FRAME_TIME_TARGET - start_time.elapsed());
        macroquad::prelude::next_frame().await
    }
}
