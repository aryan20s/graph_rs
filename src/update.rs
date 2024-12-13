use macroquad::math::Vec2;
use macroquad::prelude::*;
use miniquad::window::{get_window_position, set_window_size};

use crate::graph::{Graph, GraphTraversal};
use crate::node::{NODE_RADIUS, NODE_RADIUS_SQR};

pub struct AppState {
    pub graph: Graph,
    pub cur_traversal: Option<GraphTraversal>,
    pub node_held: Option<u64>,
    pub node_held_original_pos: Option<Vec2>,
    pub last_mouse_pos: Option<Vec2>,
    pub last_mouse_vels: Vec<Vec2>,
    pub edge_from: Option<u64>,
}

impl AppState {
    pub fn new(graph: Graph) -> AppState {
        AppState {
            graph,
            cur_traversal: None,
            node_held: None,
            node_held_original_pos: None,
            last_mouse_pos: None,
            last_mouse_vels: vec![],
            edge_from: None,
        }
    }
}

pub fn update(cur_state: AppState) -> AppState {
    let delta_time = get_frame_time();
    let mut cur_state = input(cur_state, delta_time);

    for node in cur_state.graph.get_nodes_mut() {
        if let Some(cur_held) = cur_state.node_held {
            if cur_held == node.data {
                continue;
            }
        }

        node.speed *= 1.0 - (10.0 * delta_time);
        if node.speed.length() < 0.5 {
            node.speed = Vec2::new(0.0, 0.0);
        }

        node.pos += node.speed * delta_time;

        if node.pos.x < 0.0 {
            node.speed.x = -node.speed.x;
            node.pos.x = 0.0;
        }
        if node.pos.y < 0.0 {
            node.speed.y = -node.speed.y;
            node.pos.y = 0.0;
        }
        let sx = screen_width();
        let sy = screen_height();
        if node.pos.x > sx {
            node.speed.x = -node.speed.x;
            node.pos.x = sx;
        }
        if node.pos.y > sy {
            node.speed.y = -node.speed.y;
            node.pos.y = sy;
        }
    }

    if is_key_pressed(macroquad::input::KeyCode::D) {
        if cur_state.cur_traversal.is_some() {
            cur_state.cur_traversal = cur_state
                .graph
                .iterate_dfs(cur_state.cur_traversal.unwrap());
        } else {
            cur_state.cur_traversal = Some(cur_state.graph.start_iterate(10));
        }
    }

    if is_key_pressed(macroquad::input::KeyCode::B) {
        if cur_state.cur_traversal.is_some() {
            cur_state.cur_traversal = cur_state
                .graph
                .iterate_bfs(cur_state.cur_traversal.unwrap());
        } else {
            cur_state.cur_traversal = Some(cur_state.graph.start_iterate(10));
        }
    }

    if is_key_pressed(macroquad::input::KeyCode::Escape) {
        cur_state.edge_from = None;
    }

    cur_state
}

pub fn input(cur_state: AppState, delta_time: f32) -> AppState {
    let mut cur_state = cur_state;

    let mouse_pos = mouse_position();
    let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
    if let Some(last_mouse_pos) = cur_state.last_mouse_pos {
        let delta_mouse_pos = mouse_pos - last_mouse_pos;
        let mouse_vel = delta_mouse_pos / (delta_time * 3.0);

        cur_state.last_mouse_vels.push(mouse_vel);
        if cur_state.last_mouse_vels.len() > 3 {
            cur_state.last_mouse_vels.remove(0);
        }
    }

    if let Some(node_to_move) = cur_state.node_held {
        let node = cur_state.graph.get_node_mut(node_to_move).unwrap();
        node.pos = mouse_pos.clone();

        if !is_mouse_button_down(MouseButton::Left) {
            let mut max_vel = Vec2::new(0.0, 0.0);
            for vel in &cur_state.last_mouse_vels {
                if vel.length() > max_vel.length() {
                    max_vel = vel.clone();
                }
            }
            node.speed = max_vel;

            if node
                .pos
                .distance_squared(cur_state.node_held_original_pos.unwrap())
                < NODE_RADIUS_SQR
            {
                cur_state.edge_from = cur_state.node_held;
            }

            cur_state.node_held = None;
        }
    } else {
        let mut edge_to_add = None;

        for node in cur_state.graph.get_nodes() {
            if mouse_pos.distance_squared(node.pos) <= NODE_RADIUS_SQR {
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(edge_src) = cur_state.edge_from {
                        edge_to_add = Some((edge_src, node.data));
                        cur_state.edge_from = None;
                    } else {
                        cur_state.node_held = Some(node.data);
                        cur_state.node_held_original_pos = Some(node.pos.clone());
                    }
                }
            }
        }

        if let Some(edge_to_add) = edge_to_add {
            cur_state.graph.add_edge(edge_to_add.0, edge_to_add.1);
        }
    }

    cur_state.last_mouse_pos = Some(mouse_pos);

    cur_state
}
