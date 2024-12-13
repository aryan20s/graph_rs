use macroquad::prelude::*;

use crate::graph::{self, Graph, GraphTraversal};
use crate::node::{NODE_FONT_SIZE, NODE_RADIUS, NODE_RADIUS_SQR};
use crate::update::AppState;

pub fn render(cur_state: &AppState) {
    clear_background(BLACK);

    let graph = &cur_state.graph;
    let cur_traversal = &cur_state.cur_traversal;

    let mouse_pos = mouse_position();
    let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
    let mut node_under_mouse: Option<u64> = None;
    for node in graph.get_nodes() {
        if mouse_pos.distance_squared(node.pos) <= NODE_RADIUS_SQR {
            node_under_mouse = Some(node.data);
        }
    }

    for (src, dests_vec) in graph.get_edges().iter() {
        let src_node = graph.get_node(*src).unwrap();
        for dest in dests_vec {
            let dest_node = graph.get_node(*dest).unwrap();
            draw_line(
                src_node.pos.x,
                src_node.pos.y,
                dest_node.pos.x,
                dest_node.pos.y,
                4.0,
                BLUE,
            );
        }
    }

    if let Some(edge_src) = cur_state.edge_from {
        let src_node = graph.get_node(edge_src).unwrap();
        draw_line(
            src_node.pos.x,
            src_node.pos.y,
            mouse_pos.x,
            mouse_pos.y,
            4.0,
            LIME,
        );
    }

    for node in graph.get_nodes() {
        let mut node_color = WHITE;

        if let Some(cur_traversal) = cur_traversal {
            if cur_traversal.to_visit.contains(&node.data) {
                node_color = SKYBLUE;
            }

            if cur_traversal.visited.contains(&node.data) {
                node_color = LIME;
            }

            if let Some(just_visited) = cur_traversal.just_visited {
                if just_visited == node.data {
                    node_color = ORANGE;
                }
            }
        }

        if node_under_mouse.is_some() && node_under_mouse.unwrap() == node.data {
            node_color.r *= 0.8;
            node_color.g *= 0.8;
            node_color.b *= 0.8;
        }

        draw_circle(node.pos.x, node.pos.y, NODE_RADIUS, node_color);
        let node_data_text = &node.data.to_string();
        let center = get_text_center(node_data_text, None, NODE_FONT_SIZE, 1.0, 0.0);
        draw_text(
            node_data_text,
            node.pos.x - center.x,
            node.pos.y - center.y,
            NODE_FONT_SIZE as f32,
            BLACK,
        );
    }
}
