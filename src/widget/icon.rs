// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Lazily-generated SVG icon widget for Iced.

use iced::{
    widget::{svg, Image},
    Length, ContentFit,
};
use std::borrow::Cow;
use std::hash::Hash;
use std::rc::Rc;
use derive_setters::Setters;
use crate::{Element, Renderer};

/// A lazily-generated SVG icon.
#[derive(Hash, Setters)]
pub struct Icon<'a> {
    #[setters(skip)]
    name: Cow<'a, str>,
    #[setters(into)]
    theme: Cow<'a, str>,
    style: crate::theme::Svg,
    size: u16,
    #[setters(strip_option)]
    content_fit: Option<ContentFit>,
    #[setters(strip_option)]
    width: Option<Length>,
    #[setters(strip_option)]
    height: Option<Length>,
}

pub fn image_icon(name: &str, size: u16) -> Option<Image> {
    freedesktop_icons::lookup(name)
        .with_size(size)
        .with_cache()
        .find()
        .map(|path| {
            Image::new(path)
                .width(Length::Units(size))
                .height(Length::Units(size))
        })
}
/// A lazily-generated SVG icon.
#[must_use]
pub fn icon<'a>(name: impl Into<Cow<'a, str>>, size: u16) -> Icon<'a> {
    Icon {
        content_fit: None,
        height: None,
        name: name.into(),
        size,
        style: crate::theme::Svg::default(),
        theme: Cow::Borrowed("Pop"),
        width: None,
    }
}

impl<'a> Icon<'a> {
    #[must_use]
    fn into_svg<Message: 'static>(self) -> Element<'a, Message> {
        let svg = Rc::new(self);
        let svg_clone = Rc::clone(&svg);

        iced_lazy::lazy(svg_clone, move || -> Element<Message> {
            let icon = freedesktop_icons::lookup(&svg.name)
                .with_size(svg.size)
                .with_theme(&svg.theme)
                .with_cache()
                .force_svg()
                .find();

            let handle = if let Some(path) = icon {
                svg::Handle::from_path(path)
            } else {
                eprintln!("icon '{}' size {} not found", svg.name, svg.size);
                    svg::Handle::from_memory(Vec::new())
            };

            let mut widget = svg::Svg::<Renderer>::new(handle)
                .style(svg.style)
                .width(svg.width.unwrap_or(Length::Units(svg.size)))
                .height(svg.height.unwrap_or(Length::Units(svg.size)));

            if let Some(content_fit) = svg.content_fit {
                widget = widget.content_fit(content_fit);
            }

            widget.into()
        }).into()
    }
}

impl<'a, Message: 'static> From<Icon<'a>> for Element<'a, Message> {
    fn from(icon: Icon<'a>) -> Self {
        icon.into_svg::<Message>()
    }
}
