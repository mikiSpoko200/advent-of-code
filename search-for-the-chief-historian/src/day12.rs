use std::collections::HashMap;

use crate::core;
use arrayvec::ArrayVec;

use crate::Day;

type RegionId = u8;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Coords {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plot {
    coords: Coords,
    id: RegionId,
}

pub struct Vec2d<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Vec2d<T> {
    pub fn new_from_default(width: usize, height: usize) -> Self
    where
        T: Default + Clone
    {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn from_slice(data: &[T], stride: usize) -> Self
    where
        T: Clone
    {
        assert!(data.len() % stride == 0);
        Self {
            width: stride,
            height: data.len() / stride,
            data: Vec::from(data),
        }
    }
}

impl<T> std::ops::Index<Coords> for Vec2d<T> {
    type Output = T;

    fn index(&self, Coords { x, y }: Coords) -> &Self::Output {
        &self.data[x as usize + y as usize * self.width]
    }
}

impl<T> std::ops::IndexMut<Coords> for Vec2d<T> {
    fn index_mut(&mut self, Coords { x, y }: Coords) -> &mut Self::Output {
        &mut self.data[x as usize + y as usize * self.width]
    }
}

pub struct Map {
    data: Vec2d<RegionId>
}

impl Map {
    pub fn plot(&self, coords: Coords) -> Plot {
        Plot {
            coords,
            id: self.data[coords],
        }
    }
}

pub struct Region {
    id: RegionId,
    pwbs: Vec<Pwb>,
}

impl Region {
    pub fn new(id: RegionId, pwbs: Vec<Pwb>) -> Self {
        Self {
            id,
            pwbs,
        }
    }

    pub fn area(&self) -> usize {
        self.pwbs.len()
    }

    pub fn perimeter(&self) -> usize {
        self.pwbs.iter().map(|num| 4 - num.n_neighbors as usize).sum()
    }

    pub fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    pub fn discounted_price(&self) -> usize {

    }
}

impl Map {
    pub fn width(&self) -> usize {
        self.data.width
    }

    pub fn height(&self) -> usize {
        self.data.height
    }

    fn move_coords(&self, coords: Coords, direction: Direction) -> Option<Coords> {
        match direction {
            Direction::Up    if coords.y > 0 => Some(Coords { y: coords.y - 1, ..coords }),
            Direction::Down  if coords.y < self.height() as u16 - 1 => Some(Coords { y: coords.y + 1, ..coords }),
            Direction::Left  if coords.x > 0 => Some(Coords { x: coords.x - 1, ..coords }),
            Direction::Right if coords.x < self.width() as u16 - 1 => Some(Coords { x: coords.x + 1, ..coords }),
            _ => None
        }
    }

    fn plot_neighbors(&self, plot: Plot) -> impl Iterator<Item = Coords> + '_ {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .into_iter()
            .filter_map(move |direction| self.move_coords(plot.coords, direction))
    }

    pub fn neighbors(&self, plot: &Plot) -> ArrayVec<Plot, 4> {
        self.plot_neighbors(*plot)
            .into_iter()
            .filter_map(|coords|
                self.is_in_bounds(&coords)
                    .then(|| Plot { coords, id: self.data[coords] })
                    .filter(|neighbor| neighbor.id == plot.id)
            )
            .collect::<ArrayVec<_, 4>>()
    }

    fn is_in_bounds(&self, coordinates: &Coords) -> bool {
        coordinates.x < self.width() as u16 && coordinates.y < self.height() as u16
    }

    pub fn graph(&self) -> MapGraph {
        MapGraph::new(self)
    }
}

pub struct MapGraph {
    components: HashMap<RegionId, Vec<Region>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PlotWithBorder {
    n_neighbors: u8
}
type Pwb = PlotWithBorder;

impl MapGraph {
    fn components(&self) -> &HashMap<RegionId, Vec<Region>> {
        &self.components
    }
    
    fn explore_region(map: &Map, start: &Plot, visited: &mut Vec2d<bool>) -> Region {
        let mut remaining = Vec::from([*start]);
        let mut pwbs = Vec::with_capacity(32);
        let id = start.id;

        while let Some(current) = remaining.pop() {
            if !visited[current.coords] {  // visit
                visited[current.coords] = true;
                let neighbors = map.neighbors(&current);
                pwbs.push(PlotWithBorder { n_neighbors: neighbors.len() as _ });
                remaining.extend(neighbors.into_iter());
            }
        }
        Region { id, pwbs }
    }

    pub fn new(map: &Map) -> Self {
        let mut visited = Vec2d::<bool>::new_from_default(map.width(), map.height());
        let mut components = HashMap::new();

        // keep looking tough plots
        for y in 0..map.height() as u16 {
            for x in 0..map.width() as u16 {
                let coords = (x, y).into();
                if !visited[coords] {
                    // visit
                    let start = map.plot(coords);
                    components.entry(start.id)
                        .or_insert(Vec::new())
                        .push(Self::explore_region(map, &start, &mut visited));
                }
            }
        }

        Self { components }
    }
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut stride = None;
        let map = iter.into_iter()
            .flat_map(|line| {
                stride.get_or_insert(line.len());
                line.bytes()
            })
            .collect::<Vec<_>>();

        Self {
            data: Vec2d::from_slice(&map, stride.unwrap()),
        }
    }
}

pub struct Part1;

impl core::Solution<12> for Part1 {
    type Result = u64;

    fn try_solve() -> eyre::Result<Self::Result> {
        let day = Day::new(12, 1);
        Ok(day.input()?
            .lines()
            .collect::<Map>()
            .graph()
            .components()
            .values()
            .map(|regions| regions
                .iter()
                .map(|region| region.price() as u64)
                .sum::<u64>()
            )
            .sum::<u64>())
    }
}


//
//
// 
pub mod part1 {
    use super::*;

    pub fn solve() -> eyre::Result<>{
        
    }
}


//
//
// 
//
//
//
//
//
//
//
//
//

pub struct MapNavigator {
    
}

// Region ID -> list of &Regions

// Due to "modern" business practices, the price of fence required 
// for a region is found by multiplying that region's area by its perimeter. 
// The total price of fencing all regions on a map is found by adding together 
// the price of fence for every region on the map.


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_map_from_iterator() {
        let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

        let map = input.lines().collect::<Map>();
        assert_eq!(map.width(), 10);
        println!("{}", map.height());
        assert_eq!(map.height(), 10);
    }

    #[test]
    pub fn test_neighbors_1() {
        let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

        let map = input.lines().collect::<Map>();
        let coords = Coords {
            x: 1,
            y: 1,
        };
        let plot = Plot {
            coords,
            id: b'R',
        };

        assert_eq!(map.neighbors(&plot).into_iter().collect::<Vec<_>>(), &[
            Plot {
                coords: Coords { x: 1, y: 0 },
                id: b'R',
            },
            Plot {
                coords: Coords { x: 0, y: 1 },
                id: b'R',
            },
            Plot {
                coords: Coords { x: 2, y: 1 },
                id: b'R',
            },
        ]);
    }

    #[test]
    pub fn test_neighbors_2() {
        let input = "AB\nCD";

        let map = input.lines().collect::<Map>();
        let coords = Coords {
            x: 0,
            y: 0,
        };
        let plot = Plot {
            coords,
            id: b'A',
        };

        assert_eq!(
            map.neighbors(&plot)
                .into_iter()
                .collect::<Vec<_>>(), 
                &[]
        );
    }

    #[test]
    pub fn test_neighbors() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";

        let graph = input.lines()
            .collect::<Map>()
            .graph();
        let a_components = graph.components()
            .get(&b'A').unwrap();
        assert_eq!(a_components.len(), 1);
        assert_eq!(a_components[0].pwbs.as_slice(), [
            Pwb { n_neighbors: 1 },
            Pwb { n_neighbors: 2 },
            Pwb { n_neighbors: 2 },
            Pwb { n_neighbors: 1 },
        ].as_slice());
    }

    #[test]
    pub fn test_graph_1() {
        let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        let graph = input.lines().collect::<Map>().graph();
        assert_eq!(graph.components().len(), 2);
        assert_eq!(graph.components()[&b'O'].len(), 1);
        assert_eq!(graph.components()[&b'X'].len(), 4);
        for region in &graph.components()[&b'X'] {
            assert_eq!(region.area(), 1);
            assert_eq!(region.perimeter(), 4);
        }
    }

    #[test]
    fn test_price() {
        let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
        let price = input.lines()
            .collect::<Map>()
            .graph()
            .components()
            .values()
            .map(|regions| regions
                .iter()
                .map(|region| region.price() as u64)
                .sum::<u64>()
            )
            .sum::<u64>();
        assert_eq!(price, 1930);
    }

    #[test]
    fn test_discounted() {
        let input = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
        let discounted_price = 
    }
}