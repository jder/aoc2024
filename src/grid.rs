use crate::prelude::*;
use std::collections::HashSet;

pub type Index = isize;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    contents: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Returns a cell for this location, or None if the location is out of bounds.
    pub fn cell(&self, location: Location) -> Option<Cell<T>> {
        if location.x < 0
            || location.y < 0
            || location.x >= self.width as Index
            || location.y >= self.height as Index
        {
            None
        } else {
            Some(Cell {
                grid: self,
                location,
            })
        }
    }

    pub fn set(&mut self, location: Location, value: T) {
        self.contents[location.y as usize][location.x as usize] = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> + Clone {
        let width = self.width as Index;
        let height = self.height as Index;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| Location::new(x, y)))
            .map(move |location| Cell {
                grid: self,
                location,
            })
    }

    pub fn map<U>(&self, mut f: impl FnMut(Cell<T>) -> U) -> Grid<U> {
        let mut contents = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut new_row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                new_row.push(f(Cell {
                    grid: self,
                    location: Location::new(x as Index, y as Index),
                }));
            }
            contents.push(new_row);
        }

        Grid {
            contents,
            width: self.width,
            height: self.height,
        }
    }
}

impl Grid<char> {
    pub fn new_with_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        let mut contents = Vec::new();
        let mut width = None;
        let mut height = 0;

        for line in lines {
            let mut row = Vec::new();
            for c in line.as_ref().chars() {
                row.push(c);
            }
            if let Some(w) = width {
                assert_eq!(w, row.len());
            } else {
                width = Some(row.len());
            }
            contents.push(row);
            height += 1;
        }

        Self {
            contents,
            width: width.unwrap_or_default(),
            height,
        }
    }
}

pub type Location = euclid::default::Point2D<Index>;

pub fn all_headings() -> impl Iterator<Item = (Index, Index)> {
    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
}

pub fn neighbors(l: Location) -> impl Iterator<Item = Location> {
    all_headings().map(move |(dx, dy)| l + vec2(dx, dy))
}

pub fn cardinal_headings() -> impl Iterator<Item = (Index, Index)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().copied()
}

pub fn cardinal_neighbors(l: Location) -> impl Iterator<Item = Location> {
    cardinal_headings().map(move |(dx, dy)| l + vec2(dx, dy))
}

#[derive(Debug)]
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    location: Location,
}

impl<'a, T> Cell<'a, T> {
    pub fn contents(&self) -> &T {
        self.grid
            .contents
            .get(self.location.y as usize)
            .and_then(|row| row.get(self.location.x as usize))
            .expect("Cell should always be in bounds")
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn offset(&self, dx: Index, dy: Index) -> Option<Cell<'a, T>> {
        self.grid.cell(self.location + vec2(dx, dy))
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Cell<'a, T>> {
        let grid = self.grid;
        neighbors(self.location).flat_map(move |location| grid.cell(location))
    }

    pub fn cardinal_neighbors(&self) -> impl Iterator<Item = Cell<'a, T>> {
        let grid = self.grid;
        cardinal_neighbors(self.location).flat_map(move |location| grid.cell(location))
    }

    /// Walks in the given direction until it hits the edge of the grid.
    /// This cell is not included in the iterator.
    pub fn walk(&self, dx: Index, dy: Index) -> impl Iterator<Item = Cell<'a, T>> {
        let mut cell = *self;
        std::iter::from_fn(move || {
            if let Some(next) = cell.offset(dx, dy) {
                cell = next;
                Some(cell)
            } else {
                None
            }
        })
    }

    /// Walks in the given direction until it hits the edge of the grid.
    /// This cell is included in the iterator.
    pub fn walk_inclusive(&self, dx: Index, dy: Index) -> impl Iterator<Item = Cell<'a, T>> {
        std::iter::once(*self).chain(self.walk(dx, dy))
    }
}

impl<'a, T> Clone for Cell<'a, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            location: self.location,
        }
    }
}

impl<'a, T> Copy for Cell<'a, T> {}

impl<'a, T> PartialEq for Cell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && std::ptr::eq(self.grid, other.grid)
    }
}

impl<'a, T> Eq for Cell<'a, T> {}

impl<'a, T> Ord for Cell<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.location
            .x
            .cmp(&other.location.x)
            .then(self.location.y.cmp(&other.location.y))
            .then((self.grid as *const _ as usize).cmp(&(other.grid as *const _ as usize)))
    }
}

impl<'a, T> PartialOrd for Cell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T> std::hash::Hash for Cell<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        std::ptr::hash(self.grid, state);
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    locations: HashSet<Location>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            locations: HashSet::new(),
        }
    }

    pub fn insert(&mut self, location: Location) {
        self.locations.insert(location);
    }

    pub fn contains(&self, location: Location) -> bool {
        self.locations.contains(&location)
    }

    pub fn iter(&self) -> impl Iterator<Item = Location> + '_ {
        self.locations.iter().copied()
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Location> + '_ {
        self.locations
            .iter()
            .flat_map(|location| neighbors(*location))
            .filter(move |location| !self.locations.contains(location))
            .collect::<HashSet<_>>() // to make unique
            .into_iter()
    }

    pub fn len(&self) -> usize {
        self.locations.len()
    }
}
