// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module implements the svg level loader.
//!
//! Note that [usvg](https://docs.rs/usvg) is not usable because of https://github.com/RazrFalcon/resvg/issues/588.
//! Thus this module uses a regular xml library to manually load the data and normalize the translation.

use bevy::math::Vec2;
use roxmltree::{Document, Node};

use crate::level::{Level, Rectangle};

fn load_rectangle(node: Node, offset: Vec2) -> Option<Rectangle> {
    let parse_attr = |name| get_attr(&node, name)?.parse().ok();
    let size = Vec2::new(parse_attr("width")?, parse_attr("height")?);
    let top_left = Vec2::new(parse_attr("x")?, parse_attr("y")?) - offset;
    Some(Rectangle::new(top_left, size))
}

fn load_pos(node: Node, offset: Vec2) -> Option<Vec2> {
    let rect = load_rectangle(node, offset)?;
    Some(rect.top_left + rect.size / 2.0)
}

fn load_level(node: &Node) -> Option<Level> {
    let screen = node
        .children()
        .find(|node| get_attr(node, "label") == Some("Screen"))
        .and_then(|node| load_rectangle(node, Vec2::new(0., 0.)))?;
    let offset = screen.top_left;
    println!("Got screen: {:?}", screen);
    let mut walls = vec![];
    let mut goals = vec![];
    let mut name = None;
    let mut pad = None;

    for node in node.children().filter(|node| node.is_element()) {
        let label = get_attr(&node, "label")?;
        if label.starts_with("wall-") {
            walls.push(load_rectangle(node, offset)?);
        } else if label == "launch-pad" {
            pad = load_rectangle(node, offset);
        } else if let Some(("goal", pos)) = split_pos(label) {
            goals.push((pos, load_pos(node, offset)?));
        } else if label == "name" {
            name = node
                .first_child()
                .and_then(|n| n.text())
                .map(|text| text.to_string());
        } else if label != "Screen" {
            println!("Unknown {:?}", node);
        }
    }
    println!("Finished loading level");

    Some(Level {
        name: name?,
        pad: pad?,
        goals: sort_vec(goals),
        walls,
    })
}

fn sort_vec<A>(mut vec: Vec<(usize, A)>) -> Vec<A> {
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec.into_iter().map(|(_pos, lvl)| lvl).collect()
}

fn get_attr<'a>(node: &'a Node<'a, 'a>, name: &'a str) -> Option<&'a str> {
    node.attributes()
        .find(|attr| attr.name() == name)
        .map(|attr| attr.value())
}

fn split_pos(value: &str) -> Option<(&str, usize)> {
    value
        .split_once('-')
        .and_then(|(key, pos)| pos.parse().ok().map(|pos| (key, pos)))
}

fn load_top_level(node: &Node) -> Option<(usize, Level)> {
    let label = get_attr(node, "label")?;
    if let Some(("Level", pos)) = split_pos(label) {
        Some((pos, load_level(node)?))
    } else {
        None
    }
}

pub fn load() -> Vec<Level> {
    let data = include_str!("levels.svg");
    let doc = Document::parse(data).unwrap();
    if let Some(svg) = doc
        .root()
        .children()
        .filter(|node| node.is_element())
        .find(|node| node.tag_name().name() == "svg")
    {
        let levels = svg
            .children()
            .filter(|node| node.tag_name().name() == "g")
            .filter_map(|node| load_top_level(&node))
            .collect();
        sort_vec(levels)
    } else {
        panic!("Couln't find svg root node")
    }
}
