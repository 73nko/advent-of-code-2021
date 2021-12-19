use std::{
    fmt::{Debug, Display, Write},
    hash::Hash,
    ops::Index,
};

/// Boilerplate for grid-related problems
pub struct Grid<V> {
    data: Vec<V>,
    width: usize,
    height: usize,
}

const CARDINAL: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

const DIAGONAL: [(isize, isize); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

const ADJACENT: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// The direction to go towards.
///
/// Used in a few functions, mainly with traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// The cardinal directions (North, South, East, West)
    Cardinal,
    /// The diagonal directions (North West, North East, South West, South East)
    Diagonal,
    /// All adjacent directions (NW, N, NE, W, E, SW, S, SE)
    Adjacent,
}

impl Direction {
    fn as_iter(self) -> std::slice::Iter<'static, (isize, isize)> {
        match self {
            Direction::Cardinal => CARDINAL.iter(),
            Direction::Diagonal => DIAGONAL.iter(),
            Direction::Adjacent => ADJACENT.iter(),
        }
    }
}

impl<V> Grid<V> {
    pub fn with_data(data: Vec<V>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[inline]
    const fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width as usize) + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&V> {
        if x < self.width && y < self.height {
            // safety: already bound check
            unsafe { Some(self.data.get_unchecked(self.index(x, y))) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut V> {
        if x < self.width && y < self.height {
            // safety: already bound check
            let index = self.index(x, y);
            unsafe { Some(self.data.get_unchecked_mut(index)) }
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbors(&self, x: usize, y: usize, direction: Direction) -> Directional {
        Directional {
            x: x as isize,
            y: y as isize,
            width: self.width as isize,
            height: self.height as isize,
            iter: direction.as_iter(),
        }
    }

    pub fn coordinates(&self) -> Coordinates {
        Coordinates::new(self.width, self.height)
    }

    pub fn as_vec(&self) -> &Vec<V> {
        &self.data
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<V> {
        &mut self.data
    }

    pub fn items<'a>(&'a self) -> Items<'a, V> {
        Items {
            iter: self.data.iter(),
            coordinates: self.coordinates(),
        }
    }

    pub fn items_mut<'a>(&'a mut self) -> ItemsMut<'a, V> {
        ItemsMut {
            coordinates: self.coordinates(),
            iter: self.data.iter_mut(),
        }
    }
}

impl<V> Grid<V>
where
    V: Clone + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }
}

impl<V> Index<(usize, usize)> for Grid<V> {
    type Output = V;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.index(index.0, index.1)]
    }
}

pub trait FromCell
where
    Self: Sized,
{
    fn from_cell(c: char) -> Option<Self>;
}

impl<V> Grid<V>
where
    V: FromCell,
{
    pub fn from_cells(s: &str) -> Option<Self> {
        let data: Option<Vec<V>> = s
            .lines()
            .flat_map(|x| x.chars())
            .map(|c| V::from_cell(c))
            .collect();

        let data = data?;
        let height = s.lines().count();
        let width = s.find('\n').unwrap_or_else(|| data.len() / height);

        Some(Self {
            data,
            width,
            height,
        })
    }
}

pub struct Directional {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    iter: std::slice::Iter<'static, (isize, isize)>,
}

impl Iterator for Directional {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (dx, dy) = self.iter.next()?;
            let (x, y) = (self.x + dx, self.y + dy);
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                continue;
            }

            break Some((x as usize, y as usize));
        }
    }
}

pub struct Coordinates {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for Coordinates {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            None
        } else {
            let result = (self.x, self.y);
            self.x += 1;
            Some(result)
        }
    }
}

pub struct Items<'a, V> {
    iter: std::slice::Iter<'a, V>,
    coordinates: Coordinates,
}

impl<'a, V> Iterator for Items<'a, V> {
    type Item = ((usize, usize), &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.coordinates.next()?;
        let item = self.iter.next()?;
        Some(((x, y), item))
    }
}

pub struct ItemsMut<'a, V> {
    iter: std::slice::IterMut<'a, V>,
    coordinates: Coordinates,
}

impl<'a, V> Iterator for ItemsMut<'a, V> {
    type Item = ((usize, usize), &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.coordinates.next()?;
        let item = self.iter.next()?;
        Some(((x, y), item))
    }
}

impl Grid<u8> {
    /// A new grid that's separated by newlines and only a single ASCII digit.
    pub fn single_ascii_number(input: &str) -> Self {
        let data: Vec<_> = input
            .lines()
            .flat_map(|x| x.bytes())
            .map(|x| x - b'0')
            .collect();

        let height = input.lines().count();
        let width = input.find('\n').unwrap_or_else(|| data.len() / height);

        Self {
            data,
            width,
            height,
        }
    }
}

impl<V> Debug for Grid<V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("data", &self.data)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl<V> Display for Grid<V>
where
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, item) in self.data.iter().enumerate() {
            item.fmt(f)?;
            if (index + 1) % self.width == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
