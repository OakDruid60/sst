#[cfg(test)]
//mod tests {
/*
//use super::*;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

/
#[test]
fn it_fails() {
    let result = add(2, 2);
    assert_ne!(result, 5);
}

*/
#[test]
fn complete_game() {
    let mut test_cmds_vec = vec![" ".to_string()];

    //test_cmds_vec.push("save".to_string());
    test_cmds_vec.clear();
    test_cmds_vec.push("restore test_data/test01.sst".to_string());
    test_cmds_vec.push("lrs".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 2".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 4 1".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 1".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 5 1".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 2".to_string());
    //Refuel
    test_cmds_vec.push("stat".to_string());
    test_cmds_vec.push("jum sb".to_string());
    //test_cmds_vec.push("stat".to_string());
    
    //1
    test_cmds_vec.push("jum 1 3".to_string());
    test_cmds_vec.push("tor".to_string());
    //2
    test_cmds_vec.push("jum 1 2".to_string());
    test_cmds_vec.push("tor".to_string());
    //3
    test_cmds_vec.push("jum 0 1".to_string());
    test_cmds_vec.push("tor".to_string());
    //4
    test_cmds_vec.push("jum 0 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //5
    test_cmds_vec.push("jum 1 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //6
    test_cmds_vec.push("jum 2 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //7
    test_cmds_vec.push("jum 0 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //8
    test_cmds_vec.push("jum 1 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //9
    test_cmds_vec.push("jum 2 5".to_string());
    test_cmds_vec.push("tor".to_string());
    //10
    test_cmds_vec.push("jum 7 7".to_string());
    test_cmds_vec.push("tor".to_string());
    //Refuel
    test_cmds_vec.push("stat".to_string());
    test_cmds_vec.push("jum sb".to_string());
    test_cmds_vec.push("stat".to_string());

    test_cmds_vec.push("jum 1 1".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("srs".to_string());
    test_cmds_vec.push("pha".to_string());

    test_cmds_vec.push("jum 10 10".to_string());

    test_cmds_vec.push("stat".to_string());
    test_cmds_vec.push("jum sb".to_string());
    test_cmds_vec.push("stat".to_string());

    test_cmds_vec.push("jum 0 3".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 1 4".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 4".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 0".to_string());
    test_cmds_vec.push("tor".to_string());

    test_cmds_vec.push("lrs".to_string());
    //test_cmds_vec.push("save".to_string());
    test_cmds_vec.push("qui".to_string());

    super::ui::cmd_proc::command_processor(&test_cmds_vec);
    //let result = add(2, 2);
    //assert_eq!(result, 4);
}
//}
