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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face {
    // true == face runs north-south
    vertical: bool,
    // for vertical this is the cell to the right, for horizontal it's the cell below
    start: Location,
}

impl Face {
    /// Face of the given location's cell when moving in the heading direction.
    pub fn new(location: Location, heading: (Index, Index)) -> Self {
        assert!(heading.0 == 0 || heading.1 == 0);
        assert!(heading.0.abs() == 1 || heading.1.abs() == 1);

        if heading.0 == 0 {
            // moving only in y direction, so horizontal face
            Face {
                vertical: false,
                start: if heading.1 == -1 {
                    // going up, so face is above us
                    location
                } else {
                    // going down, so face is below us
                    location + vec2(0, 1)
                },
            }
        } else {
            // moving only in x direction, so vertical face
            Face {
                vertical: true,
                start: if heading.0 == -1 {
                    // going left, so face is to the left
                    location
                } else {
                    // going right, so face is to the right
                    location + vec2(1, 0)
                },
            }
        }
    }

    pub fn touching_locations(&self) -> [Location; 2] {
        let start = self.start;
        let end = if self.vertical {
            start + vec2(-1, 0)
        } else {
            start + vec2(0, -1)
        };

        [start, end]
    }

    pub fn same_direction_neighbors(&self) -> [Face; 2] {
        let fence_direction = if self.vertical {
            vec2(0, 1)
        } else {
            vec2(1, 0)
        };

        [
            Face {
                vertical: self.vertical,
                start: self.start + fence_direction,
            },
            Face {
                vertical: self.vertical,
                start: self.start - fence_direction,
            },
        ]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_face_normalization() {
        let location = Location::new(0, 0);
        let face = Face::new(location, (0, 1));
        assert_eq!(face, Face::new(location + vec2(0, 1), (0, -1)));

        let face = Face::new(location, (1, 0));
        assert_eq!(face, Face::new(location + vec2(1, 0), (-1, 0)));
    }

    #[test]
    fn test_face_neighbors() {
        let location = Location::new(0, 0);
        let face = Face::new(location, (0, 1));
        let neighbors: HashSet<_> = face.same_direction_neighbors().into_iter().collect();
        assert_eq!(
            neighbors,
            [
                Face::new(location + vec2(-1, 0), (0, 1)),
                Face::new(location + vec2(1, 0), (0, 1)),
            ]
            .into_iter()
            .collect()
        );

        let face = Face::new(location, (1, 0));
        let neighbors: HashSet<_> = face.same_direction_neighbors().into_iter().collect();
        assert_eq!(
            neighbors,
            [
                Face::new(location + vec2(0, 1), (1, 0)),
                Face::new(location + vec2(0, -1), (1, 0)),
            ]
            .into_iter()
            .collect()
        );
    }
}
