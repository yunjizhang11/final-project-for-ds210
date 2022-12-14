// read the New York listing.csv file and clean it up.
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug,PartialEq)]
enum RoomType {
    PrivateRoom, EntireHomeApt, HotelRoom
}

#[derive(Debug,PartialEq)]
enum BedRooms {
    One, Two, Three_Five, Over_Six
}

#[derive(Debug,PartialEq)]
enum Popularity {
    Level1, Level2, Level3, Level4, Level5
}

#[derive(Debug,PartialEq)]
enum AmenitiesLevel {
    Few, Common, Abundant, Luxurious
}

#[derive(Debug,PartialEq)]
enum PriceRange {
    Under100, _100_200, _200_300, _300_400, _400_500, Above500
}

#[derive(Debug)]
struct RoomInfo
{
    room_type: RoomType,
    bedrooms: BedRooms,
    popularity: Popularity,
    amenities_level: AmenitiesLevel,
    price: PriceRange,
}

impl RoomInfo{
    fn new() -> RoomInfo{
        RoomInfo{
            room_type: RoomType::PrivateRoom,
            bedrooms: BedRooms::One,
            popularity: Popularity::Level1,
            amenities_level: AmenitiesLevel::Few,
            price: PriceRange::Under100,
        }
    }
}

#[derive(Debug,Default)]
struct DecisionTreeNode {
    attribute: String,
    children: Vec<usize>,
}

impl DecisionTreeNode{
    fn new() -> DecisionTreeNode{
        DecisionTreeNode{
            attribute: String::from("Null"),
            children: Vec::new(),
        }
    }
}

fn main() {
    let mut train_vec:Vec<RoomInfo> = Vec::new();
    let mut verify_vec:Vec<RoomInfo> = Vec::new();
    
    let mut line_num = 0;
    if let Ok(lines) = read_lines("test.csv") {
        for line in lines {
            line_num += 1;
            if line_num == 1 { continue; }
            if let Ok(line_str) = line {
                if line_num % 4 == 0 {
                    verify_vec.push(pre_treatment(line_str));
                } else {
                    train_vec.push(pre_treatment(line_str));
                }
            }
        }   
    }

    let mut root:DecisionTreeNode = DecisionTreeNode::new();

    let mut tree_vec:Vec<DecisionTreeNode> = Vec::new();


    make_tree(&mut tree_vec,&mut root, train_vec);

    tree_vec.push(root);

    for i in 0..tree_vec.len() {
        println!("{} - {:?}",i, tree_vec[i]);
    }

    // let root_id = tree_vec.len()-1;
    // for roominfo in verify_vec {
    //     println!("{}", search_tree(&tree_vec, root_id, roominfo));
    // }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pre_treatment(origin_string: String) -> RoomInfo {
    let mut roominfo:RoomInfo = RoomInfo::new();
    let mut i: u32 = 1;
    let mut amenities_num:u32 = 0;
    for _str in origin_string.split(","){
        match i {
           1 => {
                roominfo.room_type = match _str {
                    "Private room" => RoomType::PrivateRoom,
                    "Entire home/apt" => RoomType::EntireHomeApt,
                    "Hotel room" => RoomType::HotelRoom,
                    _ => RoomType::PrivateRoom
                }
           },
           2 => roominfo.bedrooms = match _str {
                "" | "1" => BedRooms::One,
                "2" => BedRooms::Two,
                "3" | "4" | "5" => BedRooms::Three_Five,
                _ => BedRooms::Over_Six
            },
           3 => roominfo.popularity = {
                let review_num = if _str == "" { 1 } else { _str.parse::<u16>().unwrap() };
                if review_num < 50 {
                    Popularity::Level1
                } else if review_num > 200 {
                    Popularity::Level5
                } else { Popularity::Level3 }
           },
           4 => {
                let review_score = if _str == "" { 0.0 } else { _str.parse::<f32>().unwrap() };
                if review_score < 4.0 {
                    roominfo.popularity = match roominfo.popularity {
                        Popularity::Level1 => Popularity::Level1,
                        Popularity::Level2 => Popularity::Level2,
                        Popularity::Level3 => Popularity::Level2,
                        Popularity::Level4 => Popularity::Level4,
                        Popularity::Level5 => Popularity::Level4,
                    }
                }
            },
            5 => roominfo.price = {
                let (_,price_str) = _str.split_at(1);
                let _price= price_str.split(".").next().unwrap();
                let price:u32 = _price.parse::<u32>().unwrap();
                if price < 100 {
                    PriceRange::Under100
                } else if price < 200 {
                    PriceRange::_100_200
                } else if price < 300 {
                    PriceRange::_200_300
                } else if price < 400 {
                    PriceRange::_300_400
                } else if price < 500 {
                    PriceRange::_400_500
                } else { PriceRange::Above500 }
            },
            _ =>  amenities_num += 1
        }
        i += 1;
    }
    roominfo.amenities_level = {
        if amenities_num < 10 {
            AmenitiesLevel::Few
        } else if amenities_num < 20 {
            AmenitiesLevel::Common
        } else if amenities_num < 30 {
            AmenitiesLevel::Abundant
        } else { AmenitiesLevel::Luxurious }
    };
    return roominfo;
}

fn info_entropy(_vec: &Vec<f64>) -> f64 {
    let mut sum:f64 = 0.0;
    for i in _vec {
        sum += i;
    }
    let mut entropy:f64 = 0.0;
    for i in _vec {
        if i == &0.0 { continue; }
        entropy -= (i/sum) * f64::log2(i/sum);
    }
    return entropy;
}

fn attribute_entropy(vec_vec: &Vec<Vec<f64>>) -> (f64,f64){
    let mut res:f64 = 0.0;
    let mut all_sum:f64 = 0.0;
    let mut sum_vec:Vec<f64> = Vec::new();
    let mut entropy_vec:Vec<f64> = Vec::new();
    for vec_i in vec_vec {
        let mut sum:f64 = 0.0;
        for i in vec_i { sum += i; }
        all_sum += sum;
        sum_vec.push(sum);
        entropy_vec.push(info_entropy(vec_i));
    }
    for i in 0..sum_vec.len() {
        res += sum_vec[i]/all_sum*entropy_vec[i];
    }
    return (res,info_entropy(&sum_vec));
}

fn chose_attribute(node_vec: &Vec<RoomInfo>) -> String {
    let mut price_dist:Vec<f64> = vec![0.0;6];

    let mut roomtype_dist:Vec<Vec<f64>> = vec![vec![0.0;6];3];
    let mut bedrooms_dist:Vec<Vec<f64>> = vec![vec![0.0;6];4];
    let mut popularity_dist:Vec<Vec<f64>> = vec![vec![0.0;6];5];
    let mut amenities_level_dist:Vec<Vec<f64>> = vec![vec![0.0;6];4];

    for roominfo in node_vec {
        match roominfo.price {
            PriceRange::Under100 => {
                price_dist[0] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][0] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][0] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][0] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][0] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][0] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][0] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][0] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][0] += 1.0,
                    Popularity::Level2 => popularity_dist[1][0] += 1.0,
                    Popularity::Level3 => popularity_dist[2][0] += 1.0,
                    Popularity::Level4 => popularity_dist[3][0] += 1.0,
                    Popularity::Level5 => popularity_dist[4][0] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][0] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][0] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][0] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][0] += 1.0
                };
            },
            PriceRange::_100_200 =>  {
                price_dist[1] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][1] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][1] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][1] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][1] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][1] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][1] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][1] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][1] += 1.0,
                    Popularity::Level2 => popularity_dist[1][1] += 1.0,
                    Popularity::Level3 => popularity_dist[2][1] += 1.0,
                    Popularity::Level4 => popularity_dist[3][1] += 1.0,
                    Popularity::Level5 => popularity_dist[4][1] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][1] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][1] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][1] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][1] += 1.0
                };
            },
            PriceRange::_200_300 =>  {
                price_dist[2] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][2] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][2] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][2] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][2] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][2] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][2] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][2] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][2] += 1.0,
                    Popularity::Level2 => popularity_dist[1][2] += 1.0,
                    Popularity::Level3 => popularity_dist[2][2] += 1.0,
                    Popularity::Level4 => popularity_dist[3][2] += 1.0,
                    Popularity::Level5 => popularity_dist[4][2] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][0] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][2] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][2] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][2] += 1.0
                };
            },
            PriceRange::_300_400 =>  {
                price_dist[3] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][3] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][3] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][3] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][3] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][3] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][3] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][3] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][3] += 1.0,
                    Popularity::Level2 => popularity_dist[1][3] += 1.0,
                    Popularity::Level3 => popularity_dist[2][3] += 1.0,
                    Popularity::Level4 => popularity_dist[3][3] += 1.0,
                    Popularity::Level5 => popularity_dist[4][3] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][3] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][3] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][3] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][3] += 1.0
                };
            },
            PriceRange::_400_500 =>  {
                price_dist[4] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][4] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][4] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][4] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][4] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][4] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][4] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][4] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][4] += 1.0,
                    Popularity::Level2 => popularity_dist[1][4] += 1.0,
                    Popularity::Level3 => popularity_dist[2][4] += 1.0,
                    Popularity::Level4 => popularity_dist[3][4] += 1.0,
                    Popularity::Level5 => popularity_dist[4][4] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][4] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][4] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][4] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][4] += 1.0
                };
            },
            PriceRange::Above500 =>  {
                price_dist[5] += 1.0;
                match roominfo.room_type {
                    RoomType::PrivateRoom => roomtype_dist[0][5] += 1.0,
                    RoomType::EntireHomeApt => roomtype_dist[1][5] += 1.0,
                    RoomType::HotelRoom => roomtype_dist[2][5] += 1.0
                };
                match roominfo.bedrooms {
                    BedRooms::One => bedrooms_dist[0][5] += 1.0,
                    BedRooms::Two => bedrooms_dist[1][5] += 1.0,
                    BedRooms::Three_Five => bedrooms_dist[2][5] += 1.0,
                    BedRooms::Over_Six => bedrooms_dist[3][5] += 1.0
                };
                match roominfo.popularity {
                    Popularity::Level1 => popularity_dist[0][5] += 1.0,
                    Popularity::Level2 => popularity_dist[1][5] += 1.0,
                    Popularity::Level3 => popularity_dist[2][5] += 1.0,
                    Popularity::Level4 => popularity_dist[3][5] += 1.0,
                    Popularity::Level5 => popularity_dist[4][5] += 1.0
                };
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => amenities_level_dist[0][5] += 1.0,
                    AmenitiesLevel::Common => amenities_level_dist[1][5] += 1.0,
                    AmenitiesLevel::Abundant => amenities_level_dist[2][5] += 1.0,
                    AmenitiesLevel::Luxurious => amenities_level_dist[3][5] += 1.0
                };
            },
        }
    }
    let info_D = info_entropy(&price_dist);
    if info_D == 0.0 {
        let mut flag:bool = false;
        for i in 0..price_dist.len() {
            if price_dist[i] > 0.0 {
                flag = true;
                match i {
                    0 => return String::from("Under100"),
                    1 => return String::from("_100_200"),
                    2 => return String::from("_200_300"),
                    3 => return String::from("_300_400"),
                    4 => return String::from("_400_500"),
                    5 => return String::from("Above500"),
                    _ => {return String::from("Leaf")}
                }
            }
            if flag == false { return String::from("Null"); }
        }
    }

    let mut IGR:Vec<f64> = Vec::new();
    let (attr_ent,attr_H) = attribute_entropy(&roomtype_dist);
    IGR.push((info_D - attr_ent)/attr_H);
    let (attr_ent,attr_H) = attribute_entropy(&bedrooms_dist);
    IGR.push((info_D - attr_ent)/attr_H);
    let (attr_ent,attr_H) = attribute_entropy(&popularity_dist);
    IGR.push((info_D - attr_ent)/attr_H);
    let (attr_ent,attr_H) = attribute_entropy(&amenities_level_dist);
    IGR.push((info_D - attr_ent)/attr_H);

    let mut max:f64 = 0.0;
    let mut max_i:usize = 0;
    for i in 0..4 {
        if IGR[i] > max { max = IGR[i]; max_i = i; };
    }
    match max_i {
        0 => return String::from("RoomType"),
        1 => return String::from("BedRooms"),
        2 => return String::from("Popularity"),
        3 => return String::from("AmenitiesLevel"),
        _ => return String::from("Leaf")
    }
}

fn make_tree(tree_vec: &mut Vec<DecisionTreeNode>, node: &mut DecisionTreeNode, node_vec: Vec<RoomInfo>){
    let choice = chose_attribute(&node_vec);
    match &choice as &str{
        "RoomType" => {
            node.attribute = String::from("RoomType");
            let mut node1: DecisionTreeNode = DecisionTreeNode::new();
            let mut node2: DecisionTreeNode = DecisionTreeNode::new();
            let mut node3: DecisionTreeNode = DecisionTreeNode::new();
            let mut nodevec1: Vec<RoomInfo> = Vec::new();
            let mut nodevec2: Vec<RoomInfo> = Vec::new();
            let mut nodevec3: Vec<RoomInfo> = Vec::new();

            for roominfo in node_vec {
                match roominfo.room_type {
                    RoomType::PrivateRoom => nodevec1.push(roominfo),
                    RoomType::EntireHomeApt => nodevec2.push(roominfo),
                    RoomType::HotelRoom => nodevec3.push(roominfo),
                }
            }
            make_tree(tree_vec,&mut node1,nodevec1);
            tree_vec.push(node1); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node2,nodevec2);
            tree_vec.push(node2); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node3,nodevec3);
            tree_vec.push(node3); 
            node.children.push(tree_vec.len()-1);
        },
        "BedRooms" => {
            node.attribute = String::from("BedRooms");
            let mut node1: DecisionTreeNode = DecisionTreeNode::new();
            let mut node2: DecisionTreeNode = DecisionTreeNode::new();
            let mut node3: DecisionTreeNode = DecisionTreeNode::new();
            let mut node4: DecisionTreeNode = DecisionTreeNode::new();
            let mut nodevec1: Vec<RoomInfo> = Vec::new();
            let mut nodevec2: Vec<RoomInfo> = Vec::new();
            let mut nodevec3: Vec<RoomInfo> = Vec::new();
            let mut nodevec4: Vec<RoomInfo> = Vec::new();

            for roominfo in node_vec {
                match roominfo.bedrooms {
                    BedRooms::One => nodevec1.push(roominfo),
                    BedRooms::Two => nodevec2.push(roominfo),
                    BedRooms::Three_Five => nodevec3.push(roominfo),
                    BedRooms::Over_Six => nodevec4.push(roominfo),
                }
            }
            make_tree(tree_vec,&mut node1,nodevec1);
            tree_vec.push(node1); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node2,nodevec2);
            tree_vec.push(node2); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node3,nodevec3);
            tree_vec.push(node3); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node4,nodevec4);
            tree_vec.push(node4); 
            node.children.push(tree_vec.len()-1);
        },
        "Popularity" => {
            node.attribute = String::from("Popularity");
            let mut node1: DecisionTreeNode = DecisionTreeNode::new();
            let mut node2: DecisionTreeNode = DecisionTreeNode::new();
            let mut node3: DecisionTreeNode = DecisionTreeNode::new();
            let mut node4: DecisionTreeNode = DecisionTreeNode::new();
            let mut node5: DecisionTreeNode = DecisionTreeNode::new();
            let mut nodevec1: Vec<RoomInfo> = Vec::new();
            let mut nodevec2: Vec<RoomInfo> = Vec::new();
            let mut nodevec3: Vec<RoomInfo> = Vec::new();
            let mut nodevec4: Vec<RoomInfo> = Vec::new();
            let mut nodevec5: Vec<RoomInfo> = Vec::new();

            for roominfo in node_vec {
                match roominfo.popularity {
                    Popularity::Level1 => nodevec1.push(roominfo),
                    Popularity::Level2 => nodevec2.push(roominfo),
                    Popularity::Level3 => nodevec3.push(roominfo),
                    Popularity::Level4 => nodevec4.push(roominfo),
                    Popularity::Level5 => nodevec5.push(roominfo),
                }
            }
            make_tree(tree_vec,&mut node1,nodevec1);
            tree_vec.push(node1); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node2,nodevec2);
            tree_vec.push(node2); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node3,nodevec3);
            tree_vec.push(node3); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node4,nodevec4);
            tree_vec.push(node4); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node5,nodevec5);
            tree_vec.push(node5);
            node.children.push(tree_vec.len()-1);
        },
        "AmenitiesLevel" => {
            node.attribute = String::from("AmenitiesLevel");
            let mut node1: DecisionTreeNode = DecisionTreeNode::new();
            let mut node2: DecisionTreeNode = DecisionTreeNode::new();
            let mut node3: DecisionTreeNode = DecisionTreeNode::new();
            let mut node4: DecisionTreeNode = DecisionTreeNode::new();
            let mut nodevec1: Vec<RoomInfo> = Vec::new();
            let mut nodevec2: Vec<RoomInfo> = Vec::new();
            let mut nodevec3: Vec<RoomInfo> = Vec::new();
            let mut nodevec4: Vec<RoomInfo> = Vec::new();

            for roominfo in node_vec {
                match roominfo.amenities_level {
                    AmenitiesLevel::Few => nodevec1.push(roominfo),
                    AmenitiesLevel::Common => nodevec2.push(roominfo),
                    AmenitiesLevel::Abundant => nodevec3.push(roominfo),
                    AmenitiesLevel::Luxurious => nodevec4.push(roominfo),
                }
            }
            make_tree(tree_vec,&mut node1,nodevec1);
            tree_vec.push(node1); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node2,nodevec2);
            tree_vec.push(node2); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node3,nodevec3);
            tree_vec.push(node3); 
            node.children.push(tree_vec.len()-1);
            make_tree(tree_vec,&mut node4,nodevec4);
            tree_vec.push(node4); 
            node.children.push(tree_vec.len()-1);
        },
        "Under100" => node.attribute = String::from("Under100"),
        "_100_200" => node.attribute = String::from("_100_200"),
        "_200_300" => node.attribute = String::from("_200_300"),
        "_300_400" => node.attribute = String::from("_300_400"),
        "_400_500" => node.attribute = String::from("_400_500"),
        "Above500" => node.attribute = String::from("Above500"),
        _ => {}
    }
}

fn search_tree(tree_vec: &Vec<DecisionTreeNode>,id: usize,roominfo: RoomInfo) -> bool{
    let label = &tree_vec[id].attribute;
    match label as &str {
        "RoomType" => {
            match roominfo.room_type {
                RoomType::PrivateRoom => return search_tree(tree_vec,tree_vec[id].children[0],roominfo),
                RoomType::EntireHomeApt => return search_tree(tree_vec,tree_vec[id].children[1],roominfo),
                RoomType::HotelRoom => return search_tree(tree_vec,tree_vec[id].children[2],roominfo),
            }
        },
        "BedRooms" => {
            match roominfo.bedrooms {
                BedRooms::One => return search_tree(tree_vec,tree_vec[id].children[0],roominfo),
                BedRooms::Two => return search_tree(tree_vec,tree_vec[id].children[1],roominfo),
                BedRooms::Three_Five => return search_tree(tree_vec,tree_vec[id].children[2],roominfo),
                BedRooms::Over_Six => return search_tree(tree_vec,tree_vec[id].children[3],roominfo),
            }
        },
        "Popularity" => {
            match roominfo.popularity {
                Popularity::Level1 => return search_tree(tree_vec,tree_vec[id].children[0],roominfo),
                Popularity::Level2 => return search_tree(tree_vec,tree_vec[id].children[1],roominfo),
                Popularity::Level3 => return search_tree(tree_vec,tree_vec[id].children[2],roominfo),
                Popularity::Level4 => return search_tree(tree_vec,tree_vec[id].children[3],roominfo),
                Popularity::Level5 => return search_tree(tree_vec,tree_vec[id].children[4],roominfo),
            }
        },
        "AmenitiesLevel" => {
            match roominfo.amenities_level {
                AmenitiesLevel::Few => return search_tree(tree_vec,tree_vec[id].children[0],roominfo),
                AmenitiesLevel::Common => return search_tree(tree_vec,tree_vec[id].children[1],roominfo),
                AmenitiesLevel::Abundant => return search_tree(tree_vec,tree_vec[id].children[2],roominfo),
                AmenitiesLevel::Luxurious => return search_tree(tree_vec,tree_vec[id].children[3],roominfo),
            }
        }
        "Under100" => if roominfo.price == PriceRange::Under100 { return true;} else { return false},
        "_100_200" => if roominfo.price == PriceRange::_100_200 { return true;} else { return false},
        "_200_300" => if roominfo.price == PriceRange::_200_300 { return true;} else { return false},
        "_300_400" => if roominfo.price == PriceRange::_300_400 { return true;} else { return false},
        "_400_500" => if roominfo.price == PriceRange::_400_500 { return true;} else { return false},
        "Above500" => if roominfo.price == PriceRange::Above500 { return true;} else { return false},
        "Null" => return false,
        _ => {return false;}
    }
}
