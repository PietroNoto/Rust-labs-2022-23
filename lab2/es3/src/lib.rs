use std::str;
use std::collections::HashMap;
use chrono::prelude::Local;
use itertools::{self, Itertools};


#[derive(Copy, Clone, PartialEq)]
pub enum FileType
{
    Text, Binary
}

#[derive(Clone, PartialEq)]
pub enum Node
{
    File(File),
    Dir(Dir)
}


pub struct MatchResult<'a>
{
    queries: Vec<&'a str>,
    nodes: Vec<&'a mut Node>
}


#[derive(Clone, PartialEq)]
pub struct File
{
    pub name: String, 
    pub content: Vec<u8>,
    pub creation_time: u64,
    pub type_: FileType
}

impl File
{
    /* Creates an empty file with given name. As default, it will be a text file */
    pub fn new(name: String) -> File
    {
        let name_ = name;
        let content_ = Vec::<u8>::new();
        let creation_time_ = Local::now().timestamp() as u64;

        File {name: name_, content: content_, creation_time: creation_time_, type_: FileType::Text}
    }


    /* Creates a non empty, previously initialized file */
    pub fn new_with_content(name: String, content: Vec<u8>, creation_time: u64, type_: FileType) -> File
    {
        File { name: name, content: content, creation_time: Local::now().timestamp() as u64, type_: type_}
    }


    /* Truncate file content to desired amount of bytes. Returns truncated copy of original file 
        or None if no truncation occured */
    pub fn truncate(&self, len: usize) -> Option<File>
    {
        if !self.content.is_empty() && self.content.len() > len
        {
            let mut trunc_content = self.content.clone();
            trunc_content.truncate(len);

            Some(File::new_with_content(self.name.clone(), trunc_content, self.creation_time, self.type_))
        }
        else 
        {
            None
        }
    }
}


#[derive(Clone, PartialEq)]
pub struct Dir
{
    pub name: String,
    pub creation_time: u64,
    children: Vec<Node>
}

impl Dir
{
    pub fn new(name: String) -> Dir
    {
        let name_ = name;
        let creation_time_ = Local::now().timestamp() as u64;
        let childen_= Vec::<Node>::new();

        Dir { name: name_, creation_time: creation_time_, children: childen_ }
    }


    pub fn lookup_dir(&mut self, name: &str) -> Option<&mut Dir>
    {
        let res = 
        self.children.iter_mut().find(|node|
            match node
            {
                Node::Dir(node) => node.name == name,
                _ => false
            });
        match res
        {
            Some(Node::Dir(dir)) => Some(dir),
            _ => None
        }

       
    }


    pub fn lookup_file(&mut self, name: &str) -> Option<&mut File>
    {
        let res = 
        self.children.iter_mut().find(|node|
            match node
            {
                Node::File(node) => node.name == name,
                _ => false
            });
        match res
        {
            Some(Node::File(f)) => Some(f),
            _ => None
        }
    }


    pub fn add_dir(&mut self, name: &str) -> Result<bool, String> 
    {
        // There must be no directories with same name
        match self.lookup_dir(name)
        {
            Some(_) => Err("Directory ".to_string() + name + "already exists!"),
            None => 
            {
                let new_dir = Dir::new(name.to_string());
                self.children.push(Node::Dir(new_dir));
                Ok(true)
            }
        }
    }


    pub fn rm_dir(&mut self, name: &str) -> Result<bool, String>
    {
        for (index, child) in self.children.iter_mut().enumerate()
        {
            match child
            {
                Node::Dir(dir) => 
                {
                    if dir.name == name
                    {
                        if !dir.is_empty()
                        {
                            return Err("Error: directory is not empty".to_string());
                        }
                        self.children.swap_remove(index);
                        return Ok(true);
                    }
                },
                _ => {}
            }
        }
        return Err("Error: can't find directory".to_string() + " " + name);
    }


    pub fn is_empty(&self) -> bool
    {
        self.children.is_empty()
    }


    pub fn add_file(&mut self, file: File) -> Result<bool, String>
    {
        // There must be no files with same name
        match self.lookup_file(&file.name)
        {
            Some(_) => Err("File ".to_string() + &file.name + "already exists!"),
            None => 
            {
                file.truncate(1000);
                self.children.push(Node::File(file));
                Ok(true)
            }
        }
    }


    pub fn rm_file(&mut self, name: &str) -> Result<bool, String>
    {
        for (index, child) in self.children.iter_mut().enumerate()
        {
            match child
            {
                Node::File(f) => 
                {
                    if f.name == name
                    {
                        self.children.swap_remove(index);
                        return Ok(true);
                    }
                },
                _ => {}
            }
        }
        return Err("Error: can't find file".to_string() + " " + name);
    }

}



pub struct FileSystem
{
    root: Dir
}

impl FileSystem
{
    /* Creates a new filesystem with only an empty root directory */
    pub fn new() -> FileSystem
    {
        FileSystem { root: Dir::new("root".to_string()) }
    }


    /* Checks whether given path exists, explores it and returns last directory of the path */
    pub fn exists_path(&mut self, entries: &Vec<&str>) -> Option<&mut Dir>
    {
        let entries: Vec<String> = entries.iter().map(|e| e.to_ascii_lowercase()).collect();
        if entries.first().unwrap().to_string() != "root" || entries.len() < 2
        {
            return None;
        }
        let mut current_dir: &mut Dir = &mut self.root;
        
        for entry in entries
        {
            match current_dir.lookup_dir(&entry)  
            {
                Some(dir) => current_dir = dir,
                _ => 
                {
                    eprint!("Error: no directory with this name: {}", entry);
                    return None;
                }
            }
        }
        return Some(current_dir);
    }

    
    /*  Creates a new directory in the filesystem given a path. */
    pub fn mk_dir(&mut self, path: &str)
    {
        let mut entries = path.split('/').collect::<Vec<&str>>();
        if entries.len() < 2
        {
            return;
        }
        // Remove last element because it's the one to be mounted
        let new_dir_name = entries.pop().unwrap();
        
        match self.exists_path(&entries)
        {
            Some(mnt) => 
            {
                match mnt.add_dir(new_dir_name)
                {
                    Err(msg) => eprint!("{}", msg),
                    _ => {},
                }
            },
            None => {}
        }
    }


    /* Removes a directory, only if it is empty */
    pub fn rm_dir(&mut self, path: &str)
    {
        let mut entries = path.split('/').collect::<Vec<&str>>();
        if entries.len() < 2 
        {
            return;
        }
        // Remove last element because it's the target of elimination
        let to_be_rm = entries.pop().unwrap();

        match self.exists_path(&entries)
        {
            Some(parent) => 
            {
                match parent.rm_dir(to_be_rm)
                {
                    Ok(_) => {},
                    Err(msg) => eprint!("{}", msg)
                }
            },
            None => {}
        }
    }


    /* Creates a new file */
    pub fn new_file(&mut self, path: &str, file: File)
    {
        let entries = path.split('/').collect::<Vec<&str>>();
        if entries.is_empty()
        {
            return;
        }
        match self.exists_path(&entries)
        {
            Some(mnt) => 
            {
                match mnt.add_file(file)
                {
                    Err(msg) => eprint!("{}", msg),
                    _ => {}
                }
            }
            None => {}
        }
    }

    /* Removes a file */
    pub fn rm_file(&mut self, path: &str)
    {
        let mut entries = path.split('/').collect::<Vec<&str>>();
        if entries.len() < 2 
        {
            return;
        }
        // Remove last element because it's the target of elimination
        let to_be_rm = entries.pop().unwrap();

        match self.exists_path(&entries)
        {
            Some(parent) => 
            {
                match parent.rm_file(to_be_rm)
                {
                    Err(msg) => eprint!("{}", msg), 
                    _ => {}
                }
            },
            None => {}
        }

    }


    /* Retrieves a file */
    pub fn get_file(&mut self, path: &str) -> Option<&mut File>
    {
        let mut entries = path.split('/').collect::<Vec<&str>>();
        if entries.len() < 2
        {
            return None;
        }

        // Remove last element because it's the file to be be returned
        let file_name = entries.pop().unwrap();

        match self.exists_path(&entries)
        {
            Some(parent) => parent.lookup_file(file_name),
            None => 
            {
                eprint!("Error: cannot retrieve file: {}", file_name);
                None
            }    
        }
    }


    /* Retrieves all files that match user-defined queries */
    pub fn search<'a>(&'a mut self, queries: &'a [&'a str]) -> Option<MatchResult>
    {
        let mut mr: MatchResult<'a> = MatchResult {queries: Vec::new(), nodes: Vec::new()};

        let mut filter_map: HashMap<&str, fn(&str, &File) -> bool> = HashMap::new();
        filter_map.insert("name", FileSystem::filter_name);
        filter_map.insert("content", FileSystem::filter_content);
        filter_map.insert("larger", FileSystem::filter_larger);
        filter_map.insert("smaller", FileSystem::filter_smaller);
        filter_map.insert("older", FileSystem::filter_older);
        filter_map.insert("newer", FileSystem::filter_newer);

        let mut visits: Vec<&mut Dir> = Vec::new();
        visits.push(&mut self.root);

        while let Some(dir) = visits.pop()
        {
            for child in dir.children.iter_mut()
            {
                match child
                {
                    Node::File(f) => 
                    {
                        match FileSystem::filters(f, queries, &filter_map)
                        {
                            Some(matched_queries) => 
                            {
                                for mq in matched_queries
                                {
                                    mr.queries.push(mq);
                                }
                                mr.nodes.push(child);
                                
                            },
                            None => {}
                        }
                    },

                    Node::Dir(d) => visits.push(d)
                }
            }
        }
        if mr.nodes.is_empty()
        {
            None
        }
        else
        {
            Some(mr)
        }

    }


    fn filters<'a, 'b>(
        node: &'b File, 
        queries: &'a [&'a str], 
        filter_map: &HashMap<&str, fn(&str, &File) -> bool>) -> Option<Vec<&'a str>>
    {
        let mut matched_queries: Vec<&str> = Vec::new();

        for query in queries
        {
            match query.split(':').collect_tuple()
            {
                Some((key, val)) =>
                {
                    match filter_map.get(key.to_ascii_lowercase().as_str())
                    {
                        Some(filter) => 
                        {
                            if filter(val, node)
                            {
                                matched_queries.push(query);
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }
        if matched_queries.is_empty()
        {
            None
        }
        else
        {
            Some(matched_queries)
        }
    }


    fn filter_name(name: &str, node: &File) -> bool
    {
        node.name.contains(name)
    }


    fn filter_content(content: &str, node: &File) -> bool
    {
        if node.type_ == FileType::Text
        {
            match str::from_utf8(&node.content)
            {
                Ok(file_content) => file_content.contains(content),
                Err(_) => false
            }
        } 
        else 
        {
            false
        }
    }


    fn filter_larger(thrs: &str, node: &File) -> bool
    {
        node.content.len() > usize::from_str_radix(thrs, 10).unwrap()
    }


    fn filter_smaller(thrs: &str, node: &File) -> bool
    {
        node.content.len() < usize::from_str_radix(thrs, 10).unwrap()
    }


    fn filter_newer(timestamp: &str, node: &File) -> bool
    {
        node.creation_time > u64::from_str_radix(timestamp, 10).unwrap()
    }


    fn filter_older(timestamp: &str, node: &File) -> bool
    {
        node.creation_time < u64::from_str_radix(timestamp, 10).unwrap()
    }

}