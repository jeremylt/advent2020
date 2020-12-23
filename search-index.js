var searchIndex = JSON.parse('{\
"advent2020":{"doc":"","i":[[3,"Results","advent2020","",null,null],[12,"part_1","","",0,null],[12,"part_2","","",0,null],[12,"times","","",0,null],[3,"Timing","","",null,null],[12,"setup","","",1,null],[12,"part_1","","",1,null],[12,"part_2","","",1,null],[12,"combined","","",1,null],[5,"main","","",null,[[]]],[0,"day01","","Day 1: This solution uses a binary mask array that…",null,null],[5,"find_two","advent2020::day01","",null,[[],["option",4]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"YEAR","","",null,null],[0,"day02","advent2020","Day 2: Regex can be expensive to create. I leveraged…",null,null],[3,"PasswordData","advent2020::day02","",null,null],[12,"lower","","",2,null],[12,"upper","","",2,null],[12,"required","","",2,null],[12,"password","","",2,null],[3,"PasswordValidityData","","",null,null],[12,"part_1","","",3,null],[12,"part_2","","",3,null],[5,"part_1","","",null,[[["passworddata",3]]]],[5,"part_2","","",null,[[["passworddata",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day03","advent2020","Day 3: An important observation from today is that you get…",null,null],[5,"hit_tree","advent2020::day03","",null,[[["string",3]]]],[5,"count_trees","","",null,[[["vec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day04","advent2020","Day 4: Interestingly, in contrast to Day 3, we achieve…",null,null],[3,"PassportData","advent2020::day04","",null,null],[12,"byr","","",4,null],[12,"iyr","","",4,null],[12,"eyr","","",4,null],[12,"hgt","","",4,null],[12,"hcl","","",4,null],[12,"ecl","","",4,null],[12,"pid","","",4,null],[12,"cid","","",4,null],[12,"len","","",4,null],[3,"PassportValidityData","","",null,null],[12,"part_1","","",5,null],[12,"part_2","","",5,null],[5,"part_1","","",null,[[["passportdata",3]]]],[5,"byr_valid","","",null,[[]]],[5,"iyr_valid","","",null,[[]]],[5,"eyr_valid","","",null,[[]]],[5,"ecl_valid","","",null,[[["string",3]]]],[5,"hcl_valid","","",null,[[["string",3]]]],[5,"pid_valid","","",null,[[["string",3]]]],[5,"hgt_valid","","",null,[[["string",3]]]],[5,"part_2","","",null,[[["passportdata",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day05","advent2020","Day 5: Two separate passes are required over the data, one…",null,null],[5,"parse_fblr_binary","advent2020::day05","",null,[[],[["result",4],["parseinterror",3]]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day06","advent2020","Day 6: This is a good example of how using the bytes array…",null,null],[5,"to_array","advent2020::day06","",null,[[],[["result",4],["parseinterror",3]]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day07","advent2020","Day 7: A particularly slow day, the performance is…",null,null],[3,"Holding","advent2020::day07","",null,null],[12,"key","","",6,null],[12,"number","","",6,null],[3,"Node","","",null,null],[12,"contained_by","","",7,null],[12,"contains","","",7,null],[3,"UNIQUES","","",null,null],[12,"__private_field","","",8,null],[5,"str_to_key","","",null,[[],[["result",4],["parseinterror",3]]]],[5,"add_to_graph","","",null,[[["fxhashmap",6]]]],[5,"part_1","","",null,[[["fxhashmap",6]]]],[5,"part_2","","",null,[[["fxhashmap",6]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[11,"new","","",6,[[]]],[11,"new","","",7,[[]]],[0,"day08","advent2020","Day 8: This is a problem that is not actually dominated by…",null,null],[3,"Node","advent2020::day08","",null,null],[12,"instruction","","",9,null],[12,"value","","",9,null],[12,"increment","","",9,null],[12,"next","","",9,null],[3,"State","","",null,null],[12,"count","","",10,null],[12,"current","","",10,null],[4,"Instruction","","",null,null],[13,"Acc","","",11,null],[13,"Jmp","","",11,null],[13,"Nop","","",11,null],[5,"part_1","","",null,[[["fxhashset",6],["fxhashmap",6]]]],[5,"part_2","","",null,[[["fxhashmap",6]]]],[5,"run_program","","",null,[[["fxhashset",6],["fxhashmap",6]]]],[5,"combined","","",null,[[["fxhashmap",6]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[11,"new","","",9,[[],[["result",4],["parseinterror",3]]]],[0,"day09","advent2020","Day 9: The parts are difficult (or impossible) to combine…",null,null],[5,"find_two","advent2020::day09","",null,[[["vec",3]],["option",4]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"WINDOW","","",null,null],[0,"day10","advent2020","Day 10: The only tricks in this one are using a mask array…",null,null],[5,"run","advent2020::day10","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day11","advent2020","Day 11: This puzzle is similar to Conway\'s Game of Life.…",null,null],[5,"game_of_life","advent2020::day11","",null,[[["arrayvec",3],["arrayvec",3],["vec",3]]]],[5,"part_1","","",null,[[],["arrayvec",3]]],[5,"part_2","","",null,[[["arrayvec",3]],["arrayvec",3]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[17,"NEIGHBORS_1","","",null,null],[17,"NEIGHBORS_2","","",null,null],[0,"day12","advent2020","Day 12: Another straightforward problem today. As usual,…",null,null],[3,"Instruction","advent2020::day12","",null,null],[12,"north","","",12,null],[12,"east","","",12,null],[12,"bearing","","",12,null],[12,"forward","","",12,null],[3,"Position","","",null,null],[12,"north","","",13,null],[12,"east","","",13,null],[12,"bearing","","",13,null],[3,"PositionWaypoint","","",null,null],[12,"north","","",14,null],[12,"east","","",14,null],[12,"waypoint_north","","",14,null],[12,"waypoint_east","","",14,null],[5,"part_1","","",null,[[["position",3],["instruction",3]],["position",3]]],[5,"part_2","","",null,[[["positionwaypoint",3],["instruction",3]],["positionwaypoint",3]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day13","advent2020","Day 13: This is a fast one so long as you use the Chinese…",null,null],[3,"Bus","advent2020::day13","",null,null],[12,"id","","",15,null],[12,"time","","",15,null],[5,"euclid_inverse","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day14","advent2020","Day 14: The bitwise operations are straightforward, but…",null,null],[3,"Instructions","advent2020::day14","",null,null],[12,"set_mask","","",16,null],[12,"clear_mask","","",16,null],[12,"updates","","",16,null],[3,"Update","","",null,null],[12,"address","","",17,null],[12,"value","","",17,null],[5,"update_memory_1","","",null,[[["fxhashset",6],["instructions",3]]]],[5,"update_memory_2","","",null,[[["fxhashset",6],["instructions",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[17,"INSTRUCTIONS","","",null,null],[17,"MAX_36_BITS","","",null,null],[0,"day15","advent2020","Day 15: Nothing clever here. I am using a vec for the…",null,null],[5,"part_1","advent2020::day15","",null,[[["vec",3]]]],[5,"part_2","","",null,[[["vec",3]]]],[5,"combined","","",null,[[["vec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"YEAR","","",null,null],[17,"REALLY_BIG","","",null,null],[17,"BREAKPOINT","","",null,null],[0,"day16","advent2020","Day 16: The range checking is somewhat expensive in this…",null,null],[3,"TicketField","advent2020::day16","",null,null],[12,"name","","",18,null],[12,"lower_range","","",18,null],[12,"upper_range","","",18,null],[5,"valid_field","","",null,[[["ticketfield",3]]]],[5,"valid_fields","","",null,[[["arrayvec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"NUMBER_FIELDS","","",null,null],[17,"CAPACITY","","",null,null],[0,"day17","advent2020","Day 17: I have experimented with maintaining a hash set of…",null,null],[5,"count_neighbors_3d","advent2020::day17","",null,[[["arrayvec",3]]]],[5,"game_of_life_3d","","",null,[[["arrayvec",3]]]],[5,"count_neighbors_4d","","",null,[[["arrayvec",3]]]],[5,"game_of_life_4d","","",null,[[["arrayvec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY_3D","","",null,null],[17,"CAPACITY_4D","","",null,null],[17,"CYCLES","","",null,null],[17,"NEIGHBOR_LIMIT","","",null,null],[0,"day18","advent2020","Day 18: Part 1 was a straightforward right-to-left…",null,null],[5,"evaluate_next","advent2020::day18","",null,[[]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day19","advent2020","Day 19: CYK is a good fit here, but with the size of the…",null,null],[4,"Rule","advent2020::day19","",null,null],[13,"Terminal","","",19,null],[13,"Any","","",19,null],[13,"All","","",19,null],[5,"check_rule","","",null,[[["rule",4],["fxhashmap",6]],["result",4]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day20","advent2020","Day 20: Well then. That was hard, but the overall…",null,null],[3,"Tile","advent2020::day20","",null,null],[12,"id","","",20,null],[12,"image","","",20,null],[12,"side","","",20,null],[12,"orientation","","",20,null],[12,"edges","","",20,null],[12,"shared_edges","","",20,null],[3,"Edge","","",null,null],[12,"radians","","",21,null],[12,"antiradians","","",21,null],[4,"Side","","",null,null],[13,"Up","","",22,null],[13,"Down","","",22,null],[4,"Orientation","","",null,null],[13,"Right","","",23,null],[13,"Top","","",23,null],[13,"Left","","",23,null],[13,"Bottom","","",23,null],[5,"is_monster","","",null,[[["vec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[5,"tile_index","","",null,[[["side",4],["orientation",4]]]],[17,"TILE_SIZE","","",null,null],[17,"MAX_TILE_SIDE","","",null,null],[17,"SEA_MONSTER_SIZE","","",null,null],[17,"SEA_MONSTER_OFFSETS","","",null,null],[17,"SEA_MONSTER_INDICES","","",null,null],[17,"TILE_INDICES","","",null,null],[11,"get_right","","",20,[[]]],[11,"get_bottom","","",20,[[]]],[11,"flip","","",20,[[]]],[11,"set_left","","",20,[[]]],[11,"set_top","","",20,[[]]],[11,"set_top_left","","",20,[[]]],[0,"day21","advent2020","Day 21: No particular tricks in this one. As usual, we can…",null,null],[3,"Food","advent2020::day21","",null,null],[12,"ingredients","","",24,null],[12,"allergens","","",24,null],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[0,"day22","advent2020","Day 22: Part 1 was straighforward. I got good performance…",null,null],[4,"Winner","advent2020::day22","",null,null],[13,"Player1","","",25,null],[13,"Player2","","",25,null],[5,"score_game","","",null,[[]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"DECK_SIZE","","",null,null],[17,"MAX_ROUNDS","","",null,null],[0,"output","advent2020","Output: This module collects some of my `println!`…",null,null],[5,"print_header","advent2020::output","",null,[[]]],[5,"print_day","","",null,[[]]],[5,"print_part","","",null,[[]]],[5,"print_timing","","",null,[[["timing",3]]]],[5,"print_days_timing","","",null,[[["vec",3],["vec",3]]]],[17,"NUMBER_DASHES","","",null,null],[0,"prelude","advent2020","",null,null],[17,"REPETITIONS","","",null,null],[11,"new","","",0,[[["timing",3]]]],[11,"new","","",1,[[["duration",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"init","","",0,[[]]],[11,"deref","","",0,[[]]],[11,"deref_mut","","",0,[[]]],[11,"drop","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"init","","",1,[[]]],[11,"deref","","",1,[[]]],[11,"deref_mut","","",1,[[]]],[11,"drop","","",1,[[]]],[11,"from","advent2020::day02","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"init","","",2,[[]]],[11,"deref","","",2,[[]]],[11,"deref_mut","","",2,[[]]],[11,"drop","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"init","","",3,[[]]],[11,"deref","","",3,[[]]],[11,"deref_mut","","",3,[[]]],[11,"drop","","",3,[[]]],[11,"from","advent2020::day04","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"init","","",4,[[]]],[11,"deref","","",4,[[]]],[11,"deref_mut","","",4,[[]]],[11,"drop","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"init","","",5,[[]]],[11,"deref","","",5,[[]]],[11,"deref_mut","","",5,[[]]],[11,"drop","","",5,[[]]],[11,"from","advent2020::day07","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"init","","",6,[[]]],[11,"deref","","",6,[[]]],[11,"deref_mut","","",6,[[]]],[11,"drop","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"init","","",7,[[]]],[11,"deref","","",7,[[]]],[11,"deref_mut","","",7,[[]]],[11,"drop","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"init","","",8,[[]]],[11,"deref","","",8,[[]]],[11,"deref_mut","","",8,[[]]],[11,"drop","","",8,[[]]],[11,"from","advent2020::day08","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"init","","",9,[[]]],[11,"deref","","",9,[[]]],[11,"deref_mut","","",9,[[]]],[11,"drop","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"init","","",10,[[]]],[11,"deref","","",10,[[]]],[11,"deref_mut","","",10,[[]]],[11,"drop","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"init","","",11,[[]]],[11,"deref","","",11,[[]]],[11,"deref_mut","","",11,[[]]],[11,"drop","","",11,[[]]],[11,"from","advent2020::day12","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"init","","",12,[[]]],[11,"deref","","",12,[[]]],[11,"deref_mut","","",12,[[]]],[11,"drop","","",12,[[]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"init","","",13,[[]]],[11,"deref","","",13,[[]]],[11,"deref_mut","","",13,[[]]],[11,"drop","","",13,[[]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"init","","",14,[[]]],[11,"deref","","",14,[[]]],[11,"deref_mut","","",14,[[]]],[11,"drop","","",14,[[]]],[11,"from","advent2020::day13","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"init","","",15,[[]]],[11,"deref","","",15,[[]]],[11,"deref_mut","","",15,[[]]],[11,"drop","","",15,[[]]],[11,"from","advent2020::day14","",16,[[]]],[11,"into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"init","","",16,[[]]],[11,"deref","","",16,[[]]],[11,"deref_mut","","",16,[[]]],[11,"drop","","",16,[[]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"init","","",17,[[]]],[11,"deref","","",17,[[]]],[11,"deref_mut","","",17,[[]]],[11,"drop","","",17,[[]]],[11,"from","advent2020::day16","",18,[[]]],[11,"into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"init","","",18,[[]]],[11,"deref","","",18,[[]]],[11,"deref_mut","","",18,[[]]],[11,"drop","","",18,[[]]],[11,"from","advent2020::day19","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"init","","",19,[[]]],[11,"deref","","",19,[[]]],[11,"deref_mut","","",19,[[]]],[11,"drop","","",19,[[]]],[11,"from","advent2020::day20","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"init","","",20,[[]]],[11,"deref","","",20,[[]]],[11,"deref_mut","","",20,[[]]],[11,"drop","","",20,[[]]],[11,"from","","",21,[[]]],[11,"into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"init","","",21,[[]]],[11,"deref","","",21,[[]]],[11,"deref_mut","","",21,[[]]],[11,"drop","","",21,[[]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"init","","",22,[[]]],[11,"deref","","",22,[[]]],[11,"deref_mut","","",22,[[]]],[11,"drop","","",22,[[]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"init","","",23,[[]]],[11,"deref","","",23,[[]]],[11,"deref_mut","","",23,[[]]],[11,"drop","","",23,[[]]],[11,"from","advent2020::day21","",24,[[]]],[11,"into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"init","","",24,[[]]],[11,"deref","","",24,[[]]],[11,"deref_mut","","",24,[[]]],[11,"drop","","",24,[[]]],[11,"from","advent2020::day22","",25,[[]]],[11,"into","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"init","","",25,[[]]],[11,"deref","","",25,[[]]],[11,"deref_mut","","",25,[[]]],[11,"drop","","",25,[[]]],[11,"clone","advent2020::day08","",11,[[],["instruction",4]]],[11,"clone","advent2020::day19","",19,[[],["rule",4]]],[11,"eq","advent2020::day20","",22,[[["side",4]]]],[11,"eq","","",23,[[["orientation",4]]]],[11,"eq","advent2020::day22","",25,[[["winner",4]]]],[11,"deref","advent2020::day07","",8,[[],["rwlock",3]]],[11,"fmt","advent2020::day02","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day04","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day07","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day08","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day12","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day13","",15,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day14","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",17,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day16","",18,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day19","",19,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day20","",20,[[["formatter",3]],["result",6]]],[11,"fmt","","",22,[[["formatter",3]],["result",6]]],[11,"fmt","","",23,[[["formatter",3]],["result",6]]],[11,"fmt","","",21,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day21","",24,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day22","",25,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"from_str","advent2020::day02","",2,[[],["result",4]]],[11,"from_str","","",3,[[],["result",4]]],[11,"from_str","advent2020::day04","",4,[[],["result",4]]],[11,"from_str","","",5,[[],["result",4]]],[11,"from_str","advent2020::day12","",12,[[],["result",4]]],[11,"from_str","advent2020::day14","",16,[[],["result",4]]],[11,"from_str","","",17,[[],["result",4]]],[11,"from_str","advent2020::day16","",18,[[],["result",4]]],[11,"from_str","advent2020::day20","",20,[[],["result",4]]],[11,"from_str","advent2020::day21","",24,[[],["result",4]]],[11,"initialize","advent2020::day07","",8,[[]]]],"p":[[3,"Results"],[3,"Timing"],[3,"PasswordData"],[3,"PasswordValidityData"],[3,"PassportData"],[3,"PassportValidityData"],[3,"Holding"],[3,"Node"],[3,"UNIQUES"],[3,"Node"],[3,"State"],[4,"Instruction"],[3,"Instruction"],[3,"Position"],[3,"PositionWaypoint"],[3,"Bus"],[3,"Instructions"],[3,"Update"],[3,"TicketField"],[4,"Rule"],[3,"Tile"],[3,"Edge"],[4,"Side"],[4,"Orientation"],[3,"Food"],[4,"Winner"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);