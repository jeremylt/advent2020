var searchIndex = JSON.parse('{\
"advent2020":{"doc":"","i":[[3,"Results","advent2020","",null,null],[12,"part_1","","",0,null],[12,"part_2","","",0,null],[12,"times","","",0,null],[3,"Timing","","",null,null],[12,"setup","","",1,null],[12,"part_1","","",1,null],[12,"part_2","","",1,null],[12,"combined","","",1,null],[5,"main","","",null,[[]]],[0,"day01","","Day 1: This solution uses a binary mask array that…",null,null],[5,"find_two","advent2020::day01","",null,[[],["option",4]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"YEAR","","",null,null],[0,"day02","advent2020","Day 2: Regex can be expensive to create. I leveraged…",null,null],[3,"PasswordData","advent2020::day02","",null,null],[12,"lower","","",2,null],[12,"upper","","",2,null],[12,"required","","",2,null],[12,"password","","",2,null],[3,"PasswordValidityData","","",null,null],[12,"part_1","","",3,null],[12,"part_2","","",3,null],[5,"part_1","","",null,[[["passworddata",3]]]],[5,"part_2","","",null,[[["passworddata",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day03","advent2020","Day 3: An important observation from today is that you get…",null,null],[5,"hit_tree","advent2020::day03","",null,[[["string",3]]]],[5,"count_trees","","",null,[[["vec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day04","advent2020","Day 4: Interestingly, in contrast to Day 3, we achieve…",null,null],[3,"PassportData","advent2020::day04","",null,null],[12,"byr","","",4,null],[12,"iyr","","",4,null],[12,"eyr","","",4,null],[12,"hgt","","",4,null],[12,"hcl","","",4,null],[12,"ecl","","",4,null],[12,"pid","","",4,null],[12,"cid","","",4,null],[12,"len","","",4,null],[3,"PassportValidityData","","",null,null],[12,"part_1","","",5,null],[12,"part_2","","",5,null],[5,"part_1","","",null,[[["passportdata",3]]]],[5,"byr_valid","","",null,[[]]],[5,"iyr_valid","","",null,[[]]],[5,"eyr_valid","","",null,[[]]],[5,"ecl_valid","","",null,[[["string",3]]]],[5,"hcl_valid","","",null,[[["string",3]]]],[5,"pid_valid","","",null,[[["string",3]]]],[5,"hgt_valid","","",null,[[["string",3]]]],[5,"part_2","","",null,[[["passportdata",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day05","advent2020","Day 5: Two separate passes are required over the data, one…",null,null],[5,"parse_fblr_binary","advent2020::day05","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day06","advent2020","Day 6: This is a good example of how using the bytes array…",null,null],[5,"to_array","advent2020::day06","",null,[[]]],[5,"part_1","","",null,[[]]],[5,"part_2","","",null,[[]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day07","advent2020","Day 7: A particularly slow day, the performance is…",null,null],[3,"Holding","advent2020::day07","",null,null],[12,"key","","",6,null],[12,"number","","",6,null],[3,"Node","","",null,null],[12,"contained_by","","",7,null],[12,"contains","","",7,null],[3,"UNIQUES","","",null,null],[12,"__private_field","","",8,null],[5,"str_to_key","","",null,[[]]],[5,"add_to_graph","","",null,[[["fxhashmap",6]]]],[5,"part_1","","",null,[[["fxhashmap",6]]]],[5,"part_2","","",null,[[["fxhashmap",6]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[11,"new","","",6,[[]]],[11,"new","","",7,[[]]],[0,"day08","advent2020","Day 8: This is a problem that is not actually dominated by…",null,null],[3,"Node","advent2020::day08","",null,null],[12,"instruction","","",9,null],[12,"value","","",9,null],[12,"increment","","",9,null],[12,"next","","",9,null],[3,"State","","",null,null],[12,"count","","",10,null],[12,"current","","",10,null],[4,"Instruction","","",null,null],[13,"Acc","","",11,null],[13,"Jmp","","",11,null],[13,"Nop","","",11,null],[5,"part_1","","",null,[[["fxhashset",6],["fxhashmap",6]]]],[5,"part_2","","",null,[[["fxhashmap",6]]]],[5,"run_program","","",null,[[["fxhashset",6],["fxhashmap",6]]]],[5,"combined","","",null,[[["fxhashmap",6]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"CAPACITY","","",null,null],[11,"new","","",9,[[]]],[0,"day09","advent2020","Day 9: The parts are difficult (or impossible) to combine…",null,null],[5,"find_two","advent2020::day09","",null,[[["vec",3]],["option",4]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"WINDOW","","",null,null],[0,"day10","advent2020","Day 10: The only tricks in this one are using a mask array…",null,null],[5,"run","advent2020::day10","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[0,"day11","advent2020","Day 11:",null,null],[5,"game_of_life","advent2020::day11","",null,[[["vec",3],["vec",3]]]],[5,"part_1","","",null,[[["vec",3]],[["arrayvec",3],["vec",3]]]],[5,"part_2","","",null,[[["vec",3]],[["arrayvec",3],["vec",3]]]],[5,"run","","",null,[[],["results",3]]],[5,"report","","",null,[[["results",3]]]],[17,"NEIGHBORS","","",null,null],[0,"output","advent2020","Output: This module collects some of my `println!`…",null,null],[5,"print_header","advent2020::output","",null,[[]]],[5,"print_day","","",null,[[]]],[5,"print_part","","",null,[[]]],[5,"print_timing","","",null,[[["timing",3]]]],[5,"print_days_timing","","",null,[[["vec",3],["vec",3]]]],[17,"NUMBER_DASHES","","",null,null],[0,"prelude","advent2020","",null,null],[17,"REPETITIONS","","",null,null],[11,"new","","",0,[[["timing",3]]]],[11,"new","","",1,[[["duration",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","advent2020::day02","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","advent2020::day04","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","advent2020::day07","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","advent2020::day08","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"clone","","",11,[[],["instruction",4]]],[11,"deref","advent2020::day07","",8,[[],["rwlock",3]]],[11,"fmt","advent2020::day02","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day04","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day07","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020::day08","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","advent2020","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"from_str","advent2020::day02","",2,[[],["result",4]]],[11,"from_str","","",3,[[],["result",4]]],[11,"from_str","advent2020::day04","",4,[[],["result",4]]],[11,"from_str","","",5,[[],["result",4]]],[11,"initialize","advent2020::day07","",8,[[]]]],"p":[[3,"Results"],[3,"Timing"],[3,"PasswordData"],[3,"PasswordValidityData"],[3,"PassportData"],[3,"PassportValidityData"],[3,"Holding"],[3,"Node"],[3,"UNIQUES"],[3,"Node"],[3,"State"],[4,"Instruction"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);