use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaveName([char; 2]);

impl FromStr for CaveName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.len() == 2 {
            let mut chars = s.chars();
            Self([chars.next().unwrap(), chars.next().unwrap()])
        } else if s.len() == 1 {
            Self([s.chars().next().unwrap(), '\0'])
        } else {
            return Err(());
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cave {
    Start,
    Small(CaveName),
    Large(CaveName),
    End,
}

impl Cave {
    pub fn is_small(&self) -> bool {
        matches!(self, Self::Small(_))
    }

    pub fn is_large(&self) -> bool {
        matches!(self, Self::Large(_))
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            name if name.len() <= 2 => {
                if name.chars().all(char::is_lowercase) {
                    Self::Small(name.parse()?)
                } else if name.chars().all(char::is_uppercase) {
                    Self::Large(name.parse()?)
                } else {
                    return Err(());
                }
            }
            _ => return Err(()),
        })
    }
}

struct CaveConnections {
    start: usize,
    len: usize,
}

pub struct CaveSystem {
    links: Vec<Cave>,
    connections: HashMap<Cave, CaveConnections>,
}

impl CaveSystem {
    pub fn connections(&self, cave: &Cave) -> Option<&[Cave]> {
        self.connections
            .get(cave)
            .map(|conn| &self.links[conn.start..conn.start + conn.len])
    }
}

impl FromStr for CaveSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();

        for l in s.split('\n') {
            if !l.is_empty() {
                let (a, b) = l.split_once('-').unwrap();
                map.entry(a).or_default().push(b);
                map.entry(b).or_default().push(a);
            }
        }

        let mut links = Vec::new();
        let mut connections = HashMap::new();

        for (src, dst) in map {
            connections.insert(
                src.parse()?,
                CaveConnections {
                    start: links.len(),
                    len: dst.len(),
                },
            );

            links.extend(dst.iter().map(|s| s.parse::<Cave>().unwrap()));
        }

        Ok(Self { links, connections })
    }
}
