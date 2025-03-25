// -----------------------------------------------------------------------------
// File: quadtrees.rs
// Description: A simple quadtree implementation.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 25, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use super::Rect;

/// Represents an item in a quadtree structure, typically used for spatial partitioning.
///
/// # Fields
/// - `x` (`f32`): The x-coordinate of the item's center.
/// - `y` (`f32`): The y-coordinate of the item's center.
/// - `tx` (`usize`): The x index of the tile in the terrain grid.
/// - `ty` (`usize`): The y index of the tile in the terrain grid.
#[derive(Debug)]
pub struct QuadTreeItem {
    pub x: f32,
    pub y: f32,
    pub tx: usize,
    pub ty: usize,
}

/// Represents a node in a quadtree structure.
///
/// # Fields
/// 
/// * `boundary` - The rectangular boundary that defines the region covered by this node.
/// * `capacity` - The maximum number of items this node can hold before it needs to subdivide.
/// * `items` - A collection of items stored in this node.
/// * `divided` - A boolean indicating whether this node has been subdivided into child nodes.
/// * `northeast` - An optional child node representing the northeast quadrant.
/// * `northwest` - An optional child node representing the northwest quadrant.
/// * `southeast` - An optional child node representing the southeast quadrant.
/// * `southwest` - An optional child node representing the southwest quadrant.
pub struct QuadTreeNode {
    boundary: Rect,
    capacity: usize,
    items: Vec<QuadTreeItem>,
    divided: bool,
    northeast: Option<Box<QuadTreeNode>>,
    northwest: Option<Box<QuadTreeNode>>,
    southeast: Option<Box<QuadTreeNode>>,
    southwest: Option<Box<QuadTreeNode>>,
}

/// A node in a quadtree data structure. Each node represents a rectangular region
/// and can contain a limited number of items. If the number of items exceeds the
/// capacity, the node subdivides into four child nodes.
///
/// # Fields
/// - `boundary`: The rectangular region represented by this node.
/// - `capacity`: The maximum number of items this node can hold before subdividing.
/// - `items`: The items contained within this node.
/// - `divided`: A flag indicating whether this node has been subdivided.
/// - `northeast`, `northwest`, `southeast`, `southwest`: Optional child nodes representing
///   the four quadrants of this node.
///
/// # Methods
/// - `new(boundary: Rect, capacity: usize) -> Self`:
///   Creates a new `QuadTreeNode` with the given boundary and capacity.
///
/// - `contains_point(rect: &Rect, x: f32, y: f32) -> bool`:
///   Checks whether a given point `(x, y)` is contained within a rectangle `rect`.
///
/// - `intersects(a: &Rect, b: &Rect) -> bool`:
///   Checks whether two rectangles `a` and `b` intersect.
///
/// - `subdivide(&mut self)`:
///   Subdivides the current node into four child nodes, each representing a quadrant
///   of the current node's boundary.
///
/// - `insert(&mut self, item: QuadTreeItem) -> bool`:
///   Attempts to insert an item into the quadtree. If the item is outside the boundary,
///   it is ignored. If the node exceeds its capacity, it subdivides and delegates the
///   insertion to its child nodes. Returns `true` if the item was successfully inserted.
///
/// - `query(&self, range: &Rect, found: &mut Vec<QuadTreeItem>)`:
///   Finds all items within a given rectangular range. If the range does not intersect
///   the node's boundary, the method returns immediately. Otherwise, it checks the items
///   in the current node and recursively queries its child nodes if they exist.
impl QuadTreeNode {
    pub fn new(boundary: Rect, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            items: Vec::new(),
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    // Checks whether the rectangle contains a point.
    fn contains_point(rect: &Rect, x: f32, y: f32) -> bool {
        x >= rect.x && x <= rect.x + rect.w &&
        y >= rect.y && y <= rect.y + rect.h
    }

    // Check if two rectangles intersect.
    fn intersects(a: &Rect, b: &Rect) -> bool {
        !(a.x > b.x + b.w ||
            a.x + a.w < b.x ||
            a.y > b.y + b.h ||
            a.y + a.h < b.y)
    }

    // Subdivide the quadtree node into four quadrants.
    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w / 2.0;
        let h = self.boundary.h / 2.0;
        self.northeast = Some(Box::new(QuadTreeNode::new(Rect::new(x + w, y, w, h), self.capacity)));
        self.northwest = Some(Box::new(QuadTreeNode::new(Rect::new(x, y, w, h), self.capacity)));
        self.southeast = Some(Box::new(QuadTreeNode::new(Rect::new(x + w, y + h, w, h), self.capacity)));
        self.southwest = Some(Box::new(QuadTreeNode::new(Rect::new(x, y + h, w, h), self.capacity)));
        self.divided = true;
    }

    // Insert an item into the quadtree.
    pub fn insert(&mut self, item: QuadTreeItem) -> bool {

        // If the item is not in boundary, skip it.
        if !Self::contains_point(&self.boundary, item.x, item.y) {
            return false;
        }

        // If the node has not reached capacity, add the item.
        if self.items.len() < self.capacity {
            self.items.push(item);
            return true;
        }

        // If the node has not been divided, subdivide it.
        if !self.divided {
            self.subdivide();
        }

        // Attempt to insert the item into the child nodes.
        if let Some(ref mut ne) = self.northeast {
            if ne.insert(item.clone()) {
                return true;
            }
        }
        if let Some(ref mut nw) = self.northwest {
            if nw.insert(item.clone()) {
                return true;
            }
        }
        if let Some(ref mut se) = self.southeast {
            if se.insert(item.clone()) {
                return true;
            }
        }
        if let Some(ref mut sw) = self.southwest {
            if sw.insert(item) {
                return true;
            }
        }
        false
    }

    // Query the quadtree for items within a given range.
    pub fn query(&self, range: &Rect, found: &mut Vec<QuadTreeItem>) {
        if !Self::intersects(&self.boundary, range) {
            return;
        }
        for item in &self.items {
            if Self::contains_point(range, item.x, item.y) {
                found.push(item.clone());
            }
        }
        if self.divided {
            if let Some(ref ne) = self.northeast {
                ne.query(range, found);
            }
            if let Some(ref nw) = self.northwest {
                nw.query(range, found);
            }
            if let Some(ref se) = self.southeast {
                se.query(range, found);
            }
            if let Some(ref sw) = self.southwest {
                sw.query(range, found);
            }
        }
    }
}

/// Implements the `Clone` trait for the `QuadTreeItem` struct.
impl Clone for QuadTreeItem {
    fn clone(&self) -> Self {
        QuadTreeItem {
            x: self.x,
            y: self.y,
            tx: self.tx,
            ty: self.ty,
        }
    }
}

/// A simple wrapper for the quadtree root.
///
/// # Fields
/// 
/// * `root` - The root node of the quadtree, which contains the entire space
///   and serves as the entry point for all operations on the tree.
pub struct QuadTree {
    pub root: QuadTreeNode,
}

/// # Methods
///
/// - `new(boundary: Rect, capacity: usize) -> Self`:
///   Creates a new `QuadTree` with the specified boundary and capacity.
/// - `insert(&mut self, item: QuadTreeItem)`:
///   Inserts an item into the `QuadTree`.
/// - `query(&self, range: Rect) -> Vec<QuadTreeItem>`:
///   Queries the `QuadTree` for items within a specified range.
impl QuadTree {
    pub fn new(boundary: Rect, capacity: usize) -> Self {
        Self { root: QuadTreeNode::new(boundary, capacity) }
    }

    pub fn insert(&mut self, item: QuadTreeItem) {
        self.root.insert(item);
    }

    pub fn query(&self, range: Rect) -> Vec<QuadTreeItem> {
        let mut found = Vec::new();
        self.root.query(&range, &mut found);
        found
    }
}