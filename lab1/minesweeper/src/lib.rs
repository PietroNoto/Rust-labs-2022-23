pub fn annotate(minefield: &[&str]) -> Vec<String> 
{
    let mut new_mf: Vec<String> = Vec::new();
    
    if !minefield.is_empty()
    {
        // Casi "banali"
        if minefield.iter().any(|row: &&str| (*row).is_empty()) 
            || match_all_characters(&minefield, '*')
            || match_all_characters(&minefield, ' ')
        {
            for i in 0..minefield.len()
            {
                new_mf.push((*minefield[i]).to_string());
            }
        }
        
        // Annotazione
        for i in 0..minefield.len()
        {
            let mut ann_line = lookup_same_line(&minefield[i].chars().collect());
            if i > 0
            {
                ann_line = lookup_above_bottom(&ann_line, &minefield[i-1].chars().collect());
            }
            if i < minefield.len() - 1
            {
                ann_line = lookup_above_bottom(&ann_line, &minefield[i+1].chars().collect());
            }
            new_mf.push(ann_line.iter().collect());
        }    
    }
    new_mf   
}


fn match_all_characters(minefield: &&[&str], ch: char) -> bool
{
    minefield.iter()
        .flat_map(|row| (*row).chars())
        .all(|c| c == ch)
    
}


fn lookup_same_line(line: &Vec<char>) -> Vec<char>
{
    let mut ann_line: Vec<char> = Vec::new();

    for i in 0..line.len()
    {
        if line[i] == ' ' || line[i].is_numeric()
        {
            let mut count: u32 = 0;
            if line[i].is_numeric()
            {
                count = char::to_digit(line[i], 10).unwrap();
            }
            if i > 0 && line[i-1] == '*'
            {
                count += 1;
            }
            if i < line.len() - 1 && line[i+1] == '*'
            {
                count += 1;
            }
            ann_line.push(char::from_digit(count, 10).unwrap());
        }
        else if line[i] == '*'
        {
            ann_line.push(line[i]);
        }
    }
    ann_line
}


fn lookup_above_bottom(current: &Vec<char>, other: &Vec<char>) -> Vec<char>
{
    let mut ann_line: Vec<char> = Vec::new();

    for i in 0..current.len()
    {
        if current[i] == ' ' || current[i].is_numeric()
        {
            let mut count: u32 = 0;
            if current[i].is_numeric()
            {
                count = char::to_digit(current[i], 10).unwrap();
            }
            if other[i] == '*'
            {
                count += 1;
            }
            if i > 0 && other[i-1] == '*'
            {
                count += 1;
            }
            if i < other.len() - 1 && other[i+1] == '*'
            {
                count += 1;
            }
            ann_line.push(char::from_digit(count, 10).unwrap());
        }
        else if current[i] == '*'
        {
            ann_line.push(current[i]);
        }
    }

    ann_line
}
